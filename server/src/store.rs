use herald_common::*;
use redis::Commands;

use crate::prelude::*;

pub trait Store {
    fn add_key(&mut self, uid: UserIdRef, key: Signed<sig::PublicKey>) -> Result<bool, Error>;
    fn read_key(&mut self, uid: UserIdRef, key: sig::PublicKey) -> Result<sig::PKMeta, Error>;
    fn deprecate_key(&mut self, uid: UserIdRef, key: Signed<sig::PublicKey>)
        -> Result<bool, Error>;

    fn key_is_valid(&mut self, uid: UserIdRef, key: sig::PublicKey) -> Result<bool, Error>;
    fn read_meta(&mut self, uid: UserIdRef) -> Result<UserMeta, Error>;

    fn add_prekey(&mut self, key: sig::PublicKey, pre: sealed::PublicKey) -> Result<bool, Error>;
    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error>;

    fn add_pending(&mut self, key: sig::PublicKey, msg: MessageToClient) -> Result<(), Error>;
    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<MessageToClient>, Error>;
}

pub(crate) fn prekeys_of(key: sig::PublicKey) -> Vec<u8> {
    let suffix = b":prekeys";
    let mut out = Vec::with_capacity(key.as_ref().len() + suffix.len());
    out.extend_from_slice(key.as_ref());
    out.extend_from_slice(suffix);
    out
}

pub(crate) fn pending_of(key: sig::PublicKey) -> Vec<u8> {
    let suffix = b":pending";
    let mut out = Vec::with_capacity(key.as_ref().len() + suffix.len());
    out.extend_from_slice(key.as_ref());
    out.extend_from_slice(suffix);
    out
}

// note: not transactional by default
// can call transactionally by wrapping calls where appropriate
impl<C: redis::ConnectionLike> Store for C {
    fn add_key(&mut self, uid: UserIdRef, key: Signed<sig::PublicKey>) -> Result<bool, Error> {
        let (key, meta) = key.split();
        Ok(self.hset_nx(uid, key.as_ref(), serde_cbor::to_vec(&meta)?.as_slice())?)
    }

    fn read_key(&mut self, uid: UserIdRef, key: sig::PublicKey) -> Result<sig::PKMeta, Error> {
        let maybe_key: Option<Vec<u8>> = self.hget(uid, key.as_ref())?;
        Ok(serde_cbor::from_slice(&maybe_key.ok_or(MissingData)?)?)
    }

    fn deprecate_key(
        &mut self,
        uid: UserIdRef,
        skey: Signed<sig::PublicKey>,
    ) -> Result<bool, Error> {
        if !skey.verify_sig() {
            return Err(InvalidSig);
        }
        let (key, sigmeta) = skey.split();

        let mut pkm = self.read_key(uid, key)?;
        let res = pkm.deprecate(sigmeta);
        self.hset(uid, key.as_ref(), serde_cbor::to_vec(&pkm)?.as_slice())?;
        Ok(res)
    }

    fn key_is_valid(&mut self, uid: UserIdRef, key: sig::PublicKey) -> Result<bool, Error> {
        let meta = self.read_key(uid, key)?;
        Ok(meta.key_is_valid(key) && self.hexists(uid, key.as_ref())?)
    }

    fn add_prekey(&mut self, key: sig::PublicKey, pre: sealed::PublicKey) -> Result<bool, Error> {
        if key != *pre.signed_by() {
            Ok(false)
        } else {
            self.rpush(prekeys_of(key), serde_cbor::to_vec(&pre)?)?;
            Ok(true)
        }
    }

    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error> {
        let maybe_key: Option<Vec<u8>> = self.lpop(prekeys_of(key))?;
        Ok(serde_cbor::from_slice(&maybe_key.ok_or(MissingData)?)?)
    }

    fn read_meta(&mut self, uid: UserIdRef) -> Result<UserMeta, Error> {
        let keys: Vec<Vec<u8>> = self.hkeys::<_, Option<_>>(uid)?.ok_or(MissingData)?;
        let mut out = UserMeta::new();
        for key in keys {
            let pk = sig::PublicKey::from_slice(&key).ok_or(BadData)?;
            let meta = self.read_key(uid, pk)?;
            out.add_key_unchecked(pk, meta);
        }
        Ok(out)
    }

    fn add_pending(&mut self, to: sig::PublicKey, msg: MessageToClient) -> Result<(), Error> {
        self.rpush(pending_of(to), serde_cbor::to_vec(&msg)?.as_slice())?;
        Ok(())
    }

    fn get_pending(&mut self, to: sig::PublicKey) -> Result<Vec<MessageToClient>, Error> {
        let msg_bs: Vec<Vec<u8>> = self.lrange(to.as_ref(), 0, -1)?;
        let mut out = Vec::with_capacity(msg_bs.len());
        for bs in msg_bs.iter().map(Vec::as_slice) {
            out.push(serde_cbor::from_slice(bs)?);
        }
        Ok(out)
    }
}
