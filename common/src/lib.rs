#![feature(try_blocks)]

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

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum SessionType {
    Register = 0,
    Login = 1,
}
impl TryFrom<u8> for SessionType {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Register),
            1 => Ok(Self::Login),
            i => Err(i),
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct GlobalId {
    pub uid: UserId,
    pub did: sig::PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserMeta {
    keys: HashMap<sig::PublicKey, sig::PKMeta>,
}

impl UserMeta {
    pub fn new() -> Self {
        UserMeta {
            keys: HashMap::new(),
        }
    }

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

    pub fn add_key_unchecked(&mut self, key: sig::PublicKey, meta: sig::PKMeta) {
        self.keys.insert(key, meta);
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

    pub fn valid_keys(&self) -> impl Iterator<Item = sig::PublicKey> + '_ {
        self.keys
            .iter()
            .filter(|(k, m)| m.key_is_valid(**k))
            .map(|(k, _)| *k)
    }
}

pub type Blob = Bytes;
pub type BlobRef<'a> = &'a [u8];

pub type MsgId = [u8; 32];
pub type ConversationId = [u8; 32];

// TODO: lifetime parameters so these are zerocopy
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    SendBlock {
        to: Vec<UserId>,
        msg: Block,
    },
    SendBlob {
        to: Vec<sig::PublicKey>,
        msg: Blob,
    },
    RequestMeta {
        qid: u64,
        of: UserId,
    },
    RegisterDevice {
        qid: u64,
        key: Signed<sig::PublicKey>,
    },
    Quit,
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
        qid: u64,
    },
    // TODO: consider doing something smarter here to allow this to work incrementally
    Catchup(Vec<MessageToClient>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Meta(UserMeta),
    DeviceRegistered(sig::PublicKey),
    DataNotFound,
    InvalidRequest,
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
