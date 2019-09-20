use crate::*;

#[async_trait]
/// Handles protocol messages.
/// For the client, this will tag each message and send it to the server.
/// For the server, this will receive the message, process it, and send the response to the client.
pub trait ProtocolHandler {
    type Error: From<std::io::Error>;
    async fn handle_fanout(
        &mut self,
        fanout: fanout::ToServer,
    ) -> Result<fanout::ServerResponse, Self::Error>;
    async fn handle_pki(
        &mut self,
        msg: pubkey::ToServer,
    ) -> Result<pubkey::ServerResponse, Self::Error>;
    async fn handle_query(
        &mut self,
        query: query::ToServer,
    ) -> Result<query::ServerResponse, Self::Error>;
}

#[async_trait]
/// `PushHandler`s must also be able to handle incoming `Push` messages.
pub trait PushHandler: ProtocolHandler {
    async fn handle_push<'a>(&mut self, push: Push<'a>) -> Result<(), Self::Error>;
}

// TODO: implement this for client-side db?
/// A store for keys
/// Is not assumed to do any checking of signatures
pub trait Store {
    type Error;

    fn add_key(&mut self, uid: UserId, key: Signed<sig::PublicKey>) -> Result<bool, Self::Error>;
    fn read_key(&mut self, uid: UserId, key: sig::PublicKey) -> Result<sig::PKMeta, Self::Error>;
    fn deprecate_key(
        &mut self,
        uid: UserId,
        key: Signed<sig::PublicKey>,
    ) -> Result<bool, Self::Error>;

    fn user_exists(&mut self, uid: UserId) -> Result<bool, Self::Error>;
    fn key_is_valid(&mut self, uid: UserId, key: sig::PublicKey) -> Result<bool, Self::Error>;
    fn read_meta(&mut self, uid: UserId) -> Result<UserMeta, Self::Error>;

    fn add_prekey(&mut self, pre: sealed::PublicKey) -> Result<bool, Self::Error>;
    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Self::Error>;

    fn add_pending(&mut self, key: sig::PublicKey, msg: Push) -> Result<(), Self::Error>;
    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Self::Error>;
    fn remove_pending(&mut self, key: sig::PublicKey) -> Result<(), Self::Error>;
}

// TODO: replace RedisError with a real error type, uncomment relevant code in each method
#[allow(unused_variables, unreachable_code)]
pub mod redis_store {
    use super::*;
    use redis::*;
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

    impl<C: ConnectionLike> Store for C {
        type Error = RedisError;
        fn add_key(
            &mut self,
            uid: UserId,
            key: Signed<sig::PublicKey>,
        ) -> Result<bool, Self::Error> {
            unimplemented!()
            // let (key, meta) = key.split();
            // Ok(self.hset_nx(uid, key.as_ref(), serde_cbor::to_vec(&meta)?.as_slice())?)
        }

        fn read_key(
            &mut self,
            uid: UserId,
            key: sig::PublicKey,
        ) -> Result<sig::PKMeta, Self::Error> {
            unimplemented!()
            // let maybe_key: Option<Vec<u8>> = self.hget(uid, key.as_ref())?;
            // Ok(serde_cbor::from_slice(&maybe_key.ok_or(MissingData)?)?)
        }

        fn deprecate_key(
            &mut self,
            uid: UserId,
            key: Signed<sig::PublicKey>,
        ) -> Result<bool, Self::Error> {
            unimplemented!()
            // if !skey.verify_sig() {
            //     return Err(InvalidSig);
            // }
            // let (key, sigmeta) = skey.split();

            // let mut pkm = self.read_key(uid, key)?;
            // let res = pkm.deprecate(sigmeta);
            // self.hset(uid, key.as_ref(), serde_cbor::to_vec(&pkm)?.as_slice())?;

            // Ok(res)
        }

        fn user_exists(&mut self, uid: UserId) -> Result<bool, Self::Error> {
            Ok(self.exists(uid.as_str())?)
        }

        fn key_is_valid(&mut self, uid: UserId, key: sig::PublicKey) -> Result<bool, Self::Error> {
            let meta = self.read_key(uid, key)?;
            Ok(meta.key_is_valid(key) && self.hexists(uid.as_str(), key.as_ref())?)
        }

        fn read_meta(&mut self, uid: UserId) -> Result<UserMeta, Self::Error> {
            let keys: Vec<Vec<u8>> = unimplemented!(); //self.hkeys::<_, Option<_>>(uid)?.ok_or(MissingData)?;
            let mut out = UserMeta::new();
            for key in keys {
                let pk = unimplemented!(); //sig::PublicKey::from_slice(&key).ok_or(BadData)?;
                let meta = self.read_key(uid, pk)?;
                out.add_key_unchecked(pk, meta);
            }
            Ok(out)
        }

        fn add_prekey(&mut self, pre: sealed::PublicKey) -> Result<bool, Self::Error> {
            unimplemented!()
            // self.rpush(prekeys_of(*pre.signed_by()), serde_cbor::to_vec(&pre)?)?;
            // Ok(true)
        }
        fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Self::Error> {
            let maybe_key: Option<Vec<u8>> = self.lpop(prekeys_of(key))?;
            // Ok(serde_cbor::from_slice(&maybe_key.ok_or(MissingData)?)?)
            unimplemented!()
        }

        fn add_pending(&mut self, key: sig::PublicKey, msg: Push) -> Result<(), Self::Error> {
            unimplemented!()
            // self.rpush(pending_of(to), serde_cbor::to_vec(&msg)?.as_slice())?;
            // Ok(())
        }
        fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Self::Error> {
            unimplemented!()
            // let msg_bs: Option<Vec<Vec<u8>>> = self.lrange(pending_of(to), 0, -1)?;
            // let msg_bs = msg_bs.unwrap_or(Vec::new());
            // let mut out = Vec::with_capacity(msg_bs.len());
            // for bs in msg_bs.iter().map(Vec::as_slice) {
            //     out.push(serde_cbor::from_slice(bs)?);
            // }
            // Ok(out)
        }
        fn remove_pending(&mut self, key: sig::PublicKey) -> Result<(), Self::Error> {
            self.expire(pending_of(key), 10)?;
            Ok(())
        }
    }
}
