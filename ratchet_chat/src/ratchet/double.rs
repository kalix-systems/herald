//! The `DoubleRatchet` can encrypt/decrypt messages while providing forward secrecy and
//! post-compromise security.
//!
//! The `DoubleRatchet` struct provides an implementation of the Double Ratchet Algorithm as
//! defined in its [specification], including the unspecified symmetric initialization. After
//! initialization (with `new_alice` or `new_bob`) the user can interact with the `DoubleRatchet`
//! using the `ratchet_encrypt` and `ratchet_decrypt` methods, which automatically takes care of
//! deriving the correct keys and updating the internal state.
//!
//! # Initialization
//!
//! When Alice and Bob want to use the `DoubleRatchet`, they need to initialize it using different
//! constructors. The "Alice" or "Bob" role follows from the design of the authenticated key
//! exchange that is used to initialize the secure communications channel. Two "modes" are
//! possible, depending on whether just one or both of the parties must be able to send the first
//! data message. See `new_alice` and `new_bob` for further details.
//!
//! # Provided security
//!
//! Conditional on the correct implementation of the `CryptoProvider`, the `DoubleRatchet` provides
//! confidentiality of the plaintext and authentication of both the ciphertext and associated data.
//! It does not provide anonymity, as the headers have to be sent in plain text and are sufficient
//! for identifying the communicating parties. See `CryptoProvider` for further details on the
//! required security properties.
//!
//! Forward secrecy (sometimes called the key-erasure property) preserves confidentiality of old
//! messages in case of a device compromise. The `DoubleRatchet` provides forward secrecy by
//! deriving a fresh key for every message: the sender deletes it immediately after encrypting and
//! the receiver deletes it immediately after successful decryption. Messages may arrive out of
//! order, in which case the receiver is able to derive and store the keys for the skipped messages
//! without compromising the forward secrecy of other messages. See [secure deletion] for further
//! discussion.
//!
//! Post-compromise security (sometimes called future secrecy or the self-healing property)
//! restores confidentiality of new messages in case of a past device compromise. The
//! `DoubleRatchet` provides future secrecy by generating a fresh `KeyPair` for every reply that is
//! being sent. See [recovery from compromise] for further discussion and [post-compromise] for an
//! in-depth analysis of the subject.
//!
//! [post-compromise]: https://eprint.iacr.org/2016/221
//! [specification]: https://signal.org/docs/specifications/doubleratchet/#double-ratchet-1
//! [secure deletion]: https://signal.org/docs/specifications/doubleratchet/#secure-deletion
//! [recovery from compromise]: https://signal.org/docs/specifications/doubleratchet/#recovery-from-compromise

use derive_getters::Getters;
use kcl::*;
use kson::prelude::*;
use std::{collections::HashMap, error::Error};
use thiserror::*;
use typed_builder::TypedBuilder;

// TODO: avoid heap allocations in encrypt/decrypt interfaces
// TODO: make stuff like MAX_SKIP and MKS_CAPACITY dynamic
// TODO: HeaderEncrypted version

// Upper limit on the receive chain ratchet steps when trying to decrypt. Prevents a
// denial-of-service attack where the attacker fakes an extremely long skip
pub const MAX_SKIP: usize = 1000;

/// Message Counter (as seen in the header)
pub type Counter = u32;

#[derive(TypedBuilder, Getters, Debug)]
pub struct DoubleRatchet {
    dhs: kx::KeyPair,
    #[builder(default)]
    dhr: Option<kx::PublicKey>,
    rk: hash::Key,
    #[builder(default)]
    cks: Option<hash::Key>,
    #[builder(default)]
    ckr: Option<hash::Key>,
    ns: Counter,
    nr: Counter,
    pn: Counter,
}

impl DoubleRatchet {
    /// Initialize "Alice": the sender of the first message.
    ///
    /// This implements `RatchetInitAlice` as defined in the [specification] when `initial_receive
    /// = None`: after initialization Alice must send a message to Bob before he is able to provide
    /// a reply.
    ///
    /// Alternatively Alice provides an extra symmetric key: `initial_receive = Some(key)`, so that
    /// both Alice and Bob can send the first message. Note however that even when Alice and Bob
    /// initialize this way the initialization is asymmetric in the sense that Alice requires Bob's
    /// public key.
    ///
    /// Either Alice and Bob must supply the same extra symmetric key or both must supply `None`.
    ///
    /// # Security considerations
    ///
    /// For security, initialization through `new_alice` has the following requirements:
    ///  - `shared_secret` must be both *confidential* and *authenticated*
    ///  - `them` must be *authenticated*
    ///  - `initial_receive` is `None` or `Some(key)` where `key` is *confidential* and *authenticated*
    ///
    /// [specification]: https://signal.org/docs/specifications/doubleratchet/#initialization
    pub fn new_alice(
        shared_secret: &RootKey,
        them: PublicKey,
        initial_receive: Option<ChainKey>,
    ) -> Self {
        let dhs = KeyPair::gen_new();
        let (rk, cks) = kdf_rk(shared_secret, &diffie_hellman(&dhs, &them));
        Self {
            dhs,
            dhr: Some(them),
            rk,
            cks: Some(cks),
            ckr: initial_receive,
            ns: 0,
            nr: 0,
            pn: 0,
        }
    }

