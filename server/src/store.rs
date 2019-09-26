use crate::{prelude::*, schema::*};
use diesel::{
    dsl::*,
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError, QueryResult},
};
use dotenv::dotenv;
use herald_common::*;
use std::{
    env,
    ops::{Deref, DerefMut},
};

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

// TODO: consider having this take slices instead of vec's
pub trait Store {
    fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error>;
    fn add_prekey(
        &mut self,
        key: sig::PublicKey,
        pre: sealed::PublicKey,
    ) -> Result<PKIResponse, Error>;
    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error>;

    fn add_key(
        &mut self,
        user_id: UserId,
        key: Signed<sig::PublicKey>,
    ) -> Result<PKIResponse, Error>;
    fn register_user(
        &mut self,
        user_id: UserId,
        key: Signed<sig::PublicKey>,
    ) -> Result<register::ToClient, Error>;
    fn read_key(&mut self, key: sig::PublicKey) -> Result<sig::PKMeta, Error>;
    fn deprecate_key(&mut self, key: Signed<sig::PublicKey>) -> Result<PKIResponse, Error>;

    fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error>;
    fn key_is_valid(&mut self, key: sig::PublicKey) -> Result<bool, Error>;
    fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error>;

    // TODO: make this take a vec of messages
    fn add_pending(&mut self, key: Vec<sig::PublicKey>, msg: Push) -> Result<(), Error>;
    // TODO: replace these w/methods that get first n, remove first n, in insertion order
    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Error>;
    fn expire_pending(&mut self, key: sig::PublicKey) -> Result<(), Error>;
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    r2d2::Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    dotenv().expect("Invalid dotenv");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn unique_violation_to_redundant<T>(query_res: QueryResult<T>) -> Result<PKIResponse, Error> {
    match query_res {
        Err(DatabaseError(UniqueViolation, _)) => Ok(PKIResponse::Redundant),
        a => {
            a?;
            Ok(PKIResponse::Success)
        }
    }
}

type RawPkMeta = (
    Vec<u8>,
    Vec<u8>,
    DateTime<Utc>,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
    Option<DateTime<Utc>>,
);

type RawKeyAndMeta = (
    Vec<u8>,
    Vec<u8>,
    DateTime<Utc>,
    Vec<u8>,
    Option<DateTime<Utc>>,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
);

impl Store for Conn {
    // TODO implement the appropriate traits for this
    // TODO read about postgres performance
    fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error> {
        use crate::schema::userkeys::dsl::*;

        Ok(select(exists(userkeys.filter(key.eq(pk.as_ref())))).get_result(self.deref_mut())?)
    }

    fn add_prekey(
        &mut self,
        key: sig::PublicKey,
        pre: sealed::PublicKey,
    ) -> Result<PKIResponse, Error> {
        use crate::schema::prekeys::dsl::*;

        let res = diesel::insert_into(prekeys)
            .values((
                signing_key.eq(key.as_ref()),
                sealed_key.eq(serde_cbor::to_vec(&pre)?),
            ))
            .execute(self.deref_mut());

        unique_violation_to_redundant(res)
    }

    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error> {
        use crate::schema::prekeys::dsl::*;

        let raw_pk: Vec<u8> = prekeys
            .filter(signing_key.eq(key.as_ref()))
            .select(sealed_key)
            .limit(1)
            .get_result(self.deref_mut())?;

        Ok(serde_cbor::from_slice(raw_pk.as_slice())?)
    }

    fn register_user(
        &mut self,
        user_id: UserId,
        key: Signed<sig::PublicKey>,
    ) -> Result<register::ToClient, Error> {
        let builder = self.build_transaction().deferrable();

        builder.run(|| {
            let query = userkeys::table.filter(userkeys::user_id.eq(user_id.as_str()));

            if select(exists(query)).get_result(&self.0)? {
                return Ok(register::ToClient::UIDTaken);
            }

            diesel::insert_into(keys::table)
                .values((
                    keys::key.eq(key.data().as_ref()),
                    keys::signed_by.eq(key.signed_by().as_ref()),
                    keys::ts.eq(key.timestamp()),
                    keys::signature.eq(key.sig().as_ref()),
                ))
                .execute(&self.0)?;

            diesel::insert_into(userkeys::table)
                .values((
                    userkeys::user_id.eq(user_id.as_str()),
                    userkeys::key.eq(key.data().as_ref()),
                ))
                .execute(&self.0)?;
            return Ok(register::ToClient::Success);
        })
    }

