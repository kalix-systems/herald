use super::*;
use std::{marker::PhantomData, ops::Deref, sync::Arc};

pub async fn handle_conn<S, P>(
    server: &S,
    conn: quinn::Connection,
    mut incoming: quinn::IncomingBiStreams,
) -> Result<(), Error>
where
    S: KrpcServer<P>,
    P: Protocol,
{
    let (tx, rx) = incoming
        .next()
        .await
        .ok_or_else(|| anyhow!("didn't receive handshake stream: {}", loc!()))??;

    let mut handshake_tx = Framed::new(tx);
    let mut handshake_rx = Framed::new(rx);
    let cinfo = server.init(&mut handshake_tx, &mut handshake_rx).await?;

    let send_pushes = server
        .pushes(&cinfo)
        .await
        .for_each_concurrent(P::MAX_CONCURRENT_PUSHES, |push| {
            async {
                let (mut push_tx, push_rx) = conn.open_bi().await?;
                let bytes = kson::to_vec(&push);

                push_tx.write_all(&bytes).await?;
                push_tx.finish().await?;

                let ack_bytes = push_rx.read_to_end(P::MAX_ACK_SIZE).await?;
                let ack = kson::from_bytes(ack_bytes.into())?;

                server.on_push_ack(push, ack).await;

                Ok(())
            }
            .unwrap_or_else(|e: Error| eprintln!("{}", e))
        })
        .boxed();

    let req_res = incoming
        .for_each_concurrent(P::MAX_CONCURRENT_REQS, |res| {
            async {
                let (mut tx, rx) = res?;

                let req_bytes = rx.read_to_end(P::MAX_REQ_SIZE).await?;
                let req = kson::from_bytes(req_bytes.into())?;

                let res = server.handle_req(&cinfo, req).await;
                let rbytes = kson::to_vec(&res);
                tx.write_all(&rbytes).await?;

                tx.finish().await?;

                Ok(())
            }
            .unwrap_or_else(|e: Error| eprintln!("{}", e))
        })
        .boxed();

    future::join(send_pushes, req_res).await;
    server.on_close(cinfo).await;

    Ok(())
}

pub async fn serve_static<S, P>(
    server: &'static S,
    endpoint_config: quinn_proto::EndpointConfig,
    server_config: quinn::ServerConfig,
    socket: &SocketAddr,
) -> Result<(), Error>
where
    S: KrpcServer<P> + Send + Sync,
    P: Protocol,
{
    let mut endpoint_builder = quinn::EndpointBuilder::new(endpoint_config);
    endpoint_builder.listen(server_config);
    let (driver, _endpoint, listener) = endpoint_builder.bind(socket)?;

    let serve_conns = listener.for_each(move |conn| {
        tokio::spawn(
            async move {
                let quinn::NewConnection {
                    driver,
                    connection,
                    bi_streams,
                    ..
                } = conn.await?;

                future::try_join(
                    driver.map_err(|e| e.into()),
                    handle_conn(server, connection, bi_streams),
                )
                .map_ok(drop)
                .await
            }
            .unwrap_or_else(|e| eprintln!("{}", e)),
        );
        future::ready(())
    });

    future::try_join(driver, serve_conns.map(Ok)).await?;

    Ok(())
}

pub async fn serve_arc<S, P>(
    server: Arc<S>,
    endpoint_config: quinn_proto::EndpointConfig,
    server_config: quinn::ServerConfig,
    socket: &SocketAddr,
) -> Result<(), Error>
where
    S: KrpcServer<P> + Send + Sync + 'static,
    P: Protocol,
{
    let mut endpoint_builder = quinn::EndpointBuilder::new(endpoint_config);
    endpoint_builder.listen(server_config);
    let (driver, _endpoint, listener) = endpoint_builder.bind(socket)?;

    let serve_conns = listener.for_each(move |conn| {
        tokio::spawn({
            let server = server.clone();
            async move {
                let quinn::NewConnection {
                    driver,
                    connection,
                    bi_streams,
                    ..
                } = conn.await?;

                future::try_join(
                    driver.map_err(|e| e.into()),
                    handle_conn(server.deref(), connection, bi_streams),
                )
                .map_ok(drop)
                .await
            }
            .unwrap_or_else(|e| eprintln!("{}", e))
        });
        future::ready(())
    });

    future::try_join(driver, serve_conns.map(Ok)).await?;

    Ok(())
}

pub struct Client<P, K>
where
    P: Protocol,
    K: KrpcClient<P>,
{
    phantom: PhantomData<P>,
    inner: Arc<K>,
    connection: quinn::Connection,
}

impl<P: Protocol, K: KrpcClient<P>> Drop for Client<P, K> {
    fn drop(&mut self) {
        self.inner.on_close()
    }
}

impl<P: Protocol, K: KrpcClient<P>> Deref for Client<P, K> {
    type Target = Arc<K>;

    fn deref(&self) -> &Arc<K> {
        &self.inner
    }
}

impl<P: Protocol, K: KrpcClient<P>> Client<P, K> {
    pub async fn connect(
        info: K::InitInfo,
        endpoint_config: quinn_proto::EndpointConfig,
        client_config: quinn::ClientConfig,
        client_socket: &SocketAddr,
        server_socket: &SocketAddr,
        server_name: &str,
    ) -> Result<Self, Error> {
        let mut endpoint_builder = quinn::EndpointBuilder::new(endpoint_config);
        endpoint_builder.default_client_config(client_config);
        let (driver, endpoint, _incoming) = endpoint_builder.bind(client_socket)?;

        tokio::spawn(driver.unwrap_or_else(|e| eprintln!("{}", e)));

        let quinn::NewConnection {
            driver,
            connection,
            bi_streams,
            ..
        } = endpoint.connect(server_socket, server_name)?.await?;

        tokio::spawn(driver.unwrap_or_else(|e| eprintln!("{}", e)));

        let (tx, rx) = connection.open_bi().await?;
        let mut handshake_tx = Framed::new(tx);
        let mut handshake_rx = Framed::new(rx);
        let inner = Arc::new(K::init(info, &mut handshake_tx, &mut handshake_rx).await?);

        tokio::spawn({
            let k = inner.clone();
            bi_streams.for_each_concurrent(P::MAX_CONCURRENT_PUSHES, move |conn| {
                let k = k.clone();
                async move {
                    let (mut tx, rx) = conn?;

                    let push_bytes = rx.read_to_end(P::MAX_PUSH_SIZE).await?;
                    let push = kson::from_bytes(push_bytes.into())?;

                    let ack = k.handle_push(push).await;

                    let ack_bytes = kson::to_vec(&ack);
                    tx.write_all(&ack_bytes).await?;
                    tx.finish().await?;

                    Ok(())
                }
                .unwrap_or_else(|e: Error| eprintln!("{}", e))
            })
        });

        Ok(Client {
            inner,
            connection,
            phantom: PhantomData,
        })
    }

    pub async fn req(
        &self,
        req: &P::Req,
    ) -> Result<P::Res, Error> {
        let (mut tx, rx) = self.connection.open_bi().await?;
        let req_bytes = kson::to_vec(req);
        tx.write_all(&req_bytes).await?;
        tx.finish().await?;
        let res_bytes = rx.read_to_end(P::MAX_RESP_SIZE).await?;
        let res = kson::from_bytes(res_bytes.into())?;
        self.on_res(req, &res).await?;
        Ok(res)
    }
}
