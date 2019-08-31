use sodiumoxide::crypto::{kx, pwhash};

use crate::{sym, ConversationId, MessageId};

pub struct RootKey(kx::SessionKey);

const ROOT_KEY_BYTES: usize = kx::SESSIONKEYBYTES;

pub struct KeyPair {
    sec_key: kx::SecretKey,
    pub_key: kx::PublicKey,
}
