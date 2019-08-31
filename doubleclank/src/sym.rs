use sodiumoxide::crypto::secretbox;

use crate::*;

pub struct Ciphertext<'a> {
    tag: secretbox::Tag,
    nonce: secretbox::Nonce,
    msg: &'a mut [u8],
}

const MSG_KEY_BYTES: usize = secretbox::KEYBYTES;
pub struct MessageKey(secretbox::Key);

const CHAIN_KEY_BYTES: usize = secretbox::KEYBYTES;
pub struct ChainKey(secretbox::Key);

impl MessageKey {
    fn seal<'a>(&self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        let nonce = secretbox::gen_nonce();
        let tag = secretbox::seal_detached(msg, &nonce, &self.0);
        Ciphertext { tag, nonce, msg }
    }

    fn open<'a>(&self, cipher: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        let Ciphertext { tag, nonce, msg } = cipher;
        if secretbox::open_detached(msg, &tag, &nonce, &self.0).is_ok() {
            Some(msg)
        } else {
            None
        }
    }
}

pub struct Chain {
    msg: MessageId,
    ctx: ConversationId,
    key: ChainKey,
}

impl Chain {
    fn ratchet(&mut self) -> MessageKey {
        let mut output = [0u8; CHAIN_KEY_BYTES + MSG_KEY_BYTES];
        crate::utils::kdf_derive(self.key.0.as_ref(), self.msg.0, self.ctx.0, &mut output);
        self.key = ChainKey(secretbox::Key::from_slice(&output[..CHAIN_KEY_BYTES]).unwrap());
        MessageKey(secretbox::Key::from_slice(&output[CHAIN_KEY_BYTES..]).unwrap())
    }

    pub fn seal<'a>(&mut self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        self.ratchet().seal(msg)
    }

    pub fn open<'a>(&mut self, cipher: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        self.ratchet().open(cipher)
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
