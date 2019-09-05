pub use arrayvec::CapacityError;
use bytes::Bytes;
use chrono::prelude::*;
pub use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

pub type UserId = String;
pub type DeviceId = u32;
pub type RawMsg = Bytes;

// the network status of a message
#[derive(
    Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq, Copy, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum MessageStatus {
    /// No ack from any third party
    NoAck,
    /// No ack from any third party
    NoAckFromServer,
    /// Received by the server, and made it to the user
    RecipReceivedAck,
    /// Read by the recipient
    RecipReadAck,
    /// The message has timedout.
    Timeout,
    /// we did not write this message
    Inbound,
    /// The user has read receipts turned off
    AckTerminal,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub num_devices: usize,
    pub blob: Bytes,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct GlobalId {
    pub uid: UserId,
    pub did: DeviceId,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct ClientMessageAck {
    pub update_code: MessageStatus,
    pub message_id: i64,
}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement 
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Message(String),
    Ack(ClientMessageAck),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    SendMsg { to: UserId, body: RawMsg },
    RequestMeta { of: UserId },
    UpdateBlob { blob: Bytes },
    RegisterDevice,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToClient {
    Push {
        from: GlobalId,
        body: RawMsg,
        time: DateTime<Utc>,
    },
    QueryResponse {
        res: Response,
        query: MessageToServer,
    },
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Meta(User),
    DeviceRegistered(DeviceId),
    DataNotFound,
}
