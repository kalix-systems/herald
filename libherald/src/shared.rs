use crossbeam_channel::*;
use dashmap::DashMap;
use herald_common::UserId;
use heraldcore::{errors::HErr, types::ConversationId};
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

/// A bus interface for pushing updates to singleton objects
pub trait SingletonBus {
    /// The type of the update message
    type Update: Send;

    /// Pushes the update through the bus
    fn push(update: Self::Update) -> Result<(), HErr>;
}

/// A bus interface for pushing updates to dynamically created objects
pub trait AddressedBus {
    /// The type of the update message
    type Update: Send;

    /// The type used to address the objects
    type Addr: std::hash::Hash;

    /// Pushes the update through the bus, to the object addressed by `to`
    fn push(to: Self::Addr, update: Self::Update) -> Result<(), HErr>;
}
