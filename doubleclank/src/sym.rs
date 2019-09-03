use sodiumoxide::crypto::secretbox;

use crate::prelude::*;

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Ciphertext {
    tag: secretbox::Tag,
    nonce: secretbox::Nonce,
    msg: BytesMut,
}

pub const MSG_KEY_BYTES: usize = secretbox::KEYBYTES;
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct MessageKey(pub(crate) secretbox::Key);

impl MessageKey {
    pub fn seal_in_place(&self, mut msg: BytesMut) -> Ciphertext {
        let nonce = secretbox::gen_nonce();
        let tag = secretbox::seal_detached(&mut msg, &nonce, &self.0);
        Ciphertext { tag, nonce, msg }
    }

    pub fn seal(&self, msg: &[u8]) -> Ciphertext {
        let buf = BytesMut::with_capacity(msg.len());
        self.seal_in_place(buf)
    }

    pub fn open(&self, cipher: Ciphertext) -> Option<BytesMut> {
        let Ciphertext {
            tag,
            nonce,
            mut msg,
        } = cipher;
        if secretbox::open_detached(&mut msg, &tag, &nonce, &self.0).is_ok() {
            Some(msg)
        } else {
            None
        }
    }
}
