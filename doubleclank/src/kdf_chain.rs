use sodiumoxide::crypto::{kx, secretbox};

use crate::sym::*;

pub const CHAIN_KEY_BYTES: usize = secretbox::KEYBYTES;
pub struct ChainKey(secretbox::Key);

impl From<kx::SessionKey> for ChainKey {
    fn from(sk: kx::SessionKey) -> Self {
        ChainKey(secretbox::Key(sk.0))
    }
}

impl ChainKey {
    fn kdf(&self, idx: u64) -> MessageKey {
        let mut output = [0u8; MSG_KEY_BYTES];
        crate::utils::kdf_derive(&(self.0).0, idx, 0, &mut output);
        MessageKey(secretbox::Key(output))
    }
}

pub struct Chain {
    msg_id: u64,
    key: ChainKey,
}

impl Chain {
    pub fn new(key: ChainKey) -> Self {
        Chain { msg_id: 0, key }
    }

    fn kdf(&self) -> MessageKey {
        self.key.kdf(self.msg_id)
    }

    fn ratchet(&mut self) -> MessageKey {
        let res = self.kdf();
        self.msg_id += 1;
        res
    }

    pub fn seal<'a>(&mut self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        self.ratchet().seal(msg)
    }

    pub fn open<'a>(&mut self, cipher: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        let msg_key = self.kdf();
        let res = msg_key.open(cipher);
        if res.is_some() {
            self.msg_id += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_length_relations() {
        assert!(
            MSG_KEY_BYTES <= libsodium_sys::crypto_kdf_blake2b_BYTES_MAX as usize,
            "keygen will fail - msg keys too long"
        );
        assert!(
            libsodium_sys::crypto_kdf_blake2b_BYTES_MIN as usize <= MSG_KEY_BYTES,
            "keygen will fail - msg keys too short"
        );
    }
}
