use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::types::{ConversationId, MsgId};
use lazy_static::*;

/// Conversation updates
pub enum ConvUpdate {
    /// A new message
    Msg(MsgId),
    /// A message has been acknowledged
    Ack(MsgId),
}

lazy_static! {
    /// Concurrent hash map from [`ConversationId`]s to an event stream.
    /// This is used to route notifications that arrive from the network.
    pub static ref CONV_MSG_RXS: DashMap<ConversationId, Receiver<ConvUpdate>> = DashMap::default();
}
