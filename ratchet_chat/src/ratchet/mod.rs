use super::*;
use bytes::*;
use kcl::*;
use kson::prelude::*;

pub const RATCHET_KEY_LEN: usize = 32;
new_type! {
    secret RatchetKey(RATCHET_KEY_LEN)
}

impl RatchetKey {
    pub fn gen_new() -> Self {
        let mut buf = [0u8; RATCHET_KEY_LEN];
        random::gen_into(&mut buf);
        RatchetKey(buf)
    }
}

pub mod double;
pub mod sender;
