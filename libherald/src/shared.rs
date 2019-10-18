use crate::interface::MessagesEmitter;
use crossbeam_channel::*;
use dashmap::DashMap;
use herald_common::UserId;
use heraldcore::types::{ConversationId, MsgId};
use lazy_static::*;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

/// Shared state related to error handling
pub mod errors {
    use super::*;
    use crate::interface::ErrorsEmitter;
    use parking_lot::Mutex;

    type Emitter = ErrorsEmitter;

    type QueueError = String;

    /// Error queue
    pub struct ErrorQueue {
        pub(crate) rx: Receiver<QueueError>,
        pub(crate) tx: Sender<QueueError>,
    }

    impl ErrorQueue {
        /// Creates new `ConvChannel`
        pub fn new() -> Self {
            let (tx, rx) = unbounded();
            Self { rx, tx }
        }
    }

    lazy_static! {
        /// Global error queue
        pub static ref ERROR_QUEUE: ErrorQueue = ErrorQueue::new();

        /// Errors emitter, filled in when the errors object is constructed
        pub static ref ERROR_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);

        /// Data associated with `ERROR_EMITTER`.
        pub static ref ERROR_TRY_POLL: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
    }

    /// Emits a signal to the QML runtime, returns `None` on failure.
    pub fn error_emit_try_poll() -> Option<()> {
        let mut lock = ERROR_EMITTER.lock();
        let emitter = lock.as_mut()?;

        ERROR_TRY_POLL.fetch_add(1, Ordering::Acquire);
        emitter.try_poll_changed();
        Some(())
    }
}

/// Shared state related to conversation members
pub mod members {
    use super::*;

    /// Conversation member related updates
    pub enum MemberUpdate {
        /// Response to a conversation add request
        ReqResp(UserId, bool),
    }

    lazy_static! {
        /// Concurrent hash map from `ConversationId`s to an event stream.
        /// This is used to route conversation members related notifications that arrive from the network.
        pub static ref MEMBER_RXS: DashMap<ConversationId, Receiver<MemberUpdate>> = DashMap::default();
    }
}

/// Shared state related to messages
pub mod messages {
    use super::*;
    use heraldcore::message::Message;

    /// Message related conversation updates
    pub enum MsgUpdate {
        /// A new message
        Msg(MsgId),
        /// A message has been acknowledged
        Receipt(MsgId),
        /// A full message
        FullMsg(Message),
        /// Save is complete
        StoreDone(MsgId),
    }

    lazy_static! {
        /// Concurrent hash map from `ConversationId`s to an event stream.
        /// This is used to route message related notifications that arrive from the network.
        pub static ref MSG_RXS: DashMap<ConversationId, Receiver<MsgUpdate>> = DashMap::default();

        /// Concurrent hash map from `ConversationId`s to an event stream.
        /// This is used to route message related notifications that arrive from the network.
        pub static ref MSG_TXS: DashMap<ConversationId, Sender<MsgUpdate>> = DashMap::default();
        /// Concurrent hash map of `MessagesEmitter`. These are removed when the
        /// `Messages` object is dropped.
        pub static ref MSG_EMITTERS: DashMap<ConversationId, MessagesEmitter> = DashMap::default();
    }
}
