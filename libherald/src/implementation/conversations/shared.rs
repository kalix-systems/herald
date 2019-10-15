use crate::interface::ConversationsEmitter as Emitter;
use crossbeam_channel::*;
use heraldcore::types::ConversationId;
use lazy_static::*;
use parking_lot::Mutex;

/// Conversation list updates
pub enum ConvUpdates {
    /// A new conversation has been added
    NewConversation(ConversationId),
    /// A conversation builder can been finalized
    BuilderFinished(ConversationId),
    /// New activity
    NewActivity(ConversationId),
}

/// Channel for global conversation list updates
pub(super) struct ConvChannel {
    pub(super) rx: Receiver<ConvUpdates>,
    pub(super) tx: Sender<ConvUpdates>,
}

impl ConvChannel {
    /// Creates new `ConvChannel`
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

lazy_static! {
    /// Statically initialized instance of `UsersUpdates` used to pass notifications
    /// from the network.
    pub(super) static ref CONV_CHANNEL: ConvChannel = ConvChannel::new();

    /// Conversations list emitter, filled in when the conversations list is constructed
    pub(super) static ref CONV_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);
}

pub fn push_conv_update(update: ConvUpdates) -> Option<()> {
    CONV_CHANNEL.tx.send(update).ok()?;
    conv_emit_new_data()?;
    Some(())
}

/// Emits a signal to the QML runtime, returns `None` on failure.
#[must_use]
fn conv_emit_new_data() -> Option<()> {
    let mut lock = CONV_EMITTER.lock();
    let emitter = lock.as_mut()?;

    emitter.new_data_ready();
    Some(())
}
