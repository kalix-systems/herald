#![allow(unused_imports)]

use anyhow::*;
use dashmap::DashMap;
use futures::{
    future::{self, TryFutureExt},
    sink::{self, Sink, SinkExt},
    stream::{self, BoxStream, Stream, StreamExt},
};
use herald_common::{
    protocol::{auth::*, *},
    *,
};
use server_errors::Error as ServerError;
use server_store::*;
use std::{error::Error as StdError, future::Future, sync::Arc, time::Duration};
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

    pub async fn handle_login<Tx, Rx, E>(
        &self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<GlobalId, anyhow::Error>
    where
        Tx: Sink<Bytes> + Unpin,
        <Tx as Sink<Bytes>>::Error: StdError + Send + Sync + 'static,
        Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
        E: StdError + Send + Sync + 'static,
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

    pub async fn catchup<Tx, Rx, E>(
        &self,
        pk: sig::PublicKey,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<(), anyhow::Error>
    where
        Tx: Sink<Bytes> + Unpin,
        <Tx as Sink<Bytes>>::Error: StdError + Send + Sync + 'static,
        Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
        E: StdError + Send + Sync + 'static,
    {
        use catchup::{Catchup, CatchupAck};

        let pending = self.new_connection().await?.get_pending(pk).await?;
        let mut unsent = &pending[..];

        while !unsent.is_empty() {
            let to_send = &unsent[..catchup::CHUNK_SIZE];

            write_ser(tx, &Catchup::NewMessages);
            write_ser(tx, &KsonIterator::new(to_send.iter().map(|(p, _)| p))).await?;

            match read_de(rx).await? {
                CatchupAck::Success => {
                    self.new_connection()
                        .await?
                        .del_pending(
                            pk,
                            stream::iter(to_send.iter().map(|(_, id): &(Push, i64)| *id)),
                        )
                        .await?;

                    unsent = &unsent[catchup::CHUNK_SIZE..];
                }
                CatchupAck::Failure => {
                    return Err(anyhow!("catchup failed")).context("ack failure");
                }
            }
        }

        tx.send(kson::to_vec(&Catchup::Done).into()).await?;

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

impl State {
    async fn init<Tx, Rx, E>(
        &self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<GlobalId, Error>
    where
        Tx: Sink<Bytes> + Unpin,
        <Tx as Sink<Bytes>>::Error: StdError + Send + Sync + 'static,
        Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
        E: StdError + Send + Sync + 'static,
    {
        let g = self.handle_login(tx, rx).await?;
        self.catchup(g.did, tx, rx).await?;
        Ok(g)
    }

    async fn pushes(
        &self,
        meta: GlobalId,
    ) -> Result<Valved<Receiver<TaggedPush>>, Error> {
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
                if let Some((_, s)) = self.active.remove(&meta.did) {
                    s.interrupt();
                }
                Err(e)
            }
            Ok(()) => {
                sem.add_permits(1);
                if let Some(mut s) = self.active.async_get_mut(meta.did).await {
                    s.ready();
                }
                Ok(output)
            }
        }
    }

    async fn on_push_ack(
        &self,
        gid: GlobalId,
        push: TaggedPush,
    ) -> Result<(), Error> {
        self.new_connection()
            .await?
            .del_pending(gid.did, stream::once(future::ready(push.id)))
            .await?;
        Ok(())
    }

    async fn on_close(
        &self,
        gid: GlobalId,
    ) {
        if let Some((_, s)) = self.active.remove(&gid.did) {
            s.interrupt()
        }
    }
}

fn write_ser<'a, Tx: Sink<Bytes> + Unpin, T: Ser>(
    tx: &'a mut Tx,
    t: &T,
) -> impl Future<Output = Result<(), Tx::Error>> + 'a {
    tx.send(kson::to_vec(t).into())
}

async fn read_de<
    E: StdError + Send + Sync + 'static,
    Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
    T: De,
>(
    rx: &mut Rx
) -> Result<T, Error> {
    let bvec = rx.next().await.ok_or_else(|| anyhow!("stream empty"))??;
    Ok(kson::from_bytes(bvec.into())?)
}
