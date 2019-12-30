use herald_attachments::AttachmentMeta;
use herald_common::*;
use herald_ids::*;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
};

mod convert;
mod display;
mod rusqlite_imp;
mod ser;

mod elider;
pub use elider::Elider;
mod reaction;
pub use reaction::*;
mod match_status;
pub use match_status::*;
mod body;
pub use body::*;

/// Message
#[derive(Clone, Debug)]
pub struct Message {
    /// Message id
    pub message_id: MsgId,
    /// Author user id
    pub author: UserId,
    /// Recipient user id
    pub conversation: ConversationId,
    /// Content of message
    pub content: Option<Item>,
    /// Message time information
    pub time: MessageTime,
    /// Message id of the message being replied to
    pub op: ReplyId,
    /// Send status
    pub send_status: MessageSendStatus,
    /// Receipts
    pub receipts: HashMap<UserId, MessageReceiptStatus>,
    /// Messages that replied to this message
    pub replies: HashSet<MsgId>,
    /// Reactions to this message
    pub reactions: Option<Reactions>,
    /// Attachment metadata
    pub attachments: AttachmentMeta,
}

impl Message {
    pub fn text(&self) -> Option<&str> {
        self.content.as_ref().and_then(Item::as_str)
    }

    pub fn split(self) -> (MessageMeta, MsgData) {
        let Message {
            message_id,
            author,
            content,
            time,
            op,
            receipts,
            attachments,
            send_status,
            reactions,
            replies,
            ..
        } = self;

        let data = MsgData {
            author,
            receipts,
            content,
            op,
            attachments,
            time,
            send_status,
            replies,
            reactions,
        };

        let message = MessageMeta {
            msg_id: message_id,
            insertion_time: time.insertion,
            match_status: MatchStatus::NotMatched,
        };

        (message, data)
    }
}

/// An isolated message receipt.
#[derive(Clone, Copy, Debug)]
pub struct MessageReceipt {
    /// The message id the receipt is associated with
    pub msg_id: MsgId,
    /// The conversation id the original message is associated with
    pub cid: ConversationId,
    /// The recipient of the message
    pub recipient: UserId,
    /// The message receipt status
    pub status: MessageReceiptStatus,
}

#[derive(Clone, Copy, Debug, Ser, De, Eq, PartialEq, Hash)]
/// In order to support expiring messages, it is necessary to indicate
/// that a message is a reply without necessarily knowing the message
pub enum ReplyId {
    /// Not a reply
    None,
    /// It is a reply, but the original message could not be located
    Dangling,
    /// The message id is known
    Known(MsgId),
}

impl ReplyId {
    /// Indicates whether `ReplyId` is `None`
    pub fn is_none(&self) -> bool {
        self == &ReplyId::None
    }

    /// Indicates whether `ReplyId` is `Dangling`
    pub fn is_dangling(&self) -> bool {
        self == &ReplyId::Dangling
    }

    /// Indicates whether `ReplyId` is `Known`
    pub fn is_known(&self) -> bool {
        if let ReplyId::Known(_) = self {
            true
        } else {
            false
        }
    }
}

/// An item in the message history
#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Plain(MessageBody),
    Update(Update),
}

impl Item {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Item::Plain(body) => Some(body.as_str()),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&MessageBody> {
        match self {
            Item::Plain(body) => Some(&body),
            _ => None,
        }
    }
}

/// An update that appears appears in the message history
#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Update {
    Color(u32),
    Title(String),
    Picture(String),
    Expiration(crate::conversation::ExpirationPeriod),
}

#[derive(Clone, Copy, Debug)]
/// Time data relating to messages
pub struct MessageTime {
    /// The `Time` the message reached the server, if applicable.
    pub server: Option<Time>,
    /// The `Time` the message was saved on this device
    pub insertion: Time,
    /// The `Time` the message will expire, if applicable
    pub expiration: Option<Time>,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
/// Send status of a message
pub enum MessageSendStatus {
    /// No ack from server
    NoAck = 0,
    /// Acknowledged by server
    Ack = 1,
    /// The message has timed-out.
    Timeout = 2,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Ord, PartialOrd)]
#[repr(u8)]
/// Receipt status of a message
pub enum MessageReceiptStatus {
    /// Not acknowledged
    Nil = 0,
    /// Received by user
    Received = 1,
    /// Read by the recipient
    Read = 2,
}

impl Default for MessageReceiptStatus {
    fn default() -> Self {
        MessageReceiptStatus::Nil
    }
}

#[derive(Clone, Debug)]
pub struct MsgData {
    pub author: UserId,
    pub content: Option<Item>,
    pub time: MessageTime,
    pub op: ReplyId,
    pub receipts: HashMap<UserId, MessageReceiptStatus>,
    pub attachments: herald_attachments::AttachmentMeta,
    pub send_status: MessageSendStatus,
    pub replies: HashSet<MsgId>,
    pub reactions: Option<Reactions>,
}

impl MsgData {
    pub fn matches(
        &self,
        pattern: &search_pattern::SearchPattern,
    ) -> bool {
        match self.content.as_ref() {
            Some(Item::Plain(body)) => pattern.is_match(body.as_str()),
            _ => false,
        }
    }

    pub fn save_all_attachments<P: AsRef<std::path::Path>>(
        &self,
        dest: P,
    ) -> Result<(), herald_attachments::Error> {
        let ext = format!(
            "{author}_{time}",
            author = self.author,
            time = self.time.insertion.as_i64()
        );

        self.attachments.save_all(dest.as_ref().join(ext))
    }

    pub fn text(&self) -> Option<&str> {
        self.content.as_ref().and_then(Item::as_str)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// A thin wrapper around a `MsgId`
pub struct MessageMeta {
    pub msg_id: MsgId,
    pub insertion_time: Time,
    pub match_status: MatchStatus,
}

#[repr(u8)]
pub enum ReplyType {
    None = 0,
    Dangling = 1,
    Known = 2,
}

pub fn reply_type(reply_id: &ReplyId) -> ReplyType {
    match reply_id {
        ReplyId::None => ReplyType::None,
        ReplyId::Dangling => ReplyType::Dangling,
        ReplyId::Known(_) => ReplyType::Known,
    }
}

impl PartialOrd for MessageMeta {
    fn partial_cmp(
        &self,
        rhs: &Self,
    ) -> Option<std::cmp::Ordering> {
        self.insertion_time.partial_cmp(&rhs.insertion_time)
    }
}

impl Ord for MessageMeta {
    fn cmp(
        &self,
        rhs: &Self,
    ) -> std::cmp::Ordering {
        match self.partial_cmp(rhs) {
            Some(ord) => ord,
            None => self.msg_id.cmp(&rhs.msg_id),
        }
    }
}
