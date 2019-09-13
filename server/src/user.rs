use bytes::Bytes;
use chrono::prelude::*;
use failure::*;
use ring::signature::VerificationAlgorithm;
use serde::{Deserialize, Serialize};
use serde_cbor::{from_slice as deserialize, to_vec as serialize};
use untrusted::Input;

pub type UserId = arrayvec::ArrayString<[u8; 256]>;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct CreatedKey {
    key: Signed<RawKey>,
    deprecated: Option<Signed<DeviceId>>,
}

impl CreatedKey {
    pub fn new(key: Signed<RawKey>) -> Self {
        CreatedKey {
            key: key,
            deprecated: None,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct OriginalKey {
    raw: RawKey,
    created_on: DateTime<Utc>,
    deprecated: Option<Signed<DeviceId>>,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Original(OriginalKey),
    Created(CreatedKey),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct UserMeta {
    original: OriginalKey,
    verified_keys: Vec<CreatedKey>,
}

#[derive(Serialize, Deserialize, Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeprecationResult {
    /// returned if the key was already deprecated
    AlreadyDeprecated,
    Success,
}

#[derive(Serialize, Deserialize, Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignatureValidity {
    Valid,
    KeyInactive,
    BadSig,
}

impl Key {
    pub fn raw_key(&self) -> &RawKey {
        match self {
            Key::Original(o) => &o.raw,
            Key::Created(c) => &c.key.data,
        }
    }

    pub fn deprecated(&self) -> Option<&Signed<DeviceId>> {
        match self {
            Key::Original(o) => o.deprecated.as_ref(),
            Key::Created(c) => c.deprecated.as_ref(),
        }
    }

    pub fn created(&self) -> DateTime<Utc> {
        match self {
            Key::Original(o) => o.created_on,
            Key::Created(c) => c.key.timestamp,
        }
    }

    pub fn check_sig<V: VerificationAlgorithm>(
        &self,
        v: &V,
        msg: &RawMsg,
        sig: &RawSig,
    ) -> SignatureValidity {
        let raw = self.raw_key();
        if self.deprecated().is_some() {
            SignatureValidity::KeyInactive
        } else if v
            .verify(Input::from(raw), Input::from(msg), Input::from(sig))
            .is_ok()
        {
            SignatureValidity::Valid
        } else {
            SignatureValidity::BadSig
        }
    }
}

impl UserMeta {
    pub fn add_new_key(&mut self, key: CreatedKey) -> DeviceId {
        let ix = self.verified_keys.len() as u64;
        self.verified_keys.push(key);
        DeviceId::Verified(ix)
    }

    /// Returns `None` if the key at `deprecated.data` does not exist.
    pub fn deprecate_key(&mut self, deprecated: Signed<DeviceId>) -> Option<DeprecationResult> {
        match deprecated.data {
            DeviceId::Original => {
                if self.original.deprecated.is_some() {
                    Some(DeprecationResult::AlreadyDeprecated)
                } else {
                    self.original.deprecated = Some(deprecated);
                    Some(DeprecationResult::Success)
                }
            }
            DeviceId::Verified(ix) => {
                let key = self.verified_keys.get_mut(ix as usize)?;
                if key.deprecated.is_some() {
                    Some(DeprecationResult::AlreadyDeprecated)
                } else {
                    key.deprecated = Some(deprecated);
                    Some(DeprecationResult::Success)
                }
            }
        }
    }

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
        self.get_key(did)
            .map(move |key| key.check_sig(v, &msg, &sig))
    }

    pub fn new_signed<V: VerificationAlgorithm, T: Serialize>(
        &self,
        v: &V,
        signer: DeviceId,
        data: T,
        date: DateTime<Utc>,
        sig: RawSig,
    ) -> Result<Signed<T>, Error> {
        let key: Key = self
            .get_key(signer)
            .ok_or(format_err!("couldn't find key"))?;
        let msg: RawMsg = Bytes::from(serialize(&(&date, &data))?);
        match key.check_sig(v, &msg, &sig) {
            SignatureValidity::Valid => Ok(Signed {
                data: data,
                timestamp: date,
                signer: signer,
                sig: sig,
            }),
            SignatureValidity::BadSig => Err(format_err!("bad signature")),
            SignatureValidity::KeyInactive => Err(format_err!("key not active")),
        }
    }
}
