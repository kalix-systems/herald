use crate::{prelude::*, store::*};
use bytes::Buf;
use dashmap::DashMap;
use futures::stream::*;
use sodiumoxide::crypto::sign;
use std::time::Duration;
use tokio::{
    prelude::*,
    sync::{
        mpsc::{
            unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
        },
        oneshot,
    },
    timer::Timeout,
};
use warp::{
    filters::ws::{self, WebSocket},
    Filter,
};

type WTx = SplitSink<WebSocket, ws::Message>;

pub struct State {
    pub active: DashMap<sig::PublicKey, Sender<()>>,
    pub pool: Pool,
}

pub mod get;
pub mod http;
pub mod login;
pub mod post;

impl State {
    pub fn new() -> Self {
        println!("starting pool");
        let pool = init_pool();
        println!("pool started");
        State {
            active: DashMap::default(),
            pool,
        }
    }

    fn new_connection(&self) -> Result<Conn, Error> {
        // TODO add error type
        Ok(Conn(self.pool.get().unwrap()))
    }

    pub async fn handle_login(&'static self, ws: WebSocket) -> Result<(), Error> {
        let mut store = self.new_connection()?;

        // all the channels we'll need for plumbing
        // first we split the websocket
        let (mut wtx, mut wrx) = ws.split();
        // bytevec messages received from the socket
        let (mut rtx, mut rrx) = channel();
        // push emitter which will be stored in the active sessions dashmap
        let (ptx, prx) = channel();
        // session-close emitter
        let (close, closed) = oneshot::channel();

        // on graceful exit, notify runtime to close channel
        // we set things up this way so that the rrx channel
        // will be populated before we call login, hence before
        // we know the gid
        tokio::spawn(async move {
            while let Some(Ok(m)) = wrx.next().await {
                if m.is_close() {
                    break;
                } else if m.is_binary() {
                    if rtx.send(m.into_bytes()).await.is_err() {
                        break;
                    }
                }
            }

            drop(close.send(()));
        });

        let gid = login::login(&self.active, &mut store, &mut wtx, &mut rrx).await?;

        self.active.insert(gid.did, ptx);

        // remove active session on graceful exit
        tokio::spawn(async move {
            drop(closed.await);
            self.active.remove(&gid.did);
        });

        // TODO: handle this error somehow?
        // for now we're just dropping it
        if catchup(gid.did, &mut store, &mut wtx, &mut rrx)
            .await
            .is_ok()
        {
            let mut prx = prx.timeout(Duration::from_secs(60));
            drop(
                self.send_pushes(&mut store, &mut wtx, &mut rrx, &mut prx, gid.did)
                    .await,
            );
        }

        self.active.remove(&gid.did);

        Ok(())
    }

    pub async fn push_users(&self, req: push_users::Req) -> Result<push_users::Res, Error> {
        let push_users::Req { to, exc, msg } = req;
        let msg = Push {
            tag: PushTag::User,
            timestamp: Utc::now(),
            msg,
        };

        let mut missing_users = Vec::new();
        let mut to_devs = Vec::new();
        let mut con = self.new_connection()?;

        for user in to {
            if !con.user_exists(&user)? {
                missing_users.push(user);
            } else {
                for dev in con.valid_keys(&user)? {
                    if dev != exc {
                        to_devs.push(dev);
                    }
                }
            }
        }

        Ok(if !missing_users.is_empty() {
            push_users::Res::Missing(missing_users)
        } else {
            self.send_push_to_devices(&mut con, to_devs, msg).await?;
            push_users::Res::Success
        })
    }

    pub async fn push_devices(&self, req: push_devices::Req) -> Result<push_devices::Res, Error> {
        let push_devices::Req { to, msg } = req;
        let msg = Push {
            tag: PushTag::Device,
            timestamp: Utc::now(),
            msg,
        };

        let mut con = self.new_connection()?;
        let mut missing_devs = Vec::new();

        for dev in to.iter() {
            if !con.device_exists(dev)? {
                missing_devs.push(*dev);
            }
        }

        Ok(if !missing_devs.is_empty() {
            push_devices::Res::Missing(missing_devs)
        } else {
            self.send_push_to_devices(&mut con, to, msg).await?;
            push_devices::Res::Success
        })
    }

    async fn send_push_to_devices(
        &self,
        con: &mut Conn,
        to_devs: Vec<sig::PublicKey>,
        msg: Push,
    ) -> Result<(), Error> {
        con.add_pending(to_devs.clone(), [msg].iter())?;

        for dev in to_devs {
            if let Some(s) = self.active.async_get(dev).await {
                let mut sender = s.clone();
                drop(sender.send(()).await);
            }
        }

        Ok(())
    }

