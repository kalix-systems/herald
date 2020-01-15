use super::*;
pub use crate::ed25519::*;

pub const SIGNATURE_LEN: usize = ffi::crypto_sign_ed25519_BYTES as usize;

new_type! {
    /// A signature
    public Signature(SIGNATURE_LEN)
}

impl SecretKey {
    pub fn sign(
        &self,
        data: &[u8],
    ) -> Signature {
        let mut sigbuf = [0u8; SIGNATURE_LEN];
        let mut siglen = 0;
        unsafe {
            ffi::crypto_sign_ed25519_detached(
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
    pub fn verify(
        &self,
        data: &[u8],
        sig: Signature,
    ) -> bool {
        let ret_code = unsafe {
            ffi::crypto_sign_ed25519_verify_detached(
                sig.0.as_ptr(),
                data.as_ptr(),
                data.len() as _,
                self.0.as_ptr(),
            )
        };

        ret_code == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sign_verify() {
        init();
        let keypair = KeyPair::gen_new();
        let mut data = [0u8; 64];
        for _ in 0..100 {
            random::gen_into(&mut data);
            let sig = keypair.secret().sign(&data);
            assert!(keypair.public().verify(&data, sig));
        }
    }
}
