use async_trait::*;
use futures::{
    future::{self, FutureExt, TryFutureExt},
    stream::{Stream, StreamExt},
};
use kson::prelude::*;
use location::*;
use std::{error::Error, fmt::Debug, io::Error as IOErr, net::SocketAddr, ops::Deref, sync::Arc};
use thiserror::Error;

#[async_trait]
pub trait KrpcServer: Sync {
    type InitError: Debug + Error + Send;
    type ConnInfo: Debug + Send + Sync;

    const MAX_CONCURRENT_REQS: usize;
    const MAX_CONCURRENT_PUSHES: usize;

    const MAX_REQ_SIZE: usize;
    const MAX_ACK_SIZE: usize;

    async fn init(
        &self,
        tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self::ConnInfo, KrpcError<Self::InitError>>;

    type Push: Ser + Send;
    type Pushes: Stream<Item = Self::Push> + Send;
    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Self::Pushes;

    type PushAck: De + Send;
    async fn on_push_ack(
        &self,
        push: Self::Push,
        ack: Self::PushAck,
    );

    type Req: De + Send;
    type Resp: Ser + Send;

    async fn handle_req(
        &self,
        conn: &Self::ConnInfo,
        req: Self::Req,
    ) -> Self::Resp;

    async fn on_close(
        &self,
        cinfo: Self::ConnInfo,
        conn: quinn::Connection,
    );
}

#[derive(Debug, Error)]
pub enum KrpcError<InitError: Error> {
    #[error("Init: {0}")]
    Init(InitError),
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
}

pub async fn handle_conn<S>(
    server: &S,
    conn: quinn::Connection,
    mut incoming: quinn::IncomingBiStreams,
) -> Result<(), KrpcError<S::InitError>>
where
    S: KrpcServer,
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
                .unwrap_or_else(|e: KrpcError<S::InitError>| eprintln!("{}", e))
        })
        .boxed();

    let req_resp = incoming
        .for_each_concurrent(S::MAX_CONCURRENT_REQS, |res| {
            async {
                let (mut tx, rx) = res?;

                let req_bytes = rx.read_to_end(S::MAX_REQ_SIZE).await?;
                let req = kson::from_bytes(req_bytes.into())?;

                let resp = server.handle_req(&cinfo, req).await;
                let rbytes = kson::to_vec(&resp);
                tx.write_all(&rbytes).await?;

                tx.finish().await?;

                Ok(())
            }
                .unwrap_or_else(|e: KrpcError<S::InitError>| eprintln!("{}", e))
        })
        .boxed();

    future::join(send_pushes, req_resp).await;
    server.on_close(cinfo, conn).await;

    Ok(())
}

pub async fn serve_static<Server: KrpcServer>(
    server: &'static Server,
    endpoint_config: quinn_proto::EndpointConfig,
    server_config: quinn::ServerConfig,
    socket: &SocketAddr,
) -> Result<(), KrpcError<Server::InitError>> {
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

pub async fn serve_arc<Server: KrpcServer + Send + 'static>(
    server: Arc<Server>,
    endpoint_config: quinn_proto::EndpointConfig,
    server_config: quinn::ServerConfig,
    socket: &SocketAddr,
) -> Result<(), KrpcError<Server::InitError>> {
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
pub trait KrpcClient: Send + Sync + Sized + 'static {
    const MAX_CONCURRENT_REQS: usize;
    const MAX_CONCURRENT_PUSHES: usize;

    const MAX_RESP_SIZE: usize;
    const MAX_PUSH_SIZE: usize;

    type Req: Ser + Send;
    type Resp: De + Send;
    type Push: De + Send;
    type PushAck: Ser + Send;

    type InitInfo: Send + Sized;
    type InitError: Debug + Error + Send;

    async fn init(
        info: Self::InitInfo,
        tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self, KrpcError<Self::InitError>>;

    async fn handle_push(
        &self,
        push: Self::Push,
    ) -> Self::PushAck;

    async fn on_resp(
        &self,
        req: &Self::Req,
        resp: &Self::Resp,
    ) -> Result<(), KrpcError<Self::InitError>>;

    fn on_close(&self);
}

pub struct Client<K: KrpcClient> {
    inner: Arc<K>,
    connection: quinn::Connection,
}

impl<K: KrpcClient> Drop for Client<K> {
    fn drop(&mut self) {
        self.inner.on_close()
    }
}

impl<K: KrpcClient> Deref for Client<K> {
    type Target = Arc<K>;

    fn deref(&self) -> &Arc<K> {
        &self.inner
    }
}

impl<K: KrpcClient> Client<K> {
    pub async fn connect(
        info: K::InitInfo,
        endpoint_config: quinn_proto::EndpointConfig,
        client_config: quinn::ClientConfig,
        client_socket: &SocketAddr,
        server_socket: &SocketAddr,
        server_name: &str,
    ) -> Result<Self, KrpcError<K::InitError>> {
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
                    .unwrap_or_else(|e: KrpcError<K::InitError>| eprintln!("{}", e))
            })
        });

        Ok(Client { inner, connection })
    }

    pub async fn req(
        &self,
        req: &K::Req,
    ) -> Result<K::Resp, KrpcError<K::InitError>> {
        let (mut tx, rx) = self.connection.open_bi().await?;
        let req_bytes = kson::to_vec(req);
        tx.write_all(&req_bytes).await?;
        tx.finish().await?;
        let resp_bytes = rx.read_to_end(K::MAX_RESP_SIZE).await?;
        let resp = kson::from_bytes(resp_bytes.into())?;
        self.on_resp(req, &resp).await?;
        Ok(resp)
    }
}
