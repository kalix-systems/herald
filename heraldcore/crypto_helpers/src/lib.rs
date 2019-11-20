use herald_common::*;
use std::convert::AsRef;

#[derive(Debug)]
pub enum Error {
    SpkToEpk,
    SskToEsk,
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        out: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        use Error::*;
        match self {
            SpkToEpk => write!(
                out,
                "failed to convert ed25519 public key to x25519 public key"
            ),
            SskToEsk => write!(
                out,
                "failed to convert ed25519 public key to x25519 public key"
            ),
        }
    }
}

impl std::error::Error for Error {}

pub fn spk_to_epk(pk: &sig::PublicKey) -> Result<box_::PublicKey, Error> {
    let mut pkbuf = [0u8; box_::PUBLICKEYBYTES];
    let ret = unsafe {
        libsodium_sys::crypto_sign_ed25519_pk_to_curve25519(
            pkbuf.as_mut_ptr(),
            pk.as_ref().as_ptr(),
        )
    };

    if ret == 0 {
        Ok(box_::PublicKey(pkbuf))
    } else {
        Err(Error::SpkToEpk)
    }
}

pub fn ssk_to_esk(sk: &sign::SecretKey) -> Result<box_::SecretKey, Error> {
    let mut skbuf = [0u8; box_::SECRETKEYBYTES];
    let ret = unsafe {
        libsodium_sys::crypto_sign_ed25519_sk_to_curve25519(
            skbuf.as_mut_ptr(),
            sk.as_ref().as_ptr(),
        )
    };
    if ret == 0 {
        Ok(box_::SecretKey(skbuf))
    } else {
        Err(Error::SskToEsk)
    }
}
