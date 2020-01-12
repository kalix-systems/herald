use super::*;
use crate::ratchet::double as dr;
use herald_common::*;
use kcl::*;
use std::error::Error as StdError;
use thiserror::*;

#[derive(Ser, De, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct PayloadId(random::UQ);

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Payload {
    Noop,
    SigUpdate(Signed<sig::SigUpdate>),
    Init(Signed<x25519::PublicKey>),
    AddToConvo(ConversationId, Vec<UserId>),
    LeaveConvo(ConversationId),
    Msg(Bytes),
}

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Msg<E: StdError + Send + 'static> {
    Encrypted {
        id: PayloadId,
        header: dr::Header,
        payload: Bytes,
    },
    Success(PayloadId),
    Failed {
        id: PayloadId,
        reason: Option<dr::DecryptError<E>>,
    },
}

#[derive(Error, Debug, Ser, De)]
pub enum TransitError<E: StdError + Send + 'static> {
    #[error("Failed to decrypt: {0}")]
    Decryption(#[from] dr::DecryptError<E>),
    #[error("Failed to deserialize: {0}")]
    Kson(String),
    #[error("Bare store error: {0}")]
    Store(E),
    #[error("Tried to retreive ratchet with {0:#?} but none were found")]
    NoSession(sig::PublicKey),
    #[error("Tried to encrypt using uninitialized ratchet with {0:#?}, THIS SHOULD NEVER HAPPEN")]
    Uninit(sig::PublicKey),
}

pub trait RatchetStore: StoreLike {
    fn get_ratchet(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<dr::DoubleRatchet>, Self::Error>;

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error>;
}

pub fn start_session<S: RatchetStore>(
    store: &mut S,
    me: &sig::KeyPair,
    them: sig::PublicKey,
) -> Result<Msg<S::Error>, TransitError<S::Error>> {
    let xkp = x25519::KeyPair::gen_new();
    let signed_pub = sig::sign_ser(me, *xkp.public());
    let them_as_kx = x25519::PublicKey::from(them);
    let secret = dr::diffie_hellman(&xkp, &them_as_kx);

    let ratchet = dr::DoubleRatchet::new_alice(&secret, them_as_kx, None);
    store
        .store_ratchet(them, ratchet)
        .map_err(TransitError::Store)?;

    let payload = Payload::Init(signed_pub);
    let msg = encrypt_payload(store, me, them, &payload)?;

    Ok(msg)
}

pub fn encrypt_payload<S: RatchetStore>(
    store: &mut S,
    me: &sig::KeyPair,
    to: sig::PublicKey,
    payload: &Payload,
) -> Result<Msg<S::Error>, TransitError<S::Error>> {
    let id = PayloadId(UQ::gen_new());
    let ad = mk_ad(*me.public(), id);
    let mut ratchet = store
        .get_ratchet(to)
        .map_err(TransitError::Store)?
        .ok_or(TransitError::NoSession(to))?;
    let (header, ct) = ratchet
        .ratchet_encrypt(&kson::to_vec(&payload), ad.as_ref())
        .ok_or(TransitError::Uninit(to))?;
    let msg = Msg::Encrypted {
        id,
        header,
        payload: Bytes::from(ct),
    };
    store
        .store_ratchet(to, ratchet)
        .map_err(TransitError::Store)?;
    Ok(msg)
}

pub fn decrypt_payload<S: RatchetStore + dr::KeyStore>(
    store: &mut S,
    me: &sig::KeyPair,
    from: sig::PublicKey,
    id: PayloadId,
    header: dr::Header,
    payload: Bytes,
) -> Result<Payload, TransitError<S::Error>> {
    let ad = mk_ad(*me.public(), id);
    let mut ratchet = store
        .get_ratchet(from)
        .map_err(TransitError::Store)?
        .unwrap_or_else(|| {
            let xkp = x25519::KeyPair::from(me.clone());
            let secret = dr::diffie_hellman(&xkp, header.dh());
            let ratchet = dr::DoubleRatchet::new_bob(secret, xkp, *header.dh(), None);
            ratchet
        });

    let decrypted = ratchet.ratchet_decrypt(store, &header, &payload, &ad)?;
    store
        .store_ratchet(from, ratchet)
        .map_err(TransitError::Store)?;
    let payload =
        kson::from_bytes(decrypted.into()).map_err(|e| TransitError::Kson(format!("{}", e)))?;

    Ok(payload)
}

pub trait SigStore: StoreLike {
    fn start_sigchain(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<(), Self::Error>;

    fn extend_sigchain(
        &mut self,
        from: UserId,
        update: Signed<sig::SigUpdate>,
    ) -> Result<(), Self::Error>;

    fn get_sigchain(
        &mut self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Self::Error>;

    fn active_keys(
        &mut self,
        of: UserId,
    ) -> Result<Vec<sig::PublicKey>, Self::Error>;

    fn key_is_valid(
        &mut self,
        key: sig::PublicKey,
        valid_for: UserId,
    ) -> Result<bool, Self::Error>;
}

pub trait ConversationStore: StoreLike {
    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error>;

    fn left_convo(
        &mut self,
        cid: ConversationId,
        from: UserId,
    ) -> Result<(), Self::Error>;

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Option<Vec<UserId>>, Self::Error>;

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error>;
}

#[derive(Debug, Error)]
pub enum PayloadError<E: StdError + Send + 'static> {
    #[error("Bare store error: {0}")]
    Store(E),
    #[error("Invalid signature: {0:#?}")]
    BadSig(SigValid),
    #[error("Message should not have been sent by this device, the sender is being sketchy")]
    InvalidSender,
}

pub struct PayloadResult {
    pub msg: Option<Bytes>,
    pub forward: bool,
}

pub fn handle_payload<S: SigStore + ConversationStore>(
    store: &mut S,
    from: GlobalId,
    payload: Payload,
) -> Result<PayloadResult, PayloadError<S::Error>> {
    use Payload::*;

    let mut msg = None;
    let mut forward = false;

    match payload {
        Noop => {}
        SigUpdate(sig) => {
            let valid = sig::validate_update(&sig);
            if valid != SigValid::Yes {
                return Err(PayloadError::BadSig(valid));
            }

            if !store
                .key_is_valid(from.did, from.uid)
                .map_err(PayloadError::Store)?
            {
                return Err(PayloadError::InvalidSender);
            }

            forward = true;

            store
                .extend_sigchain(from.uid, sig)
                .map_err(PayloadError::Store)?;
        }
        Init(sig) => {
            let valid = sig.verify_sig();
            if valid != SigValid::Yes {
                return Err(PayloadError::BadSig(valid));
            }
            // TODO: remove this check after setting up a "KnownSigner" struct, eventually...
            if *sig.signed_by() != from.did {
                return Err(PayloadError::InvalidSender);
            }
        }
        AddToConvo(cid, mems) => {
            if !store
                .member_of(cid, from.uid)
                .map_err(PayloadError::Store)?
            {
                return Err(PayloadError::InvalidSender);
            }
            store.add_to_convo(cid, mems).map_err(PayloadError::Store)?;
        }
        LeaveConvo(cid) => {
            store
                .left_convo(cid, from.uid)
                .map_err(PayloadError::Store)?;
        }
        Msg(content) => {
            msg = Some(content);
        }
    }

    Ok(PayloadResult { msg, forward })
}

// TODO: replace this with [u8;64]
fn mk_ad(
    pk: sig::PublicKey,
    id: PayloadId,
) -> Bytes {
    pk.as_ref()
        .iter()
        .chain(id.0.as_ref().iter())
        .copied()
        .collect()
}
