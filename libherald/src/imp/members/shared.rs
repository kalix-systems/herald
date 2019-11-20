use super::*;
use crate::shared::AddressedBus;
use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::{channel_send_err, errors::HErr};
use lazy_static::*;

/// Conversation member related updates
pub enum MemberUpdate {
    /// Response to a conversation add request
    ReqResp(UserId, bool),
}

lazy_static! {
    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route conversation members related notifications that arrive from the network.
    pub(super) static ref RXS: DashMap<ConversationId, Receiver<MemberUpdate>> = DashMap::default();

    /// Concurrent hash map from `ConversationId`s to a channel sendder.
    /// This is used to route members related notifications that arrive from the network.
    pub(super) static ref TXS: DashMap<ConversationId, Sender<MemberUpdate>> = DashMap::default();
    /// Concurrent hash map of `MembersEmitter`. These are removed when the associated
    /// `Members` object is dropped.
    pub(super) static ref EMITTERS: DashMap<ConversationId, MembersEmitter> = DashMap::default();
}

impl AddressedBus for Members {
    type Update = MemberUpdate;
    type Addr = ConversationId;

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
