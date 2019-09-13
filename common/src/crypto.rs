use crate::*;

use sodiumoxide::crypto::{
    aead::xchacha20poly1305_ietf as aead, box_, generichash as hash, sealedbox, sign,
};

/// A signed and dated piece of data.
/// A `Signed{data, timestamp, signer, sig}` is valid if and only if `sig` is a valid signature for
/// the device with id `signer` of `(data, timestamp)` serialized with `CBOR`.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct Signed<T: Serialize> {
    data: T,
    timestamp: DateTime<Utc>,
    sig: sign::Signature,
    signed_by: sign::PublicKey,
}

impl<T: Serialize> Signed<T> {
    pub fn into_data(self) -> T {
        self.data
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn compute_signing_data(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(&(&self.timestamp, &self.data))
    }

    pub fn verify_sig(&self) -> Result<bool, serde_cbor::Error> {
        Ok(sign::verify_detached(
            &self.sig,
            &self.compute_signing_data()?,
            &self.signed_by,
        ))
    }

    pub fn signed_by(&self) -> &sign::PublicKey {
        &self.signed_by
    }
}

pub mod sig {
    use super::*;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct PublicKey {
        key: Signed<sign::PublicKey>,
        deprecated: Option<Signed<DeviceId>>,
    }

    impl PublicKey {
        pub fn key(&self) -> &Signed<sign::PublicKey> {
            &self.key
        }

        pub fn deprecated(&self) -> Option<&Signed<DeviceId>> {
            self.deprecated.as_ref()
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct KeyPair {
        public: sign::PublicKey,
        secret: sign::SecretKey,
    }

    impl KeyPair {
        pub fn new() -> Self {
            sodiumoxide::init().expect("failed to init libsodium");
            let (public, secret) = sign::gen_keypair();
            KeyPair { public, secret }
        }

        pub fn public_key(&self) -> &sign::PublicKey {
            &self.public
        }

        pub fn sign<T: Serialize>(&self, data: T) -> Result<Signed<T>, serde_cbor::Error> {
            let timestamp = Utc::now();
            let to_sign = serde_cbor::to_vec(&(&timestamp, &data))?;
            let sig = sign::sign_detached(&to_sign, &self.secret);
            Ok(Signed {
                data,
                timestamp,
                sig,
                signed_by: self.public,
            })
        }
    }
}

pub mod pk {
    use super::*;

    pub struct PublicKey(Signed<box_::PublicKey>);

    impl PublicKey {
        pub fn signed_by(&self) -> &sign::PublicKey {
            self.0.signed_by()
        }

        pub fn timestamp(&self) -> &DateTime<Utc> {
            self.0.timestamp()
        }

        pub fn check_sig(&self) -> Result<bool, serde_cbor::Error> {
            self.0.verify_sig()
        }
    }
}
