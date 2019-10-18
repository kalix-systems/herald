use super::*;
use crate::shared::AddressedBus;
use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::{channel_send_err, message::Message};
use lazy_static::*;

/// Message related conversation updates
pub enum MsgUpdate {
    /// A new message
    Msg(MsgId),
    /// A message has been acknowledged
    Receipt(MsgId),
    /// A full message
    FullMsg(Message),
    /// Save is complete
    StoreDone(MsgId),
}

lazy_static! {
    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route message related notifications that arrive from the network.
    pub static ref MSG_RXS: DashMap<ConversationId, Receiver<MsgUpdate>> = DashMap::default();

    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route message related notifications that arrive from the network.
    pub static ref MSG_TXS: DashMap<ConversationId, Sender<MsgUpdate>> = DashMap::default();
    /// Concurrent hash map of `MessagesEmitter`. These are removed when the
    /// `Messages` object is dropped.
    pub static ref MSG_EMITTERS: DashMap<ConversationId, MessagesEmitter> = DashMap::default();
}

impl AddressedBus for Messages {
    type Addr = ConversationId;
    type Update = MsgUpdate;

    fn push(to: Self::Addr, update: Self::Update) -> Result<(), HErr> {
        let tx = match MSG_TXS.get(&to) {
            Some(tx) => tx.clone(),
            None => {
                let (tx, rx) = unbounded();

                MSG_TXS.insert(to, (&tx).clone());
                MSG_RXS.insert(to, rx);
                tx
            }
        };

        tx.send(update).map_err(|_| channel_send_err!())?;
        if let Some(mut emitter) = MSG_EMITTERS.get_mut(&to) {
            emitter.new_data_ready();
        }
        Ok(())
    }
}
