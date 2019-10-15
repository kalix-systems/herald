#![allow(clippy::new_without_default)]

use crate::*;
pub use chrono::prelude::*;
pub use serde::*;
pub use sodiumoxide::crypto::{box_, generichash as hash, sealedbox, sign};

new_type! {
    /// A unique identifier
    public UQ(32)
}

impl UQ {
    /// Generate a new `[UQ]`. Guaranteed never to collide with another instance.
    pub fn new() -> Self {
        use sodiumoxide::randombytes::randombytes_into;
        sodiumoxide::init().expect("failed to init libsodium - what have you done");
        let mut buf = [0u8; 32];
        randombytes_into(&mut buf);
        UQ(buf)
    }
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigValid {
    Yes,
    BadTime {
        signer_time: DateTime<Utc>,
        verify_time: DateTime<Utc>,
    },
    BadSign,
}

/// How far in the future a signature can be stamped and still considered valid, in seconds.
pub const TIMESTAMP_FUZZ: i64 = 3600;

/// A signed and dated piece of data.
/// A `Signed{data, timestamp, signer, sig}` is valid if and only if `sig` is a valid signature for
/// the device with id `signer` of a bytestring consisting of `data` followed by
/// `timestamp.timestamp`
#[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signed<T: AsRef<[u8]>> {
    data: T,
    timestamp: DateTime<Utc>,
    sig: sign::Signature,
    signed_by: sign::PublicKey,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigMeta {
    timestamp: DateTime<Utc>,
    sig: sign::Signature,
    signed_by: sign::PublicKey,
}

impl<T: AsRef<[u8]>> From<(T, SigMeta)> for Signed<T> {
    fn from(pair: (T, SigMeta)) -> Signed<T> {
        let (
            data,
            SigMeta {
                timestamp,
                sig,
                signed_by,
            },
        ) = pair;
        Signed {
            data,
            timestamp,
            sig,
            signed_by,
        }
    }
}

fn compute_signing_data<Tz: TimeZone>(slice: &[u8], ts: DateTime<Tz>) -> Vec<u8> {
    let mut out = Vec::with_capacity(slice.len() + 8);
    out.extend_from_slice(slice);
    out.extend_from_slice(&i64::to_le_bytes(ts.timestamp()));
    out
}

impl<T: AsRef<[u8]>> Signed<T> {
    pub fn into_data(self) -> T {
        self.split().0
    }

    pub fn sig(&self) -> sign::Signature {
        self.sig
    }

