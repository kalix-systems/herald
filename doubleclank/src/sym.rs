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
    fn kdf(&self) -> (Self, MessageKey) {
        let mut output = [0u8; CHAIN_KEY_BYTES + MSG_KEY_BYTES];
        crate::utils::kdf_derive(self.0.as_ref(), 0, 0, &mut output);
        (
            ChainKey(secretbox::Key::from_slice(&output[..CHAIN_KEY_BYTES]).unwrap()),
            MessageKey(secretbox::Key::from_slice(&output[CHAIN_KEY_BYTES..]).unwrap()),
        )
    }

    fn ratchet(&mut self) -> MessageKey {
        let (key, msg) = self.kdf();
        self.0 = key.0;
        msg
    }

    pub fn seal<'a>(&mut self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        self.ratchet().seal(msg)
    }

    pub fn open<'a>(&mut self, cipher: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        let (chain_key, msg_key) = self.kdf();
        let res = msg_key.open(cipher);
        if res.is_some() {
            self.0 = chain_key.0;
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
