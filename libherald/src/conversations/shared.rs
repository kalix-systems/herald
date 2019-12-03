use super::{types::Data, *};
use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use heraldcore::{conversation::settings::SettingsUpdate, types::ConversationId};
use once_cell::sync::OnceCell;

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
    conv_data().insert(cid, data);
}

pub(crate) fn data(cid: &ConversationId) -> Option<Ref> {
    conv_data().get(cid)
}

pub(crate) fn title(cid: &ConversationId) -> Option<String> {
    conv_data().get(cid)?.title.clone()
}

pub(crate) fn picture(cid: &ConversationId) -> Option<String> {
    conv_data().get(cid)?.picture.clone()
}

pub(crate) fn color(cid: &ConversationId) -> Option<u32> {
    Some(conv_data().get(cid)?.color)
}

pub(crate) fn pairwise(cid: &ConversationId) -> Option<bool> {
    Some(conv_data().get(cid)?.pairwise)
}

pub(crate) fn data_mut(cid: &ConversationId) -> Option<RefMut> {
    conv_data().get_mut(cid)
}

pub(crate) fn conv_data() -> &'static DashMap<ConversationId, Data> {
    CONV_DATA.get_or_init(|| DashMap::default())
}

static CONV_DATA: OnceCell<DashMap<ConversationId, Data>> = OnceCell::new();
