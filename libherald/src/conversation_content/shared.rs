use super::*;
use crate::messages::MsgUpdate;
use crossbeam_channel::*;
use heraldcore::{channel_send_err, errors::HErr};
use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;

/// Concurrent hash map from `ConversationId`s to an event stream.
/// This is used to route conversation members related notifications that arrive from the network.
static RXS: OnceCell<RwLock<HashMap<ConversationId, Receiver<ContentUpdate>>>> = OnceCell::new();

/// Concurrent hash map from `ConversationId`s to a channel sendder.
/// This is used to route members related notifications that arrive from the network.
static TXS: OnceCell<RwLock<HashMap<ConversationId, Sender<ContentUpdate>>>> = OnceCell::new();
/// Concurrent hash map of `MembersEmitter`. These are removed when the associated
/// `Members` object is dropped.
static EMITTERS: OnceCell<Mutex<HashMap<ConversationId, Emitter>>> = OnceCell::new();

pub(super) fn txs() -> &'static RwLock<HashMap<ConversationId, Sender<ContentUpdate>>> {
    TXS.get_or_init(|| RwLock::new(HashMap::default()))
}

pub(super) fn rxs() -> &'static RwLock<HashMap<ConversationId, Receiver<ContentUpdate>>> {
    RXS.get_or_init(|| RwLock::new(HashMap::default()))
}

pub(super) fn emitters() -> &'static Mutex<HashMap<ConversationId, Emitter>> {
    EMITTERS.get_or_init(|| Mutex::new(HashMap::default()))
}

/// An update to a conversation's message model or members model
#[derive(Debug)]
pub(crate) enum ContentUpdate {
    /// Messages model update
    Msg(MsgUpdate),
    /// Members model update
    Member(MemberUpdate),
}

impl From<MemberUpdate> for ContentUpdate {
    fn from(update: MemberUpdate) -> ContentUpdate {
        ContentUpdate::Member(update)
    }
}

impl From<MsgUpdate> for ContentUpdate {
    fn from(update: MsgUpdate) -> ContentUpdate {
        ContentUpdate::Msg(update)
    }
}

pub(crate) fn content_push<T: Into<ContentUpdate>>(
    cid_addr: ConversationId,
    update: T,
) -> Result<(), HErr> {
    let maybe_tx = {
        let lock = txs().read();
        let maybe = lock.get(&cid_addr).cloned();
        drop(lock);
        maybe
    };

    let new_channel = || {
        let (tx, rx) = unbounded();

        {
            let mut tx_lock = txs().write();
            tx_lock.insert(cid_addr, (&tx).clone());
            drop(tx_lock);
        }

        {
            let mut rx_lock = rxs().write();
            rx_lock.insert(cid_addr, rx);
            drop(rx_lock);
        }

        tx
    };

    let tx = match maybe_tx {
        Some(tx) => tx,
        None => new_channel(),
    };

    if let Err(update) = tx.send(update.into()) {
        let tx = new_channel();
        tx.send(update.into_inner())
            .map_err(|_| channel_send_err!())?;
    }

    let mut emit_lock = emitters().lock();
    let mut maybe_emitter = emit_lock.get_mut(&cid_addr).map(|e| e.clone());
    drop(emit_lock);

    if let Some(emitter) = maybe_emitter.as_mut() {
        emitter.try_poll();
    }

    Ok(())
}

impl super::ConversationContent {
    pub(super) fn process_updates(&mut self) -> Option<()> {
        let id = self.id?;

        for update in rxs().read().get(&id)?.try_iter() {
            use ContentUpdate::*;
            match update {
                Msg(update) => {
                    self.messages.process_update(update);
                }
                Member(update) => {
                    self.members.process_update(update);
                }
            }
        }

        Some(())
    }

    #[must_use]
    pub(super) fn register_model(&mut self) -> Option<()> {
        let id = self.id?;

        emitters().lock().insert(id, self.emit.clone());

        Some(())
    }
}

impl Drop for ConversationContent {
    fn drop(&mut self) {
        use shared::*;
        if let Some(cid) = self.id {
            let mut tx_lock = txs().write();
            let mut rx_lock = rxs().write();
            let mut emitter_lock = emitters().lock();

            emitter_lock.remove(&cid);
            tx_lock.remove(&cid);
            rx_lock.remove(&cid);
        }
    }
}
