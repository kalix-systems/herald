use bytes::Bytes;
use chrono::prelude::*;
use ring::signature::VerificationAlgorithm;
use serde::{Deserialize, Serialize};
use untrusted::Input;

pub type UserId = u64;
pub type RawKey = Bytes;
pub type RawSig = Bytes;
pub type RawMsg = Bytes;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceId {
    Original,
    Verified(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signed<T> {
    pub data: T,
    pub timestamp: DateTime<Utc>,
    pub signer: DeviceId,
    pub sig: RawSig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedKey {
    raw: Signed<RawKey>,
    deprecated: Option<Signed<()>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OriginalKey {
    raw: RawKey,
    created_on: DateTime<Utc>,
    deprecated: Option<Signed<()>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Key {
    Original(OriginalKey),
    Created(CreatedKey),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMeta {
    original: OriginalKey,
    verified_keys: Vec<CreatedKey>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignatureValidity {
    Valid,
    KeyInactive,
    BadSig,
}

impl Key {
    pub fn raw_key(&self) -> &RawKey {
        match self {
            Key::Original(o) => &o.raw,
            Key::Created(c) => &c.raw.data,
        }
    }

    pub fn deprecated(&self) -> Option<&Signed<()>> {
        match self {
            Key::Original(o) => o.deprecated.as_ref(),
            Key::Created(c) => c.deprecated.as_ref(),
        }
    }

    pub fn created(&self) -> DateTime<Utc> {
        match self {
            Key::Original(o) => o.created_on,
            Key::Created(c) => c.raw.timestamp,
        }
    }

    pub fn check_sig<V: VerificationAlgorithm>(
        &self,
        v: &V,
        msg: RawMsg,
        sig: RawSig,
    ) -> SignatureValidity {
        let raw = self.raw_key();
        if self.deprecated().is_some() {
            SignatureValidity::KeyInactive
        } else if v
            .verify(Input::from(&raw), Input::from(&msg), Input::from(&sig))
            .is_ok()
        {
            SignatureValidity::Valid
        } else {
            SignatureValidity::BadSig
        }
    }
}

impl UserMeta {
    pub fn get_key(&self, did: DeviceId) -> Option<Key> {
        match did {
            DeviceId::Original => Some(Key::Original(self.original.clone())),
            DeviceId::Verified(i) => self
                .verified_keys
                .get(i as usize)
                .cloned()
                .map(Key::Created),
        }
    }

    /// Checks a signature `sig` on message `msg` using the public key from `did`.
    /// Returns `None` if the key doesn't exist.
    pub fn check_sig<V: VerificationAlgorithm>(
        &self,
        v: &V,
        did: DeviceId,
        msg: RawMsg,
        sig: RawSig,
    ) -> Option<SignatureValidity> {
        self.get_key(did).map(move |key| key.check_sig(v, msg, sig))
    }
}
