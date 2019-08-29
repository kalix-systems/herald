use crate::*;
use core::convert::TryFrom;
use pqcrypto_ntru::ntruhrss701 as ntru;
use pqcrypto_traits::kem::*;

/// Length of secret key in bytes.
pub const SEC_BYTES: usize = 1450;
const SEC_BYTES_UPPER: usize = 2048;
/// Length of public key in bytes.
pub const PUB_BYTES: usize = 1138;
const PUB_BYTES_UPPER: usize = 2048;
/// Length of ciphertext in bytes.
pub const CIPHER_BYTES: usize = 1138;
const CIPHER_BYTES_UPPER: usize = 2048;
/// Length of shared secret in bytes.
pub const SHARED_BYTES: usize = 32;
const SHARED_BYTES_UPPER: usize = 32;

pub_secret_types!(ntru, SEC_BYTES_UPPER, PUB_BYTES_UPPER);

impl Pub {
    pub fn encapsulate(&self) -> Capsule {
        let (ss, ct) = ntru::encapsulate(&self.inner);

        let shared = Shared { inner: ss };
        let cipher = Cipher { inner: ct };

        Capsule { shared, cipher }
    }
}

impl Sec {
    pub fn decapsulate(&self, ct: &Cipher) -> Shared {
        Shared {
            inner: ntru::decapsulate(&ct.inner, &self.inner),
        }
    }
}

serde_array!(acipher, ntru::Ciphertext, CIPHER_BYTES_UPPER);
serde_array!(ashared, ntru::SharedSecret, SHARED_BYTES_UPPER);

#[derive(Serialize, Deserialize)]
pub struct Cipher {
    #[serde(with = "acipher")]
    inner: ntru::Ciphertext,
}

#[derive(Serialize, Deserialize)]
pub struct Shared {
    #[serde(with = "ashared")]
    inner: ntru::SharedSecret,
}

byte_array_impls!(Cipher, ntru::Ciphertext, inner);
byte_array_impls!(Shared, ntru::SharedSecret, inner);

#[derive(Hash)]
pub struct Capsule {
    shared: Shared,
    cipher: Cipher,
}

impl Capsule {
    pub fn shared(&self) -> &Shared {
        &self.shared
    }

    pub fn cipher(&self) -> &Cipher {
        &self.cipher
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lengths_corr() {
        assert_eq!(SEC_BYTES, ntru::secret_key_bytes());
        assert_eq!(PUB_BYTES, ntru::public_key_bytes());
        assert_eq!(CIPHER_BYTES, ntru::ciphertext_bytes());
        assert_eq!(SHARED_BYTES, ntru::shared_secret_bytes());
    }

    const CHECK_ITERS: usize = 1000;

    #[test]
    fn kem_works() {
        let pair = Pair::new();
        for _ in 0..CHECK_ITERS {
            let capsule = pair.pub_key().encapsulate();
            let ss = pair.sec_key().decapsulate(capsule.cipher());
            assert!(capsule.shared() == &ss);
        }
    }
}