    /// Initialize "Bob": the receiver of the first message.
    ///
    /// This implements `RatchetInitBob` as defined in the [specification] when `initial_send =
    /// None`: after initialization Bob must receive a message from Alice before he can send his
    /// first message.
    ///
    /// Alternatively Bob provides an extra symmetric key: `initial_send = Some(key)`, so that both
    /// Alice and Bob can send the first message. Note however that even when Alice and Bob
    /// initialize this way the initialization is asymmetric in the sense that Bob must provide his
    /// public key to Alice.
    ///
    /// Either Alice and Bob must supply the same extra symmetric key or both must supply `None`.
    ///
    /// # Security considerations
    ///
    /// For security, initialization through `new_bob` has the following requirements:
    ///  - `shared_secret` must be both *confidential* and *authenticated*
    ///  - the private key of `us` must remain secret on Bob's device
    ///  - `initial_send` is `None` or `Some(key)` where `key` is *confidential* and *authenticated*
    ///
    /// [specification]: https://signal.org/docs/specifications/doubleratchet/#initialization
    pub fn new_bob(
        shared_secret: RootKey,
        us: KeyPair,
        them: PublicKey,
        initial_send: Option<ChainKey>,
    ) -> Self {
        let (_rk, ckr) = kdf_rk(&shared_secret, &diffie_hellman(&us, &them));
        Self {
            dhs: us,
            dhr: None,
            rk: shared_secret,
            cks: initial_send,
            ckr: Some(ckr),
            ns: 0,
            nr: 0,
            pn: 0,
        }
    }

    /// Encrypt the `plaintext`, ratchet forward and return the (header, ciphertext) pair.
    ///
    /// Implements `RatchetEncrypt` as defined in the [specification]. The header should be sent
    /// along the ciphertext in order for the recipient to be able to `ratchet_decrypt`. The
    /// ciphertext is encrypted in some
    /// [AEAD](https://en.wikipedia.org/wiki/Authenticated_encryption) mode, which encrypts the
    /// `plaintext` and authenticates the `plaintext`, `associated_data` and the header.
    ///
    /// The internal state of the `DoubleRatchet` is automatically updated so that the next message
    /// key be sent with a fresh key.
    ///
    /// Note that `rng` is only used for updating the internal state and not for encrypting the
    /// data.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not initialized for sending yet. If this is a concern, use
    /// `try_ratchet_encrypt` instead to avoid panics.
    ///
    /// [specification]: https://signal.org/docs/specifications/doubleratchet/#encrypting-messages
    pub fn ratchet_encrypt(
        &mut self,
        plaintext: &[u8],
        associated_data: &[u8],
    ) -> Option<(Header<PublicKey>, Vec<u8>)> {
        // TODO: is this the correct place for clear_stack_on_return?
        let (h, mk) = self.ratchet_send_chain()?;
        let pt = encrypt(&mk, plaintext, &Self::concat(&h, associated_data));
        Some((h, pt))
    }

    // Ratcheting forward the DH chain for sending is delayed until the first message in that chain
    // is going to be sent.
    //
    // [specification]: https://signal.org/docs/specifications/doubleratchet/#deferring-new-ratchet-key-generation
    //
    // # Panics
    //
    // Panics if encrypting is not yet initialized
    fn ratchet_send_chain(&mut self) -> Option<(Header<PublicKey>, MessageKey)> {
        if self.cks.is_none() {
            let dhr = self.dhr.as_ref()?;
            self.dhs = KeyPair::gen_new();
            let (rk, cks) = kdf_rk(&self.rk, &diffie_hellman(&self.dhs, dhr));
            self.rk = rk;
            self.cks = Some(cks);
            self.pn = self.ns;
            self.ns = 0;
        }
        let h = Header {
            dh: self.dhs.public,
            n: self.ns,
            pn: self.pn,
        };
        let (cks, mk) = kdf_ck(self.cks.as_ref().unwrap());
        self.cks = Some(cks);
        self.ns += 1;
        Some((h, mk))
    }

