use sodiumoxide::crypto::{self, secretbox};

pub struct Ciphertext<'a> {
    tag: secretbox::Tag,
    nonce: secretbox::Nonce,
    msg: &'a mut [u8],
}

pub const MSG_KEY_BYTES: usize = secretbox::KEYBYTES;
pub struct MessageKey(secretbox::Key);

pub const CHAIN_KEY_BYTES: usize = secretbox::KEYBYTES;
pub struct ChainKey(secretbox::Key);

impl From<crypto::kx::SessionKey> for ChainKey {
    fn from(sk: crypto::kx::SessionKey) -> Self {
        ChainKey(secretbox::Key(sk.0))
    }
}

impl MessageKey {
    pub fn seal<'a>(&self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        let nonce = secretbox::gen_nonce();
        let tag = secretbox::seal_detached(msg, &nonce, &self.0);
        Ciphertext { tag, nonce, msg }
    }

    pub fn open<'a>(&self, cipher: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        let Ciphertext { tag, nonce, msg } = cipher;
        if secretbox::open_detached(msg, &tag, &nonce, &self.0).is_ok() {
            Some(msg)
        } else {
            None
        }
    }
}

impl ChainKey {
    fn kdf(&self, idx: u64) -> MessageKey {
        let mut output = [0u8; MSG_KEY_BYTES];
        crate::utils::kdf_derive(self.0.as_ref(), idx, 0, &mut output);
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
            CHAIN_KEY_BYTES + MSG_KEY_BYTES <= libsodium_sys::crypto_kdf_blake2b_BYTES_MAX as usize,
            "keygen will fail - chain and msg keys too long"
        );
        assert!(
            libsodium_sys::crypto_kdf_blake2b_BYTES_MIN as usize <= CHAIN_KEY_BYTES + MSG_KEY_BYTES,
            "keygen will fail - chain and msg keys too short"
        );
    }
}
