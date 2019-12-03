use super::*;
use crate::messages::MsgUpdate;
use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::{channel_send_err, errors::HErr};
use once_cell::sync::OnceCell;

/// Concurrent hash map from `ConversationId`s to an event stream.
/// This is used to route conversation members related notifications that arrive from the network.
static RXS: OnceCell<DashMap<ConversationId, Receiver<ContentUpdate>>> = OnceCell::new();

/// Concurrent hash map from `ConversationId`s to a channel sendder.
/// This is used to route members related notifications that arrive from the network.
static TXS: OnceCell<DashMap<ConversationId, Sender<ContentUpdate>>> = OnceCell::new();
/// Concurrent hash map of `MembersEmitter`. These are removed when the associated
/// `Members` object is dropped.
static EMITTERS: OnceCell<DashMap<ConversationId, Emitter>> = OnceCell::new();

pub(super) fn txs() -> &'static DashMap<ConversationId, Sender<ContentUpdate>> {
    TXS.get_or_init(|| DashMap::default())
}

pub(super) fn rxs() -> &'static DashMap<ConversationId, Receiver<ContentUpdate>> {
    RXS.get_or_init(|| DashMap::default())
}

pub(super) fn emitters() -> &'static DashMap<ConversationId, Emitter> {
    EMITTERS.get_or_init(|| DashMap::default())
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
    let tx = match txs().get(&to) {
        Some(tx) => tx.clone(),
        None => {
            let (tx, rx) = unbounded();

            txs().insert(to, (&tx).clone());
            rxs().insert(to, rx);
            tx
        }
    };

    tx.send(update.into()).map_err(|_| channel_send_err!())?;
    if let Some(mut emitter) = emitters().get_mut(&to) {
        emitter.new_data_ready();
    }
    Ok(())
}

pub(super) fn more_updates(cid: &ConversationId) -> bool {
    match rxs().get(cid) {
        Some(rx) => !rx.is_empty(),
        None => false,
    }
}

impl super::ConversationContent {
    pub(super) fn process_updates(&mut self) -> Option<()> {
        let id = self.id?;

        for update in rxs().get(&id)?.try_iter() {
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

        emitters().insert(id, self.emit.clone());

        Some(())
    }
}

impl Drop for ConversationContent {
    fn drop(&mut self) {
        use shared::*;
        if let Some(cid) = self.id {
            emitters().remove(&cid);
            txs().remove(&cid);
            rxs().remove(&cid);
        }
    }
}
