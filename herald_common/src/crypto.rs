#![allow(clippy::new_without_default)]

use crate::*;
use kcl::*;
use kson::*;

#[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum SigValid {
    Yes,
    BadTime {
        signer_time: Time,
        verify_time: Time,
    },
    BadSign,
    BadSigner,
}

impl SigValid {
    pub fn and<F: FnOnce() -> SigValid>(
        self,
        other: F,
    ) -> SigValid {
        match self {
            SigValid::Yes => other(),
            s => s,
        }
    }
}

/// How far in the future a signature can be stamped and still considered valid, in milliseconds.
pub const TIMESTAMP_FUZZ: i64 = 3_600_000;

/// A signed and dated piece of data.
/// A `Signed{data, timestamp, signer, sig}` is valid if and only if `sig` is a valid signature for
/// the device with id `signer` of a bytestring consisting of `data` followed by
/// `timestamp.timestamp`
#[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signed<T> {
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

impl<T> From<(T, SigMeta)> for Signed<T> {
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

impl<T: Ser> Signed<T> {
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
        verify_sig(
            &kson::to_vec(&self.data),
            self.timestamp,
            self.sig,
            self.signed_by,
        )
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
    pub use sign::{KeyPair, PublicKey, Signature};

    pub const PUBLIC_KEY_BYTES: usize = sign::PUBLIC_KEY_LEN;
    pub const SIGNATURE_BYTES: usize = sign::SIGNATURE_LEN;

    pub fn sign_ser<T: Ser>(
        keys: &KeyPair,
        data: T,
    ) -> Signed<T> {
        let timestamp = Time::now();
        let to_sign = compute_signing_data(&kson::to_vec(&data), timestamp);
        let sig = keys.secret().sign(&to_sign);
        let signed = Signed {
            data,
            timestamp,
            sig,
            signed_by: *keys.public(),
        };
        debug_assert_eq!(signed.verify_sig(), SigValid::Yes);
        signed
    }

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SigUpdate {
        Endorse(Signed<UserId>),
        Deprecate(sig::PublicKey),
    }

    pub fn validate_update(sig: &Signed<SigUpdate>) -> SigValid {
        sig.verify_sig().and(|| match sig.data() {
            SigUpdate::Endorse(e) => e.verify_sig(),
            SigUpdate::Deprecate(_) => SigValid::Yes,
        })
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct SigChain {
        pub initial: Signed<UserId>,
        pub sig_chain: Vec<Signed<SigUpdate>>,
    }

    impl SigChain {
        pub fn validate(&self) -> SigValid {
            use std::collections::HashSet;
            use SigUpdate::*;

            self.sig_chain
                .iter()
                .fold(
                    (HashSet::new(), self.initial.verify_sig()),
                    |(mut valid_keys, status), update| {
                        let new_status = status.and(|| {
                            validate_update(update).and(|| {
                                if valid_keys.contains(update.signed_by()) {
                                    match update.data() {
                                        Endorse(e) => {
                                            valid_keys.insert(*e.signed_by());
                                        }
                                        Deprecate(k) => {
                                            valid_keys.remove(k);
                                        }
                                    }
                                    SigValid::Yes
                                } else {
                                    SigValid::BadSigner
                                }
                            })
                        });
                        (valid_keys, new_status)
                    },
                )
                .1
        }

        pub fn active_keys(&self) -> std::collections::HashSet<sig::PublicKey> {
            let mut keys = std::collections::HashSet::new();
            keys.insert(*self.initial.signed_by());
            for update in self.sig_chain.iter() {
                match &update.data {
                    SigUpdate::Endorse(e) => {
                        keys.insert(*e.signed_by());
                    }
                    SigUpdate::Deprecate(k) => {
                        keys.remove(k);
                    }
                }
            }
            keys
        }
    }
}

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prekey(pub kcl::kx::PublicKey);

impl Prekey {
    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        kcl::kx::PublicKey::from_slice(bytes).map(Self)
    }
}
