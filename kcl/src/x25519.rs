use super::*;
use derive_getters::Getters;
use kson::prelude::*;
use typed_builder::TypedBuilder;

pub const PUBLIC_KEY_LEN: usize = ffi::crypto_scalarmult_curve25519_BYTES as usize;
pub const SECRET_KEY_LEN: usize = ffi::crypto_scalarmult_curve25519_SCALARBYTES as usize;
pub const SEED_LEN: usize = ffi::crypto_box_curve25519xchacha20poly1305_SEEDBYTES as usize;

new_type! {
    public PublicKey(PUBLIC_KEY_LEN)
}

new_type! {
    secret SecretKey(SECRET_KEY_LEN)
}

new_type! {
    secret Seed(SEED_LEN)
}

impl From<ed25519::PublicKey> for PublicKey {
    fn from(key: ed25519::PublicKey) -> Self {
        let mut pkbuf = [0u8; PUBLIC_KEY_LEN];
        let res = unsafe {
            ffi::crypto_sign_ed25519_pk_to_curve25519(pkbuf.as_mut_ptr(), key.as_ref().as_ptr())
        };
        assert_eq!(res, 0);
        PublicKey(pkbuf)
    }
}

impl From<ed25519::SecretKey> for SecretKey {
    fn from(key: ed25519::SecretKey) -> Self {
        let mut pkbuf = [0u8; SECRET_KEY_LEN];
        let res = unsafe {
            ffi::crypto_sign_ed25519_sk_to_curve25519(pkbuf.as_mut_ptr(), key.as_ref().as_ptr())
        };
        assert_eq!(res, 0);
        SecretKey(pkbuf)
    }
}

impl Seed {
    pub fn gen_new() -> Self {
        let mut seed = [0u8; SEED_LEN];
        random::gen_into(&mut seed);
        Seed(seed)
    }

    pub fn gen_keypair(&self) -> KeyPair {
        let mut pk_buf = [0u8; PUBLIC_KEY_LEN];
        let mut sk_buf = [0u8; SECRET_KEY_LEN];

        unsafe {
            ffi::crypto_box_curve25519xchacha20poly1305_seed_keypair(
                pk_buf.as_mut_ptr(),
                sk_buf.as_mut_ptr(),
                self.as_ref().as_ptr(),
            )
        };

        KeyPair {
            public: PublicKey(pk_buf),
            secret: SecretKey(sk_buf),
        }
    }
}

#[derive(Ser, De, Clone, Debug, Getters, TypedBuilder)]
pub struct KeyPair {
    pub(crate) public: PublicKey,
    pub(crate) secret: SecretKey,
}

impl From<ed25519::KeyPair> for KeyPair {
    fn from(keys: ed25519::KeyPair) -> Self {
        KeyPair {
            public: keys.public.into(),
            secret: keys.secret.into(),
        }
    }
}

impl KeyPair {
    pub fn gen_new() -> Self {
        let mut pk_buf = [0u8; PUBLIC_KEY_LEN];
        let mut sk_buf = [0u8; SECRET_KEY_LEN];
        let res = unsafe { ffi::crypto_kx_keypair(pk_buf.as_mut_ptr(), sk_buf.as_mut_ptr()) };
        assert_eq!(res, 0);
        let public = PublicKey(pk_buf);
        let secret = SecretKey(sk_buf);
        KeyPair { public, secret }
    }
}
