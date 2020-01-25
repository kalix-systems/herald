use super::{types::Data, *};
use heraldcore::types::ConversationId;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Conversation list updates
#[derive(Debug)]
pub enum ConvUpdate {
    /// A new conversation has been added
    NewConversation(ConversationMeta),
    /// A conversation builder has been finalized
    BuilderFinished(ConversationMeta),
    /// Initial data, sent when the conversations list is constructed
    Init(Vector<Conversation>),
    /// New activity in a conversation
    NewActivity(ConversationId),
}

impl From<ConvUpdate> for crate::Update {
    fn from(update: ConvUpdate) -> crate::Update {
        crate::Update::Conv(update)
    }
}

pub(crate) fn insert_data(
    cid: ConversationId,
    data: Data,
) {
    conv_data().write().insert(cid, data);
}

pub(crate) fn title(cid: &ConversationId) -> Option<String> {
    conv_data().read().get(cid)?.title.clone()
}

pub(crate) fn picture(cid: &ConversationId) -> Option<String> {
    conv_data().read().get(cid)?.picture.clone()
}

pub(crate) fn color(cid: &ConversationId) -> Option<u32> {
    Some(conv_data().read().get(cid)?.color)
}

pub(crate) fn pairwise(cid: &ConversationId) -> Option<bool> {
    Some(conv_data().read().get(cid)?.pairwise_uid.is_some())
}

pub(crate) fn conv_data() -> &'static RwLock<HashMap<ConversationId, Data>> {
    CONV_DATA.get_or_init(|| RwLock::new(HashMap::default()))
}

static CONV_DATA: OnceCell<RwLock<HashMap<ConversationId, Data>>> = OnceCell::new();
