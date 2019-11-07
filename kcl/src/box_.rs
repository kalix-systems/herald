use crate::{hash, new_type, random};
use kson::prelude::*;
use libsodium_sys::*;

pub const SECRET_KEY_LEN: usize = crypto_box_curve25519xchacha20poly1305_SECRETKEYBYTES as usize;
pub const PUBLIC_KEY_LEN: usize = crypto_box_curve25519xchacha20poly1305_PUBLICKEYBYTES as usize;
pub const SEED_LEN: usize = crypto_box_curve25519xchacha20poly1305_SEEDBYTES as usize;
pub const NONCE_LEN: usize = crypto_box_curve25519xchacha20poly1305_NONCEBYTES as usize;
pub const MAC_LEN: usize = crypto_box_curve25519xchacha20poly1305_MACBYTES as usize;

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

new_type! {
    secret Seed(SEED_LEN)
}

pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut pk_buf = [0u8; PUBLIC_KEY_LEN];
    let mut sk_buf = [0u8; SECRET_KEY_LEN];

    unsafe {
        crypto_box_curve25519xchacha20poly1305_keypair(pk_buf.as_mut_ptr(), sk_buf.as_mut_ptr())
    };

    (PublicKey(pk_buf), SecretKey(sk_buf))
}

impl Seed {
    pub fn new() -> Self {
        let mut seed = [0u8; SEED_LEN];
        random::gen_into(&mut seed);
        Seed(seed)
    }

    pub fn gen_keypair(&self) -> (PublicKey, SecretKey) {
        let mut pk_buf = [0u8; PUBLIC_KEY_LEN];
        let mut sk_buf = [0u8; SECRET_KEY_LEN];

        unsafe {
            crypto_box_curve25519xchacha20poly1305_seed_keypair(
                pk_buf.as_mut_ptr(),
                sk_buf.as_mut_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        (PublicKey(pk_buf), SecretKey(sk_buf))
    }
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
        // we'll then xor this with the nonce to ensure nonce uniqueness
        // this makes encryption slightly slower but also harder to screw up
        let mut hasher = hash::Builder::new().out_len(NONCE_LEN).build();
        hasher.update(them.as_ref());
        hasher.update(msg);
        let hash = hasher.finalize();

        for (n, h) in nonce_buf.iter_mut().zip(hash.0) {
            *n ^= h;
        }

        let res = unsafe {
            crypto_box_curve25519xchacha20poly1305_detached(
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
            crypto_box_curve25519xchacha20poly1305_open_detached(
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
