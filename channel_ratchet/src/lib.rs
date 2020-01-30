pub use kcl;

use bytes::BytesMut;
use kcl::*;
use kson::prelude::*;
use std::ops::DerefMut;

pub const RATCHET_KEY_LEN: usize = 64;
new_type! {
    /// A secret key that is used in the kdf ratchet
    secret RatchetKey(RATCHET_KEY_LEN)
}

impl RatchetKey {
    pub fn gen_new() -> Self {
        let mut buf = [0u8; RATCHET_KEY_LEN];
        random::gen_into(&mut buf);
        RatchetKey(buf)
    }
}

#[derive(Debug, Clone, Ser, De, Eq, PartialEq)]
pub struct RatchetState {
    ix: u64,
    base_key: hash::Key,
    ratchet_key: RatchetKey,
}

impl RatchetState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let base_key = hash::Key::gen_new();
        let ratchet_key = RatchetKey::gen_new();

        RatchetState {
            ix: 0,
            base_key,
            ratchet_key,
        }
    }

    pub fn mk(
        ix: u64,
        base_key: hash::Key,
        ratchet_key: RatchetKey,
    ) -> Self {
        RatchetState {
            ix,
            base_key,
            ratchet_key,
        }
    }

    pub fn ix(&self) -> u64 {
        self.ix
    }

    pub fn base_key(&self) -> &hash::Key {
        &self.base_key
    }

    pub fn ratchet_key(&self) -> &RatchetKey {
        &self.ratchet_key
    }
}

#[must_use = "you should check if the decryption was successful and store the generated message keys"]
pub enum DecryptionResult {
    Success {
        extra_keys: Vec<(u64, aead::Key)>,
        ad: Bytes,
        pt: BytesMut,
    },
    IndexTooHigh {
        cipher_index: u64,
        ratchet_index: u64,
    },
    Failed {
        extra_keys: Vec<(u64, aead::Key)>,
    },
}

impl DecryptionResult {
    fn extra_keys_mut(&mut self) -> Option<&mut Vec<(u64, aead::Key)>> {
        use DecryptionResult::*;
        match self {
            Success { extra_keys, .. } => Some(extra_keys),
            Failed { extra_keys, .. } => Some(extra_keys),
            IndexTooHigh { .. } => None,
        }
    }
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

impl RatchetState {
    fn kdf(&mut self) -> aead::Key {
        let mut ratchetkey_buf = [0u8; RATCHET_KEY_LEN];
        let mut messagekey_buf = [0u8; aead::KEY_LEN];

        let mut bufs: [&mut [u8]; 2] = [&mut ratchetkey_buf, &mut messagekey_buf];
        self.base_key.hash_into_many(
            self.ratchet_key.as_ref(),
            bufs.iter_mut().map(DerefMut::deref_mut),
        );

        self.ix += 1;
        self.ratchet_key = RatchetKey(ratchetkey_buf);

        aead::Key(messagekey_buf)
    }

    pub fn open(
        &mut self,
        cipher: Cipher,
    ) -> DecryptionResult {
        let num_extra = cipher.index.saturating_add(1).saturating_sub(self.ix);

        if num_extra == 0 {
            return DecryptionResult::IndexTooHigh {
                cipher_index: cipher.index,
                ratchet_index: self.ix,
            };
        }

        let mut extra_keys = Vec::with_capacity(num_extra as usize);

        for i in self.ix..cipher.index {
            let key = self.kdf();
            extra_keys.push((i, key));
        }

        let key = self.kdf();
        extra_keys.push((cipher.index, key.clone()));

        let mut res = cipher.open_with(key);

        if let Some(extra) = res.extra_keys_mut() {
            extra.append(&mut extra_keys);
        }

        res
    }

    pub fn seal(
        &mut self,
        ad: Bytes,
        mut msg: BytesMut,
    ) -> CipherData {
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

#[derive(Debug, Clone, Ser, De, Eq, PartialEq)]
pub struct Cipher {
    pub index: u64,
    pub tag: aead::Tag,
    pub ad: Bytes,
    pub msg: Bytes,
}

impl Cipher {
    pub fn open_with(
        self,
        key: aead::Key,
    ) -> DecryptionResult {
        let Cipher { tag, ad, msg, .. } = self;

        let extra_keys = Vec::new();
        let mut msg = BytesMut::from(msg);

        if key.open(&ad, tag, &mut msg) {
            DecryptionResult::Success {
                extra_keys,
                ad,
                pt: msg,
            }
        } else {
            DecryptionResult::Failed { extra_keys }
        }
    }
}
