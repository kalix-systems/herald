use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::types::{ConversationId, MsgId};
use lazy_static::*;

pub enum ConvUpdate {
    Msg(MsgId),
    Ack(MsgId),
}

lazy_static! {
    pub static ref CONV_MSG_RXS: DashMap<ConversationId, Receiver<ConvUpdate>> = DashMap::default();
}
