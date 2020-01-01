#![allow(unused_imports)]

use dashmap::DashMap;
use futures::{
    future::{self, TryFutureExt},
    stream::{self, BoxStream, Stream, StreamExt},
};
use herald_common::{
    protocol::{auth::*, *},
    *,
};
use krpc::*;
use server_errors::Error as ServerError;
use server_store::*;
use std::sync::Arc;
use std::time::Duration;
use stream_cancel::{Trigger, Tripwire, Valved};
use tokio::{
    prelude::*,
    sync::{
        mpsc::{
            unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
        },
        oneshot, Semaphore,
    },
    time,
};

mod handlers;
mod login;

pub struct ActiveSession {
    interrupt: Trigger,
    emitter: Sender<TaggedPush>,
    ready: Option<Arc<Semaphore>>,
}

impl ActiveSession {
    pub fn interrupt(self) {
        self.interrupt.cancel()
    }

    pub async fn push(
        &self,
        push: Push,
        id: i64,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<TaggedPush>> {
        if let Some(s) = &self.ready {
            let _guard = s.acquire().await;
            self.emitter.clone().send(TaggedPush { push, id })
        } else {
            self.emitter.clone().send(TaggedPush { push, id })
        }
    }

    pub fn ready(&mut self) {
        self.ready = None;
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
        self.pool
            .get()
            .await
            .context("failed to get connection to postgres")
    }

    pub async fn handle_login<Tx, Rx>(
        &self,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<GlobalId, anyhow::Error>
    where
        Tx: AsyncWrite + Unpin,
        Rx: AsyncRead + Unpin,
    {
        let mut state = AuthState::AwaitMethod;
        loop {
            match state {
                AuthState::Done(g) => {
                    return Ok(g);
                }
                s => {
                    state = self.auth_transition(s, tx, rx).await?;
                }
            }
        }
    }

    pub async fn catchup<Tx, Rx>(
        &self,
        pk: sig::PublicKey,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<(), anyhow::Error>
    where
        Tx: AsyncWrite + Unpin,
        Rx: AsyncRead + Unpin,
    {
        const CATCHUP_CHUNK_SIZE: usize = 100;
        let pending = self.new_connection().await?.get_pending(pk).await?;
        let mut unsent = &pending[..];

        while !unsent.is_empty() {
            let to_send = &unsent[..CATCHUP_CHUNK_SIZE];

            tx.write_u8(0).await?;
            tx.write_ser(&KsonIterator::new(
                to_send.iter().map(|(p, _): &(Push, i64)| p),
            ))
            .await?;
            if rx.read_u8().await? != 1 {
                Err(anyhow!("catchup failed").context(format!("{:x?}", pk)))?;
            }
            self.new_connection()
                .await?
                .del_pending(
                    pk,
                    stream::iter(to_send.iter().map(|(_, id): &(Push, i64)| *id)),
                )
                .await?;

            unsent = &unsent[CATCHUP_CHUNK_SIZE..];
        }

        tx.write_u8(1).await?;

        Ok(())
    }

    pub async fn reset(&self) -> Result<(), Error> {
        self.new_connection().await?.reset_all().await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TaggedPush {
    id: i64,
    push: Push,
}

impl From<TaggedPush> for Push {
    fn from(t: TaggedPush) -> Push {
        t.push
    }
}

#[async_trait]
impl KrpcServer<HeraldProtocol> for State {
    type ConnInfo = GlobalId;

    async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        &self,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self::ConnInfo, Error> {
        let g = self.handle_login(tx, rx).await?;
        self.catchup(g.did, tx, rx).await?;
        Ok(g)
    }

    type ServePush = TaggedPush;
    type Pushes = Valved<Receiver<TaggedPush>>;

    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Result<Self::Pushes, Error> {
        let (sender, receiver) = channel();
        let (interrupt, output) = Valved::new(receiver);
        let sem = Arc::new(Semaphore::new(0));
        let sess = ActiveSession {
            interrupt,
            emitter: sender.clone(),
            ready: Some(sem.clone()),
        };
        self.active.insert(meta.did, sess);

        let catchup = async {
            for (push, id) in self.new_connection().await?.get_pending(meta.did).await? {
                sender.send(TaggedPush { push, id })?;
            }
            Ok(())
        };

        match catchup.await {
            Err(e) => {
                sem.add_permits(usize::max_value());
                self.active.remove(&meta.did).map(|(_, s)| s.interrupt());
                Err(e)
            }
            Ok(()) => {
                sem.add_permits(1);
                self.active.async_get_mut(meta.did).await.map(|mut s| {
                    s.ready();
                });
                Ok(output)
            }
        }
    }

    async fn on_push_ack(
        &self,
        gid: &Self::ConnInfo,
        push: Self::ServePush,
        _ack: PushAck,
    ) -> Result<(), Error> {
        self.new_connection()
            .await?
            .del_pending(gid.did, stream::once(future::ready(push.id)))
            .await?;
        Ok(())
    }

    async fn handle_req(
        &self,
        gid: &Self::ConnInfo,
        req: Request,
    ) -> Response {
        let res_fut = async {
            Ok(match req {
                Request::GetSigchain(u) => Response::GetSigchain(self.get_sigchain(u).await?),
                Request::RecipExists(r) => Response::RecipExists(self.recip_exists(r).await?),
                Request::NewSig(n) => Response::NewSig(self.new_sig(n).await?),
                Request::NewPrekey(keys) => Response::NewPrekey(self.new_prekeys(keys).await?),
                Request::GetPrekey(keys) => Response::GetPrekey(self.get_prekeys(keys).await?),
                Request::Push(push::Req { to, msg }) => {
                    Response::Push(self.send_push(*gid, to, msg).await?)
                }
            })
        };
        res_fut
            .unwrap_or_else(|e: Error| Response::Err(format!("{}", e)))
            .await
    }

    async fn on_close(
        &self,
        gid: Self::ConnInfo,
    ) {
        self.active.remove(&gid.did).map(|(_, s)| s.interrupt());
    }
}