    fn add_key(
        &mut self,
        user_id_arg: UserId,
        new_key: Signed<sig::PublicKey>,
    ) -> Result<PKIResponse, Error> {
        let builder = self.build_transaction().deferrable();
        builder.run(|| {
            let query = userkeys::table.filter(userkeys::user_id.eq(user_id_arg.as_str()));

            if !select(exists(query)).get_result(&self.0)? {
                return Err(MissingData);
            }

            let res = diesel::insert_into(keys::table)
                .values((
                    keys::key.eq(new_key.data().as_ref()),
                    keys::signed_by.eq(new_key.signed_by().as_ref()),
                    keys::ts.eq(new_key.timestamp()),
                    keys::signature.eq(new_key.sig().as_ref()),
                ))
                .execute(&self.0);

            unique_violation_to_redundant(res)?;

            let res = diesel::insert_into(userkeys::table)
                .values((
                    userkeys::user_id.eq(user_id_arg.as_str()),
                    userkeys::key.eq(new_key.data().as_ref()),
                ))
                .execute(&self.0);

            unique_violation_to_redundant(res)
        })
    }

    fn read_key(&mut self, key_arg: sig::PublicKey) -> Result<sig::PKMeta, Error> {
        let (signed_by, sig, ts, dep_signed_by, dep_signature, dep_ts): RawPkMeta = keys::table
            .filter(keys::key.eq(key_arg.as_ref()))
            .select((
                keys::signed_by,
                keys::signature,
                keys::ts,
                keys::dep_signed_by,
                keys::dep_signature,
                keys::dep_ts,
            ))
            .get_result(self.deref_mut())?;

        let sig = sig::Signature::from_slice(sig.as_slice()).ok_or(InvalidSig)?;
        let signed_by = sig::PublicKey::from_slice(signed_by.as_slice()).ok_or(InvalidKey)?;
        let sig_meta = SigMeta::new(sig, signed_by, ts);

        let dep_is_some = (&dep_signature, &dep_ts, &dep_signed_by) != (&None, &None, &None);

        let dep_sig_meta = if dep_is_some {
            let dep_ts = dep_ts.ok_or(MissingData)?;

            let dep_signed_by =
                sig::PublicKey::from_slice(&dep_signed_by.ok_or(MissingData)?).ok_or(InvalidKey)?;

            let dep_signature =
                sig::Signature::from_slice(&dep_signature.ok_or(MissingData)?).ok_or(InvalidSig)?;

            Some(SigMeta::new(dep_signature, dep_signed_by, dep_ts))
        } else {
            None
        };

        Ok(sig::PKMeta::new(sig_meta, dep_sig_meta))
    }

    fn deprecate_key(&mut self, signed_key: Signed<sig::PublicKey>) -> Result<PKIResponse, Error> {
        use crate::schema::keys::dsl::*;

        let (data, meta) = signed_key.split();
        let filter = keys
            .filter(key.eq(data.as_ref()))
            .filter(dep_ts.is_null())
            .filter(dep_signature.is_null())
            .filter(dep_signed_by.is_null());

        let num_updated = update(filter)
            .set((
                dep_ts.eq(meta.timestamp().naive_utc()),
                dep_signed_by.eq(meta.signed_by().as_ref()),
                dep_signature.eq(meta.sig().as_ref()),
            ))
            .execute(self.deref_mut())?;

        if num_updated != 1 {
            return Ok(PKIResponse::Redundant);
        }

        Ok(PKIResponse::Success)
    }

    fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error> {
        use crate::schema::userkeys::dsl::*;

        let query = userkeys.filter(user_id.eq(uid.as_str()));

        Ok(select(exists(query)).get_result(self.deref_mut())?)
    }

    fn key_is_valid(&mut self, key_arg: sig::PublicKey) -> Result<bool, Error> {
        use crate::schema::userkeys::dsl::*;

        let query = userkeys
            .filter(key.eq(key_arg.as_ref()))
            .inner_join(keys::table)
            .filter(keys::dep_ts.is_null())
            .filter(keys::dep_signed_by.is_null())
            .filter(keys::dep_signature.is_null());

        Ok(select(exists(query)).get_result(self.deref_mut())?)
    }

    fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error> {
        let keys: Vec<RawKeyAndMeta> = userkeys::table
            .filter(userkeys::user_id.eq(uid.as_str()))
            .inner_join(keys::table)
            .select((
                keys::key,
                keys::signed_by,
                keys::ts,
                keys::signature,
                keys::dep_ts,
                keys::dep_signed_by,
                keys::dep_signature,
            ))
            .get_results(self.deref_mut())?;

        let meta_inner: Result<HashMap<sig::PublicKey, sig::PKMeta>, Error> = keys
            .into_iter()
            .map(
                |(
                    key,
                    signed_by,
                    creation_ts,
                    signature,
                    deprecation_ts,
                    dep_signed_by,
                    dep_signature,
                )| {
                    let key = sig::PublicKey::from_slice(&key).ok_or(InvalidKey)?;
                    let signed_by = sig::PublicKey::from_slice(&signed_by).ok_or(InvalidKey)?;
                    let timestamp = creation_ts;
                    let signature = sig::Signature::from_slice(&signature).ok_or(InvalidSig)?;

                    let dep_is_some = deprecation_ts.is_some()
                        || dep_signed_by.is_some()
                        || dep_signature.is_some();

                    let dep_sig_meta = if dep_is_some {
                        let dep_sig =
                            sig::Signature::from_slice(&dep_signature.ok_or(MissingData)?)
                                .ok_or(InvalidSig)?;

                        let dep_signed_by =
                            sig::PublicKey::from_slice(&dep_signed_by.ok_or(MissingData)?)
                                .ok_or(InvalidKey)?;

                        let dep_ts = deprecation_ts.ok_or(MissingData)?;

                        Some(SigMeta::new(dep_sig, dep_signed_by, dep_ts))
                    } else {
                        None
                    };

                    let sig_meta = SigMeta::new(signature, signed_by, timestamp);
                    let pkmeta = sig::PKMeta::new(sig_meta, dep_sig_meta);
                    Ok((key, pkmeta))
                },
            )
            .collect();

        Ok(UserMeta { keys: meta_inner? })
    }

    fn add_pending(&mut self, key_arg: Vec<sig::PublicKey>, msg: Push) -> Result<(), Error> {
        let push_row_id: i64 = {
            use crate::schema::pushes::dsl::*;

            let push_vec = serde_cbor::to_vec(&msg)?;
            insert_into(pushes)
                .values(push_data.eq(push_vec))
                .returning(push_id)
                .get_result(self.deref_mut())?
        };

        use crate::schema::pending::dsl::*;

        let keys: Vec<_> = key_arg
            .into_iter()
            .map(|k| (key.eq(k.as_ref().to_vec()), push_id.eq(push_row_id)))
            .collect();

        insert_into(pending)
            .values(keys)
            .execute(self.deref_mut())?;

        Ok(())
    }

    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Error> {
        let pushes: Vec<Vec<u8>> = pending::table
            .inner_join(pushes::table)
            .filter(pending::key.eq(key.as_ref()))
            .select(pushes::push_data)
            .get_results(self.deref_mut())?;

        let mut out = Vec::with_capacity(pushes.len());

        for p in pushes.into_iter() {
            out.push(serde_cbor::from_slice(&p)?);
        }

        Ok(out)
    }

    fn expire_pending(&mut self, key: sig::PublicKey) -> Result<(), Error> {
        let push_ids = pending::table
            .inner_join(pushes::table)
            .filter(pending::key.eq(key.as_ref()))
            .select(pushes::push_id);

        delete(pushes::table.filter(pushes::push_id.eq_any(push_ids))).execute(self.deref_mut())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    fn open_conn() -> Conn {
        let pool = init_pool();

        let conn = pool.get().expect("Failed to get connection");
        diesel::delete(pending::table)
            .execute(conn.deref())
            .expect(womp!());
        diesel::delete(pushes::table)
            .execute(conn.deref())
            .expect(womp!());
        diesel::delete(prekeys::table)
            .execute(conn.deref())
            .expect(womp!());
        diesel::delete(userkeys::table)
            .execute(conn.deref())
            .expect(womp!());
        diesel::delete(keys::table)
            .execute(conn.deref())
            .expect(womp!());

        Conn(conn)
    }

    #[test]
    #[serial]
    fn device_exists() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.device_exists(kp.public_key()).unwrap());

        conn.register_user(user_id, signed_pk).unwrap();

        assert!(conn.device_exists(kp.public_key()).unwrap());
    }

