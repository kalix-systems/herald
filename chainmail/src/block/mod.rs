use crate::prelude::*;
use sodiumoxide::randombytes::randombytes_into;
use std::collections::BTreeSet;

pub const BLOCKHASH_BYTES: usize = 32;
new_type! {
    /// The hash of a `Block`
    public BlockHash(BLOCKHASH_BYTES);
}

pub const CHAINKEY_BYTES: usize = hash::DIGEST_MAX;
new_type! {
    /// A key which is used for kdf purposes
    secret ChainKey(CHAINKEY_BYTES);
}

impl ChainKey {
    fn new() -> Self {
        sodiumoxide::init().expect("failed to initialize libsodium");
        let mut buf = [0u8; CHAINKEY_BYTES];
        randombytes_into(&mut buf);
        ChainKey(buf)
    }
}

impl PartialOrd for ChainKey {
    fn partial_cmp(
        &self,
        other: &ChainKey,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChainKey {
    fn cmp(
        &self,
        other: &ChainKey,
    ) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub const MESSAGEKEY_BYTES: usize = aead::KEYBYTES;
pub type MessageKey = aead::Key;

pub const NONCE_BYTES: usize = aead::NONCEBYTES;
pub type Nonce = aead::Nonce;

pub const SALT_BYTES: usize = 32;
new_type! {
    /// A salt included with blocks, used for hashing
    nonce Salt(SALT_BYTES);
}

impl Salt {
    fn new() -> Salt {
        let mut buf = [0u8; SALT_BYTES];
        randombytes_into(&mut buf);
        Salt(buf)
    }
}

pub const CHANNELKEY_BYTES: usize = 64;
new_type! {
    /// A base shared secret that is mixed into each new block
    secret ChannelKey(CHANNELKEY_BYTES);
}

impl ChannelKey {
    fn new() -> ChannelKey {
        sodiumoxide::init().expect("failed to initialize libsodium");
        let mut buf = [0u8; CHANNELKEY_BYTES];
        randombytes_into(&mut buf);
        ChannelKey(buf)
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Block {
    parent_hashes: BTreeSet<BlockHash>,
    salt: Salt,
    sig: sign::Signature,
    tag: aead::Tag,
    #[cfg_attr(feature = "serde_support", serde(with = "serde_bytes"))]
    msg: Vec<u8>,
}

pub struct OpenData {
    pub msg: Vec<u8>,
    pub hash: BlockHash,
    pub key: ChainKey,
}

pub struct SealData {
    pub block: Block,
    pub key: ChainKey,
}

impl Block {
    pub fn parent_hashes(&self) -> &BTreeSet<BlockHash> {
        &self.parent_hashes
    }

    pub fn compute_hash(&self) -> Option<BlockHash> {
        let mut state = hash::State::new(BLOCKHASH_BYTES, None).ok()?;
        for parent in self.parent_hashes.iter() {
            state.update(parent.as_ref()).ok()?;
        }
        state.update(self.salt.as_ref()).ok()?;
        state.update(self.sig.as_ref()).ok()?;
        // we specifically don't include the message content in the hash for deniability purposes
        let digest = state.finalize().ok()?;
        BlockHash::from_slice(digest.as_ref())
    }

    pub fn seal(
        seckey: &sign::SecretKey,
        channel_key: &ChannelKey,
        parent_keys: &BTreeSet<ChainKey>,
        parent_hashes: BTreeSet<BlockHash>,
        mut msg: Vec<u8>,
    ) -> Option<SealData> {
        let salt = Salt::new();
        let dat = compute_block_signing_data(&parent_hashes, salt);
        let sig = sign::sign_detached(&dat, seckey);
        let (c, k, n) = kdf(&channel_key, &parent_keys, salt, sig)?;
        let ad = compute_block_ad(&parent_hashes, salt, sig);
        let tag = aead::seal_detached(&mut msg, Some(&ad), &n, &k);

        Some(SealData {
            block: Block {
                parent_hashes,
                salt,
                sig,
                tag,
                msg,
            },
            key: c,
        })
    }

    pub fn open(
        self,
        channel_key: &ChannelKey,
        signer: &sign::PublicKey,
        parent_keys: &BTreeSet<ChainKey>,
    ) -> Result<OpenData, ChainError> {
        let hash = self.compute_hash().ok_or(CryptoError)?;

        let Block {
            parent_hashes,
            salt,
            sig,
            tag,
            mut msg,
        } = self;

        let dat = compute_block_signing_data(&parent_hashes, salt);
        if sign::verify_detached(&sig, &dat, signer) {
            let (c, k, n) = kdf(&channel_key, &parent_keys, salt, sig).ok_or(CryptoError)?;
            let ad = compute_block_ad(&parent_hashes, salt, sig);
            aead::open_detached(&mut msg, Some(&ad), &tag, &n, &k).map_err(|_| DecryptionError)?;
            Ok(OpenData { msg, hash, key: c })
        } else {
            Err(BadSig)
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Genesis {
    channel_key: ChannelKey,
    root: ChainKey,
    sig: sign::Signature,
}

impl Genesis {
    pub fn new(seckey: &sign::SecretKey) -> Self {
        let channel_key = ChannelKey::new();
        let root = ChainKey::new();
        let dat = Self::compute_signing_data(&channel_key, &root);
        let sig = sign::sign_detached(&dat, seckey);
        Genesis {
            channel_key,
            root,
            sig,
        }
    }

    fn compute_signing_data(
        channel_key: &ChannelKey,
        root: &ChainKey,
    ) -> [u8; CHANNELKEY_BYTES + CHAINKEY_BYTES] {
        let mut buf = [0u8; CHANNELKEY_BYTES + CHAINKEY_BYTES];
        for (b, o) in channel_key
            .as_ref()
            .iter()
            .chain(root.as_ref())
            .zip(buf.iter_mut())
        {
            *o = *b;
        }
        buf
    }

    pub fn compute_hash(&self) -> Option<BlockHash> {
        let mut state = hash::State::new(BLOCKHASH_BYTES, None).ok()?;
        state.update(self.channel_key.as_ref()).ok()?;
        state.update(self.root.as_ref()).ok()?;
        state.update(self.sig.as_ref()).ok()?;
        let digest = state.finalize().ok()?;
        BlockHash::from_slice(digest.as_ref())
    }

    pub fn channel_key(&self) -> &ChannelKey {
        &self.channel_key
    }

    pub fn root(&self) -> &ChainKey {
        &self.root
    }

    pub fn verify_sig(
        &self,
        pk: &sign::PublicKey,
    ) -> bool {
        sign::verify_detached(
            &self.sig,
            &Self::compute_signing_data(&self.channel_key, &self.root),
            pk,
        )
    }
}

fn compute_block_signing_data(
    hashes: &BTreeSet<BlockHash>,
    salt: Salt,
) -> Vec<u8> {
    let capacity = hashes.len() * BLOCKHASH_BYTES + SALT_BYTES;
    let mut data = Vec::with_capacity(capacity);
    for hash in hashes {
        data.extend_from_slice(hash.as_ref());
    }
    data.extend_from_slice(salt.as_ref());
    data
}

fn compute_block_ad(
    parents: &BTreeSet<BlockHash>,
    salt: Salt,
    sig: sign::Signature,
) -> Vec<u8> {
    let capacity = parents.len() * BLOCKHASH_BYTES + SALT_BYTES + sign::SIGNATUREBYTES;
    let mut ad = Vec::with_capacity(capacity);
    for parent in parents.iter() {
        ad.extend_from_slice(parent.as_ref());
    }
    ad.extend_from_slice(salt.as_ref());
    ad.extend_from_slice(sig.as_ref());
    ad
}

fn hash_keys_with_salt_and_index(
    len: usize,
    chan_key: &ChannelKey,
    data: &BTreeSet<ChainKey>,
    salt: Salt,
    sig: sign::Signature,
    idx: u8,
) -> Option<hash::Digest> {
    debug_assert!(
        hash::DIGEST_MIN <= len && len <= hash::DIGEST_MAX,
        "BAD DIGEST LENGTH\nExpected: length between {} and {}\nCalled with {}",
        hash::DIGEST_MIN,
        hash::DIGEST_MAX,
        len
    );
    let mut state = hash::State::new(len, None).ok()?;
    state.update(chan_key.as_ref()).ok()?;
    for d in data.iter() {
        state.update(d.as_ref()).ok()?;
    }
    state.update(salt.as_ref()).ok()?;
    state.update(sig.as_ref()).ok()?;
    state.update(&[idx]).ok()?;
    state.finalize().ok()
}

fn kdf(
    chan_key: &ChannelKey,
    keys: &BTreeSet<ChainKey>,
    salt: Salt,
    sig: sign::Signature,
) -> Option<(ChainKey, MessageKey, Nonce)> {
    let chainkey_bytes =
        hash_keys_with_salt_and_index(CHAINKEY_BYTES, chan_key, keys, salt, sig, 0)?;
    let msgkey_bytes =
        hash_keys_with_salt_and_index(MESSAGEKEY_BYTES, chan_key, keys, salt, sig, 0)?;
    let nonce_bytes = hash_keys_with_salt_and_index(NONCE_BYTES, chan_key, keys, salt, sig, 0)?;

    let chainkey = ChainKey::from_slice(chainkey_bytes.as_ref())?;
    let msgkey = MessageKey::from_slice(msgkey_bytes.as_ref())?;
    let nonce = Nonce::from_slice(nonce_bytes.as_ref())?;

    Some((chainkey, msgkey, nonce))
}

#[cfg(test)]
mod test;
