use sodiumoxide::crypto::secretbox;

pub struct Ciphertext<'a> {
    tag: secretbox::Tag,
    nonce: secretbox::Nonce,
    msg: &'a mut [u8],
}

pub const MSG_KEY_BYTES: usize = secretbox::KEYBYTES;
pub struct MessageKey(pub(crate) secretbox::Key);

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
