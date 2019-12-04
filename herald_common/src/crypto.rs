#![allow(clippy::new_without_default)]

use crate::*;
use kcl::*;
use kson::*;

#[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigValid {
    Yes,
    BadTime {
        signer_time: Time,
        verify_time: Time,
    },
    BadSign,
}

/// How far in the future a signature can be stamped and still considered valid, in milliseconds.
pub const TIMESTAMP_FUZZ: i64 = 3_600_000;

/// A signed and dated piece of data.
/// A `Signed{data, timestamp, signer, sig}` is valid if and only if `sig` is a valid signature for
/// the device with id `signer` of a bytestring consisting of `data` followed by
/// `timestamp.timestamp`
#[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signed<T: AsRef<[u8]>> {
    data: T,
    timestamp: Time,
    sig: sign::Signature,
    signed_by: sign::PublicKey,
}

#[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigMeta {
    timestamp: Time,
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

fn compute_signing_data(
    slice: &[u8],
    ts: Time,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(slice.len() + 8);
    out.extend_from_slice(slice);
    out.extend_from_slice(&ts.to_le_bytes());
    out
}

fn verify_sig(
    slice: &[u8],
    signer_time: Time,
    sig: sign::Signature,
    signed_by: sign::PublicKey,
) -> SigValid {
    let verify_time = Time::now();
    let dat = compute_signing_data(slice, signer_time);
    let ts_valid = signer_time <= verify_time || signer_time.within(TIMESTAMP_FUZZ, verify_time);
    if !ts_valid {
        SigValid::BadTime {
            signer_time,
            verify_time,
        }
    } else if !signed_by.verify(&dat, sig) {
        SigValid::BadSign
    } else {
        SigValid::Yes
    }
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

    pub fn timestamp(&self) -> &Time {
        &self.timestamp
    }

    pub fn verify_sig(&self) -> SigValid {
        verify_sig(self.data.as_ref(), self.timestamp, self.sig, self.signed_by)
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

impl SigMeta {
    pub fn new(
        sig: sign::Signature,
        signed_by: sign::PublicKey,
        timestamp: Time,
    ) -> Self {
        Self {
            sig,
            signed_by,
            timestamp,
        }
    }

    pub fn timestamp(&self) -> &Time {
        &self.timestamp
    }

    pub fn sig(&self) -> sign::Signature {
        self.sig
    }

    pub fn verify_sig(
        &self,
        msg: &[u8],
    ) -> SigValid {
        verify_sig(msg, self.timestamp, self.sig, self.signed_by)
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

pub mod sig {
    use super::*;
    pub use sign::{PublicKey, Signature};

    pub const PUBLIC_KEY_BYTES: usize = sign::PUBLIC_KEY_LEN;
    pub const SIGNATURE_BYTES: usize = sign::SIGNATURE_LEN;

    #[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct PKMeta {
        pub sig: SigMeta,
        pub deprecated: Option<SigMeta>,
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
        pub fn new(
            sig: SigMeta,
            deprecated: Option<SigMeta>,
        ) -> Self {
            Self { sig, deprecated }
        }

        pub fn key_is_valid(
            &self,
            key: PublicKey,
        ) -> bool {
            if let Some(d) = self.deprecated {
                if d.verify_sig(key.as_ref()) == SigValid::Yes {
                    return false;
                }
            }

            self.sig.verify_sig(key.as_ref()) == SigValid::Yes
        }

        pub fn deprecate(
            &mut self,
            deprecation: SigMeta,
        ) -> bool {
            if self.deprecated.is_some() {
                false
            } else {
                self.deprecated = Some(deprecation);
                true
            }
        }
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct KeyPair {
        public: sign::PublicKey,
        secret: sign::SecretKey,
    }

    impl KeyPair {
        pub fn gen_new() -> Self {
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

        pub fn sign<T: AsRef<[u8]>>(
            &self,
            data: T,
        ) -> Signed<T> {
            let timestamp = Time::now();
            let to_sign = compute_signing_data(data.as_ref(), timestamp);
            let sig = self.secret.sign(&to_sign);
            let signed = Signed {
                data,
                timestamp,
                sig,
                signed_by: self.public,
            };
            debug_assert!(signed.verify_sig() == SigValid::Yes);
            signed
        }

        pub fn sign_detached(
            &self,
            data: &[u8],
        ) -> SigMeta {
            let timestamp = Time::now();
            let to_sign = compute_signing_data(data, timestamp);
            let sig = self.secret.sign(&to_sign);
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

// pub mod sealed {
//     use super::*;
//     use std::ops::Deref;

//     #[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
//     pub struct PublicKey(pub Signed<box_::PublicKey>);

//     impl Deref for PublicKey {
//         type Target = Signed<box_::PublicKey>;
//         fn deref(&self) -> &Self::Target {
//             &self.0
//         }
//     }

//     impl PublicKey {
//         pub fn seal(&self, msg: &[u8]) -> Vec<u8> {
//             sealedbox::seal(msg, &self.0.data)
//         }
//     }

//     #[derive(Ser, De, Debug, Clone)]
//     pub struct KeyPair {
//         sealed: box_::PublicKey,
//         sk: box_::SecretKey,
//     }

//     impl KeyPair {
//         pub fn gen_new() -> Self {
//             sodiumoxide::init().expect("failed to init libsodium");
//             let (sealed, sk) = box_::gen_keypair();
//             KeyPair { sealed, sk }
//         }

//         pub fn public_key(&self) -> &box_::PublicKey {
//             &self.sealed
//         }

//         // TODO: figure out if this will ever fail
//         pub fn sign_pub(&self, pair: &sig::KeyPair) -> PublicKey {
//             PublicKey(pair.sign(self.sealed))
//         }

//         pub fn open(&self, msg: &[u8]) -> Option<Vec<u8>> {
//             sealedbox::open(msg, &self.sealed, &self.sk).ok()
//         }
//     }
// }
