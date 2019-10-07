use crossbeam_channel::*;
use dashmap::DashMap;
use herald_common::UserId;
use heraldcore::{
    contact,
    types::{ConversationId, MessageReceiptStatus, MsgId},
};
use lazy_static::*;

lazy_static! {
    /// Concurrent hashmap from `UserId` to `Contact`. Used to avoid data replication.
    pub static ref USER_DATA: DashMap<UserId, contact::Contact> = DashMap::default();
}

/// Shared state related to error handling
pub mod errors {
    use super::*;
    use parking_lot::Mutex;
    use std::collections::VecDeque;

    type QueueError = (u8, String);

    /// Error queue
    // TODO this should just be channel
    #[derive(Default)]
    pub struct ErrorQueue(Mutex<VecDeque<QueueError>>);

    impl ErrorQueue {
        /// Reads the oldest unread error.
        pub fn read(&self) -> Option<QueueError> {
            self.0.lock().pop_front()
        }

        /// Adds a new error to the back of the queue.
        pub fn push(&self, e: QueueError) {
            self.0.lock().push_back(e)
        }
    }

    lazy_static! {
        /// Global error queue
        pub static ref ERROR_QUEUE: ErrorQueue = ErrorQueue::default();
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
    }
}

/// Shared state related to global user list
pub mod user_global {
    use super::*;

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
        Receipt {
            /// The message that was received
            mid: MsgId,
            /// The receipt status
            stat: MessageReceiptStatus,
            /// The user that received the message
            by: UserId,
        },
    }

    lazy_static! {
        /// Concurrent hash map from `ConversationId`s to an event stream.
        /// This is used to route message related notifications that arrive from the network.
        pub static ref MSG_RXS: DashMap<ConversationId, Receiver<MsgUpdate>> = DashMap::default();
    }
}
