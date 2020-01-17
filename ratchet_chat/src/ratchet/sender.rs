use super::*;

use std::ops::DerefMut;

#[derive(Ser, De)]
pub struct Ratchet {
    generation: u32,
    ix: u32,
    base_key: hash::Key,
    ratchet_key: RatchetKey,
}

impl Ratchet {
    pub fn mk(
        generation: u32,
        ix: u32,
        base_key: hash::Key,
        ratchet_key: RatchetKey,
    ) -> Self {
        Self {
            generation,
            ix,
            base_key,
            ratchet_key,
        }
    }

    pub fn gen_new(generation: u32) -> Self {
        let base_key = hash::Key::gen_new();

        let mut rkey_buf = [0u8; RATCHET_KEY_LEN];
        base_key.hash_into(&mut rkey_buf, &generation.to_le_bytes());
        let ratchet_key = RatchetKey(rkey_buf);

        Ratchet {
            ix: 0,
            generation,
            base_key,
            ratchet_key,
        }
    }

    /// Runs the kdf ratchet forward, generating a new message key
    pub fn ratchet_ix(&mut self) -> (u32, aead::Key) {
        let mut rkey_buf = [0u8; RATCHET_KEY_LEN];
        let mut mkey_buf = [0u8; aead::KEY_LEN];
        let mut bufs: [&mut [u8]; 2] = [&mut rkey_buf, &mut mkey_buf];
        self.base_key.hash_into_many(
            self.ratchet_key.as_ref(),
            bufs.iter_mut().map(DerefMut::deref_mut),
        );

        let ix = self.ix;
        self.ix += 1;
        self.ratchet_key = RatchetKey(rkey_buf);

        (ix, aead::Key(mkey_buf))
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn ix(&self) -> u32 {
        self.ix
    }

    pub fn base_key(&self) -> &hash::Key {
        &self.base_key
    }

    pub fn ratchet_key(&self) -> &RatchetKey {
        &self.ratchet_key
    }

    pub fn seal(
        &mut self,
        ad: Bytes,
        mut msg: BytesMut,
    ) -> Cipher {
        let (index, key) = self.ratchet_ix();
        let tag = key.seal(&ad, &mut msg);
        Cipher {
            generation: self.generation,
            index,
            tag,
            ad,
            ct: msg.freeze(),
        }
    }

    pub fn open(
        &mut self,
        cipher: Cipher,
    ) -> Decrypted {
        if cipher.generation != self.generation {
            return Decrypted::WrongGeneration;
        } else if cipher.index < self.ix {
            return Decrypted::IndexTooLow;
        }

        let mut extra = Vec::with_capacity((cipher.index - self.ix + 1) as usize);

        for _ in self.ix..cipher.index {
            extra.push(self.ratchet_ix());
        }

        let (i, key) = self.ratchet_ix();

        let Cipher { tag, ad, ct, .. } = cipher;
        let mut pt_buf = ct.to_vec();

        if key.open(&ad, tag, &mut pt_buf) {
            Decrypted::Success {
                pt: Bytes::from(pt_buf),
                ad,
                extra,
            }
        } else {
            extra.push((i, key));
            Decrypted::DecryptionFailed(extra)
        }
    }
}

#[derive(Debug, Clone, Ser, De, Eq, PartialEq)]
pub struct Cipher {
    pub generation: u32,
    pub index: u32,
    pub tag: aead::Tag,
    pub ad: Bytes,
    pub ct: Bytes,
}

pub type ExtraKeys = Vec<(u32, aead::Key)>;

#[must_use]
#[derive(Debug, Clone, Ser, De, Eq, PartialEq)]
pub enum Decrypted {
    Success {
        pt: Bytes,
        ad: Bytes,
        extra: ExtraKeys,
    },
    WrongGeneration,
    IndexTooLow,
    DecryptionFailed(ExtraKeys),
}
