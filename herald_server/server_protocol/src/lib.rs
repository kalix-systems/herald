#![allow(unused_imports)]

use anyhow::*;
use dashmap::DashMap;
use futures::stream::{self, Stream, StreamExt};
use herald_common::protocol::auth::*;
use herald_common::*;
use krpc::*;
use server_errors::Error as ServerError;
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

mod handlers;
mod login;

pub struct ActiveSession {
    interrupt: Trigger,
    done: Tripwire,
    emitter: Sender<(Push, i64)>,
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
        push: Push,
        id: i64,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<(Push, i64)>> {
        self.emitter.clone().send((push, id))
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
        let mut pending = self.new_connection().await?.get_pending(pk).await?;

        while !pending.is_empty() {
            let rest = pending.split_off(CATCHUP_CHUNK_SIZE);

            tx.write_ser(&KsonIterator::new(
                pending.iter().map(|(p, _): &(Push, i64)| p),
            ))
            .await?;
            if rx.read_u8().await? != 1 {
                Err(anyhow!("catchup failed").context(format!("{:x?}", pk)))?;
            }
            self.new_connection()
                .await?
                .del_pending(
                    pk,
                    stream::iter(pending.iter().map(|(_, id): &(Push, i64)| *id)),
                )
                .await?;

            pending = rest;
        }

        Ok(())
    }
}