    pub fn split(self) -> (T, SigMeta) {
        let Signed {
            data,
            timestamp,
            sig,
            signed_by,
        } = self;
        let meta = SigMeta {
            timestamp,
            sig,
            signed_by,
        };
        (data, meta)
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn verify_sig(&self) -> SigValid {
        let verify_time = Utc::now();
        let signer_time = self.timestamp;
        let dat = compute_signing_data(self.data.as_ref(), signer_time);
        if !check_ts(signer_time, verify_time) {
            SigValid::BadTime {
                signer_time,
                verify_time,
            }
        } else if !sign::verify_detached(&self.sig, &dat, &self.signed_by) {
            SigValid::BadSign
        } else {
            SigValid::Yes
        }
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

fn check_ts<Tz: TimeZone>(signer_time: DateTime<Tz>, verify_time: DateTime<Tz>) -> bool {
    (signer_time <= verify_time)
        || ((verify_time.timestamp() - signer_time.timestamp()).abs() <= TIMESTAMP_FUZZ)
}

impl SigMeta {
    pub fn new(sig: sign::Signature, signed_by: sign::PublicKey, timestamp: DateTime<Utc>) -> Self {
        Self {
            sig,
            signed_by,
            timestamp,
        }
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn sig(&self) -> sign::Signature {
        self.sig
    }

    pub fn verify_sig(&self, msg: &[u8]) -> SigValid {
        let verify_time = Utc::now();
        let signer_time = self.timestamp;
        let signed = compute_signing_data(msg, signer_time);
        if !check_ts(signer_time, verify_time) {
            SigValid::BadTime {
                signer_time,
                verify_time,
            }
        } else if sign::verify_detached(&self.sig, &signed, &self.signed_by) {
            SigValid::BadSign
        } else {
            SigValid::Yes
        }
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

pub mod sig {
    use super::*;
    pub use sign::{PublicKey, Signature};

    pub const PUBLIC_KEY_BYTES: usize = sign::PUBLICKEYBYTES;
    pub const SIGNATURE_BYTES: usize = sign::SIGNATUREBYTES;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct PKMeta {
        sig: SigMeta,
        deprecated: Option<SigMeta>,
    }

    impl From<SigMeta> for PKMeta {
        fn from(sig: SigMeta) -> Self {
            PKMeta {
                sig,
                deprecated: None,
            }
        }
    }

    impl PKMeta {
        pub fn new(sig: SigMeta, deprecated: Option<SigMeta>) -> Self {
            Self { sig, deprecated }
        }

        pub fn key_is_valid(&self, key: PublicKey) -> bool {
            if let Some(d) = self.deprecated {
                if d.verify_sig(key.as_ref()) == SigValid::Yes {
                    return false;
                }
            }

            self.sig.verify_sig(key.as_ref()) == SigValid::Yes
        }

        pub fn deprecate(&mut self, deprecation: SigMeta) -> bool {
            if self.deprecated.is_some() {
                false
            } else {
                self.deprecated = Some(deprecation);
                true
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct KeyPair {
        public: sign::PublicKey,
        secret: sign::SecretKey,
    }

    impl KeyPair {
        pub fn gen_new() -> Self {
            sodiumoxide::init().expect("failed to init libsodium");
            let (public, secret) = sign::gen_keypair();
            KeyPair { public, secret }
        }

        // TODO: make this copy public key instead of referencing it
        pub fn public_key(&self) -> &sign::PublicKey {
            &self.public
        }

        pub fn secret_key(&self) -> &sign::SecretKey {
            &self.secret
        }

        pub fn sign<T: AsRef<[u8]>>(&self, data: T) -> Signed<T> {
            let timestamp = Utc::now();
            let to_sign = compute_signing_data(data.as_ref(), timestamp);
            let sig = sign::sign_detached(&to_sign, &self.secret);
            let signed = Signed {
                data,
                timestamp,
                sig,
                signed_by: self.public,
            };
            debug_assert!(signed.verify_sig() == SigValid::Yes);
            signed
        }

        pub fn raw_sign_detached(&self, data: &[u8]) -> Signature {
            sign::sign_detached(data, &self.secret)
        }

        pub fn sign_detached(&self, data: &[u8]) -> SigMeta {
            let timestamp = Utc::now();
            let to_sign = compute_signing_data(data, timestamp);
            let sig = sign::sign_detached(&to_sign, &self.secret);
            let meta = SigMeta {
                timestamp,
                sig,
                signed_by: self.public,
            };
            debug_assert!(meta.verify_sig(data) == SigValid::Yes);
            meta
        }
    }
}

pub mod sealed {
    use super::*;
    use std::ops::Deref;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PublicKey(pub Signed<box_::PublicKey>);

    impl Deref for PublicKey {
        type Target = Signed<box_::PublicKey>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl PublicKey {
        pub fn seal(&self, msg: &[u8]) -> Vec<u8> {
            sealedbox::seal(msg, &self.0.data)
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct KeyPair {
        sealed: box_::PublicKey,
        sk: box_::SecretKey,
    }

    impl KeyPair {
        pub fn gen_new() -> Self {
            sodiumoxide::init().expect("failed to init libsodium");
            let (sealed, sk) = box_::gen_keypair();
            KeyPair { sealed, sk }
        }

        pub fn public_key(&self) -> &box_::PublicKey {
            &self.sealed
        }

        // TODO: figure out if this will ever fail
        pub fn sign_pub(&self, pair: &sig::KeyPair) -> PublicKey {
            PublicKey(pair.sign(self.sealed))
        }

        pub fn open(&self, msg: &[u8]) -> Option<Vec<u8>> {
            sealedbox::open(msg, &self.sealed, &self.sk).ok()
        }
    }
}

pub fn hash_slice(slice: &[u8]) -> Option<[u8; 32]> {
    let mut state = hash::State::new(32, None).ok()?;
    state.update(slice).ok()?;
    let digest = state.finalize().ok()?;
    if digest.as_ref().len() != 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(digest.as_ref());
    Some(out)
}

pub fn hash_and_hex(slice: &[u8]) -> Option<String> {
    hash_slice(slice).map(|h| format!("{:x?}", h))
}
