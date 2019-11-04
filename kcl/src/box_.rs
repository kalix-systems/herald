use crate::{hash, new_type, random};
use kson::prelude::*;
use libsodium_sys::*;

pub const SECRET_KEY_LEN: usize = crypto_box_SECRETKEYBYTES as usize;
pub const PUBLIC_KEY_LEN: usize = crypto_box_PUBLICKEYBYTES as usize;
pub const SEED_LEN: usize = crypto_box_SEEDBYTES as usize;
pub const NONCE_LEN: usize = crypto_box_NONCEBYTES as usize;
pub const MAC_LEN: usize = crypto_box_MACBYTES as usize;

new_type! {
    secret SecretKey(SECRET_KEY_LEN)
}

new_type! {
    public PublicKey(PUBLIC_KEY_LEN)
}

new_type! {
    nonce Nonce(NONCE_LEN)
}

new_type! {
    public Mac(MAC_LEN)
}

#[derive(Ser, De, Copy, Clone)]
pub struct Tag(Mac, Nonce);

#[must_use = "you should definitely check if the decryption was successful"]
pub struct OpenSucceeded(pub bool);

impl SecretKey {
    pub fn seal(&self, them: PublicKey, msg: &mut [u8]) -> Tag {
        let mut mac_buf = [0u8; MAC_LEN];
        let mut nonce_buf = [0u8; NONCE_LEN];

        // generate a random nonce
        random::gen_into(&mut nonce_buf);

        // take the hash code of their public key and the msg
        let mut hasher = hash::Builder::new().out_len(NONCE_LEN).build();
        hasher.update(them.as_ref());
        hasher.update(msg);
        let hash = hasher.finalize();

        // xor the hash code with the nonce to ensure nonce uniqueness
        for (n, h) in nonce_buf.iter_mut().zip(hash.0) {
            *n ^= h;
        }

        let res = unsafe {
            crypto_box_detached(
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

    pub fn open(&self, them: PublicKey, tag: Tag, msg: &mut [u8]) -> OpenSucceeded {
        let Tag(mac, nonce) = tag;

        let res = unsafe {
            crypto_box_open_detached(
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
