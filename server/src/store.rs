use diesel::{pg::PgConnection, prelude::*};

use herald_common::*;
// use redis::Commands;

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
    fn add_prekey(&mut self, key: sig::PublicKey, pre: sealed::PublicKey) -> Result<(), Error>;
    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error>;

    fn add_key(&mut self, user_id: UserId, key: Signed<sig::PublicKey>) -> Result<(), Error>;
    fn read_key(&mut self, key: sig::PublicKey) -> Result<sig::PKMeta, Error>;
    fn deprecate_key(&mut self, key: Signed<sig::PublicKey>) -> Result<(), Error>;

    fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error>;
    fn key_is_valid(&mut self, key: sig::PublicKey) -> Result<bool, Error>;
    fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error>;

    fn add_pending(&mut self, key: Vec<sig::PublicKey>, msg: Push) -> Result<(), Error>;
    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Error>;
    fn expire_pending(&mut self, key: sig::PublicKey) -> Result<(), Error>;
}

mod pgstore {
    use super::*;
    use crate::schema::*;
    use diesel::dsl::*;

    impl Store for PgConnection {
        // TODO implement the appropriate traits for this
        // TODO read about postgres performance
        fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error> {
            use crate::schema::userkeys::dsl::*;

            Ok(select(exists(userkeys.filter(key.eq(pk.as_ref())))).get_result(self)?)
        }

        fn add_prekey(&mut self, key: sig::PublicKey, pre: sealed::PublicKey) -> Result<(), Error> {
            use crate::schema::prekeys::dsl::*;

            diesel::insert_into(prekeys)
                .values((
                    signing_key.eq(key.as_ref()),
                    sealed_key.eq(serde_cbor::to_vec(&pre)?),
                ))
                .execute(self)?;
            Ok(())
        }

        fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error> {
            use crate::schema::prekeys::dsl::*;

            let raw_pk: Vec<u8> = prekeys
                .filter(signing_key.eq(key.as_ref()))
                .select(sealed_key)
                .limit(1)
                .get_result(self)?;

            Ok(serde_cbor::from_slice(raw_pk.as_slice())?)
        }

        fn add_key(
            &mut self,
            user_id_arg: UserId,
            new_key: Signed<sig::PublicKey>,
        ) -> Result<(), Error> {
            diesel::insert_into(keys::table)
                .values((
                    keys::key.eq(new_key.data().as_ref()),
                    keys::signed_by.eq(new_key.signed_by().as_ref()),
                    keys::ts.eq(new_key.timestamp()),
                    keys::signature.eq(new_key.sig().as_ref()),
                ))
                .execute(self)?;

            diesel::insert_into(userkeys::table)
                .values((
                    userkeys::user_id.eq(user_id_arg.as_str()),
                    userkeys::key.eq(new_key.data().as_ref()),
                ))
                .execute(self)?;

            Ok(())
        }

        fn read_key(&mut self, key_arg: sig::PublicKey) -> Result<sig::PKMeta, Error> {
            let (signed_by, creation_ts, sig, dep_ts, dep_signed_by, dep_signature): (
                Vec<u8>,
                DateTime<Utc>,
                Vec<u8>,
                Option<DateTime<Utc>>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
            ) = keys::table
                .filter(keys::key.eq(key_arg.as_ref()))
                .select((
                    keys::signed_by,
                    keys::ts,
                    keys::signature,
                    keys::dep_ts,
                    keys::dep_signed_by,
                    keys::dep_signature,
                ))
                .get_result(self)?;

            let sig_meta = SigMeta::new(
                serde_cbor::from_slice(&sig)?,
                serde_cbor::from_slice(&signed_by)?,
                creation_ts,
            );

            let dep_sig_meta =
                if dep_signature.is_some() || dep_ts.is_some() || dep_signed_by.is_some() {
                    let dep_ts = dep_ts.ok_or(MissingData)?;
                    let dep_signed_by = serde_cbor::from_slice(&dep_signed_by.ok_or(MissingData)?)?;
                    let dep_signature = serde_cbor::from_slice(&dep_signature.ok_or(MissingData)?)?;
                    Some(SigMeta::new(dep_signature, dep_signed_by, dep_ts))
                } else {
                    None
                };

            Ok(sig::PKMeta::new(sig_meta, dep_sig_meta))
        }

