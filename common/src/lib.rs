#![feature(try_blocks)]
#![allow(warnings)]

mod crypto;
pub use crypto::*;

pub use bytes::Bytes;
pub use chainmail::block::*;
pub use chrono::prelude::*;
pub use serde::*;
pub use std::convert::{TryFrom, TryInto};
pub use tokio::prelude::*;

pub type UserId = String;
pub type UserIdRef<'a> = &'a str;
pub type DeviceId = u32;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct GlobalId {
    uid: UserId,
    did: DeviceId,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct User {
    uid: UserId,
    devices: Vec<sig::PublicKey>,
}

pub type Blob = Bytes;
pub type BlobRef<'a> = &'a [u8];

pub type MsgId = [u8; 32];
pub type ConversationId = [u8; 32];

// TODO: lifetime parameters so these are zerocopy
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    SendBlock { to: Vec<UserId>, msg: Block },
    SendBlob { to: Vec<DeviceId>, msg: Blob },
    RequestMeta(UserId),
    RegisterDevice(sig::PublicKey),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToClient {
    NewBlock {
        from: GlobalId,
        time: DateTime<Utc>,
        body: Block,
    },
    NewBlob {
        from: GlobalId,
        time: DateTime<Utc>,
        body: Blob,
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
    eprintln!("length is {:x?}", len);
    s.write_all(&len).await?;
    s.write_all(&vec).await?;
    Ok(())
}

pub async fn read_cbor<S: AsyncRead + Unpin, T: for<'a> Deserialize<'a>>(
    s: &mut S,
) -> Result<T, TransportError> {
    let mut buf = [0u8; 8];
    eprintln!("reading length");
    s.read_exact(&mut buf).await?;
    let len = u64::from_le_bytes(buf) as usize;
    eprintln!("length read, was {}", len);
    eprintln!("now reading data");
    let mut buf = vec![0u8; len];
    s.read_exact(&mut buf).await?;
    eprintln!("read data, bytes were {:x?}", &buf);
    let res = serde_cbor::from_slice(&buf).map_err(TransportError::De)?;
    Ok(res)
}
