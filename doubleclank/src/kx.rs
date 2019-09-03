use sodiumoxide::crypto::kx;

use crate::prelude::*;
use crate::{kdf_chain::Chain, sym::Ciphertext};

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct KeyPair {
    sec_key: kx::SecretKey,
    pub_key: kx::PublicKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let (pub_key, sec_key) = kx::gen_keypair();
        KeyPair { sec_key, pub_key }
    }

    pub fn pub_key(&self) -> &kx::PublicKey {
        &self.pub_key
    }

    pub fn sec_key(&self) -> &kx::SecretKey {
        &self.sec_key
    }
}

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Session {
    pub send_ratchet: Chain,
    pub recv_ratchet: Chain,
}

impl KeyPair {
    pub fn init_with(&self, them: &kx::PublicKey) -> Option<Session> {
        let (rx, tx) = kx::client_session_keys(&self.pub_key, &self.sec_key, them).ok()?;
        Some(Session {
            send_ratchet: Chain::new(rx),
            recv_ratchet: Chain::new(tx),
        })
    }

    pub fn recv_init(&self, them: &kx::PublicKey) -> Option<Session> {
        let (rx, tx) = kx::server_session_keys(&self.pub_key, &self.sec_key, them).ok()?;
        Some(Session {
            send_ratchet: Chain::new(tx),
            recv_ratchet: Chain::new(rx),
        })
    }
}

impl Session {
    pub fn send_msg(&mut self, msg: &[u8]) -> Ciphertext {
        self.send_ratchet.seal(msg)
    }

    pub fn recv_msg(&mut self, msg: Ciphertext) -> Option<BytesMut> {
        self.recv_ratchet.open(msg)
    }
}