    /// Verify-decrypt the `ciphertext`, update `self` and return the plaintext.
    ///
    /// Implements `RatchetDecrypt` as defined in the [specification]. Decryption of the ciphertext
    /// includes verifying the authenticity of the `header`, `ciphertext` and `associated_data`
    /// (optional).
    ///
    /// `self` is automatically updated upon successful decryption. This includes ratcheting
    /// forward the receiving key-chain and DH key-chain (if necessary) and storing the
    /// `MessageKeys` of any skipped messages so these messages can be decrypted if they arrive out
    /// of order.
    ///
    /// Returns a `DecryptError<KS::Error>` when the plaintext could not be decrypted: `self` remains
    /// unchanged in that case. There could be many reasons: inspect the returned error-value for
    /// further details.
    ///
    /// [specification]: https://signal.org/docs/specifications/doubleratchet/#decrypting-messages-1
    pub fn ratchet_decrypt<KS: KeyStore>(
        &mut self,
        store: &mut KS,
        header: &Header<PublicKey>,
        ciphertext: &[u8],
        associated_data: &[u8],
    ) -> Result<Vec<u8>, DecryptError<KS::Error>> {
        // TODO: is this the correct place for clear_stack_on_return?
        let (diff, pt) = self.try_decrypt(
            store,
            header,
            ciphertext,
            &Self::concat(&header, associated_data),
        )?;
        self.update(store, diff, header)?;
        Ok(pt)
    }

    fn should_fetch_key<KS: KeyStore>(
        &self,
        store: &mut KS,
        h: &Header<PublicKey>,
    ) -> Result<bool, DecryptError<KS::Error>> {
        if self.dhr == Some(h.dh) {
            Ok(h.n < self.nr)
        } else {
            Ok(store.contains_pk(h.dh)?)
        }
    }

    // The actual decryption. Gets a (non-mutable) reference to self to ensure that the state is
    // not changed. Upon successful decryption the state must be updated. The minimum amount of work
    // is done in order to retrieve the correct `MessageKey`: the returned `Diff` object contains
    // the result of that work to avoid doing the work again.
    fn try_decrypt<KS: KeyStore>(
        &self,
        store: &mut KS,
        h: &Header<PublicKey>,
        ct: &[u8],
        ad: &[u8],
    ) -> Result<(Diff, Vec<u8>), DecryptError<KS::Error>> {
        use Diff::*;

        if self.should_fetch_key(store, h)? {
            let key = store
                .get_key(h.dh, h.n)?
                .ok_or(DecryptError::MessageKeyNotFound(h.dh, h.n))?;
            Ok((OldKey, decrypt::<KS>(&key, ct, ad)?))
        } else if self.dhr.as_ref() == Some(&h.dh) {
            let skip = h.n.wrapping_sub(self.nr) as usize;
            if skip <= MAX_SKIP {
                let (ckr, mut mks) =
                    Self::skip_message_keys(self.ckr.as_ref().ok_or(DecryptError::Uninit)?, skip);
                let mk = mks.pop().unwrap();
                Ok((CurrentChain(ckr, mks), decrypt::<KS>(&mk, ct, ad)?))
            } else {
                Err(DecryptError::SkipTooLarge(skip))
            }
        } else {
            let skip = h.n as usize;
            let (rk, ckr) = kdf_rk(&self.rk, &diffie_hellman(&self.dhs, &h.dh));
            if skip > MAX_SKIP {
                Err(DecryptError::SkipTooLarge(skip))
            } else {
                let (ckr, mut mks) = Self::skip_message_keys(&ckr, skip);
                let mk = mks.pop().unwrap();
                Ok((NextChain(rk, ckr, mks), decrypt::<KS>(&mk, ct, ad)?))
            }
        }
    }

    // Update the internal state. Assumes that the validity of `h` has already been checked.
    fn update<KS: KeyStore>(
        &mut self,
        store: &mut KS,
        diff: Diff,
        h: &Header<PublicKey>,
    ) -> Result<(), DecryptError<KS::Error>> {
        use Diff::*;
        match diff {
            OldKey => {
                store.remove_key(h.dh, h.n)?;
            }
            CurrentChain(ckr, mks) => {
                store.extend(h.dh, self.nr, mks)?;
                self.ckr = Some(ckr);
                self.nr = h.n + 1;
            }
            NextChain(rk, ckr, mks) => {
                if self.ckr.is_some() && self.nr < h.pn {
                    let ckr = self.ckr.as_ref().ok_or(DecryptError::Uninit)?;
                    let dhr = self.dhr.as_ref().ok_or(DecryptError::Uninit)?.clone();
                    let (_, prev_mks) = Self::skip_message_keys(ckr, (h.pn - self.nr - 1) as usize);
                    store.extend(dhr, self.nr, prev_mks)?;
                }
                self.dhr = Some(h.dh);
                self.rk = rk;
                self.cks = None;
                self.ckr = Some(ckr);
                self.nr = h.n + 1;
                store.extend(h.dh, 0, mks)?;
            }
        }
        Ok(())
    }

