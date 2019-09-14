#![feature(try_blocks)]
#![allow(warnings)]

mod crypto;
pub use crypto::*;

pub use bytes::Bytes;
pub use chainmail::block::*;
pub use chrono::prelude::*;
pub use serde::*;
pub use std::collections::HashMap;
pub use std::convert::{TryFrom, TryInto};
pub use tokio::prelude::*;

pub type UserId = String;
pub type UserIdRef<'a> = &'a str;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct GlobalId {
    uid: UserId,
    did: sig::PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserMeta {
    keys: HashMap<sig::PublicKey, sig::PKMeta>,
}

impl UserMeta {
    pub fn key_is_valid(&self, key: sig::PublicKey) -> bool {
        let maybe_kmeta = self.keys.get(&key);
        if maybe_kmeta.is_none() {
            return false;
        }
        maybe_kmeta.unwrap().key_is_valid(key)
    }

    pub fn verify_sig<T: AsRef<[u8]>>(&self, data: &Signed<T>) -> bool {
        self.key_is_valid(*data.signed_by()) && data.verify_sig()
    }

    pub fn add_new_key(&mut self, new: Signed<sig::PublicKey>) -> bool {
        if !self.verify_sig(&new) {
            return false;
        }
        let (pk, sig) = new.split();
        self.keys.insert(pk, sig.into());
        true
    }

    pub fn deprecate_key(&mut self, dep: Signed<sig::PublicKey>) -> bool {
        // cannot have a key deprecate itself
        if !self.verify_sig(&dep) || *dep.signed_by() == *dep.data() {
            return false;
        }
        let (pk, sig) = dep.split();
        self.keys.get_mut(&pk).unwrap().deprecate(sig);
        true
    }
}

pub type Blob = Bytes;
pub type BlobRef<'a> = &'a [u8];

pub type MsgId = [u8; 32];
pub type ConversationId = [u8; 32];

// TODO: lifetime parameters so these are zerocopy
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    SendBlock { to: Vec<UserId>, msg: Block },
    SendBlob { to: Vec<GlobalId>, msg: Blob },
    RequestMeta(UserId),
    RegisterDevice(sig::PublicKey),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Meta(UserMeta),
    DeviceRegistered(sig::PublicKey),
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
