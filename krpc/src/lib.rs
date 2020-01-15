pub use anyhow::*;
pub use kson::prelude::*;
pub use kson_channel::*;
use std::fmt::Debug;

#[cfg(feature = "quic")]
pub mod quic;

#[cfg(feature = "ws")]
pub mod ws;

pub trait Protocol {
    type Req: Ser + De + Send + Sync + Debug + 'static;
    type Res: Ser + De + Send + Sync + Debug + 'static;
    type Push: Ser + De + Send + Sync + Debug + 'static;
    type PushAck: Ser + De + Send + Sync + Debug + 'static;

    const MAX_CONCURRENT_REQS: usize;
    const MAX_CONCURRENT_PUSHES: usize;

    const MAX_REQ_SIZE: usize;
    const MAX_ACK_SIZE: usize;

    const MAX_RESP_SIZE: usize;
    const MAX_PUSH_SIZE: usize;
}

#[cfg(any(feature = "ws", feature = "quic", feature = "async"))]
pub use asynchronous::*;

#[cfg(any(feature = "ws", feature = "quic", feature = "async"))]
mod asynchronous {
    use super::*;

    pub use async_trait::*;
    pub use futures::stream::Stream;
    pub use tokio::prelude::*;

    #[async_trait]
    pub trait KrpcServer<P: Protocol>: Sync {
        type ConnInfo: Debug + Send + Sync;

        type ServePush: Send + Sync;
        type Pushes: Stream<Item = Self::ServePush> + Send + Unpin;

        async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
            &self,
            tx: &mut Framed<Tx>,
            rx: &mut Framed<Rx>,
        ) -> Result<Self::ConnInfo, Error>;

        async fn pushes(
            &self,
            meta: &Self::ConnInfo,
        ) -> Result<Self::Pushes, Error>;

        async fn on_push_ack(
            &self,
            meta: &Self::ConnInfo,
            push: Self::ServePush,
            ack: P::PushAck,
        ) -> Result<(), Error>;

        async fn handle_req(
            &self,
            conn: &Self::ConnInfo,
            req: P::Req,
        ) -> P::Res;

        async fn on_close(
            &self,
            cinfo: Self::ConnInfo,
        );
    }

    #[async_trait]
    pub trait KrpcClient<P: Protocol>: Send + Sync + Sized + 'static {
        type InitInfo: Send + Sized;

        async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
            info: Self::InitInfo,
            tx: &mut Framed<Tx>,
            rx: &mut Framed<Rx>,
        ) -> Result<Self, Error>;

        async fn handle_push(
            &self,
            push: P::Push,
        ) -> P::PushAck;

        async fn on_res(
            &self,
            req: &P::Req,
            res: &P::Res,
        ) -> Result<(), Error>;

        fn on_close(&self);
    }
}

pub trait SyncClient<P: Protocol>: Sized {
    type InitInfo;

    fn init<Tx: std::io::Write, Rx: std::io::Read>(
        info: Self::InitInfo,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error>;

    fn handle_push(
        &self,
        push: P::Push,
    ) -> P::PushAck;

    fn on_res(
        &self,
        req: &P::Req,
        res: &P::Res,
    ) -> Result<(), Error>;

    fn on_close(&self);
}