    // Do `skip + 1` ratchet steps in the receive chain. Return the last ChainKey
    // and all computed MessageKeys.
    fn skip_message_keys(
        ckr: &ChainKey,
        skip: usize,
    ) -> (ChainKey, Vec<MessageKey>) {
        // Note: should use std::iter::unfold (currently still in nightly)
        let mut mks = Vec::with_capacity(skip + 1);
        let (mut ckr, mk) = kdf_ck(&ckr);
        mks.push(mk);
        for _ in 0..skip {
            let cm = kdf_ck(&ckr);
            ckr = cm.0;
            mks.push(cm.1);
        }
        (ckr, mks)
    }

    // Concatenate `h` and `ad` in a single byte-vector.
    fn concat(
        h: &Header<PublicKey>,
        ad: &[u8],
    ) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(ad);
        h.extend_bytes_into(&mut v);
        v
    }
}

/// The Header that should be sent alongside the ciphertext.
///
/// The Header contains the information for the `DoubleRatchet` to find the correct `MessageKey` to
/// decrypt the message. It is generated by `ratchet_encrypt`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header<PublicKey> {
    /// The public half of the key-pair of the sender
    pub dh: PublicKey,

    /// Counts the number of messages that have been sent in the current symmetric ratchet
    pub n: Counter,

    /// Counts the number of messages that have been sent in the previous symmetric ratchet
    pub pn: Counter,
}

impl<PK: AsRef<[u8]>> Header<PK> {
    // yikes
    fn extend_bytes_into(
        &self,
        v: &mut Vec<u8>,
    ) {
        v.extend_from_slice(self.dh.as_ref());
        v.extend_from_slice(&self.n.to_be_bytes());
        v.extend_from_slice(&self.pn.to_be_bytes());
    }
}

pub type PublicKey = kx::PublicKey;
pub type KeyPair = kx::KeyPair;
pub type SharedSecret = hash::Key;
pub type RootKey = hash::Key;
pub type ChainKey = hash::Key;
pub type MessageKey = aead::Key;

/// Perform the Diffie-Hellman operation on the sender side.
fn diffie_hellman(
    us: &KeyPair,
    them: &PublicKey,
) -> SharedSecret {
    let mut hk_buf = [0u8; hash::KEY_LEN];
    us.symmetric_kx_into(*them, &mut hk_buf);
    hash::Key(hk_buf)
}

/// Derive a new root-key/chain-key pair from the old root-key and a fresh shared secret.
fn kdf_rk(
    root_key: &RootKey,
    shared_secret: &SharedSecret,
) -> (RootKey, ChainKey) {
    let mut rk_buf = [0u8; hash::KEY_LEN];
    let mut ck_buf = [0u8; hash::KEY_LEN];
    let mut bufs: [&mut [u8]; 2] = [&mut rk_buf, &mut ck_buf];
    root_key.hash_into_many(
        shared_secret.as_ref(),
        bufs.iter_mut().map(std::ops::DerefMut::deref_mut),
    );
    (hash::Key(rk_buf), hash::Key(ck_buf))
}

/// Derive a new chain-key/message-key pair from the old chain-key.
fn kdf_ck(chain_key: &ChainKey) -> (ChainKey, MessageKey) {
    let mut ck_buf = [0u8; hash::KEY_LEN];
    let mut mk_buf = [0u8; aead::KEY_LEN];
    let mut bufs: [&mut [u8]; 2] = [&mut ck_buf, &mut mk_buf];
    chain_key.hash_into_many(&[], bufs.iter_mut().map(std::ops::DerefMut::deref_mut));
    (hash::Key(ck_buf), aead::Key(mk_buf))
}

/// Authenticate-encrypt the plaintext and associated data.
///
/// This method MUST authenticate `associated_data`, because it contains the header bytes.
fn encrypt(
    key: &MessageKey,
    plaintext: &[u8],
    associated_data: &[u8],
) -> Vec<u8> {
    key.seal_attached(associated_data, plaintext)
}

