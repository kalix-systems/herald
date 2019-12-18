use async_trait::*;
use futures::{
    future::{self, FutureExt, TryFutureExt},
    stream::{Stream, StreamExt},
};
use kson::prelude::*;
use location::*;
use std::{
    fmt::Debug, io::Error as IOErr, marker::PhantomData, net::SocketAddr, ops::Deref, sync::Arc,
};
use thiserror::Error;

pub trait Protocol {
    type Req: Ser + De + Send;
    type Res: Ser + De + Send;
    type Push: Ser + De + Send;
    type PushAck: Ser + De + Send;
}

#[async_trait]
pub trait KrpcServer<P: Protocol>: Sync {
    type ConnInfo: Debug + Send + Sync;

    const MAX_CONCURRENT_REQS: usize;
    const MAX_CONCURRENT_PUSHES: usize;

    const MAX_REQ_SIZE: usize;
    const MAX_ACK_SIZE: usize;

    async fn init(
        &self,
        tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self::ConnInfo, KrpcError>;

    type Pushes: Stream<Item = P::Push> + Send;
    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Self::Pushes;

    async fn on_push_ack(
        &self,
        push: P::Push,
        ack: P::PushAck,
    );

    async fn handle_req(
        &self,
        conn: &Self::ConnInfo,
        req: P::Req,
    ) -> P::Res;

    async fn on_close(
        &self,
        cinfo: Self::ConnInfo,
        conn: quinn::Connection,
    );
}

#[derive(Debug, Error)]
pub enum KrpcError {
    #[error("Connection: {0}")]
    Connection(#[from] quinn::ConnectionError),
    #[error("Connect: {0}")]
    Connect(#[from] quinn::ConnectError),
    #[error("Error writing to stream: {0}")]
    Write(#[from] quinn::WriteError),
    #[error("Error reading from stream: {0}")]
    ReadToEnd(#[from] quinn::ReadToEndError),
    #[error("Error setting up endpoint: {0}")]
    Endpoint(#[from] quinn::EndpointError),
    #[error("IO: {0}")]
    IO(#[from] IOErr),
    #[error("Kson: {0}")]
    Kson(#[from] KsonError),
    #[error("Special: {0}")]
    Special(String, Location),
    #[error("Dyn: {0}")]
    Dyn(#[from] Box<dyn std::error::Error + Send>),
}

pub async fn handle_conn<S, P>(
    server: &S,
    conn: quinn::Connection,
    mut incoming: quinn::IncomingBiStreams,
) -> Result<(), KrpcError>
where
    S: KrpcServer<P>,
    P: Protocol,
{
    let (handshake_tx, handshake_rx) = incoming
        .next()
        .await
        .ok_or_else(|| KrpcError::Special("didn't receive handshake stream".into(), loc!()))??;

    let cinfo = server.init(handshake_tx, handshake_rx).await?;

    let send_pushes = server
        .pushes(&cinfo)
        .await
        .for_each_concurrent(S::MAX_CONCURRENT_PUSHES, |push| {
            async {
                let (mut push_tx, push_rx) = conn.open_bi().await?;
                let bytes = kson::to_vec(&push);

                push_tx.write_all(&bytes).await?;
                push_tx.finish().await?;

                let ack_bytes = push_rx.read_to_end(S::MAX_ACK_SIZE).await?;
                let ack = kson::from_bytes(ack_bytes.into())?;

                server.on_push_ack(push, ack).await;

                Ok(())
            }
                .unwrap_or_else(|e: KrpcError| eprintln!("{}", e))
        })
        .boxed();

    let req_res = incoming
        .for_each_concurrent(S::MAX_CONCURRENT_REQS, |res| {
            async {
                let (mut tx, rx) = res?;

                let req_bytes = rx.read_to_end(S::MAX_REQ_SIZE).await?;
                let req = kson::from_bytes(req_bytes.into())?;

                let res = server.handle_req(&cinfo, req).await;
                let rbytes = kson::to_vec(&res);
                tx.write_all(&rbytes).await?;

                tx.finish().await?;

                Ok(())
            }
                .unwrap_or_else(|e: KrpcError| eprintln!("{}", e))
        })
        .boxed();

    future::join(send_pushes, req_res).await;
    server.on_close(cinfo, conn).await;

    Ok(())
}

pub async fn serve_static<S, P>(
    server: &'static S,
    endpoint_config: quinn_proto::EndpointConfig,
    server_config: quinn::ServerConfig,
    socket: &SocketAddr,
) -> Result<(), KrpcError>
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
) -> Result<(), KrpcError>
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

#[async_trait]
pub trait KrpcClient<P: Protocol>: Send + Sync + Sized + 'static {
    const MAX_CONCURRENT_REQS: usize;
    const MAX_CONCURRENT_PUSHES: usize;

    const MAX_RESP_SIZE: usize;
    const MAX_PUSH_SIZE: usize;

    type InitInfo: Send + Sized;

    async fn init(
        info: Self::InitInfo,
        tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self, KrpcError>;

    async fn handle_push(
        &self,
        push: P::Push,
    ) -> P::PushAck;

    async fn on_res(
        &self,
        req: &P::Req,
        res: &P::Res,
    ) -> Result<(), KrpcError>;

    fn on_close(&self);
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
    ) -> Result<Self, KrpcError> {
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

        let (handshake_tx, handshake_rx) = connection.open_bi().await?;

        let inner = Arc::new(K::init(info, handshake_tx, handshake_rx).await?);

        tokio::spawn({
            let k = inner.clone();
            bi_streams.for_each_concurrent(K::MAX_CONCURRENT_PUSHES, move |conn| {
                let k = k.clone();
                async move {
                    let (mut tx, rx) = conn?;

                    let push_bytes = rx.read_to_end(K::MAX_PUSH_SIZE).await?;
                    let push = kson::from_bytes(push_bytes.into())?;

                    let ack = k.handle_push(push).await;

                    let ack_bytes = kson::to_vec(&ack);
                    tx.write_all(&ack_bytes).await?;
                    tx.finish().await?;

                    Ok(())
                }
                    .unwrap_or_else(|e: KrpcError| eprintln!("{}", e))
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
    ) -> Result<P::Res, KrpcError> {
        let (mut tx, rx) = self.connection.open_bi().await?;
        let req_bytes = kson::to_vec(req);
        tx.write_all(&req_bytes).await?;
        tx.finish().await?;
        let res_bytes = rx.read_to_end(K::MAX_RESP_SIZE).await?;
        let res = kson::from_bytes(res_bytes.into())?;
        self.on_res(req, &res).await?;
        Ok(res)
    }
}
