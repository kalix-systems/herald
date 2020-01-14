use super::*;
use crate::ratchet::double as dr;
use herald_common::*;
use kcl::*;
use std::error::Error as StdError;
use thiserror::*;

mod errors;
mod traits;
pub use errors::*;
pub use traits::*;

#[derive(Ser, De, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct PayloadId(random::UQ);

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Payload {
    Noop,
    SigUpdate(Signed<sig::SigUpdate>),
    Forwarded(UserId, Signed<sig::SigUpdate>),
    Init(Signed<x25519::PublicKey>),
    AddToConvo(ConversationId, Vec<UserId>),
    LeaveConvo(ConversationId),
    Msg(Bytes),
}

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Msg {
    Encrypted {
        id: PayloadId,
        header: dr::Header,
        payload: Bytes,
    },
    Ack(Ack),
}

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Ack {
    Success(PayloadId),
    Failed {
        id: PayloadId,
        reason: FailureReason,
    },
}

pub fn start_session<S: RatchetStore>(
    store: &mut S,
    me: &sig::KeyPair,
    them: sig::PublicKey,
) -> Result<Msg, TransitError<S::Error>> {
    let xkp = x25519::KeyPair::gen_new();
    let signed_pub = sig::sign_ser(me, *xkp.public());
    let them_as_kx = x25519::PublicKey::from(them);
    let secret = dr::diffie_hellman(&xkp, &them_as_kx);

    let ratchet = dr::DoubleRatchet::new_alice(&secret, them_as_kx, None);
    store
        .store_ratchet(them, ratchet)
        .map_err(TransitError::Store)?;

    let payload = Payload::Init(signed_pub);
    let id = PayloadId(UQ::gen_new());
    let msg = encrypt_payload(store, me, them, id, &payload)?;

    Ok(msg)
}

pub fn encrypt_payload<S: RatchetStore>(
    store: &mut S,
    me: &sig::KeyPair,
    to: sig::PublicKey,
    id: PayloadId,
    payload: &Payload,
) -> Result<Msg, TransitError<S::Error>> {
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
    let payload = kson::from_bytes(decrypted.into())?;

    Ok(payload)
}

pub struct PayloadResult {
    pub msg: Option<Bytes>,
    pub forward: Option<Payload>,
}

pub fn handle_payload<S: SigStore + ConversationStore>(
    store: &mut S,
    from: GlobalId,
    payload: Payload,
) -> Result<PayloadResult, PayloadError<S::Error>> {
    use Payload::*;

    let mut msg = None;
    let mut forward = None;

    match payload {
        Noop => {}
        Forwarded(uid, sig) => {
            let valid = sig::validate_update(&sig);
            if valid != SigValid::Yes {
                return Err(PayloadError::BadSig(valid));
            }

            if !store
                .key_is_valid(*sig.signed_by(), uid)
                .map_err(PayloadError::Store)?
            {
                return Err(PayloadError::InvalidSender);
            }

            store
                .extend_sigchain(uid, sig)
                .map_err(PayloadError::Store)?;
        }
        SigUpdate(sig) => {
            forward = Some(Forwarded(from.uid, sig));

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

fn keys_of_cid<S: ConversationStore + SigStore>(
    store: &mut S,
    my_key: sig::PublicKey,
    cid: ConversationId,
) -> Result<Vec<sig::PublicKey>, S::Error> {
    let mems = store.get_members(cid)?;
    let mut out = Vec::with_capacity(mems.len());
    for mem in mems {
        let keys = store.active_keys(mem)?;
        out.reserve(keys.len());
        for key in keys.into_iter().filter(|k| k != &my_key) {
            out.push(key);
        }
    }
    Ok(out)
}

fn prepare_send_to_keys<S>(
    store: &mut S,
    my_keypair: &sig::KeyPair,
    send_to: Vec<sig::PublicKey>,
    payload: Payload,
) -> Result<Vec<(sig::PublicKey, Msg)>, TransitError<S::Error>>
where
    S: dr::KeyStore + RatchetStore + PendingStore,
{
    let id = PayloadId(UQ::gen_new());
    store
        .add_pending_payload(id, payload.clone(), &send_to)
        .map_err(TransitError::Store)?;

    let mut out = Vec::with_capacity(send_to.len());
    for key in send_to {
        let msg = encrypt_payload(store, my_keypair, key, id, &payload)?;
        out.push((key, msg));
    }

    Ok(out)
}

pub fn prepare_send_to_self<S>(
    store: &mut S,
    my_keypair: &sig::KeyPair,
    my_uid: UserId,
    payload: Payload,
) -> Result<Vec<(sig::PublicKey, Msg)>, TransitError<S::Error>>
where
    S: dr::KeyStore + RatchetStore + PendingStore + SigStore,
{
    let keys = store
        .active_keys(my_uid)
        .map_err(TransitError::Store)?
        .into_iter()
        .filter(|k| k != my_keypair.public())
        .collect();

    prepare_send_to_keys(store, my_keypair, keys, payload)
}

pub fn prepare_send_to_convo<S>(
    store: &mut S,
    my_keypair: &sig::KeyPair,
    cid: ConversationId,
    msg: Bytes,
) -> Result<Vec<(sig::PublicKey, Msg)>, TransitError<S::Error>>
where
    S: dr::KeyStore + RatchetStore + ConversationStore + PendingStore + SigStore,
{
    let keys = keys_of_cid(store, *my_keypair.public(), cid).map_err(TransitError::Store)?;
    let payload = Payload::Msg(msg);
    prepare_send_to_keys(store, my_keypair, keys, payload)
}

pub fn prepare_send_to_all<S>(
    store: &mut S,
    my_keypair: &sig::KeyPair,
    payload: Payload,
) -> Result<Vec<(sig::PublicKey, Msg)>, TransitError<S::Error>>
where
    S: dr::KeyStore + RatchetStore + PendingStore + SigStore,
{
    let keys = store.all_active_keys().map_err(TransitError::Store)?;
    prepare_send_to_keys(store, my_keypair, keys, payload)
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