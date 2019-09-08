use arrayvec::ArrayVec;
pub use arrayvec::CapacityError;
use bytes::Bytes;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub type UserId = String;
pub type UserIdRef<'a> = &'a str;
pub type DeviceId = u32;
pub type RawMsg = Bytes;

pub type MsgId = ArrayVec<[u8; 32]>;
pub type ConversationId = ArrayVec<[u8; 32]>;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum MessageSendStatus {
    /// No ack from server
    NoAck = 0,
    /// Acknowledged by server
    Ack = 1,
    /// The message has timed-out.
    Timeout = 2,
}

// the network status of a message
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
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
pub struct MessageAck {
    pub update_code: MessageSendStatus,
    pub message_id: MsgId,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct MessageReceipt {
    pub update_code: MessageReceiptStatus,
    pub message_id: MsgId,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Message(String),
    Ack(MessageAck),
    Receipt(MessageReceiptStatus),
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
        msg_id: MsgId,
        from: GlobalId,
        conversation_id: ConversationId,
        op_msg_id: MsgId,
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