    #[test]
    #[serial]
    fn register_and_add() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.device_exists(kp.public_key()).unwrap());

        conn.register_user(user_id, signed_pk).unwrap();

        assert!(conn.device_exists(kp.public_key()).unwrap());

        let kp = sig::KeyPair::gen_new();
        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.device_exists(kp.public_key()).unwrap());

        conn.add_key(user_id, signed_pk).unwrap();

        assert!(conn.device_exists(kp.public_key()).unwrap());
    }

    #[test]
    #[serial]
    fn register_twice() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());

        conn.register_user(user_id, signed_pk).unwrap();

        let kp = sig::KeyPair::gen_new();
        let signed_pk = kp.sign(*kp.public_key());

        match conn.register_user(user_id, signed_pk) {
            Ok(register::ToClient::UIDTaken) => {}
            _ => panic!(),
        }
    }

    #[test]
    #[serial]
    fn read_key() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        assert!(conn.read_key(*kp.public_key()).is_err());

        let signed_pk = kp.sign(*kp.public_key());

        conn.register_user(user_id, signed_pk).unwrap();

        assert!(conn.key_is_valid(*kp.public_key()).unwrap());

        let meta = conn
            .read_key(*kp.public_key())
            .expect("Couldn't read key meta");

        assert!(meta.key_is_valid(*kp.public_key()));

        conn.deprecate_key(signed_pk).unwrap();

        let meta = conn
            .read_key(*kp.public_key())
            .expect("Couldn't read key meta");

        assert!(!meta.key_is_valid(*kp.public_key()));
    }

    #[test]
    #[serial]
    fn user_exists() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.user_exists(&user_id).unwrap());

        conn.register_user(user_id, signed_pk).unwrap();

        assert!(conn.user_exists(&user_id).unwrap());
    }

    #[test]
    #[serial]
    fn read_meta() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());

        conn.register_user(user_id, signed_pk).unwrap();

        let keys = conn.read_meta(&user_id).unwrap().keys;
        assert_eq!(keys.len(), 1);
    }

    #[test]
    #[serial]
    fn add_get_expire_pending() {
        let mut conn = open_conn();

        let kp_other = sig::KeyPair::gen_new();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        conn.register_user(user_id, signed_pk).unwrap();

        let pending = conn.get_pending(*kp.public_key()).unwrap();
        assert_eq!(pending.len(), 0);

        let from = GlobalId {
            did: *kp_other.public_key(),
            uid: "World".try_into().unwrap(),
        };

        let push = Push::NewUMessage {
            from,
            msg: bytes::Bytes::new(),
            timestamp: Utc::now(),
        };

        assert!(conn.add_pending(vec![*kp.public_key()], push).is_ok());

        let pending = conn.get_pending(*kp.public_key()).unwrap();
        assert_eq!(pending.len(), 1);

        assert!(conn.expire_pending(*kp.public_key()).is_ok());

        let pending = conn.get_pending(*kp.public_key()).unwrap();
        assert!(pending.is_empty());
    }

    #[test]
    #[serial]
    fn add_and_get_prekey() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let signed_pk = kp.sign(*kp.public_key());
        let user_id = "Hello".try_into().unwrap();
        conn.register_user(user_id, signed_pk).unwrap();

        let sealed_kp = sealed::KeyPair::gen_new();

        let sealed_pk = sealed_kp.sign_pub(&kp);

        conn.add_prekey(*kp.public_key(), sealed_pk).unwrap();
        let retrieved = conn.get_prekey(*kp.public_key()).unwrap();
        assert_eq!(retrieved, sealed_pk);
    }

    #[test]
    #[serial]
    fn key_is_valid() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.key_is_valid(*kp.public_key()).unwrap());

        conn.register_user(user_id, signed_pk).unwrap();

        assert!(conn.key_is_valid(*kp.public_key()).unwrap());

        let signed_pk = kp.sign(*kp.public_key());

        conn.deprecate_key(signed_pk).unwrap();

        assert!(!conn.key_is_valid(*kp.public_key()).unwrap());
    }

    #[test]
    #[serial]
    fn double_deprecation() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());

        conn.register_user(user_id, signed_pk).unwrap();

        conn.deprecate_key(signed_pk).unwrap();

        match conn.deprecate_key(signed_pk) {
            Ok(PKIResponse::Redundant) => {}
            _ => panic!(),
        }
    }
}
