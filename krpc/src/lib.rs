use async_trait::*;
use futures::{future::*, stream::*};
use kson::prelude::*;
use std::io::Error as IOErr;
use tokio::prelude::*;

#[async_trait]
pub trait MuxConn<Tx, Rx>
where
    Tx: AsyncWrite,
    Rx: AsyncRead,
{
    type Error;

    async fn create_chan(&self) -> Result<(Tx, Rx), Self::Error>;
    async fn accept_chan(&self) -> Result<(Tx, Rx), Self::Error>;
}

#[async_trait]
pub trait KrpcServer<Req: De, Resp: Ser, Push: Ser> {
    type InitError;
    type ConnInfo;
    type Tx: AsyncWrite + Unpin;
    type Rx: AsyncRead + Unpin;

    async fn init(
        &self,
        tx: Self::Tx,
        rx: Self::Rx,
    ) -> Result<Self::ConnInfo, Self::InitError>;

    type Pushes: Stream<Item = Vec<Push>> + Unpin;
    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Self::Pushes;

    async fn on_pushes_sent(
        &self,
        pushes: Vec<Push>,
    );

    async fn handle_req(
        &self,
        conn: &Self::ConnInfo,
        req: Req,
    ) -> Resp;

    async fn on_close(
        &self,
        conn: Self::ConnInfo,
    );
}

pub async fn handle_conn<Req, Resp, Push, Server, Conn, Error>(
    server: &Server,
    conn: &Conn,
) -> Result<(), Error>
where
    Req: De,
    Resp: Ser,
    Push: Ser,
    Server: KrpcServer<Req, Resp, Push>,
    Conn: MuxConn<Server::Tx, Server::Rx>,
    Error: From<Server::InitError> + From<Conn::Error> + From<IOErr> + From<KsonError>,
{
    let (handshake_tx, handshake_rx) = conn.accept_chan().await?;
    let cinfo = server.init(handshake_tx, handshake_rx).await?;

    let send_pushes = async {
        let mut pushes = server.pushes(&cinfo).await;

        let (mut push_tx, mut push_rx) = conn.create_chan().await?;

        while let Some(pushes) = pushes.next().await {
            let bytes = kson::to_vec(&pushes);
            let len = bytes.len() as u64;
            push_tx.write_all(&len.to_le_bytes()).await?;
            push_tx.write_all(&bytes).await?;
            push_rx.read_exact(&mut [0]).await?;
            server.on_pushes_sent(pushes).await;
        }

        Ok(())
    };

    let req_resp = async {
        loop {
            let (mut tx, mut rx) = conn.accept_chan().await?;

            let mut len_buf = [0u8; 4];
            rx.read_exact(&mut len_buf).await?;
            let len = u32::from_le_bytes(len_buf) as usize;

            let mut req_buf = vec![0u8; len];
            rx.read_exact(&mut req_buf).await?;

            let as_req = kson::from_bytes(req_buf.into())?;
            let resp = server.handle_req(&cinfo, as_req).await;

            let rbytes = kson::to_vec(&resp);
            let len_bytes = (rbytes.len() as u64).to_le_bytes();

            tx.write_all(&len_bytes).await?;
            tx.write_all(&rbytes).await?;
        }
    };

    let _: Result<((), ()), Error> = try_join(send_pushes, req_resp).await;

    server.on_close(cinfo).await;

    Ok(())
}

#[async_trait]
pub trait KrpcClient<Req: Ser, Resp: De, Push: De> {
    type InitError;
    type ConnInfo;

    async fn init<Tx, Rx>(
        &self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<Self::ConnInfo, Self::InitError>
    where
        Tx: AsyncWrite + Unpin,
        Rx: AsyncRead + Unpin;

    type PushError;
    async fn handle_push(
        &self,
        conn: &Self::ConnInfo,
        push: Push,
    ) -> Result<(), Self::PushError>;

    async fn on_close(
        &self,
        conn: Self::ConnInfo,
    );
}
