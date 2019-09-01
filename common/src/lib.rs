use bytes::Bytes;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
pub use arrayvec::CapacityError;

pub type UserId = String;
pub type DeviceId = usize;
pub type RawMsg = Bytes;

// the network status of a message
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageStatus {
    /// No ack from any third party
    NoAck = 0,
    /// Received by the server, and made it to the user
    ReceivedAck = 1,
    /// Received by the recipient
    RecpientReadAck = 2,
    /// The message has timedout.
    Timeout = 3,
    /// we did not write this message
    Inbound = 4,
    /// The user has read receipts turned off
    AckTerminal = 5,
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
pub enum MessageToServer {
    SendMsg { to: UserId, text: RawMsg },
    RequestMeta { of: UserId },
    ClientMessageAck {
        to: UserId,
        update_code: MessageStatus,
        message_id: i64,// currently just acks with the row in the DB... change this
    },
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
    ServerMessageAck {
        from: GlobalId,
        update_code: MessageStatus,
        message_id: i64,// currently just acks with the row in the DB... change this
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
