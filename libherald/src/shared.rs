use crossbeam_channel::*;
use dashmap::DashMap;
use herald_common::UserId;
use heraldcore::{
    contact,
    types::{ConversationId, MsgId},
};
use lazy_static::*;
use parking_lot::Mutex;
use std::collections::VecDeque;

lazy_static! {
    /// Concurrent hashmap from `UserId` to `Contact`. Used to avoid data replication.
    pub static ref USER_DATA: DashMap<UserId, contact::Contact> = DashMap::default();
}

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

/// Conversation list updates
pub enum ConvUpdates {
    /// A new conversation has been added
    NewConversation(ConversationId),
}

/// Channel for global user list updates
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
}

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

/// Member related conversations updates
pub enum MemberUpdate {
    /// Response to a conversation add request
    ReqResp(UserId, bool),
}

lazy_static! {
    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route conversation members related notifications that arrive from the network.
    pub static ref MEMBER_RXS: DashMap<ConversationId, Receiver<MemberUpdate>> = DashMap::default();
}

/// Message related conversation updates
pub enum MsgUpdate {
    /// A new message
    Msg(MsgId),
    /// A message has been acknowledged
    Ack(MsgId),
}

lazy_static! {
    /// Concurrent hash map from `ConversationId`s to an event stream.
    /// This is used to route message related notifications that arrive from the network.
    pub static ref MSG_RXS: DashMap<ConversationId, Receiver<MsgUpdate>> = DashMap::default();
}
