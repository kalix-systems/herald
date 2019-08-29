use crate::*;
use core::convert::{TryFrom, TryInto};
use pqcrypto_ntru::ntruhrss701 as ntru;
use pqcrypto_traits::kem::{PublicKey, SecretKey};

pub const SEC_BYTES: usize = 1450;
const SEC_BYTES_UPPER: usize = 2048;
pub const PUB_BYTES: usize = 1138;
const PUB_BYTES_UPPER: usize = 2048;
pub const CIPHER_BYTES: usize = 1138;
const CIPHER_BYTES_UPPER: usize = 2048;
pub const SHARED_BYTES: usize = 32;
pub const SHARED_BYTES_UPPER: usize = 32;

serde_array!(apub, PUB_BYTES, PUB_BYTES_UPPER);
serde_array!(asec, SEC_BYTES, SEC_BYTES_UPPER);
serde_array!(acipher, CIPHER_BYTES, CIPHER_BYTES_UPPER);
serde_array!(ashared, SHARED_BYTES, SHARED_BYTES_UPPER);

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
pub struct Cipher {
    #[serde(with = "acipher")]
    inner: [u8; CIPHER_BYTES],
}

deref_struct!(Cipher, [u8; CIPHER_BYTES], inner);
byte_array_hash!(Cipher, inner);
byte_array_eq!(Cipher, inner);
byte_array_from!(Cipher, CIPHER_BYTES);

#[derive(Serialize, Deserialize)]
pub struct Shared {
    #[serde(with = "ashared")]
    inner: [u8; SHARED_BYTES],
}

deref_struct!(Shared, [u8; SHARED_BYTES], inner);
byte_array_hash!(Shared, inner);
byte_array_eq!(Shared, inner);
byte_array_from!(Shared, SHARED_BYTES);

#[derive(Hash, Serialize, Deserialize)]
pub struct Pair {
    pub pub_key: Pub,
    pub sec_key: Sec,
}

impl Pair {
    fn new() -> Self {
        let (prepub, presec) = ntru::keypair();
        let pub_key: Pub = prepub.as_bytes().try_into().expect("pubkey had bad length");
        let sec_key: Sec = presec.as_bytes().try_into().expect("seckey had bad length");
        Pair { pub_key, sec_key }
    }
}