/// Verify-decrypt the ciphertext and associated data.
fn decrypt<KS: KeyStore>(
    key: &MessageKey,
    ciphertext: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>, DecryptError<KS::Error>> {
    key.open_attached(associated_data, ciphertext)
        .ok_or(DecryptError::DecryptFailure)
}

// impl fmt::Debug for KeyStore
// where
//     CP: CryptoProvider,
//     CP::PublicKey: fmt::Debug,
//     CP::MessageKey: fmt::Debug,
// {
//     fn fmt(
//         &self,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         write!(f, "KeyStore({:?})", self.0)
//     }
// }

// Required information for updating the state after successful decryption
enum Diff {
    // Key was found amongst old key
    OldKey,

    // Key was part of the current receive chain
    CurrentChain(ChainKey, Vec<MessageKey>),

    // Key was part of the next receive chain
    NextChain(RootKey, ChainKey, Vec<MessageKey>),
}

// /// Error that occurs on `try_ratchet_encrypt` before the state is initialized.
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct EncryptUninit;

// #[cfg(feature = "std")]
// impl Error for EncryptUninit {}

// impl fmt::Display for EncryptUninit {
//     fn fmt(
//         &self,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         write!(
//             f,
//             "Encrypt not yet initialized (you must receive a message first)"
//         )
//     }
// }

/// Error that may occur during `ratchet_decrypt`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum DecryptError<E: Error + Send + 'static> {
    /// Could not verify-decrypt the ciphertext + associated data + header
    #[error("Could not verify-decrypt the ciphertext + associated data + header")]
    DecryptFailure,

    #[error("KeyStore Error: {0}")]
    StoreError(#[from] E),

    /// Message key not found
    #[error("Message key not found for key {:x?} with index {1}")]
    MessageKeyNotFound(kx::PublicKey, Counter),

    /// Header message counter is too large (either `n` or `pn`)
    #[error("Header message counter was {0}, which is too large (either `n` or `pn`)")]
    SkipTooLarge(usize),

    #[error("Chainkeys were uninitialized")]
    Uninit,
}

/// A KeyStore holds the skipped `MessageKey`s.
///
/// When messages can arrive out of order, the DoubleRatchet must store the MessageKeys
/// corresponding to the messages that were skipped over. See also the [specification] for further
/// discussion.
///
/// [specification]: https://signal.org/docs/specifications/doubleratchet/#deletion-of-skipped-message-keys
pub trait KeyStore {
    type Error: std::error::Error + Send + 'static;

    fn get_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
    ) -> Result<Option<aead::Key>, Self::Error>;

    fn store_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
        key: aead::Key,
    ) -> Result<(), Self::Error>;

    fn extend(
        &mut self,
        pk: kx::PublicKey,
        init_ix: Counter,
        keys: Vec<aead::Key>,
    ) -> Result<(), Self::Error> {
        for (n, key) in keys.into_iter().enumerate() {
            self.store_key(pk, init_ix + n as Counter, key)?;
        }
        Ok(())
    }

    fn remove_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
    ) -> Result<(), Self::Error>;

    fn contains_pk(
        &mut self,
        pk: kx::PublicKey,
    ) -> Result<bool, Self::Error>;
}

impl<S1, S2> KeyStore for HashMap<kx::PublicKey, HashMap<Counter, aead::Key, S2>, S1>
where
    S1: std::hash::BuildHasher,
    S2: std::hash::BuildHasher + Default,
{
    type Error = void::Void;

    fn get_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
    ) -> Result<Option<aead::Key>, Self::Error> {
        Ok(self.get(&pk).and_then(|h| h.get(&ix).cloned()))
    }

    fn store_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
        key: aead::Key,
    ) -> Result<(), Self::Error> {
        self.entry(pk).or_default().insert(ix, key);
        Ok(())
    }

    fn remove_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
    ) -> Result<(), Self::Error> {
        let should_del: bool;
        if let Some(h) = self.get_mut(&pk) {
            let _ = h.remove(&ix);
            should_del = h.is_empty();
        } else {
            should_del = false;
        }

        if should_del {
            self.remove(&pk);
        }

        Ok(())
    }

    fn contains_pk(
        &mut self,
        pk: kx::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(self.contains_key(&pk))
    }
}

pub type ExtraKeys = Vec<(u32, aead::Key)>;

#[must_use]
#[derive(Debug, Clone, Ser, De, Eq, PartialEq)]
pub enum Decrypted {
    Success { pt: Vec<u8>, extra: ExtraKeys },
    IndexTooLow,
    DecryptionFailed(ExtraKeys),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::*;

    type DR = DoubleRatchet;

    fn asymmetric_setup() -> (DR, DR) {
        kcl::init();

        let a_pair = KeyPair::gen_new();
        let b_pair = KeyPair::gen_new();
        let secret = diffie_hellman(&a_pair, &b_pair.public);
        let (secret, ck_init) = kdf_rk(&secret, &secret);
        let alice = DR::new_alice(&secret, b_pair.public, Some(ck_init));
        let bob = DR::new_bob(secret, b_pair, alice.dhs.public, None);
        (alice, bob)
    }

    fn symmetric_setup() -> (DR, DR) {
        kcl::init();

        let pair = KeyPair::gen_new();
        let secret = diffie_hellman(&pair, &pair.public);
        let (secret, ck_init) = kdf_rk(&secret, &secret);

        let alice = DR::new_alice(&secret, pair.public, Some(ck_init.clone()));
        let bob = DR::new_bob(secret, pair.clone(), pair.public, Some(ck_init));
        (alice, bob)
    }

    fn new_store() -> impl KeyStore<Error = void::Void> {
        let out: HashMap<kx::PublicKey, HashMap<Counter, aead::Key>> = HashMap::new();
        out
    }

