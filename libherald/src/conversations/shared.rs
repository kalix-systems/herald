use super::{types::Data, *};

use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use heraldcore::{conversation::settings::SettingsUpdate, types::ConversationId};
use lazy_static::*;

/// Conversation list updates
#[derive(Debug)]
pub enum ConvUpdate {
    /// A new conversation has been added
    NewConversation(ConversationMeta),
    /// A conversation builder has been finalized
    BuilderFinished(ConversationMeta),
    /// New activity
    NewActivity(ConversationId),
    /// Conversataion settings has been updated
    Settings(ConversationId, SettingsUpdate),
    /// Initial data, sent when the conversations list is constructed
    Init(Vector<Conversation>),
}

impl From<ConvUpdate> for crate::Update {
    fn from(update: ConvUpdate) -> crate::Update {
        crate::Update::Conv(update)
    }
}

pub(crate) type Ref<'a> = DashMapRef<'a, ConversationId, Data>;
pub(crate) type RefMut<'a> = DashMapRefMut<'a, ConversationId, Data>;

pub(crate) fn insert_data(
    cid: ConversationId,
    data: Data,
) {
    CONV_DATA.insert(cid, data);
}

pub(crate) fn data(cid: &ConversationId) -> Option<Ref> {
    CONV_DATA.get(cid)
}

pub(crate) fn title(cid: &ConversationId) -> Option<String> {
    CONV_DATA.get(cid)?.title.clone()
}

pub(crate) fn picture(cid: &ConversationId) -> Option<String> {
    CONV_DATA.get(cid)?.picture.clone()
}

pub(crate) fn color(cid: &ConversationId) -> Option<u32> {
    Some(CONV_DATA.get(cid)?.color)
}

pub(crate) fn pairwise(cid: &ConversationId) -> Option<bool> {
    Some(CONV_DATA.get(cid)?.pairwise)
}

pub(crate) fn data_mut(cid: &ConversationId) -> Option<RefMut> {
    CONV_DATA.get_mut(cid)
}

lazy_static! {
    pub(crate) static ref CONV_DATA: DashMap<ConversationId, Data> = DashMap::default();
}
