use crate::{prelude::*, store::*};
use bytes::Buf;
use dashmap::DashMap;
use futures::compat::*;
use sodiumoxide::{crypto::sign, randombytes::randombytes_into};
use std::convert::TryInto;
use tokio::sync::mpsc::{
    unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use warp::{
    filters::{body, path, ws},
    Filter,
};

pub struct State {
    pub active: DashMap<sig::PublicKey, Sender<Push>>,
    pub pool: Pool,
}

pub mod get;
pub mod login;
pub mod post;

impl State {
    pub fn new() -> Self {
        State {
            active: DashMap::default(),
            pool: init_pool(),
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
            // TODO: maybe handle this one too?
            // again just dropping it since the flow must go on
            drop(send_pushes(ws, &mut receiver).await);
        }
        self.active.remove(&did);
        archive_pushes(&mut store, receiver, did).await?;

        Ok(())
    }

    pub async fn send_push(&self, req: push::Req) -> Result<push::Res, Error> {
        use push::*;
        let Req {
            to_users,
            mut to_devs,
            msg,
        } = req;

        let mut missing_users = Vec::new();
        let mut missing_devs = Vec::new();
        let mut con = self.new_connection()?;

        for device in to_devs.iter() {
            if !con.key_is_valid(*device)? {
                missing_devs.push(*device);
            }
        }

        for user in to_users {
            if con.user_exists(&user)? {
                for key in con.read_meta(&user)?.valid_keys() {
                    to_devs.push(key);
                }
            } else {
                missing_users.push(user);
            }
        }

        if missing_users.is_empty() && missing_devs.is_empty() {
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

            Ok(Res::Success)
        } else {
            Ok(Res::Missing(missing_users, missing_devs))
        }
    }
}

async fn send_pushes<Tx, E, Rx>(tx: &mut Tx, rx: &mut Rx) -> Result<(), Error>
where
    Tx: Sink<ws::Message, Error = E> + Unpin,
    Error: From<E>,
    Rx: Stream<Item = Push> + Unpin,
{
    while let Some(p) = rx.next().await {
        tx.send(ws::Message::binary(serde_cbor::to_vec(&p)?))
            .await?;
    }
    Ok(())
}

async fn archive_pushes<S, Rx>(store: &mut S, mut rx: Rx, to: sig::PublicKey) -> Result<(), Error>
where
    S: Store,
    Rx: Stream<Item = Push> + Unpin,
{
    while let Some(p) = rx.next().await {
        store.add_pending(vec![to], p)?;
    }
    Ok(())
}

async fn catchup<S, W, E>(did: sign::PublicKey, s: &mut S, ws: &mut W) -> Result<(), Error>
where
    S: Store,
    W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
    Error: From<E>,
{
    use catchup::*;
    let pending = s.get_pending(did)?;

    // TCP over TCP...
    for chunk in pending.chunks(CHUNK_SIZE) {
        // TODO: remove unnecessary memcpy here by using a draining chunk iterator?
        let msg = Catchup(Vec::from(chunk));
        loop {
            ws.send(ws::Message::binary(serde_cbor::to_vec(&msg)?))
                .await?;

            let m = ws.next().await.ok_or(CatchupFailed)??;

            if CatchupAck(chunk.len() as u64) == serde_cbor::from_slice(m.as_bytes())? {
                break;
            }
        }
    }

    s.expire_pending(did)?;

    Ok(())
}
