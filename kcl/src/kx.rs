use super::*;
use kson::prelude::*;

pub const PUBLIC_KEY_LEN: usize = ffi::crypto_kx_PUBLICKEYBYTES as usize;
pub const SECRET_KEY_LEN: usize = ffi::crypto_kx_SECRETKEYBYTES as usize;
pub const SESSION_KEY_LEN: usize = ffi::crypto_kx_SESSIONKEYBYTES as usize;

new_type! {
    secret SecretKey(SECRET_KEY_LEN)
}

new_type! {
    public PublicKey(PUBLIC_KEY_LEN)
}

new_type! {
    secret SessionKey(SESSION_KEY_LEN)
}

#[derive(Ser, De, Clone, Debug)]
pub struct Session {
    pub rx: SessionKey,
    pub tx: SessionKey,
}

#[derive(Ser, De, Clone, Debug)]
pub struct KeyPair {
    pub public: PublicKey,
    pub secret: SecretKey,
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

    pub fn server_kx(
        &self,
        other: PublicKey,
    ) -> Session {
        let mut rx_buf = [0u8; SESSION_KEY_LEN];
        let mut tx_buf = [0u8; SESSION_KEY_LEN];
        let res = unsafe {
            ffi::crypto_kx_server_session_keys(
                rx_buf.as_mut_ptr(),
                tx_buf.as_mut_ptr(),
                self.public.as_ref().as_ptr(),
                self.secret.as_ref().as_ptr(),
                other.as_ref().as_ptr(),
            )
        };
        assert_eq!(res, 0);
        let rx = SessionKey(rx_buf);
        let tx = SessionKey(tx_buf);
        Session { rx, tx }
    }

    pub fn client_kx(
        &self,
        other: PublicKey,
    ) -> Session {
        let mut rx_buf = [0u8; SESSION_KEY_LEN];
        let mut tx_buf = [0u8; SESSION_KEY_LEN];
        let res = unsafe {
            ffi::crypto_kx_client_session_keys(
                rx_buf.as_mut_ptr(),
                tx_buf.as_mut_ptr(),
                self.public.as_ref().as_ptr(),
                self.secret.as_ref().as_ptr(),
                other.as_ref().as_ptr(),
            )
        };
        assert_eq!(res, 0);
        let rx = SessionKey(rx_buf);
        let tx = SessionKey(tx_buf);
        Session { rx, tx }
    }

    pub fn symmetric_kx_into(
        &self,
        other: PublicKey,
        secret_buf: &mut [u8],
    ) {
        // temprorary hack before I write a scalarmult module
        let mut p = [0u8; ffi::crypto_scalarmult_curve25519_BYTES as usize];
        unsafe {
            assert_eq!(
                0,
                ffi::crypto_scalarmult_curve25519(
                    p.as_mut_ptr(),
                    self.secret.as_ref().as_ptr(),
                    other.as_ref().as_ptr()
                )
            );
        }

        let mut hasher = hash::Builder::new().out_len(secret_buf.len()).build();
        hasher.update(&p);

        unsafe {
            ffi::sodium_memzero(
                p.as_mut_ptr() as *mut _,
                ffi::crypto_scalarmult_BYTES as usize,
            );
        }

        let mut salt_buf = [0u8; PUBLIC_KEY_LEN];
        for ((p1, p2), o) in self
            .public
            .as_ref()
            .iter()
            .zip(other.as_ref().iter())
            .zip(salt_buf.iter_mut())
        {
            *o = p1 ^ p2
        }

        hasher.update(&salt_buf);

        hasher.finalize_into(secret_buf);
    }
}
