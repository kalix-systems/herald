#![feature(try_blocks)]

use bytes::Bytes;
use chrono::prelude::*;
use serde::*;
use std::convert::{TryFrom, TryInto};
use tokio::prelude::*;

pub type UserId = String;
pub type DeviceId = u32;
pub type RawMsg = Bytes;

// the network status of a message
#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum MessageStatus {
    /// No ack from any third party
    NoAck = 0,
    /// No ack from any third party
    NoAckFromServer = 1,
    /// Received by the server, and made it to the user
    RecipReceivedAck = 2,
    /// Read by the recipient
    RecipReadAck = 3,
    /// The message has timedout.
    Timeout = 4,
    /// we did not write this message
    Inbound = 5,
    /// The user has read receipts turned off
    AckTerminal = 6,
}

const MESSAGE_STATUS_MIN: u8 = 0;
const MESSAGE_STATUS_MAX: u8 = 6;

impl TryFrom<u8> for MessageStatus {
    type Error = u8;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use MessageStatus::*;
        try {
            match u {
                0 => NoAck,
                1 => NoAckFromServer,
                2 => RecipReceivedAck,
                3 => RecipReadAck,
                4 => Timeout,
                5 => Inbound,
                6 => AckTerminal,
                u => return Err(u),
            }
        }
    }
}

impl Serialize for MessageStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(self.clone() as u8)
    }
}

impl<'de> Deserialize<'de> for MessageStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::deserialize(d)?;
        u.try_into().map_err(|u| {
            Error::invalid_value(
                Unexpected::Unsigned(u as u64),
                &format!(
                    "expected a value between {} and {}",
                    MESSAGE_STATUS_MIN, MESSAGE_STATUS_MAX
                )
                .as_str(),
            )
        })
    }
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

#[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientMessageAck {
    pub update_code: MessageStatus,
    pub message_id: i64,
}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToPeer {
    // TODO: replace this with an &str
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

#[derive(Debug)]
pub enum TransportError {
    Io(tokio::io::Error),
    De(serde_cbor::Error),
    Se(serde_cbor::Error),
}

impl From<tokio::io::Error> for TransportError {
    fn from(e: tokio::io::Error) -> Self {
        TransportError::Io(e)
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransportError::Io(e) => write!(f, "IO error during cbor transport, msg was {}", e),
            TransportError::De(e) => write!(
                f,
                "Deserialization error during cbor transport, msg was {}",
                e
            ),
            TransportError::Se(e) => write!(
                f,
                "Serialization error during cbor transport, msg was {}",
                e
            ),
        }
    }
}
impl std::error::Error for TransportError {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            TransportError::Io(e) => e,
            TransportError::Se(e) => e,
            TransportError::De(e) => e,
        })
    }
}

pub async fn send_cbor<S: AsyncWrite + Unpin, T: Serialize>(
    s: &mut S,
    t: &T,
) -> Result<(), TransportError> {
    let vec = serde_cbor::to_vec(t).map_err(TransportError::Se)?;
    let len = u64::to_le_bytes(vec.len() as u64);
    s.write_all(&len).await?;
    s.write_all(&vec).await?;
    Ok(())
}

pub async fn read_cbor<S: AsyncRead + Unpin, T: for<'a> Deserialize<'a>>(
    s: &mut S,
) -> Result<T, TransportError> {
    let mut buf = [0u8; 8];
    s.read_exact(&mut buf).await?;
    let len = u64::from_le_bytes(buf) as usize;
    let mut buf = vec![0u8; len];
    s.read_exact(&mut buf).await?;
    dbg!(&buf);
    let res = serde_cbor::from_slice(&buf).map_err(TransportError::De)?;
    Ok(res)
}
