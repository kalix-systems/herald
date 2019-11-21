use crate::{hash, new_type, random};
use kson::prelude::*;
use libsodium_sys::*;

pub const KEY_LEN: usize = crypto_aead_xchacha20poly1305_ietf_KEYBYTES as usize;
pub const NONCE_LEN: usize = crypto_aead_xchacha20poly1305_ietf_NPUBBYTES as usize;
pub const MAC_LEN: usize = crypto_aead_xchacha20poly1305_ietf_ABYTES as usize;

new_type! {
    secret Key(KEY_LEN)
}

new_type! {
    nonce Nonce(NONCE_LEN)
}

new_type! {
    public Mac(MAC_LEN)
}

#[derive(Ser, De, Copy, Clone, Debug)]
pub struct Tag(Mac, Nonce);

#[must_use = "you should definitely check if the decryption was successful"]
pub struct OpenSucceeded(pub bool);

impl Key {
    pub fn new() -> Self {
        let mut buf = [0u8; KEY_LEN];
        random::gen_into(&mut buf);
        Key(buf)
    }

    pub fn seal(
        &self,
        ad: &[u8],
        msg: &mut [u8],
    ) -> Tag {
        let mut mac_buf = [0u8; MAC_LEN];
        let mut nonce_buf = [0u8; NONCE_LEN];

        // generate a random nonce
        random::gen_into(&mut nonce_buf);

        // take the hash code of the associated data and the message
        // we'll then xor this with the nonce to ensure nonce uniqueness
        // this makes encryption slightly slower but also harder to screw up
        let mut hasher = hash::Builder::new().out_len(NONCE_LEN).build();
        hasher.update(ad);
        hasher.update(msg);
        let hash = hasher.finalize();

        for (n, h) in nonce_buf.iter_mut().zip(hash.0) {
            *n ^= h;
        }

        let mut mac_len = 0u64;
        let res = unsafe {
            crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
                msg.as_mut_ptr(),
                mac_buf.as_mut_ptr(),
                (&mut mac_len) as _,
                msg.as_ptr(),
                msg.len() as _,
                ad.as_ptr(),
                ad.len() as _,
                // should always be null according to libsodium docs
                std::ptr::null(),
                nonce_buf.as_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        assert_eq!(res, 0);
        assert_eq!(mac_len, MAC_LEN as u64);

        Tag(Mac(mac_buf), Nonce(nonce_buf))
    }

    pub fn open(
        &self,
        ad: &[u8],
        tag: Tag,
        msg: &mut [u8],
    ) -> OpenSucceeded {
        let Tag(mac, nonce) = tag;

        let res = unsafe {
            crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
                msg.as_mut_ptr(),
                std::ptr::null_mut(),
                msg.as_ptr(),
                msg.len() as _,
                mac.as_ref().as_ptr(),
                ad.as_ptr(),
                ad.len() as _,
                nonce.as_ref().as_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        OpenSucceeded(res == 0)
    }
}
