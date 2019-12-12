use async_trait::*;
use futures::{future::*, stream::*};
use kson::prelude::*;
use location::*;
use std::{error::Error, fmt::Debug, io::Error as IOErr};
use thiserror::Error;

#[async_trait]
pub trait KrpcServer {
    type InitError: Debug + Error;
    type ConnInfo: Debug;

    const MAX_CONCURRENT_REQS: usize;
    const MAX_CONCURRENT_PUSHES: usize;

    const MAX_REQ_SIZE: usize;
    const MAX_ACK_SIZE: usize;

    async fn init(
        &self,
        tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self::ConnInfo, Self::InitError>;

    type Push: Ser;
    type Pushes: Stream<Item = Self::Push> + Unpin;
    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Self::Pushes;

    type PushAck: De;
    async fn on_push_ack(
        &self,
        push: Self::Push,
        ack: Self::PushAck,
    );

    type Req: De;
    type Resp: Ser;

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
    WriteError(#[from] quinn::WriteError),
    #[error("Error reading from stream: {0}")]
    ReadExactError(#[from] quinn::ReadToEndError),
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

    let send_pushes =
        server
            .pushes(&cinfo)
            .await
            .for_each_concurrent(Server::MAX_CONCURRENT_PUSHES, |push| {
                let fut = async {
                    let (mut push_tx, push_rx) = conn.open_bi().await?;
                    let bytes = kson::to_vec(&push);

                    push_tx.write_all(&bytes).await?;
                    push_tx.finish();

                    let ack_bytes = push_rx.read_to_end(Server::MAX_ACK_SIZE).await?;
                    let ack = kson::from_bytes(ack_bytes.into())?;

                    server.on_push_ack(push, ack).await;

                    Ok::<(), ServerError<Server::InitError>>(())
                };

                async {
                    if let Err(e) = fut.await {
                        // TODO: do something better here
                        eprintln!(
                            "error sending push along connection {:?}, error was:\n{}",
                            cinfo, e
                        );
                    }
                }
            });

    let req_resp = incoming.for_each_concurrent(Server::MAX_CONCURRENT_REQS, |res| {
        let fut = async {
            let (mut tx, rx) = res?;

            let req_bytes = rx.read_to_end(Server::MAX_REQ_SIZE).await?;
            let req = kson::from_bytes(req_bytes.into())?;

            let resp = server.handle_req(&cinfo, req).await;
            let rbytes = kson::to_vec(&resp);
            tx.write_all(&rbytes).await?;

            tx.finish().await?;

            Ok::<(), ServerError<Server::InitError>>(())
        };

        async {
            if let Err(e) = fut.await {
                // TODO: do something better here
                eprintln!("error in connection {:?}, error was:\n{}", cinfo, e);
            }
        }
    });

    join(send_pushes, req_resp).await;
    server.on_close(cinfo, conn).await;

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
