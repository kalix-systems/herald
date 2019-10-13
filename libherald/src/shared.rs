use crossbeam_channel::*;
use dashmap::DashMap;
use herald_common::UserId;
use heraldcore::{
    contact,
    types::{ConversationId, MsgId},
};
use lazy_static::*;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

lazy_static! {
    /// Concurrent hashmap from `UserId` to `Contact`. Used to avoid data replication.
    pub static ref USER_DATA: DashMap<UserId, contact::Contact> = DashMap::default();
}

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

        /// Data associated with `CONV_EMITTER`.
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

/// Shared state related to global conversation list
pub mod conv_global {
    use super::*;
    use crate::interface::ConversationsEmitter as Emitter;
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
    pub struct ConvChannel {
        pub(crate) rx: Receiver<ConvUpdates>,
        pub(crate) tx: Sender<ConvUpdates>,
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
        pub static ref CONV_CHANNEL: ConvChannel = ConvChannel::new();

        /// Conversations list emitter, filled in when the conversations list is constructed
        pub static ref CONV_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);

        /// Data associated with `CONV_EMITTER`.
        pub static ref CONV_TRY_POLL: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
    }

    /// Emits a signal to the QML runtime, returns `None` on failure.
    #[must_use]
    pub fn conv_emit_try_poll() -> Option<()> {
        let mut lock = CONV_EMITTER.lock();
        let emitter = lock.as_mut()?;

        CONV_TRY_POLL.fetch_add(1, Ordering::Acquire);
        emitter.try_poll_changed();
        Some(())
    }
}

/// Shared state related to global user list
pub mod user_global {
    use super::*;
    use crate::interface::UsersEmitter as Emitter;
    use parking_lot::Mutex;

    /// User list updates
    pub enum UsersUpdates {
        /// A new user has been added
        NewUser(UserId),
        /// A contact request has been responded to
        ReqResp(UserId, bool),
    }

    /// Channel for global user list updates
    pub struct UserChannel {
        pub(crate) rx: Receiver<UsersUpdates>,
        pub(crate) tx: Sender<UsersUpdates>,
    }

    impl UserChannel {
        /// Creates new `UserChannel`
        pub fn new() -> Self {
            let (tx, rx) = unbounded();
            Self { rx, tx }
        }
    }

    lazy_static! {
        /// Statically initialized instance of `UsersUpdates` used to pass notifications
        /// from the network.
        pub static ref USER_CHANNEL: UserChannel = UserChannel::new();

        /// Users list emitter, filled in when the users list is constructed
        pub static ref USER_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);

        /// Data associated with `CONV_EMITTER`.
        pub static ref USER_TRY_POLL: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
    }

    /// Emits a signal to the QML runtime, returns `None` on failure.
    #[must_use]
    pub fn users_emit_try_poll() -> Option<()> {
        let mut lock = USER_EMITTER.lock();
        let emitter = lock.as_mut()?;

        USER_TRY_POLL.fetch_add(1, Ordering::Acquire);
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

    /// Message related conversation updates
    pub enum MsgUpdate {
        /// A new message
        Msg(MsgId),
        /// A message has been acknowledged
        Receipt(MsgId),
    }

    lazy_static! {
        /// Concurrent hash map from `ConversationId`s to an event stream.
        /// This is used to route message related notifications that arrive from the network.
        pub static ref MSG_RXS: DashMap<ConversationId, Receiver<MsgUpdate>> = DashMap::default();
    }
}
