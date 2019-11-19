pub use kcl;

use bytes::BytesMut;
use kcl::*;
use kson::prelude::*;
use std::ops::DerefMut;

pub const CHAIN_KEY_BYTES: usize = 64;
new_type! {
    /// A secret key that is used in the kdf ratchet
    secret ChainKey(CHAIN_KEY_BYTES)
}

impl ChainKey {
    pub fn new() -> Self {
        let mut buf = [0u8; CHAIN_KEY_BYTES];
        random::gen_into(&mut buf);
        ChainKey(buf)
    }
}

#[derive(Debug, Clone, Ser, De)]
pub struct ChainState {
    ix: u64,
    base_key: hash::Key,
    chain_key: ChainKey,
}

impl ChainState {
    pub fn new() -> Self {
        let base_key = hash::Key::new();
        let chain_key = ChainKey::new();

        ChainState {
            ix: 0,
            base_key,
            chain_key,
        }
    }
}

#[must_use = "you should check if the decryption was successful and store the generated message keys"]
pub enum DecryptionResult {
    Success {
        extra_keys: Vec<(u64, aead::Key)>,
        plaintext: BytesMut,
    },
    IndexTooHigh {
        cipher_index: u64,
        chain_index: u64,
    },
    Failed {
        extra_keys: Vec<(u64, aead::Key)>,
    },
}

#[must_use = "you should make sure to store the message key in case anyone else uses it"]
pub struct CipherData {
    ix: u64,
    key: aead::Key,
    cipher: Cipher,
}

impl CipherData {
    pub fn destruct(self) -> (u64, aead::Key, Cipher) {
        (self.ix, self.key, self.cipher)
    }
}

impl ChainState {
    pub fn ix(&self) -> u64 {
        self.ix
    }

    fn kdf(&mut self) -> aead::Key {
        let mut chainkey_buf = [0u8; CHAIN_KEY_BYTES];
        let mut messagekey_buf = [0u8; aead::KEY_LEN];

        let mut bufs: [&mut [u8]; 2] = [&mut chainkey_buf, &mut messagekey_buf];
        self.base_key.hash_into_many(
            self.chain_key.as_ref(),
            bufs.iter_mut().map(DerefMut::deref_mut),
        );

        self.ix += 1;
        self.chain_key = ChainKey(chainkey_buf);

        aead::Key(messagekey_buf)
    }

    pub fn open(&mut self, cipher: Cipher) -> DecryptionResult {
        let Cipher {
            index,
            tag,
            ad,
            msg,
        } = cipher;

        let num_extra = index.saturating_add(1).saturating_sub(self.ix);

        if num_extra == 0 {
            return DecryptionResult::IndexTooHigh {
                cipher_index: index,
                chain_index: self.ix,
            };
        }

        let mut extra_keys = Vec::with_capacity(num_extra as usize);

        for i in self.ix..index {
            let key = self.kdf();
            extra_keys.push((i, key));
        }

        let key = self.kdf();
        extra_keys.push((index, key.clone()));

        let mut msg = BytesMut::from(msg);

        if key.open(&ad, tag, &mut msg).0 {
            DecryptionResult::Success {
                extra_keys,
                plaintext: msg,
            }
        } else {
            DecryptionResult::Failed { extra_keys }
        }
    }

    pub fn seal(&mut self, ad: Bytes, mut msg: BytesMut) -> CipherData {
        let ix = self.ix;

        let key = self.kdf();
        let tag = key.seal(&ad, &mut msg);
        let msg = msg.freeze();

        let cipher = Cipher {
            index: ix,
            tag,
            ad,
            msg,
        };

        CipherData { ix, key, cipher }
    }
}

#[derive(Debug, Clone, Ser, De)]
pub struct Cipher {
    index: u64,
    tag: aead::Tag,
    ad: Bytes,
    msg: Bytes,
}
