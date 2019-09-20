use herald_common::*;
use redis::Commands;

use crate::prelude::*;

pub fn prekeys_of(key: sig::PublicKey) -> Vec<u8> {
    let suffix = b":prekeys";
    let mut out = Vec::with_capacity(key.as_ref().len() + suffix.len());
    out.extend_from_slice(key.as_ref());
    out.extend_from_slice(suffix);
    out
}

pub fn pending_of(key: sig::PublicKey) -> Vec<u8> {
    let suffix = b":pending";
    let mut out = Vec::with_capacity(key.as_ref().len() + suffix.len());
    out.extend_from_slice(key.as_ref());
    out.extend_from_slice(suffix);
    out
}

pub trait Store {
    fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error>;
    fn add_prekey(&mut self, key: sig::PublicKey, pre: sealed::PublicKey) -> Result<bool, Error>;
    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error>;

    fn add_key(&mut self, uid: &UserId, key: Signed<sig::PublicKey>) -> Result<bool, Error>;
    fn read_key(&mut self, uid: &UserId, key: sig::PublicKey) -> Result<sig::PKMeta, Error>;
    fn deprecate_key(&mut self, uid: &UserId, key: Signed<sig::PublicKey>) -> Result<bool, Error>;

    fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error>;
    // TODO: make this not require uid when we switch to postgres
    fn key_is_valid(&mut self, &UserId, key: sig::PublicKey) -> Result<bool, Error>;
    fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error>;

    fn add_pending(&mut self, key: sig::PublicKey, msg: Push) -> Result<(), Error>;
    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Error>;
    fn expire_pending(&mut self, key: sig::PublicKey) -> Result<(), Error>;
    fn persist_pending(&mut self, to: sig::PublicKey) -> Result<(), Error>;
}

// note: not transactional by default
// can call transactionally by wrapping calls where appropriate
impl<C: redis::ConnectionLike> Store for C {
    fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error> {
        unimplemented!()
    }

    fn add_key(&mut self, uid: &UserId, key: Signed<sig::PublicKey>) -> Result<bool, Error> {
        let (key, meta) = key.split();
        Ok(self.hset_nx(
            uid.as_str(),
            key.as_ref(),
            serde_cbor::to_vec(&meta)?.as_slice(),
        )?)
    }

    fn read_key(&mut self, uid: &UserId, key: sig::PublicKey) -> Result<sig::PKMeta, Error> {
        let maybe_key: Option<Vec<u8>> = self.hget(uid.as_str(), key.as_ref())?;
        Ok(serde_cbor::from_slice(&maybe_key.ok_or(MissingData)?)?)
    }

    fn deprecate_key(&mut self, uid: &UserId, skey: Signed<sig::PublicKey>) -> Result<bool, Error> {
        if !skey.verify_sig() {
            return Err(InvalidSig);
        }
        let (key, sigmeta) = skey.split();

        let mut pkm = self.read_key(uid, key)?;
        let res = pkm.deprecate(sigmeta);
        self.hset(
            uid.as_str(),
            key.as_ref(),
            serde_cbor::to_vec(&pkm)?.as_slice(),
        )?;

        Ok(res)
    }

    fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error> {
        Ok(self.exists(uid.as_str())?)
    }

    fn key_is_valid(&mut self, uid: &UserId, key: sig::PublicKey) -> Result<bool, Error> {
        let meta = self.read_key(uid, key)?;
        Ok(meta.key_is_valid(key) && self.hexists(uid.as_str(), key.as_ref())?)
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

    fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error> {
        let keys: Vec<Vec<u8>> = self
            .hkeys::<_, Option<_>>(uid.as_str())?
            .ok_or(MissingData)?;
        let mut out = UserMeta::new();
        for key in keys {
            let pk = sig::PublicKey::from_slice(&key).ok_or(BadData)?;
            let meta = self.read_key(uid, pk)?;
            out.add_key_unchecked(pk, meta);
        }
        Ok(out)
    }

    fn add_pending(&mut self, to: sig::PublicKey, msg: Push) -> Result<(), Error> {
        self.rpush(pending_of(to), serde_cbor::to_vec(&msg)?.as_slice())?;
        Ok(())
    }

    fn get_pending(&mut self, to: sig::PublicKey) -> Result<Vec<Push>, Error> {
        let msg_bs: Option<Vec<Vec<u8>>> = self.lrange(pending_of(to), 0, -1)?;
        let msg_bs = msg_bs.unwrap_or(Vec::new());
        let mut out = Vec::with_capacity(msg_bs.len());
        for bs in msg_bs.iter().map(Vec::as_slice) {
            out.push(serde_cbor::from_slice(bs)?);
        }
        Ok(out)
    }

    fn expire_pending(&mut self, to: sig::PublicKey) -> Result<(), Error> {
        self.expire(pending_of(to), 10)?;
        Ok(())
    }

    fn persist_pending(&mut self, to: sig::PublicKey) -> Result<(), Error> {
        self.persist(pending_of(to))?;
        Ok(())
    }
}
