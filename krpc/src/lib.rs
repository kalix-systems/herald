pub use anyhow::*;
pub use kson::prelude::*;
pub use kson_channel::*;
use std::fmt::Debug;

#[cfg(feature = "quic")]
pub mod quic;

#[cfg(feature = "ws-sync")]
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
pub use self::asynchronous::*;

#[cfg(any(feature = "ws", feature = "quic", feature = "async"))]
mod asynchronous;

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
