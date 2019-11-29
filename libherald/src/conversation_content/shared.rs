use super::*;
use crate::messages::MsgUpdate;
use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::{channel_send_err, errors::HErr};
use lazy_static::*;

lazy_static! {
    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route conversation members related notifications that arrive from the network.
    pub(super) static ref RXS: DashMap<ConversationId, Receiver<ContentUpdate>> = DashMap::default();

    /// Concurrent hash map from `ConversationId`s to a channel sendder.
    /// This is used to route members related notifications that arrive from the network.
    pub(super) static ref TXS: DashMap<ConversationId, Sender<ContentUpdate>> = DashMap::default();
    /// Concurrent hash map of `MembersEmitter`. These are removed when the associated
    /// `Members` object is dropped.
    pub(super) static ref EMITTERS: DashMap<ConversationId, Emitter> = DashMap::default();
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
    let tx = match TXS.get(&to) {
        Some(tx) => tx.clone(),
        None => {
            let (tx, rx) = unbounded();

            TXS.insert(to, (&tx).clone());
            RXS.insert(to, rx);
            tx
        }
    };

    tx.send(update.into()).map_err(|_| channel_send_err!())?;
    if let Some(mut emitter) = EMITTERS.get_mut(&to) {
        emitter.new_data_ready();
    }
    Ok(())
}

pub(super) fn more_updates(cid: &ConversationId) -> bool {
    match RXS.get(cid) {
        Some(rx) => !rx.is_empty(),
        None => false,
    }
}

impl super::ConversationContent {
    pub(super) fn process_updates(&mut self) -> Option<()> {
        let id = self.id?;

        for update in RXS.get(&id)?.try_iter() {
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
}

impl Drop for ConversationContent {
    fn drop(&mut self) {
        use shared::*;
        if let Some(cid) = self.id {
            EMITTERS.remove(&cid);
            TXS.remove(&cid);
            RXS.remove(&cid);
        }
    }
}
