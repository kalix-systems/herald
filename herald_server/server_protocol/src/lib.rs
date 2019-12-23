use dashmap::DashMap;
use futures::stream::*;
use herald_common::*;
use server_errors::*;
use server_store::*;
use std::time::Duration;
use stream_cancel::{Trigger, Tripwire, Valved};
use tokio::{
    prelude::*,
    sync::{
        mpsc::{
            unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
        },
        oneshot,
    },
};

pub struct ActiveSession {
    interrupt: Trigger,
    done: Tripwire,
    emitter: Sender<Push>,
}

impl ActiveSession {
    #[allow(clippy::unneeded_field_pattern)]
    pub async fn interrupt(self) -> bool {
        let ActiveSession {
            interrupt,
            done,
            emitter: _,
        } = self;

        interrupt.cancel();
        done.await
    }

    pub fn push(
        &self,
        push: &Push,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Push>> {
        self.emitter.clone().send(push.clone())
    }
}

type ActiveSessions = DashMap<sig::PublicKey, ActiveSession>;

#[derive(Default)]
pub struct State {
    pub active: ActiveSessions,
    pub pool: Pool,
}

impl State {
    pub fn new() -> Self {
        State {
            active: DashMap::default(),
            pool: Pool::new(),
        }
    }

    pub async fn new_connection(&self) -> Result<Conn, Error> {
        Ok(self.pool.get().await?)
    }

    // pub async fn handle_login(
    //     &'static self,
    //     ws: WebSocket,
    // ) -> Result<(), Error> {
    //     let mut store: Conn = self.new_connection().await?;

    //     // all the channels we'll need for plumbing
    //     // first we split the websocket
    //     let (mut wtx, mut wrx) = ws.split();
    //     // bytevec messages received from the socket
    //     let (mut rtx, mut rrx) = channel::<Vec<u8>>();
    //     // push emitter which will be stored in the active sessions dashmap
    //     let (ptx, prx) = channel::<()>();
    //     // session-close emitter
    //     let (close, closed) = oneshot::channel::<()>();

    //     // on graceful exit, notify runtime to close channel
    //     // we set things up this way so that the rrx channel
    //     // will be populated before we call login, hence before
    //     // we know the gid
    //     tokio::spawn(async move {
    //         while let Some(Ok(m)) = wrx.next().await {
    //             if m.is_close() || (m.is_binary() && rtx.send(m.into_bytes()).await.is_err()) {
    //                 break;
    //             }
    //         }

    //         close.send(()).ok();
    //     });

    //     let gid: GlobalId = login::login(&self.active, &mut store, &mut wtx, &mut rrx).await?;

    //     let (interrupt, prx) = Valved::new(prx);
    //     let (trigger_done, done) = Tripwire::new();
    //     let sess = ActiveSession {
    //         interrupt,
    //         done,
    //         emitter: ptx,
    //     };
    //     self.active.insert(gid.did, sess);

    //     // remove active session on graceful exit
    //     tokio::spawn(async move {
    //         drop(closed.await);
    //         if let Some((_, s)) = self.active.remove(&gid.did) {
    //             s.interrupt().await;
    //         }
    //     });

    //     // TODO: handle this error somehow?
    //     // for now we're just dropping it
    //     if catchup(gid.did, &mut store, &mut wtx, &mut rrx)
    //         .await
    //         .is_ok()
    //     {
    //         let mut prx: Timeout<Valved<Receiver<()>>> = prx.timeout(Duration::from_secs(60));
    //         drop(
    //             self.send_pushes(&mut store, &mut wtx, &mut rrx, &mut prx, gid.did)
    //                 .await,
    //         );
    //         trigger_done.cancel();
    //     }

    //     if let Some((_, s)) = self.active.remove(&gid.did) {
    //         s.interrupt().await;
    //     }

    //     Ok(())
    // }

    // async fn send_pushes(
    //     &self,
    //     store: &mut Conn,
    //     wtx: &mut WTx,
    //     rrx: &mut Receiver<Vec<u8>>,
    //     rx: &mut Timeout<Valved<Receiver<()>>>,
    //     did: sig::PublicKey,
    // ) -> Result<(), Error> {
    //     while let Some(p) = rx.next().await {
    //         if p.is_ok() {
    //             catchup(did, store, wtx, rrx).await?;
    //         } else {
    //             wtx.send(ws::Message::ping(vec![0u8]))
    //                 .timeout(Duration::from_secs(5))
    //                 .await??;
    //         }
    //     }

    //     Ok(())
    // }
}

// async fn catchup(
//     did: sig::PublicKey,
//     s: &mut Conn,
//     wtx: &mut WTx,
//     rrx: &mut Receiver<Vec<u8>>,
// ) -> Result<(), Error> {
//     use catchup::*;

//     loop {
//         let pending: Vec<Push> = s.get_pending(did, CHUNK_SIZE).await?;
//         if pending.is_empty() {
//             break;
//         } else {
//             let len = pending.len() as u64;
//             let msg = Catchup::Messages(pending);

//             loop {
//                 write_msg(&msg, wtx, rrx).await?;

//                 if CatchupAck(len) == read_msg(rrx).await? {
//                     s.expire_pending(did, len as u32).await?;
//                     break;
//                 }
//             }
//         }
//     }

//     write_msg(&Catchup::Done, wtx, rrx).await?;

//     Ok(())
// }

// const TIMEOUT_DUR: std::time::Duration = Duration::from_secs(10);

// async fn read_msg<T>(rx: &mut Receiver<Vec<u8>>) -> Result<T, Error>
// where
//     T: De,
// {
//     let m = rx.next().await.ok_or(StreamDied)?;
//     let t = kson::from_slice(&m)?;
//     Ok(t)
// }

// fn ser_msg<T: Ser>(t: &T) -> ws::Message {
//     ws::Message::binary(kson::to_vec(t))
// }

// async fn write_msg<T>(
//     t: &T,
//     wtx: &mut WTx,
//     rrx: &mut Receiver<Vec<u8>>,
// ) -> Result<(), Error>
// where
//     T: Ser,
// {
//     let bvec = Bytes::from(kson::to_vec(t));
//     let packets = Packet::from_bytes(bvec);
//     let len = packets.len() as u64;

//     loop {
//         wtx.send(ser_msg(&len)).timeout(TIMEOUT_DUR).await??;

//         if len == read_msg::<u64>(rrx).timeout(TIMEOUT_DUR).await?? {
//             wtx.send(ser_msg(&PacketResponse::Success))
//                 .timeout(TIMEOUT_DUR)
//                 .await??;
//             break;
//         } else {
//             wtx.send(ser_msg(&PacketResponse::Retry))
//                 .timeout(TIMEOUT_DUR)
//                 .await??;
//         }
//     }

//     loop {
//         for packet in packets.iter() {
//             wtx.send(ser_msg(packet)).timeout(TIMEOUT_DUR).await??;
//         }

//         match read_msg(rrx).timeout(TIMEOUT_DUR).await?? {
//             PacketResponse::Success => break,
//             PacketResponse::Retry => {}
//         }
//     }

//     Ok(())
// }
