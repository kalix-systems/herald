use super::*;
use crate::shared::AddressedBus;
use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::{channel_send_err, errors::HErr};
use lazy_static::*;

/// Message preview update
#[derive(Debug)]
pub enum Update {
    /// A new conversation has been added
    SetDangling,
    /// Initial data
    Init {
        time: Time,
        body: Option<MessageBody>,
        author: UserId,
        has_attachments: bool,
    },
}

lazy_static! {
    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route conversation members related notifications that arrive from the network.
    pub(super) static ref RXS: DashMap<MsgId, Receiver<Update>> = DashMap::default();

    /// Concurrent hash map from `ConversationId`s to a channel sendder.
    /// This is used to route members related notifications that arrive from the network.
    pub(super) static ref TXS: DashMap<MsgId, Sender<Update>> = DashMap::default();
    /// Concurrent hash map of `MembersEmitter`. These are removed when the associated
    /// `Members` object is dropped.
    pub(super) static ref EMITTERS: DashMap<MsgId, Emitter> = DashMap::default();
}

impl AddressedBus for MessagePreview {
    type Update = Update;
    type Addr = MsgId;

    fn push(
        to: Self::Addr,
        update: Self::Update,
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

        tx.send(update).map_err(|_| channel_send_err!())?;

        if let Some(mut emitter) = EMITTERS.get_mut(&to) {
            emitter.new_data_ready();
        }

        Ok(())
    }
}