        fn deprecate_key(&mut self, signed_key: Signed<sig::PublicKey>) -> Result<(), Error> {
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
                .execute(self)?;

            if num_updated != 1 {
                return Err(Error::RedundantDeprecation);
            }

            Ok(())
        }

        fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error> {
            use crate::schema::userkeys::dsl::*;

            let query = userkeys.filter(user_id.eq(uid.as_str()));

            Ok(select(exists(query)).get_result(self)?)
        }

        fn key_is_valid(&mut self, key_arg: sig::PublicKey) -> Result<bool, Error> {
            use crate::schema::userkeys::dsl::*;

            let query = userkeys
                .filter(key.eq(key_arg.as_ref()))
                .inner_join(keys::table)
                .filter(keys::dep_ts.is_null())
                .filter(keys::dep_signed_by.is_null())
                .filter(keys::dep_signature.is_null());

            Ok(select(exists(query)).get_result(self)?)
        }

        fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error> {
            let keys: Vec<(
                Vec<u8>,
                Vec<u8>,
                DateTime<Utc>,
                Vec<u8>,
                Option<DateTime<Utc>>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
            )> = userkeys::table
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
                .get_results(self)?;

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
                        let key: sig::PublicKey = serde_cbor::from_slice(&key)?;
                        let signed_by = serde_cbor::from_slice(&signed_by)?;
                        let timestamp = creation_ts;
                        let signature = serde_cbor::from_slice(&signature)?;

                        let dep_sig_meta = if deprecation_ts.is_some()
                            || dep_signed_by.is_some()
                            || dep_signature.is_some()
                        {
                            Some(SigMeta::new(
                                serde_cbor::from_slice(&dep_signature.ok_or(MissingData)?)?,
                                serde_cbor::from_slice(&dep_signed_by.ok_or(MissingData)?)?,
                                deprecation_ts.ok_or(MissingData)?,
                            ))
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
                    .get_result(self)?
            };

            use crate::schema::pending::dsl::*;

            let keys: Vec<_> = key_arg
                .into_iter()
                .map(|k| (key.eq(k.as_ref().to_vec()), push_id.eq(push_row_id)))
                .collect();

            insert_into(pending).values(keys).execute(self)?;

            Ok(())
        }

        fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Error> {
            let pushes: Vec<Vec<u8>> = pending::table
                .inner_join(pushes::table)
                .filter(pending::key.eq(key.as_ref()))
                .select(pushes::push_data)
                .get_results(self)?;

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

            delete(pushes::table.filter(pushes::push_id.eq_any(push_ids))).execute(self)?;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::pg::PgConnection;
    use dotenv::dotenv;
    use serial_test_derive::serial;
    use std::{convert::TryInto, env};

    fn open_conn() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        // so none of the changes are committed
        conn.begin_test_transaction()
            .expect("Couldn't start test transaction");
        conn
    }

    #[test]
    #[serial]
    fn device_exists() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.device_exists(kp.public_key()).unwrap());

        conn.add_key(user_id, signed_pk).unwrap();

        assert!(conn.device_exists(kp.public_key()).unwrap());
    }

    #[test]
    #[serial]
    fn user_exists() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.user_exists(&user_id).unwrap());

        conn.add_key(user_id, signed_pk).unwrap();

        assert!(conn.user_exists(&user_id).unwrap());
    }

    #[test]
    #[serial]
    fn key_is_valid() {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        assert!(!conn.key_is_valid(*kp.public_key()).unwrap());

        conn.add_key(user_id, signed_pk).unwrap();

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

        conn.add_key(user_id, signed_pk).unwrap();

        conn.deprecate_key(signed_pk).unwrap();
        assert!(conn.deprecate_key(signed_pk).is_err());
    }
}
