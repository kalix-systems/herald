use super::*;
use crate::messages::MsgUpdate;
use crossbeam_channel::*;
use heraldcore::{channel_send_err, errors::HErr};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Concurrent hash map from `ConversationId`s to an event stream.
/// This is used to route conversation members related notifications that arrive from the network.
static RXS: OnceCell<RwLock<HashMap<ConversationId, Receiver<ContentUpdate>>>> = OnceCell::new();

/// Concurrent hash map from `ConversationId`s to a channel sendder.
/// This is used to route members related notifications that arrive from the network.
static TXS: OnceCell<RwLock<HashMap<ConversationId, Sender<ContentUpdate>>>> = OnceCell::new();
/// Concurrent hash map of `MembersEmitter`. These are removed when the associated
/// `Members` object is dropped.
static EMITTERS: OnceCell<RwLock<HashMap<ConversationId, Emitter>>> = OnceCell::new();

pub(super) fn txs() -> &'static RwLock<HashMap<ConversationId, Sender<ContentUpdate>>> {
    TXS.get_or_init(|| RwLock::new(HashMap::default()))
}

pub(super) fn rxs() -> &'static RwLock<HashMap<ConversationId, Receiver<ContentUpdate>>> {
    RXS.get_or_init(|| RwLock::new(HashMap::default()))
}

pub(super) fn emitters() -> &'static RwLock<HashMap<ConversationId, Emitter>> {
    EMITTERS.get_or_init(|| RwLock::new(HashMap::default()))
}

/// An update to a conversation's message model or members model
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
    to: ConversationId,
    update: T,
) -> Result<(), HErr> {
    let maybe_tx = { txs().read().get(&to).cloned() };

    let tx = match maybe_tx {
        Some(tx) => tx,
        None => {
            let (tx, rx) = unbounded();

            {
                let mut tx_lock = txs().write();
                tx_lock.insert(to, (&tx).clone());
            }

            {
                let mut rx_lock = rxs().write();
                rx_lock.insert(to, rx);
            }

            tx
        }
    };

    tx.send(update.into()).map_err(|_| channel_send_err!())?;

    if let Some(emitter) = emitters().write().get_mut(&to) {
        emitter.new_data_ready();
    }

    Ok(())
}

pub(super) fn more_updates(cid: &ConversationId) -> bool {
    match rxs().read().get(cid) {
        Some(rx) => !rx.is_empty(),
        None => false,
    }
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

        emitters().write().insert(id, self.emit.clone());

        Some(())
    }
}

impl Drop for ConversationContent {
    fn drop(&mut self) {
        use shared::*;
        if let Some(cid) = self.id {
            emitters().write().remove(&cid);
            txs().write().remove(&cid);
            rxs().write().remove(&cid);
        }
    }
}
