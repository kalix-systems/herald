use super::*;
pub use crate::x25519::*;
use kson::prelude::*;

pub const NONCE_LEN: usize = ffi::crypto_box_curve25519xchacha20poly1305_NONCEBYTES as usize;
pub const MAC_LEN: usize = ffi::crypto_box_curve25519xchacha20poly1305_MACBYTES as usize;

new_type! {
    nonce Nonce(NONCE_LEN)
}

new_type! {
    public Mac(MAC_LEN)
}

pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut pk_buf = [0u8; PUBLIC_KEY_LEN];
    let mut sk_buf = [0u8; SECRET_KEY_LEN];

    unsafe {
        ffi::crypto_box_curve25519xchacha20poly1305_keypair(
            pk_buf.as_mut_ptr(),
            sk_buf.as_mut_ptr(),
        )
    };

    (PublicKey(pk_buf), SecretKey(sk_buf))
}

#[derive(Debug, Hash, Ser, De, Copy, Clone, Eq, PartialEq)]
pub struct Tag(Mac, Nonce);

#[must_use = "you should definitely check if the decryption was successful"]
pub struct OpenSucceeded(pub bool);

impl SecretKey {
    pub fn seal(
        &self,
        them: PublicKey,
        msg: &mut [u8],
    ) -> Tag {
        let mut mac_buf = [0u8; MAC_LEN];
        let mut nonce_buf = [0u8; NONCE_LEN];

        // generate a random nonce
        random::gen_into(&mut nonce_buf);

        let res = unsafe {
            ffi::crypto_box_curve25519xchacha20poly1305_detached(
                msg.as_mut_ptr(),
                mac_buf.as_mut_ptr(),
                msg.as_ptr(),
                msg.len() as _,
                nonce_buf.as_ptr(),
                them.as_ref().as_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        assert_eq!(res, 0);

        let mac = Mac(mac_buf);
        let nonce = Nonce(nonce_buf);

        Tag(mac, nonce)
    }

    pub fn open(
        &self,
        them: PublicKey,
        tag: Tag,
        msg: &mut [u8],
    ) -> OpenSucceeded {
        let Tag(mac, nonce) = tag;

        let res = unsafe {
            ffi::crypto_box_curve25519xchacha20poly1305_open_detached(
                msg.as_mut_ptr(),
                msg.as_ptr(),
                mac.as_ref().as_ptr(),
                msg.len() as _,
                nonce.as_ref().as_ptr(),
                them.as_ref().as_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        OpenSucceeded(res == 0)
    }
}
