use crate::{attachments::AttachmentMeta, ids::*};
use herald_common::*;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
};
use unicode_segmentation::UnicodeSegmentation;

mod convert;
mod display;
mod rusqlite_imp;
mod ser;

/// Message
#[derive(Clone, Debug)]
pub struct Message {
    /// Message id
    pub message_id: MsgId,
    /// Author user id
    pub author: UserId,
    /// Recipient user id
    pub conversation: ConversationId,
    /// Body of message
    pub body: Option<MessageBody>,
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
    /// Attachment metadata
    pub attachments: AttachmentMeta,
}

/// An isolated message receipt.
#[derive(Clone, Copy, Debug)]
pub struct MessageReceipt {
    /// The message id
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
/// that a message is a reply without necessarily knowing
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

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A message body
pub struct MessageBody(String);

impl MessageBody {
    /// Returns `MessageBody` as `&str`
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Returns `MessageBody` as `&[u8]`
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }

    /// Returns inner `String`
    pub fn inner(self) -> String {
        self.0
    }
}

#[derive(Debug)]
/// Error returned when trying to creat an empty message body
pub struct EmptyMessageBody;

impl std::error::Error for EmptyMessageBody {}

#[derive(Debug)]
/// Error returned if an inbound message is missing data
pub enum MissingInboundMessageField {
    /// Message id was missing
    MissingMessageId,
    /// Body was missing
    MissingBody,
    /// Conversation id was missing
    MissingConversationId,
    /// Timestamp was missing
    MissingTimestamp,
    /// Author was missing
    MissingAuthor,
}

impl std::error::Error for MissingInboundMessageField {}

#[derive(Debug)]
/// Error returned if an outbound message is missing data
pub enum MissingOutboundMessageField {
    /// Message body was missing
    MissingBody,
    /// Conversation id was missing
    MissingConversationId,
}

impl std::error::Error for MissingOutboundMessageField {}

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
    NoAck = 0,
    /// Received by user
    Received = 1,
    /// Read by the recipient
    Read = 2,
    /// The user has read receipts turned off
    AckTerminal = 3,
}

#[derive(Clone, Debug)]
pub struct MsgData {
    pub author: UserId,
    pub body: Option<MessageBody>,
    pub time: MessageTime,
    pub op: ReplyId,
    pub receipts: HashMap<UserId, MessageReceiptStatus>,
    pub attachments: crate::attachments::AttachmentMeta,
    pub send_status: MessageSendStatus,
    pub replies: HashSet<MsgId>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchStatus {
    NotMatched = 0,
    Matched = 1,
    Focused = 2,
}

impl MatchStatus {
    pub fn is_match(self) -> bool {
        self == MatchStatus::Matched || self == MatchStatus::Focused
    }
}

impl MsgData {
    pub fn matches(
        &self,
        pattern: &search_pattern::SearchPattern,
    ) -> bool {
        match self.body.as_ref() {
            Some(body) => pattern.is_match(body.as_str()),
            None => false,
        }
    }

    pub fn save_all_attachments<P: AsRef<std::path::Path>>(
        &self,
        dest: P,
    ) -> Result<(), crate::attachments::Error> {
        let ext = format!(
            "{author}_{time}",
            author = self.author,
            time = self.time.insertion.as_i64()
        );

        self.attachments.save_all(dest.as_ref().join(ext))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// A thin wrapper around a `MsgId`
pub struct MessageMeta {
    pub msg_id: MsgId,
    pub insertion_time: Time,
    pub match_status: MatchStatus,
}

pub fn split_msg(msg: Message) -> (MessageMeta, MsgData) {
    let Message {
        message_id,
        author,
        body,
        time,
        op,
        receipts,
        attachments,
        send_status,
        replies,
        ..
    } = msg;

    let data = MsgData {
        author,
        receipts,
        body,
        op,
        attachments,
        time,
        send_status,
        replies,
    };

    let message = MessageMeta {
        msg_id: message_id,
        insertion_time: time.insertion,
        match_status: MatchStatus::NotMatched,
    };

    (message, data)
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

pub struct Elider {
    pub line_count: usize,
    pub char_count: usize,
    pub char_per_line: usize,
}

impl Default for Elider {
    fn default() -> Self {
        let line_count = 30;
        let char_per_line = 25;
        let char_count = line_count * char_per_line;

        Self {
            line_count,
            char_per_line,
            char_count,
        }
    }
}

impl Elider {
    pub fn set_line_count(
        &mut self,
        line_count: usize,
    ) {
        self.line_count = line_count
    }

    pub fn set_char_count(
        &mut self,
        char_count: usize,
    ) {
        self.char_count = char_count
    }

    pub fn set_char_per_line(
        &mut self,
        char_per_line: usize,
    ) {
        self.char_per_line = char_per_line;
    }

    pub fn elided_body(
        &self,
        body: MessageBody,
    ) -> String {
        let graphemes = UnicodeSegmentation::graphemes(body.as_str(), true);
        let mut char_count = 0;
        let mut line_count = 0;

        for s in graphemes {
            if char_count >= self.char_count || line_count >= self.line_count {
                break;
            }

            char_count += 1;

            line_count += s.lines().count();
        }

        if char_count < self.char_count && line_count < self.line_count {
            return body.inner();
        }

        let chars_to_take = self.char_count.min(self.line_count * self.char_per_line);

        let mut out: String = UnicodeSegmentation::graphemes(body.as_str(), true)
            .take(chars_to_take)
            .collect();

        out.push_str("...");

        out
    }
}
