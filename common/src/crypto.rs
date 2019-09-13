use crate::*;

pub(crate) use sodiumoxide::crypto::{
    aead::xchacha20poly1305_ietf as aead, box_, generichash as hash, sealedbox, sign,
};

/// How far in the future a signature can be stamped and still considered valid, in seconds.
pub const TIMESTAMP_FUZZ: i64 = 3600;

/// A signed and dated piece of data.
/// A `Signed{data, timestamp, signer, sig}` is valid if and only if `sig` is a valid signature for
/// the device with id `signer` of a bytestring consisting of `data` followed by
/// `timestamp.timestamp`
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
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

    pub fn verify_sig(&self) -> bool {
        let ctime = Utc::now();
        let stime = self.timestamp;
        if stime > ctime || (ctime.timestamp() - stime.timestamp()).abs() > TIMESTAMP_FUZZ {
            return false;
        }
        let dat = compute_signing_data(self.data.as_ref(), stime);
        sign::verify_detached(&self.sig, &dat, &self.signed_by)
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

impl SigMeta {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn verify_sig(&self, msg: &[u8]) -> bool {
        let ctime = Utc::now();
        let stime = self.timestamp;
        if stime > ctime || (ctime.timestamp() - stime.timestamp()).abs() > TIMESTAMP_FUZZ {
            return false;
        }
        let signed = compute_signing_data(msg, stime);
        sign::verify_detached(&self.sig, &signed, &self.signed_by)
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

pub mod sig {
    use super::*;

    pub type PublicKey = sign::PublicKey;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct PKMeta {
        sig: SigMeta,
        pub(crate) deprecated: Option<SigMeta>,
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
        pub fn key_is_valid(&self, key: PublicKey) -> bool {
            if let Some(d) = self.deprecated {
                if d.verify_sig(key.as_ref()) {
                    return false;
                }
            }

            self.sig.verify_sig(key.as_ref())
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

        pub fn public_key(&self) -> &sign::PublicKey {
            &self.public
        }

        pub fn sign<T: AsRef<[u8]>>(&self, data: T) -> Signed<T> {
            let timestamp = Utc::now();
            let to_sign = compute_signing_data(data.as_ref(), timestamp);
            let sig = sign::sign_detached(&to_sign, &self.secret);
            Signed {
                data,
                timestamp,
                sig,
                signed_by: self.public,
            }
        }

        pub fn sign_detached(&self, data: &[u8]) -> SigMeta {
            let timestamp = Utc::now();
            let to_sign = compute_signing_data(data, timestamp);
            let sig = sign::sign_detached(&to_sign, &self.secret);
            SigMeta {
                timestamp,
                sig,
                signed_by: self.public,
            }
        }
    }
}

pub mod pk {
    use super::*;
    use std::ops::Deref;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct PublicKey(Signed<box_::PublicKey>);

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
        pk: box_::PublicKey,
        sk: box_::SecretKey,
    }

    impl KeyPair {
        pub fn gen_new() -> Self {
            sodiumoxide::init().expect("failed to init libsodium");
            let (pk, sk) = box_::gen_keypair();
            KeyPair { pk, sk }
        }

        pub fn public_key(&self) -> &box_::PublicKey {
            &self.pk
        }

        // TODO: figure out if this will ever fail
        pub fn sign_pub(&self, pair: &sig::KeyPair) -> PublicKey {
            PublicKey(pair.sign(self.pk))
        }

        pub fn open(&self, msg: &[u8]) -> Option<Vec<u8>> {
            sealedbox::open(msg, &self.pk, &self.sk).ok()
        }
    }
}