    #[test]
    #[serial]
    fn test_asymmetric_setup() {
        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();

        // Alice can encrypt, Bob can't
        let (pt_a, ad_a) = (b"Hi Bobby", b"A2B");
        let (pt_b, ad_b) = (b"What's up Al?", b"B2A");
        let (h_a, ct_a) = alice
            .ratchet_encrypt(pt_a, ad_a)
            .expect("alice should be able to encrypt");
        assert_eq!(None, bob.ratchet_encrypt(pt_b, ad_b));
        assert_eq!(
            Ok(Vec::from(&pt_a[..])),
            bob.ratchet_decrypt(&mut store, &h_a, &ct_a, ad_a)
        );

        // but after decryption Bob can encrypt
        let (h_b, ct_b) = bob
            .ratchet_encrypt(pt_b, ad_b)
            .expect("bob should be able to encrypt now");
        assert_eq!(
            Ok(Vec::from(&pt_b[..])),
            alice.ratchet_decrypt(&mut store, &h_b, &ct_b, ad_b)
        );
    }

    #[test]
    #[serial]
    fn test_symmetric_setup() {
        let (mut alice, mut bob) = symmetric_setup();
        let mut store = new_store();
        // Alice can encrypt, Bob can too
        let (pt_a, ad_a) = (b"Hi Bobby", b"A2B");
        let (pt_b, ad_b) = (b"What's up Al?", b"B2A");
        let (h_a, ct_a) = alice
            .ratchet_encrypt(pt_a, ad_a)
            .expect("alice should be able to encrypt");
        let (h_b, ct_b) = bob
            .ratchet_encrypt(pt_b, ad_b)
            .expect("bob should be able to encrypt");
        assert_eq!(
            Ok(Vec::from(&pt_a[..])),
            bob.ratchet_decrypt(&mut store, &h_a, &ct_a, ad_a)
        );
        assert_eq!(
            Ok(Vec::from(&pt_b[..])),
            alice.ratchet_decrypt(&mut store, &h_b, &ct_b, ad_b)
        );
    }

    #[test]
    #[serial]
    fn symmetric_out_of_order() {
        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();
        let (ad_a, ad_b) = (b"A2B", b"B2A");

        // Alice's message arrive out of order, some are even missing
        let pt_a_0 = b"Hi Bobby";
        let (h_a_0, ct_a_0) = alice
            .ratchet_encrypt(pt_a_0, ad_a)
            .expect("alice should be able to encrypt");
        for _ in 1..9 {
            alice
                .ratchet_encrypt(b"hello?", ad_a)
                .expect("alice should be able to encrypt many"); // drop these messages
        }
        let pt_a_9 = b"are you there?";
        let (h_a_9, ct_a_9) = alice
            .ratchet_encrypt(pt_a_9, ad_a)
            .expect("alice should be able to encrypt");
        assert_eq!(
            Ok(Vec::from(&pt_a_9[..])),
            bob.ratchet_decrypt(&mut store, &h_a_9, &ct_a_9, ad_a)
        );
        assert_eq!(
            Ok(Vec::from(&pt_a_0[..])),
            bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
        );

        // Bob's replies also arrive out of order
        let pt_b_0 = b"Yes I'm here";
        let (h_b_0, ct_b_0) = bob
            .ratchet_encrypt(pt_b_0, ad_b)
            .expect("bob should be able to encrypt now");
        for _ in 1..9 {
            bob.ratchet_encrypt(b"why?", ad_b)
                .expect("bob should be able to encrypt now"); // drop these messages
        }
        let pt_b_9 = b"Tell me why!!!";
        let (h_b_9, ct_b_9) = bob
            .ratchet_encrypt(pt_b_9, ad_b)
            .expect("bob should be able to encrypt now");
        assert_eq!(
            Ok(Vec::from(&pt_b_9[..])),
            alice.ratchet_decrypt(&mut store, &h_b_9, &ct_b_9, ad_b)
        );
        assert_eq!(
            Ok(Vec::from(&pt_b_0[..])),
            alice.ratchet_decrypt(&mut store, &h_b_0, &ct_b_0, ad_b)
        );
    }

