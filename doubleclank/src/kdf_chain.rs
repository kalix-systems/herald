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
    fn kdf(&self) -> (ChainKey, MessageKey) {
        let mut chain_out = [0u8; CHAIN_KEY_BYTES];
        let mut msg_out = [0u8; MSG_KEY_BYTES];

        crate::utils::kdf_derive(&(self.0).0, 0, 0, &mut chain_out);
        crate::utils::kdf_derive(&(self.0).0, 1, 0, &mut msg_out);

        let chain_key = ChainKey(secretbox::Key(chain_out));
        let msg_key = MessageKey(secretbox::Key(msg_out));

        (chain_key, msg_key)
    }
}

pub struct Chain {
    key: ChainKey,
}

impl Chain {
    pub fn new(key: ChainKey) -> Self {
        Chain { key }
    }

    fn ratchet(&mut self) -> MessageKey {
        let (new, res) = self.key.kdf();
        self.key = new;
        res
    }

    pub fn with_key<F, X>(&mut self, f: F) -> Option<X>
    where
        F: FnOnce(MessageKey) -> Option<X>,
    {
        let (new, msg_key) = self.key.kdf();
        let res = f(msg_key);
        if res.is_some() {
            self.key = new;
        }
        res
    }

    pub fn seal<'a>(&mut self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        self.ratchet().seal(msg)
    }

    pub fn open<'a>(&mut self, cipher: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        self.with_key(move |key| key.open(cipher))
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
        assert!(
            CHAIN_KEY_BYTES <= libsodium_sys::crypto_kdf_blake2b_BYTES_MAX as usize,
            "keygen will fail - chain keys too long"
        );
        assert!(
            libsodium_sys::crypto_kdf_blake2b_BYTES_MIN as usize <= CHAIN_KEY_BYTES,
            "keygen will fail - chain keys too short"
        );
    }
}
