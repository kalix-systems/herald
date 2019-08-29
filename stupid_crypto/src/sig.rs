use core::convert::TryFrom;
use pqcrypto_falcon::falcon512 as falcon;
use pqcrypto_traits::sign::*;
use serde::*;

use crate::*;

pub const SEC_BYTES: usize = 1281;
const SEC_BYTES_UPPER: usize = 2048;
pub const PUB_BYTES: usize = 897;
const PUB_BYTES_UPPER: usize = 1024;
pub const SIG_BYTES: usize = 690;
const SIG_BYTES_UPPER: usize = 768;

pub_secret_types!(falcon, SEC_BYTES_UPPER, PUB_BYTES_UPPER);

impl Pub {
    pub fn verify(&self, msg: &[u8], sig: &Sig) -> bool {
        falcon::verify_detached_signature(&sig.inner, msg, &self.inner).is_ok()
    }
}

impl Sec {
    pub fn sign(&self, msg: &[u8]) -> Sig {
        Sig {
            inner: falcon::detached_sign(msg, &self.inner),
        }
    }
}

serde_array!(asig, falcon::DetachedSignature, SIG_BYTES_UPPER);
byte_array_impls!(Sig, falcon::DetachedSignature, inner);

#[derive(Serialize, Deserialize)]
pub struct Sig {
    #[serde(with = "asig")]
    inner: falcon::DetachedSignature,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lengths_corr() {
        assert_eq!(SEC_BYTES, falcon::secret_key_bytes());
        assert_eq!(PUB_BYTES, falcon::public_key_bytes());
        assert_eq!(SIG_BYTES, falcon::signature_bytes());
    }

    const CHECK_ITERS: usize = 100;

    #[test]
    fn sig_works() {
        let pair = Pair::new();
        for i in 0..CHECK_ITERS {
            let msg = usize::to_le_bytes(i);
            let sig = pair.sec_key().sign(&msg);
            assert!(pair.pub_key().verify(&msg, &sig));
        }
    }
}
