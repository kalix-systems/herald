use serde::*;

use crate::*;

pub const PUB_BYTES: usize = 1793;
const PUB_BYTES_UPPER: usize = 2048;
pub const SEC_BYTES: usize = 2305;
const SEC_BYTES_UPPER: usize = 4096;
pub const SIG_BYTES: usize = 1330;
const SIG_BYTES_UPPER: usize = 2048;

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

#[derive(Serialize, Deserialize)]
pub struct Sec {
    #[serde(with = "asec")]
    inner: [u8; SEC_BYTES],
}

deref_struct!(Sec, [u8; SEC_BYTES], inner);
byte_array_hash!(Sec, inner);
byte_array_eq!(Sec, inner);

#[derive(Serialize, Deserialize)]
pub struct Sig {
    #[serde(with = "asig")]
    inner: [u8; SIG_BYTES],
}

deref_struct!(Sig, [u8; SIG_BYTES], inner);
byte_array_hash!(Sig, inner);
byte_array_eq!(Sig, inner);

#[derive(Hash, Serialize, Deserialize)]
pub struct Pair {
    pub pub_key: Pub,
    pub sec_key: Sec,
}
