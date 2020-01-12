use super::*;
use derive_getters::Getters;
use kson::prelude::*;
use typed_builder::TypedBuilder;

pub const PUBLIC_KEY_LEN: usize = ffi::crypto_core_ed25519_BYTES as usize;
pub const SECRET_KEY_LEN: usize = ffi::crypto_core_ed25519_SCALARBYTES as usize;
pub const SEED_LEN: usize = ffi::crypto_sign_ed25519_SEEDBYTES as usize;

new_type! {
    /// A public key for signing data
    public PublicKey(PUBLIC_KEY_LEN)
}

new_type! {
    /// A secret key for signing data
    secret SecretKey(SECRET_KEY_LEN)
}

new_type! {
    /// A seed for randomly generating a secret key
    secret Seed(SEED_LEN)
}

#[derive(Ser, De, Clone, Debug, Getters, TypedBuilder)]
pub struct KeyPair {
    pub(crate) public: PublicKey,
    pub(crate) secret: SecretKey,
}

impl KeyPair {
    pub fn gen_new() -> Self {
        let mut pk_buf = [0u8; PUBLIC_KEY_LEN];
        let mut sk_buf = [0u8; SECRET_KEY_LEN];
        let res = unsafe { ffi::crypto_sign_keypair(pk_buf.as_mut_ptr(), sk_buf.as_mut_ptr()) };
        assert_eq!(res, 0);
        let public = PublicKey(pk_buf);
        let secret = SecretKey(sk_buf);
        KeyPair { public, secret }
    }
}

impl Seed {
    pub fn gen_new() -> Self {
        let mut seed_buf = [0u8; SEED_LEN];
        random::gen_into(&mut seed_buf);
        Seed(seed_buf)
    }

    pub fn gen_keypair(&self) -> KeyPair {
        let mut pkbuf = [0; PUBLIC_KEY_LEN];
        let mut skbuf = [0; SECRET_KEY_LEN];
        let ret_code = unsafe {
            ffi::crypto_sign_ed25519_seed_keypair(
                pkbuf.as_mut_ptr(),
                skbuf.as_mut_ptr(),
                self.0.as_ptr(),
            )
        };
        assert_eq!(ret_code, 0);
        KeyPair {
            public: PublicKey(pkbuf),
            secret: SecretKey(skbuf),
        }
    }
}
