use sodiumoxide::crypto::kx;

use crate::sym::{ChainKey, Ciphertext};

pub struct RootKey(kx::SessionKey);
pub const ROOT_KEY_BYTES: usize = kx::SESSIONKEYBYTES;

pub struct KeyPair {
    sec_key: kx::SecretKey,
    pub_key: kx::PublicKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let (pub_key, sec_key) = kx::gen_keypair();
        KeyPair { sec_key, pub_key }
    }
}

pub struct Session {
    send_ratchet: ChainKey,
    recv_ratchet: ChainKey,
}

impl KeyPair {
    pub fn init_with(&self, them: &kx::PublicKey) -> Option<Session> {
        let (rx, tx) = kx::client_session_keys(&self.pub_key, &self.sec_key, them).ok()?;
        Some(Session {
            send_ratchet: rx.into(),
            recv_ratchet: tx.into(),
        })
    }

    pub fn recv_init(&self, them: &kx::PublicKey) -> Option<Session> {
        let (rx, tx) = kx::server_session_keys(&self.pub_key, &self.sec_key, them).ok()?;
        Some(Session {
            send_ratchet: tx.into(),
            recv_ratchet: rx.into(),
        })
    }
}

impl Session {
    pub fn send_msg<'a>(&mut self, msg: &'a mut [u8]) -> Ciphertext<'a> {
        self.send_ratchet.seal(msg)
    }

    pub fn recv_msg<'a>(&mut self, msg: Ciphertext<'a>) -> Option<&'a mut [u8]> {
        self.recv_ratchet.open(msg)
    }
}
