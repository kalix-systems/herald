use crate::{new_type, random};
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

#[derive(Debug, Hash, Ser, De, Copy, Clone, Eq, PartialEq)]
pub struct Tag(Mac, Nonce);

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

    #[must_use = "you should definitely check if the decryption was successful"]
    pub fn open(
        &self,
        ad: &[u8],
        tag: Tag,
        msg: &mut [u8],
    ) -> bool {
        let Tag(mac, nonce) = tag;

        let res = unsafe {
            crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
                msg.as_mut_ptr(),
                // should always be null according to libsodium docs
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

        res == 0
    }

    pub fn seal_attached(
        &self,
        ad: &[u8],
        pt: &[u8],
    ) -> Vec<u8> {
        let mut output = vec![0; NONCE_LEN + MAC_LEN + pt.len()];

        let (nonce, rest) = output[..].split_at_mut(NONCE_LEN);
        // generate a random nonce
        random::gen_into(nonce);

        let res = unsafe {
            crypto_aead_xchacha20poly1305_ietf_encrypt(
                rest.as_mut_ptr(),
                rest.len() as _,
                pt.as_ptr(),
                pt.len() as _,
                ad.as_ptr(),
                ad.len() as _,
                // should always be null according to libsodium docs
                std::ptr::null(),
                nonce.as_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        assert_eq!(res, 0);

        output
    }

    #[must_use]
    pub fn open_attached(
        &self,
        ad: &[u8],
        ct: &[u8],
    ) -> Option<Vec<u8>> {
        let mut output = vec![0; ct.len() - NONCE_LEN - MAC_LEN];
        let (nonce, ct) = ct[..].split_at(NONCE_LEN);

        let res = unsafe {
            crypto_aead_xchacha20poly1305_ietf_decrypt(
                output.as_mut_ptr(),
                output.len() as _,
                // should always be null according to libsodium docs
                std::ptr::null_mut(),
                ct.as_ptr(),
                ct.len() as _,
                ad.as_ptr(),
                ad.len() as _,
                nonce.as_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        if res != 0 {
            None
        } else {
            Some(output)
        }
    }
}
