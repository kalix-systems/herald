#![feature(try_blocks)]

use bytes::Bytes;
pub use ::serde;
use chrono::*;
use serde::*;
use std::convert::{TryFrom, TryInto};
use tokio::prelude::*;

pub const UID_LEN: usize = 32;

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub struct MsgId([u8; UID_LEN]);

pub struct MsgIdCapacityError;

impl std::fmt::Display for MsgIdCapacityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "MsgIdCapacityError")
    }
}

impl MsgId {
    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn into_array(self) -> [u8; UID_LEN] {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0 as &[u8]
    }
}

impl From<[u8; UID_LEN]> for MsgId {
    fn from(arr: [u8; UID_LEN]) -> Self {
        Self(arr)
    }
}

impl TryFrom<Vec<u8>> for MsgId {
    type Error = MsgIdCapacityError;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(MsgIdCapacityError)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.into_iter().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}

impl TryFrom<&[u8]> for MsgId {
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


// TODO: lifetime parameters so these are zerocopy
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
        op_msg_id: Option<MsgId>,
        body: RawMsg,
        time: DateTime<Utc>,
    },

    QueryResponse {
        res: Response,
        query: MessageToServer,
    },
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct ClientMessageAck {
    pub update_code: MessageReceiptStatus,
    pub message_id: MsgId,
}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct MessageReceipt {
    pub update_code: MessageReceiptStatus,
    pub message_id: MsgId,
}


pub type UserId = String;
pub type UserIdRef<'a> = &'a str;
pub type DeviceId = u32;
pub type RawMsg = Bytes;


#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
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

// the network status of a message
#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
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
    eprintln!("length is {:x?}", len);
    s.write_all(&len).await?;
    s.write_all(&vec).await?;
    Ok(())
}

pub async fn read_cbor<S: AsyncRead + Unpin, T: for<'a> Deserialize<'a>>(
    s: &mut S,
) -> Result<T, TransportError> {
    let mut buf = [0u8; 8];
    eprintln!("waiting to read length");
    s.read_exact(&mut buf).await?;
    let len = u64::from_le_bytes(buf) as usize;
    eprintln!("length read, was {}", len);
    eprintln!("waiting to read data");
    let mut buf = vec![0u8; len];
    s.read_exact(&mut buf).await?;
    eprintln!("read data, bytes were {:x?}", &buf);
    let res = serde_cbor::from_slice(&buf).map_err(TransportError::De)?;
    Ok(res)
}
