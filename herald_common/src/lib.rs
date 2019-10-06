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

pub use bytes::Bytes;
pub use chainmail::block::*;
pub use chrono::prelude::*;
pub use serde_cbor;
pub use std::collections::HashMap;

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