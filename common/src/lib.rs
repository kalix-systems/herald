pub use arrayvec::CapacityError;
use bytes::Bytes;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub type UserId = String;
pub type DeviceId = usize;
pub type RawMsg = Bytes;

// the network status of a message
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum MessageStatus {
    /// No ack from any third party
    NoAck,
    /// Received by the server, and made it to the user
    ReceivedAck,
    /// Read by the recipient
    RecipientReadAck,
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
    update_code: MessageStatus,
    message_id: i64,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    SendMsg { to: UserId, text: RawMsg },
    RequestMeta { of: UserId },
    UpdateBlob { blob: Bytes },
    RegisterDevice,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToClient {
    NewMessage {
        from: GlobalId,
        text: RawMsg,
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
