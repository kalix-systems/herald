use crate::{prelude::*, store::*};
use bytes::Buf;
use dashmap::DashMap;
use sodiumoxide::crypto::sign;
use tokio::{
    prelude::*,
    sync::mpsc::{unbounded_channel as channel, UnboundedSender as Sender},
};
use warp::{filters::ws, Filter};

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

    pub async fn handle_login(&self, mut ws: warp::filters::ws::WebSocket) -> Result<(), Error> {
        let mut con = self.new_connection()?;
        let gid = login::login(&mut con, &mut ws).await?;
        self.add_active(gid.did, &mut ws).await?;
        Ok(())
    }

    pub async fn add_active<W, E>(&self, did: sig::PublicKey, ws: &mut W) -> Result<(), Error>
    where
        W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
        Error: From<E>,
    {
        let mut store = self.new_connection()?;
        let (sender, mut receiver) = channel();
        self.active.insert(did, sender);
        // TODO: handle this error somehow?
        // for now we're just dropping it
        if catchup(did, &mut store, ws).await.is_ok() {
            drop(self.send_pushes(ws, &mut receiver, did).await);
            archive_pushes(&mut store, receiver, did).await?;
        } else {
            self.active.remove(&did);
            archive_pushes(&mut store, receiver, did).await?;
        }

        Ok(())
    }

    pub async fn push_users(&self, req: push_users::Req) -> Result<push_users::Res, Error> {
        let push_users::Req { to, msg } = req;
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
                to_devs.extend_from_slice(&con.valid_keys(&user)?);
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
                // TODO: handle this error?
                drop(sender.send(msg.clone()).await);
            } else {
                to_pending.push(dev);
            }
        }

        con.add_pending(to_pending, msg)?;

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

    async fn send_pushes<Tx, E, Rx>(
        &self,
        tx: &mut Tx,
        rx: &mut Rx,
        did: sig::PublicKey,
    ) -> Result<(), Error>
    where
        Tx: Sink<ws::Message, Error = E> + Unpin,
        Error: From<E>,
        Rx: Stream<Item = Push> + Unpin,
    {
        while let Some(p) = rx.next().await {
            match tx.send(ws::Message::binary(serde_cbor::to_vec(&p)?)).await {
                Ok(_) => {}
                Err(_) => {
                    // TODO: figure out a better way to cause this to happen
                    // probably involves splitting the sender
                    self.active.remove(&did);
                    break;
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
        store.add_pending(vec![to], p)?;
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
