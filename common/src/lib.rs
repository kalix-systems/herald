mod crypto;
pub use crypto::*;
#[cfg(feature = "diesel_pg")]
mod diesel_impls;
#[cfg(feature = "rusqlite_")]
mod rusqlite_impls;
mod types;
pub use types::*;
#[macro_use]
mod newtype_macros;

pub use async_trait::*;
pub use bytes::Bytes;
pub use chainmail::block::*;
pub use chrono::prelude::*;
pub use serde_cbor;
pub use std::collections::HashMap;
pub use tokio::prelude::*;

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
