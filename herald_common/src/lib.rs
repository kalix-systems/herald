mod crypto;
pub use crypto::*;
#[cfg(feature = "rusqlite_")]
mod rusqlite_impls;
mod types;
pub use types::*;
mod time;
pub use time::*;

pub use kcl::random::UQ;
pub use kson::{self, prelude::*};
pub use std::collections::BTreeMap;

impl UserMeta {
    pub fn new(initial: Signed<UserId>) -> Self {
        UserMeta {
            initial,
            initial_dep: None,
            keys: BTreeMap::new(),
        }
    }

    pub fn key_is_valid(
        &self,
        key: sig::PublicKey,
    ) -> bool {
        if key == *self.initial.signed_by() {
            self.initial_dep.is_none()
        } else if let Some(m) = self.keys.get(&key) {
            m.key_is_valid(key)
        } else {
            false
        }
    }

    pub fn verify_sig<T: Ser>(
        &self,
        data: &Signed<T>,
    ) -> bool {
        self.key_is_valid(*data.signed_by()) && data.verify_sig().eq(&SigValid::Yes)
    }

    pub fn add_new_key(
        &mut self,
        new: Signed<sig::Endorsement>,
    ) -> bool {
        if !self.verify_sig(&new) {
            return false;
        }
        let (pk, sig) = new.split();
        self.keys.insert(pk.0, sig.into());
        true
    }

    pub fn add_key_unchecked(
        &mut self,
        key: sig::PublicKey,
        meta: sig::PKMeta,
    ) {
        self.keys.insert(key, meta);
    }

    pub fn deprecate_key(
        &mut self,
        dep: Signed<sig::Deprecation>,
    ) -> bool {
        // cannot have a key deprecate itself
        if !self.verify_sig(&dep) || *dep.signed_by() == dep.data().0 {
            return false;
        }
        let (pk, sig) = dep.split();
        self.keys.get_mut(&pk.0).unwrap().deprecate(sig);
        true
    }

    pub fn valid_keys(&self) -> impl Iterator<Item = sig::PublicKey> + '_ {
        let init = if self.initial_dep.is_some() {
            None
        } else {
            Some(*self.initial.signed_by())
        };
        init.into_iter().chain(
            self.keys
                .iter()
                .filter(|(_, m)| m.dep().is_none())
                .map(|(k, _)| *k),
        )
    }
}
