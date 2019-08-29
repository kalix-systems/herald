use core::convert::{TryFrom, TryInto};
use pqcrypto_falcon::falcon512 as falcon;
use pqcrypto_traits::sign::{PublicKey, SecretKey};
use serde::*;

use crate::*;

pub const SEC_BYTES: usize = 1281;
const SEC_BYTES_UPPER: usize = 2048;
pub const PUB_BYTES: usize = 897;
const PUB_BYTES_UPPER: usize = 1024;
pub const SIG_BYTES: usize = 690;
const SIG_BYTES_UPPER: usize = 768;

serde_array!(apub, PUB_BYTES, PUB_BYTES_UPPER);
serde_array!(asec, SEC_BYTES, SEC_BYTES_UPPER);
serde_array!(asig, SIG_BYTES, SIG_BYTES_UPPER);

#[derive(Serialize, Deserialize)]
pub struct Pub {
    #[serde(with = "apub")]
    inner: [u8; PUB_BYTES],
}

deref_struct!(Pub, [u8; PUB_BYTES], inner);
byte_array_hash!(Pub, inner);
byte_array_eq!(Pub, inner);
byte_array_from!(Pub, PUB_BYTES);

#[derive(Serialize, Deserialize)]
pub struct Sec {
    #[serde(with = "asec")]
    inner: [u8; SEC_BYTES],
}
deref_struct!(Sec, [u8; SEC_BYTES], inner);
byte_array_hash!(Sec, inner);
byte_array_eq!(Sec, inner);
byte_array_from!(Sec, SEC_BYTES);

#[derive(Serialize, Deserialize)]
pub struct Sig {
    #[serde(with = "asig")]
    inner: [u8; SIG_BYTES],
}

deref_struct!(Sig, [u8; SIG_BYTES], inner);
byte_array_hash!(Sig, inner);
byte_array_eq!(Sig, inner);
byte_array_from!(Sig, SIG_BYTES);

#[derive(Hash, Serialize, Deserialize)]
pub struct Pair {
    pub pub_key: Pub,
    pub sec_key: Sec,
}

impl Pair {
    fn new() -> Self {
        let (prepub, presec) = falcon::keypair();
        let pub_key: Pub = prepub.as_bytes().try_into().expect("pubkey had bad length");
        let sec_key: Sec = presec.as_bytes().try_into().expect("seckey had bad length");
        Pair { pub_key, sec_key }
    }
}
