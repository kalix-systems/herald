use async_trait::*;
use futures::{
    future::{self, FutureExt, TryFutureExt},
    stream::*,
};
use kson::prelude::*;
use location::*;
use std::{error::Error, fmt::Debug, io::Error as IOErr};
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
    ) -> Result<Self::ConnInfo, Self::InitError>;

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
pub enum ServerError<InitError: Error> {
    #[error("Init: {0}")]
    Init(InitError),
    #[error("Connection: {0}")]
    Connection(#[from] quinn::ConnectionError),
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

pub async fn handle_conn<Server>(
    server: &Server,
    conn: quinn::Connection,
    mut incoming: quinn::IncomingBiStreams,
) -> Result<(), ServerError<Server::InitError>>
where
    Server: KrpcServer,
{
    let (handshake_tx, handshake_rx) = incoming
        .next()
        .await
        .ok_or_else(|| ServerError::Special("didn't receive handshake stream".into(), loc!()))??;

    let cinfo = server
        .init(handshake_tx, handshake_rx)
        .await
        .map_err(ServerError::Init)?;

    let send_pushes = server
        .pushes(&cinfo)
        .await
        .for_each_concurrent(Server::MAX_CONCURRENT_PUSHES, |push| {
            async {
                let (mut push_tx, push_rx) = conn.open_bi().await?;
                let bytes = kson::to_vec(&push);

                push_tx.write_all(&bytes).await?;
                push_tx.finish();

                let ack_bytes = push_rx.read_to_end(Server::MAX_ACK_SIZE).await?;
                let ack = kson::from_bytes(ack_bytes.into())?;

                server.on_push_ack(push, ack).await;

                Ok::<(), ServerError<Server::InitError>>(())
            }
                .unwrap_or_else(|e| eprintln!("{}", e))
        })
        .boxed();

    let req_resp = incoming
        .for_each_concurrent(Server::MAX_CONCURRENT_REQS, |res| {
            async {
                let (mut tx, rx) = res?;

                let req_bytes = rx.read_to_end(Server::MAX_REQ_SIZE).await?;
                let req = kson::from_bytes(req_bytes.into())?;

                let resp = server.handle_req(&cinfo, req).await;
                let rbytes = kson::to_vec(&resp);
                tx.write_all(&rbytes).await?;

                tx.finish().await?;

                Ok::<(), ServerError<Server::InitError>>(())
            }
                .unwrap_or_else(|e| eprintln!("{}", e))
        })
        .boxed();

    future::join(send_pushes, req_resp).await;
    server.on_close(cinfo, conn).await;

    Ok(())
}

pub async fn serve<Server: KrpcServer>(
    server: &'static Server,
    endpoint_config: quinn_proto::EndpointConfig,
    server_config: quinn::ServerConfig,
    socket: &std::net::SocketAddr,
) -> Result<(), ServerError<Server::InitError>> {
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

// #[async_trait]
// pub trait KrpcClient {
//     type Req: Ser;
//     type Resp: De;
//     type Push: De;

//     type InitError;
//     type ConnInfo;

//     async fn init<Tx, Rx>(
//         &self,
//         tx: &mut Tx,
//         rx: &mut Rx,
//     ) -> Result<Self::ConnInfo, Self::InitError>
//     where
//         Tx: AsyncWrite + Unpin,
//         Rx: AsyncRead + Unpin;

//     type PushError;
//     async fn handle_push(
//         &self,
//         conn: &Self::ConnInfo,
//         push: Push,
//     ) -> Result<(), Self::PushError>;

//     async fn on_close(
//         &self,
//         conn: Self::ConnInfo,
//     );
// }
