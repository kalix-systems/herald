use crate::prelude::*;
use bytes::Buf;
use dashmap::DashMap;
use futures::stream::*;
use server_errors::*;
use server_store::*;
// use sodiumoxide::crypto::sign;
use std::time::Duration;
use tokio::{
    prelude::*,
    sync::mpsc::{
        unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
    },
    timer::Timeout,
};

mod framed;
pub use framed::*;

#[derive(Default)]
pub struct State {
    pub active: DashMap<sig::PublicKey, Sender<()>>,
    pub pool: Pool,
}

// pub mod http;
pub mod login;
pub mod rpc_impl;

impl State {
    pub fn new() -> Self {
        State {
            active: DashMap::default(),
            pool: Pool::new(),
        }
    }

    async fn new_connection(&self) -> Result<Conn, Error> {
        Ok(self.pool.get().await?)
    }

    pub async fn handle_login<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        &'static self,
        stream: S,
    ) -> Result<(), Error> {
        let mut stream = Framed::new(stream);
        let mut store = self.new_connection().await?;

        let gid: GlobalId = login::login(&self.active, &mut store, &mut stream).await?;

        // all the channels we'll need for plumbing
        // push emitter which will be stored in the active sessions dashmap
        let (ptx, prx) = channel::<()>();
        self.active.insert(gid.did, ptx);

        // TODO: handle this error somehow?
        // for now we're just dropping it
        if catchup(gid.did, &mut store, &mut stream).await.is_ok() {
            let mut prx: Timeout<Receiver<()>> = prx.timeout(Duration::from_secs(60));
            drop(self.send_pushes(&mut stream, &mut prx, gid.did).await);
        }

        self.active.remove(&gid.did);

        Ok(())
    }

    pub async fn push_users(&self, req: push_users::Req) -> Result<push_users::Res, Error> {
        let push_users::Req { to, exc, msg } = req;
        let msg: Push = Push {
            tag: PushTag::User,
            timestamp: Time::now(),
            msg,
        };

        let mut missing_users: Vec<UserId> = Vec::new();
        let mut to_devs: Vec<sig::PublicKey> = Vec::new();
        let mut conn: Conn = self.new_connection().await?;

        for user in to {
            if !conn.user_exists(&user).await? {
                missing_users.push(user);
            } else {
                for dev in conn.valid_keys(&user).await? {
                    if dev != exc {
                        to_devs.push(dev);
                    }
                }
            }
        }

        Ok(if !missing_users.is_empty() {
            push_users::Res::Missing(missing_users)
        } else {
            self.send_push_to_devices(&mut conn, to_devs, msg).await?;
            push_users::Res::Success
        })
    }

    pub async fn push_devices(&self, req: push_devices::Req) -> Result<push_devices::Res, Error> {
        let push_devices::Req { to, msg } = req;
        let msg = Push {
            tag: PushTag::Device,
            timestamp: Time::now(),
            msg,
        };

        let mut conn = self.new_connection().await?;
        let mut missing_devs: Vec<sig::PublicKey> = Vec::new();

        for dev in to.iter() {
            if !conn.device_exists(dev).await? {
                missing_devs.push(*dev);
            }
        }

        Ok(if !missing_devs.is_empty() {
            push_devices::Res::Missing(missing_devs)
        } else {
            self.send_push_to_devices(&mut conn, to, msg).await?;
            push_devices::Res::Success
        })
    }

    async fn send_push_to_devices(
        &self,
        con: &mut Conn,
        to_devs: Vec<sig::PublicKey>,
        msg: Push,
    ) -> Result<(), Error> {
        con.add_pending(to_devs.clone(), [msg].iter()).await?;

        for dev in to_devs {
            if let Some(s) = self.active.async_get(dev).await {
                let mut sender = s.clone();
                drop(sender.send(()).await);
            }
        }

        Ok(())
    }

    async fn send_pushes<S>(
        &self,
        stream: &mut Framed<S>,
        rx: &mut Timeout<Receiver<()>>,
        did: sig::PublicKey,
    ) -> Result<(), Error>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        while let Some(p) = rx.next().await {
            if p.is_ok() {
                let mut conn = self.new_connection().await?;
                catchup(did, &mut conn, stream).await?;
            } else {
                stream
                    .write(&ServerTransmission::KeepAlive)
                    .timeout(Duration::from_secs(5))
                    .await??;
            }
        }

        Ok(())
    }
}

async fn catchup<S>(did: sign::PublicKey, s: &mut Conn, stream: &mut Framed<S>) -> Result<(), Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    use catchup::*;

    loop {
        let pending: Vec<Push> = s.get_pending(did, CHUNK_SIZE).await?;
        if pending.is_empty() {
            break;
        } else {
            let len = pending.len() as u64;
            let msg = Catchup::Messages(pending);

            loop {
                stream.write_packeted(&msg).await?;

                if CatchupAck(len) == stream.read().await? {
                    s.expire_pending(did, len as u32).await?;
                    break;
                }
            }
        }
    }

    stream.write_packeted(&Catchup::Done).await?;

    Ok(())
}