    pub(crate) fn req_handler<B, I, O, F>(&self, req: B, f: F) -> Result<Vec<u8>, Error>
    where
        B: Buf,
        I: for<'a> Deserialize<'a>,
        O: Serialize,
        F: FnOnce(&mut Conn, I) -> Result<O, Error>,
    {
        let mut con = self.new_connection()?;
        let buf: Vec<u8> = req.collect();
        let req = serde_cbor::from_slice(&buf)?;
        let res = f(&mut con, req)?;
        let res_ser = serde_cbor::to_vec(&res)?;
        Ok(res_ser)
    }

    pub(crate) async fn req_handler_async<'a, B, I, O, F, Fut>(
        &'a self,
        req: B,
        f: F,
    ) -> Result<Vec<u8>, Error>
    where
        B: Buf,
        I: for<'b> Deserialize<'b>,
        O: Serialize,
        F: FnOnce(&'a Self, I) -> Fut,
        Fut: Future<Output = Result<O, Error>>,
    {
        let buf: Vec<u8> = req.collect();
        let req = serde_cbor::from_slice(&buf)?;
        let res = f(self, req).await?;
        let res_ser = serde_cbor::to_vec(&res)?;
        Ok(res_ser)
    }

    async fn send_pushes(
        &self,
        store: &mut Conn,
        wtx: &mut WTx,
        rrx: &mut Receiver<Vec<u8>>,
        rx: &mut Timeout<Receiver<()>>,
        did: sig::PublicKey,
    ) -> Result<(), Error> {
        while let Some(p) = rx.next().await {
            if p.is_ok() {
                catchup(did, store, wtx, rrx).await?;
            } else {
                wtx.send(ws::Message::ping(vec![0u8]))
                    .timeout(Duration::from_secs(5))
                    .await??;
            }
        }

        Ok(())
    }
}

async fn catchup(
    did: sign::PublicKey,
    s: &mut Conn,
    wtx: &mut WTx,
    rrx: &mut Receiver<Vec<u8>>,
) -> Result<(), Error> {
    use catchup::*;

    loop {
        let pending = s.get_pending(did, CHUNK_SIZE)?;
        if pending.is_empty() {
            break;
        } else {
            let len = pending.len() as u64;
            let msg = Catchup::Messages(pending);

            loop {
                write_msg(&msg, wtx, rrx).await?;

                if CatchupAck(len) == read_msg(rrx).await? {
                    s.expire_pending(did, CHUNK_SIZE)?;
                    break;
                }
            }
        }
    }

    write_msg(&Catchup::Done, wtx, rrx).await?;

    Ok(())
}

const TIMEOUT_DUR: std::time::Duration = Duration::from_secs(10);

async fn read_msg<T>(rx: &mut Receiver<Vec<u8>>) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let m = rx.next().await.ok_or(StreamDied)?;
    let t = serde_cbor::from_slice(&m)?;
    Ok(t)
}

fn ser_msg<T: Serialize>(t: &T) -> Result<ws::Message, Error> {
    Ok(ws::Message::binary(serde_cbor::to_vec(t)?))
}

async fn write_msg<T>(t: &T, wtx: &mut WTx, rrx: &mut Receiver<Vec<u8>>) -> Result<(), Error>
where
    T: Serialize,
{
    let bvec = Bytes::from(serde_cbor::to_vec(t)?);
    let packets = Packet::from_bytes(bvec);
    let len = packets.len() as u64;

    loop {
        wtx.send(ser_msg(&len)?).timeout(TIMEOUT_DUR).await??;

        if len == read_msg::<u64>(rrx).timeout(TIMEOUT_DUR).await?? {
            wtx.send(ser_msg(&PacketResponse::Success)?)
                .timeout(TIMEOUT_DUR)
                .await??;
            break;
        } else {
            wtx.send(ser_msg(&PacketResponse::Retry)?)
                .timeout(TIMEOUT_DUR)
                .await??;
        }
    }

    loop {
        for packet in packets.iter() {
            wtx.send(ser_msg(packet)?).timeout(TIMEOUT_DUR).await??;
        }

        match read_msg(rrx).timeout(TIMEOUT_DUR).await?? {
            PacketResponse::Success => break,
            PacketResponse::Retry => {}
        }
    }

    Ok(())
}
