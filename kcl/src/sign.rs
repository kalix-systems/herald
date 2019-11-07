use crate::new_type;

use libsodium_sys::*;

pub const SECRET_KEY_LEN: usize = crypto_sign_ed25519_SECRETKEYBYTES as usize;
pub const PUBLIC_KEY_LEN: usize = crypto_sign_ed25519_PUBLICKEYBYTES as usize;
pub const SIGNATURE_LEN: usize = crypto_sign_ed25519_BYTES as usize;
pub const SEED_LEN: usize = crypto_sign_ed25519_SEEDBYTES as usize;

new_type! {
    /// A secret key for signing data
    secret SecretKey(SECRET_KEY_LEN)
}

new_type! {
    /// A seed for randomly generating a secret key
    secret Seed(SEED_LEN)
}

new_type! {
    /// A public key for signing data
    public PublicKey(PUBLIC_KEY_LEN)
}

new_type! {
    /// A signature
    public Signature(SIGNATURE_LEN)
}

impl SecretKey {
    pub fn sign(&self, data: &[u8]) -> Signature {
        let mut sigbuf = [0u8; SIGNATURE_LEN];
        let mut siglen = 0;
        unsafe {
            crypto_sign_ed25519_detached(
                sigbuf.as_mut_ptr(),
                &mut siglen,
                data.as_ptr(),
                data.len() as _,
                self.0.as_ptr(),
            );
        }
        assert_eq!(siglen, SIGNATURE_LEN as _);
        Signature(sigbuf)
    }
}

impl PublicKey {
    pub fn verify(&self, data: &[u8], sig: Signature) -> bool {
        let ret_code = unsafe {
            crypto_sign_ed25519_verify_detached(
                sig.0.as_ptr(),
                data.as_ptr(),
                data.len() as _,
                self.0.as_ptr(),
            )
        };

        ret_code == 0
    }
}

pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut pkbuf = [0; PUBLIC_KEY_LEN];
    let mut skbuf = [0; SECRET_KEY_LEN];
    let ret_code = unsafe { crypto_sign_ed25519_keypair(pkbuf.as_mut_ptr(), skbuf.as_mut_ptr()) };
    assert_eq!(ret_code, 0);
    (PublicKey(pkbuf), SecretKey(skbuf))
}

impl Seed {
    pub fn gen_keypair(&self) -> (PublicKey, SecretKey) {
        let mut pkbuf = [0; PUBLIC_KEY_LEN];
        let mut skbuf = [0; SECRET_KEY_LEN];
        let ret_code = unsafe {
            crypto_sign_ed25519_seed_keypair(
                pkbuf.as_mut_ptr(),
                skbuf.as_mut_ptr(),
                self.0.as_ptr(),
            )
        };
        assert_eq!(ret_code, 0);
        (PublicKey(pkbuf), SecretKey(skbuf))
    }
}
