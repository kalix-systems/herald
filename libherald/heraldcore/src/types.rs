use herald_common::{serde::*, *};
use std::convert::TryFrom;

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
    Ack(ClientMessageAck),
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
