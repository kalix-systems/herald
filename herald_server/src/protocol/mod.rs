use crate::{prelude::*, store::*};
use bytes::Buf;
use dashmap::DashMap;
use sodiumoxide::crypto::sign;
use std::time::Duration;
use tokio::{
    prelude::*,
    sync::mpsc::{
        unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
    },
    timer::Timeout,
};
use warp::{
    filters::ws::{self, WebSocket},
    Filter,
};

pub struct State {
    pub active: DashMap<sig::PublicKey, Sender<Push>>,
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

    pub async fn handle_login(&'static self, mut ws: WebSocket) -> Result<(), Error> {
        let mut con = self.new_connection()?;
        let gid = login::login(&mut con, &mut ws).await?;
        self.add_active(gid.did, ws).await?;
        Ok(())
    }

    pub async fn add_active(
        &'static self,
        did: sig::PublicKey,
        mut ws: WebSocket,
    ) -> Result<(), Error> {
        let mut store = self.new_connection()?;
        let (sender, receiver) = channel();
        self.active.insert(did, sender);
        // TODO: handle this error somehow?
        // for now we're just dropping it
        if catchup(did, &mut store, &mut ws).await.is_ok() {
            let mut receiver = receiver.timeout(Duration::from_secs(60));
            drop(
                self.send_pushes(&mut store, &mut ws, &mut receiver, did)
                    .await,
            );
            archive_pushes(&mut store, receiver.into_inner(), did).await?;
        } else {
            self.active.remove(&did);
            archive_pushes(&mut store, receiver, did).await?;
        }

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
        let mut to_pending = Vec::new();

        for dev in to_devs {
            if let Some(s) = self.active.async_get(dev).await {
                let mut sender = s.clone();
                if sender.send(msg.clone()).await.is_err() {
                    to_pending.push(dev);
                }
            } else {
                to_pending.push(dev);
            }
        }

        con.add_pending(to_pending, [msg].iter())?;

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
        ws: &mut WebSocket,
        rx: &mut Timeout<Receiver<Push>>,
        did: sig::PublicKey,
    ) -> Result<(), Error> {
        while let Some(p) = rx.next().await {
            match p {
                Ok(p) => {
                    if write_msg(&p, ws).await.is_err() {
                        self.active.remove(&did);
                        store.add_pending(vec![did], [p].iter())?;
                        break;
                    }
                }
                Err(_) => {
                    ws.send(ws::Message::ping(vec![0u8]))
                        .timeout(Duration::from_secs(5))
                        .await??;
                }
            }
        }

        Ok(())
    }
}

async fn archive_pushes<Rx>(store: &mut Conn, mut rx: Rx, to: sig::PublicKey) -> Result<(), Error>
where
    Rx: Stream<Item = Push> + Unpin,
{
    while let Some(p) = rx.next().await {
        // TODO: handle this error, add the rest?
        store.add_pending(vec![to], [p].iter())?;
    }
    Ok(())
}

async fn catchup<W, E>(did: sign::PublicKey, s: &mut Conn, ws: &mut W) -> Result<(), Error>
where
    W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
    Error: From<E>,
{
    use catchup::*;
    let pending = s.get_pending(did)?;

    // TCP over TCP...
    for chunk in pending.chunks(CHUNK_SIZE) {
        // TODO: remove unnecessary memcpy here by using a draining chunk iterator?
        let msg = Catchup::Messages(Vec::from(chunk));
        loop {
            ws.send(ws::Message::binary(serde_cbor::to_vec(&msg)?))
                .await?;

            let m = ws.next().await.ok_or(CatchupFailed)??;

            if CatchupAck(chunk.len() as u64) == serde_cbor::from_slice(m.as_bytes())? {
                break;
            }
        }
    }

    ws.send(ws::Message::binary(serde_cbor::to_vec(&Catchup::Done)?))
        .await?;

    s.expire_pending(did)?;

    Ok(())
}

const TIMEOUT_DUR: std::time::Duration = Duration::from_secs(10);

async fn read_msg<T>(ws: &mut WebSocket) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let m = ws.next().await.ok_or(LoginFailed)??;
    let t = serde_cbor::from_slice::<T>(m.as_bytes())?;
    Ok(t)
}

fn ser_msg<T: Serialize>(t: &T) -> Result<ws::Message, Error> {
    Ok(ws::Message::binary(serde_cbor::to_vec(t)?))
}

async fn write_msg<T>(t: &T, ws: &mut WebSocket) -> Result<(), Error>
where
    T: Serialize,
{
    let bvec = serde_cbor::to_vec(t)?;
    let packets = Packet::from_slice(&bvec);
    let len = packets.len() as u64;

    loop {
        ws.send(ser_msg(&len)?).timeout(TIMEOUT_DUR).await??;

        if len == read_msg::<u64>(ws).timeout(TIMEOUT_DUR).await?? {
            ws.send(ser_msg(&PacketResponse::Success)?)
                .timeout(TIMEOUT_DUR)
                .await??;
            break;
        } else {
            ws.send(ser_msg(&PacketResponse::Retry)?)
                .timeout(TIMEOUT_DUR)
                .await??;
        }
    }

    loop {
        for packet in packets.iter() {
            ws.send(ser_msg(packet)?).timeout(TIMEOUT_DUR).await??;
        }

        match read_msg(ws).timeout(TIMEOUT_DUR).await?? {
            PacketResponse::Success => break,
            PacketResponse::Retry => {}
        }
    }

    Ok(())
}