    #[test]
    #[serial]
    fn dh_out_of_order() {
        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();
        let (ad_a, ad_b) = (b"A2B", b"B2A");

        let pt_a_0 = b"Good day Robert";
        let (h_a_0, ct_a_0) = alice
            .ratchet_encrypt(pt_a_0, ad_a)
            .expect("alice should be able to encrypt");
        assert_eq!(
            Ok(Vec::from(&pt_a_0[..])),
            bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
        );
        let pt_a_1 = b"Do you like Rust?";
        let (h_a_1, ct_a_1) = alice
            .ratchet_encrypt(pt_a_1, ad_a)
            .expect("alice should be able to encrypt");
        // Bob misses pt_a_1

        let pt_b_0 = b"Salutations Allison";
        let (h_b_0, ct_b_0) = bob
            .ratchet_encrypt(pt_b_0, ad_b)
            .expect("bob should be able toe cnrypt now");
        // Alice misses pt_b_0
        let pt_b_1 = b"How is your day going?";
        let (h_b_1, ct_b_1) = bob
            .ratchet_encrypt(pt_b_1, ad_b)
            .expect("bob should be able toe cnrypt now");
        assert_eq!(
            Ok(Vec::from(&pt_b_1[..])),
            alice.ratchet_decrypt(&mut store, &h_b_1, &ct_b_1, ad_b)
        );

        let pt_a_2 = b"My day is fine.";
        let (h_a_2, ct_a_2) = alice
            .ratchet_encrypt(pt_a_2, ad_a)
            .expect("alice should be able to encrypt");
        assert_eq!(
            Ok(Vec::from(&pt_a_2[..])),
            bob.ratchet_decrypt(&mut store, &h_a_2, &ct_a_2, ad_a)
        );
        // now Bob receives pt_a_1
        assert_eq!(
            Ok(Vec::from(&pt_a_1[..])),
            bob.ratchet_decrypt(&mut store, &h_a_1, &ct_a_1, ad_a)
        );

        let pt_b_2 = b"Yes I like Rust";
        let (h_b_2, ct_b_2) = bob
            .ratchet_encrypt(pt_b_2, ad_b)
            .expect("bob should be able toe cnrypt now");
        assert_eq!(
            Ok(Vec::from(&pt_b_2[..])),
            alice.ratchet_decrypt(&mut store, &h_b_2, &ct_b_2, ad_b)
        );
        // now Alice receives pt_b_0
        assert_eq!(
            Ok(Vec::from(&pt_b_0[..])),
            alice.ratchet_decrypt(&mut store, &h_b_0, &ct_b_0, ad_b)
        );
    }

    #[test]
    #[serial]
    fn encrypt_error() {
        let (_alice, mut bob) = asymmetric_setup();

        assert_eq!(None, bob.ratchet_encrypt(b"", b"",));
    }

