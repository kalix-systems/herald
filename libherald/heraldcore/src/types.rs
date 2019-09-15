use herald_common::{serde::*, *};
use std::convert::{TryFrom, TryInto};

//#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
///// An acknowledgement of message receipt.
//pub struct ClientMessageAck {
//    /// The receipt status of the message
//    pub update_code: MessageReceiptStatus,
//    /// The message id
//    pub message_id: MsgId,
//}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct MessageReceipt {
    /// The receipt status of the message
    pub update_code: MessageReceiptStatus,
    /// The message id
    pub message_id: MsgId,
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

impl TryFrom<u8> for MessageSendStatus {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoAck),
            1 => Ok(Self::Ack),
            2 => Ok(Self::Timeout),
            i => Err(i),
        }
    }
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

impl TryFrom<u8> for MessageReceiptStatus {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoAck),
            1 => Ok(Self::Received),
            2 => Ok(Self::Read),
            3 => Ok(Self::AckTerminal),
            i => Err(i),
        }
    }
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

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToPeer {
    // TODO: replace this with an &str
    /// A message
    Message {
        /// Body of the message
        body: String,
        /// Message id
        msg_id: MsgId,
        /// Conversation the message is associated with
        conversation_id: ConversationId,
    },
    /// A request to start a conversation.
    AddRequest(ConversationId),
    /// A response to a request to start conversation
    AddResponse(ConversationId, bool),
    /// An acknowledgement of a previous message
    Ack(MessageReceipt),
}
#[derive(Default, Hash, Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
/// Conversation ID
pub struct ConversationId([u8; UID_LEN]);

/// Conversation
pub struct InvalidConversationId;

impl std::fmt::Display for InvalidConversationId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "MsgIdCapacityError")
    }
}

impl ConversationId {
    /// Converts `ConversationId` to `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Converts `ConversationId` into a fixed length array.
    pub fn into_array(self) -> [u8; UID_LEN] {
        self.0
    }

    /// `ConversationId` as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0 as &[u8]
    }
}

impl From<[u8; UID_LEN]> for ConversationId {
    fn from(arr: [u8; UID_LEN]) -> Self {
        Self(arr)
    }
}

impl TryFrom<Vec<u8>> for ConversationId {
    type Error = InvalidConversationId;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(InvalidConversationId)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.into_iter().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}

impl TryFrom<&[u8]> for ConversationId {
    type Error = MsgIdCapacityError;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(MsgIdCapacityError)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.iter().copied().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}
