#![feature(try_blocks)]

pub use ::serde;
use bytes::Bytes;
use chrono::*;
use serde::*;
use std::convert::TryFrom;
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

pub type UserId = String;
pub type UserIdRef<'a> = &'a str;
pub type DeviceId = u32;
pub type RawMsg = Bytes;

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