    #[test]
    #[serial]
    fn decrypt_failure() {
        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();
        let (ad_a, ad_b) = (b"A2B", b"B2A");

        // Next chain
        let (h_a_0, ct_a_0) = alice
            .ratchet_encrypt(b"Hi Bob", ad_a)
            .expect("alice can always encrypt");
        let mut ct_a_0_err = ct_a_0.clone();
        ct_a_0_err[2] ^= 0x80;
        let mut h_a_0_err = h_a_0.clone();
        h_a_0_err.pn = 1;
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0_err, ad_a)
        );
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_0_err, &ct_a_0, ad_a)
        );
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_b)
        );

        // Current Chain
        let (h_a_1, ct_a_1) = alice
            .ratchet_encrypt(b"Hi Bob", ad_a)
            .expect("alice can always encrypt");
        bob.ratchet_decrypt(&mut store, &h_a_1, &ct_a_1, ad_a)
            .unwrap();
        let (h_a_2, ct_a_2) = alice
            .ratchet_encrypt(b"Hi Bob", ad_a)
            .expect("alice can always encrypt");
        let mut h_a_2_err = h_a_2.clone();
        h_a_2_err.pn += 1;
        let mut ct_a_2_err = ct_a_2.clone();
        ct_a_2_err[0] ^= 0x04;

        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_2, &ct_a_2_err, ad_a)
        );
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_2_err, &ct_a_2, ad_a)
        );
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_2, &ct_a_2, ad_b)
        );

        // Previous chain
        let (h_b, ct_b) = bob
            .ratchet_encrypt(b"Hi Alice", ad_b)
            .expect("bob should be able to encrypt now");
        alice
            .ratchet_decrypt(&mut store, &h_b, &ct_b, ad_b)
            .unwrap();
        let (h_a_3, ct_a_3) = alice
            .ratchet_encrypt(b"Hi Bob", ad_a)
            .expect("alice can always encrypt");
        bob.ratchet_decrypt(&mut store, &h_a_3, &ct_a_3, ad_a)
            .unwrap();

        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_2, &ct_a_2_err, ad_a)
        );
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_2_err, &ct_a_2, ad_a)
        );
        assert_eq!(
            Err(DecryptError::DecryptFailure),
            bob.ratchet_decrypt(&mut store, &h_a_2, &ct_a_2, ad_b)
        );
    }

    #[test]
    #[serial]
    fn double_sending() {
        // The implementation is unable to consistently detect why decryption fails when receiving
        // double messages: the only requirement should be that *any* error is triggered.

        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();
        let (ad_a, ad_b) = (b"A2B", b"B2A");

        let (h_a_0, ct_a_0) = alice
            .ratchet_encrypt(b"Whatever", ad_a)
            .expect("alice should always be able to encrypt");
        bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
            .expect("bob failed to decrypt");
        assert!(
            bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
                .is_err(),
            "bob should only be able to decrypt once"
        );

        let (h_b_0, ct_b_0) = bob
            .ratchet_encrypt(b"Whatever", ad_b)
            .expect("bob should be able to encrypt now");
        alice
            .ratchet_decrypt(&mut store, &h_b_0, &ct_b_0, ad_b)
            .expect("alice failed to decrypt");
        assert!(
            alice
                .ratchet_decrypt(&mut store, &h_b_0, &ct_b_0, ad_b)
                .is_err(),
            "alice should only be able to decrypt once"
        );
        let (h_a_1, ct_a_1) = alice
            .ratchet_encrypt(b"Whatever", ad_a)
            .expect("alice should always be able to encrypt");
        bob.ratchet_decrypt(&mut store, &h_a_1, &ct_a_1, ad_a)
            .expect("bob failed to decrypt");
        assert!(
            bob.ratchet_decrypt(&mut store, &h_a_1, &ct_a_1, ad_a)
                .is_err(),
            "bob should only be able to decrypt once"
        );
        let (h_b_1, ct_b_1) = bob
            .ratchet_encrypt(b"Whatever", ad_b)
            .expect("bob should be able to encrypt now");
        alice
            .ratchet_decrypt(&mut store, &h_b_1, &ct_b_1, ad_b)
            .expect("alice failed to decrypt");
        assert!(
            alice
                .ratchet_decrypt(&mut store, &h_b_1, &ct_b_1, ad_b)
                .is_err(),
            "alice should only be able to decrypt once"
        );

        assert!(
            bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
                .is_err(),
            "bob really should have thrown the original keys out by now"
        );
        assert!(
            alice
                .ratchet_decrypt(&mut store, &h_b_0, &ct_b_0, ad_b)
                .is_err(),
            "alice should have thrown out her keys too"
        );
    }

    #[test]
    #[serial]
    fn invalid_header() {
        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();
        let (ad_a, ad_b) = (b"A2B", b"B2A");
        let (h_a_0, ct_a_0) = alice
            .ratchet_encrypt(b"Hi Bob", ad_a)
            .expect("alice can always encrypt");
        bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
            .expect("bob failed to decrypt");
        let (h_b_0, ct_b_0) = bob
            .ratchet_encrypt(b"Hi Alice", ad_b)
            .expect("bob should be able to encrypt now");
        alice
            .ratchet_decrypt(&mut store, &h_b_0, &ct_b_0, ad_b)
            .unwrap();
        let (mut h_a_1, ct_a_1) = alice
            .ratchet_encrypt(b"I will lie to you now", ad_a)
            .expect("alice can always encrypt");
        assert_eq!(h_a_1.pn, 1);
        h_a_1.pn = 0;
        assert!(bob
            .ratchet_decrypt(&mut store, &h_a_1, &ct_a_1, ad_a)
            .is_err());
    }

    #[test]
    #[serial]
    fn skip_too_large() {
        let (mut alice, mut bob) = asymmetric_setup();
        let mut store = new_store();
        let (ad_a, ad_b) = (b"A2B", b"B2A");
        let (h_a_0, ct_a_0) = alice
            .ratchet_encrypt(b"Hi Bob", ad_a)
            .expect("alice should always be able to encrypt");
        for _ in 0..=MAX_SKIP {
            alice
                .ratchet_encrypt(b"Not sending this", ad_a)
                .expect("alice should always be able to encrypt");
        }
        let (h_a_1, ct_a_1) = alice
            .ratchet_encrypt(b"n > MAXSKIP", ad_a)
            .expect("alice should always be able to encrypt");
        match bob.ratchet_decrypt(&mut store, &h_a_1, &ct_a_1, ad_a) {
            Err(DecryptError::SkipTooLarge(..)) => {}
            Err(e) => {
                panic!("unexpected error when decrypting, error was: {}", e);
            }
            Ok(v) => {
                panic!(
                    "should have failed to skip due to large skip, instead found:\n{:#?}",
                    v
                );
            }
        }
        bob.ratchet_decrypt(&mut store, &h_a_0, &ct_a_0, ad_a)
            .expect("bob failed to decrypt");
        let (h_b, ct_b) = bob
            .ratchet_encrypt(b"Hi Alice", ad_b)
            .expect("bob should be able to encrypt");
        alice
            .ratchet_decrypt(&mut store, &h_b, &ct_b, ad_b)
            .expect("alice failed to decrypt");
        let (h_a_2, ct_a_2) = alice
            .ratchet_encrypt(b"pn > MAXSKIP", ad_a)
            .expect("alice should always be able to encrypt");
        assert_eq!(h_a_2.n, 0);
        bob.ratchet_decrypt(&mut store, &h_a_2, &ct_a_2, ad_a)
            .expect("bob should be able to decrypt now");
    }
}
