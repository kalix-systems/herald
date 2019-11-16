use super::*;
use crate::interface::ConversationsEmitter as Emitter;
use crate::shared::SingletonBus;
use crossbeam_channel::*;
use heraldcore::{
    channel_send_err, conversation::settings::SettingsUpdate, types::ConversationId, NE,
};
use lazy_static::*;
use parking_lot::Mutex;

/// Conversation list updates
#[derive(Debug)]
pub enum ConvUpdate {
    /// A new conversation has been added
    NewConversation(ConversationId),
    /// A conversation builder has been finalized
    BuilderFinished(ConversationId),
    /// New activity
    NewActivity(ConversationId),
    /// Conversataion settings has been updated
    Settings(ConversationId, SettingsUpdate),
    /// Initial data, sent when the conversations list is constructed
    Init(Vector<Conversation>),
}

/// Channel for global conversation list updates
pub(crate) struct ConvBus {
    pub(super) rx: Receiver<ConvUpdate>,
    pub(super) tx: Sender<ConvUpdate>,
}

impl ConvBus {
    /// Creates new `ConvChannel`
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

lazy_static! {
    /// Statically initialized instance of `UsersUpdates` used to pass notifications
    /// from the network.
    pub(super) static ref CONV_BUS: ConvBus= ConvBus::new();

    /// Conversations list emitter, filled in when the conversations list is constructed
    pub(super) static ref CONV_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);
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
