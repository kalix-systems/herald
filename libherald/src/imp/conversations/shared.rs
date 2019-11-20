use super::{types::Data, *};
use crate::{interface::ConversationsEmitter as Emitter, shared::SingletonBus};
use crossbeam_channel::*;
use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use heraldcore::{
    channel_send_err, conversation::settings::SettingsUpdate, types::ConversationId, NE,
};
use lazy_static::*;
use parking_lot::Mutex;

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

pub(super) type Ref<'a> = DashMapRef<'a, ConversationId, Data>;
pub(super) type RefMut<'a> = DashMapRefMut<'a, ConversationId, Data>;

/// Channel for global conversation list updates
pub(crate) struct ConvBus {
    pub(super) rx: Receiver<ConvUpdate>,
    pub(super) tx: Sender<ConvUpdate>,
}

pub(super) fn insert_data(
    cid: ConversationId,
    data: Data,
) {
    CONV_DATA.insert(cid, data);
}

pub(super) fn data(cid: &ConversationId) -> Option<Ref> {
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

pub(super) fn data_mut(cid: &ConversationId) -> Option<RefMut> {
    CONV_DATA.get_mut(cid)
}

lazy_static! {
    /// Statically initialized instance of `UsersUpdates` used to pass notifications
    /// from the network.
    pub(super) static ref CONV_BUS: ConvBus= ConvBus::new();

    /// Conversations list emitter, filled in when the conversations list is constructed
    pub(super) static ref CONV_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);


    pub(super) static ref CONV_DATA: DashMap<ConversationId, Data> = DashMap::default();
}

impl ConvBus {
    /// Creates new `ConvChannel`
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

impl SingletonBus for super::Conversations {
    type Update = ConvUpdate;

    fn push(update: Self::Update) -> Result<(), heraldcore::errors::HErr> {
        CONV_BUS
            .tx
            .clone()
            .send(update)
            .map_err(|_| channel_send_err!())?;
        conv_emit_new_data().ok_or(NE!())?;
        Ok(())
    }
}

/// Emits a signal to the QML runtime, returns `None` on failure.
#[must_use]
fn conv_emit_new_data() -> Option<()> {
    let mut lock = CONV_EMITTER.lock();
    let emitter = lock.as_mut()?;

    emitter.new_data_ready();
    Some(())
}
