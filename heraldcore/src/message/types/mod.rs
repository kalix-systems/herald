use super::*;
use std::convert::TryInto;
use std::fmt;

mod convert;
mod rusqlite_imp;

/// Message
#[derive(Clone, Debug)]
pub struct Message {
    /// Local message id
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
    /// Indicates whether the message has attachments
    pub has_attachments: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
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

    #[cfg(test)]
    pub(crate) fn unwrap(self) -> MsgId {
        match self {
            ReplyId::Known(mid) => mid,
            ReplyId::Dangling => panic!("Tried to unwrap `Dangling` `ReplyId`"),
            ReplyId::None => panic!("Tried to unwrap `None` `ReplyId`"),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A message body
pub struct MessageBody(String);

impl fmt::Display for MessageBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MessageBody {
    /// Returns `MessageBody` as `&str`
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Returns `MessageBody` as `&[u8]`
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }

    /// Parses the text as markdown, rendering it to HTML
    pub fn parse_markdown(&self) -> Result<Self, EmptyMessageBody> {
        use pulldown_cmark::{html, Parser};

        let body_str = self.as_str();

        let parser = Parser::new(body_str);
        let mut buf = String::with_capacity(body_str.len());
        html::push_html(&mut buf, parser);

        buf.try_into()
    }
}

#[derive(Debug)]
/// Error returned when trying to creat an empty message body
pub struct EmptyMessageBody;

impl fmt::Display for EmptyMessageBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message bodies must have at least one character")
    }
}

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

impl fmt::Display for MissingInboundMessageField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingInboundMessageField::MissingMessageId => write!(f, "Message id was missing"),
            MissingInboundMessageField::MissingBody => write!(f, "Body was missing"),
            MissingInboundMessageField::MissingConversationId => {
                write!(f, "Conversation id was missing")
            }
            MissingInboundMessageField::MissingTimestamp => write!(f, "Timestamp was missing"),
            MissingInboundMessageField::MissingAuthor => write!(f, "Author was missing"),
        }
    }
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

impl fmt::Display for MissingOutboundMessageField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingOutboundMessageField::MissingBody => write!(f, "Body was missing"),
            MissingOutboundMessageField::MissingConversationId => {
                write!(f, "Conversation id was missing")
            }
        }
    }
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

impl Serialize for MessageSendStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for MessageSendStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::deserialize(d)?;
        u.try_into().map_err(|u| {
            Error::invalid_value(
                Unexpected::Unsigned(u64::from(u)),
                &format!("expected a value between {} and {}", 0, 2).as_str(),
            )
        })
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
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

impl Serialize for MessageReceiptStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for MessageReceiptStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::deserialize(d)?;
        u.try_into().map_err(|u| {
            Error::invalid_value(
                Unexpected::Unsigned(u64::from(u)),
                &format!("expected a value between {} and {}", 0, 3).as_str(),
            )
        })
    }
}
