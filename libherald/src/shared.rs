use crossbeam_channel::*;
use dashmap::DashMap;
use heraldcore::types::{ConversationId, MsgId};
use lazy_static::*;
use parking_lot::Mutex;
use std::collections::VecDeque;

/// Conversation updates
pub enum ConvUpdate {
    /// A new message
    Msg(MsgId),
    /// A message has been acknowledged
    Ack(MsgId),
}

type QueueError = (u8, String);

/// Error queue
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
    /// Concurrent hash map from [`ConversationId`]s to an event stream.
    /// This is used to route notifications that arrive from the network.
    pub static ref CONV_MSG_RXS: DashMap<ConversationId, Receiver<ConvUpdate>> = DashMap::default();
}

lazy_static! {
    /// Global error queue
    pub static ref ERROR_QUEUE: ErrorQueue = ErrorQueue::default();
}
