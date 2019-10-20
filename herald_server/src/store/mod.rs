use crate::prelude::*;
use dotenv::dotenv;
use futures::FutureExt;
use herald_common::*;
use lazy_pond::LazyPond;
use std::{
    env,
    ops::{Deref, DerefMut},
};
use tokio_postgres::{types::Type, Client, Connection, Error as PgError, NoTls, Row};

//pub type Pool = LazyPond;
//
//pub fn init_pool() -> Pool {
//}

fn database_url() -> String {
    dotenv().expect("Invalid dotenv");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct Conn(pub Client);
//
//impl Default for Conn {
//    fn default() ->  Self {
//        let (mut client, connection) =
//            tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
//
//        // The connection object performs the actual communication with the database,
//        // so spawn it off to run on its own.
//        let connection = connection.map(|r| {
//            if let Err(e) = r {
//                eprintln!("connection error: {}", e);
//            }
//        });
//        tokio::spawn(connection);
//    }
//}
//
impl Deref for Conn {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn is_unique_violation(query_res: &Result<u64, PgError>) -> bool {
    use tokio_postgres::error::SqlState;
    if let Err(e) = query_res {
        if let Some(code) = e.code() {
            code == &SqlState::UNIQUE_VIOLATION
        } else {
            false
        }
    } else {
        false
    }
}

fn unique_violation_to_redundant(query_res: Result<u64, PgError>) -> Result<PKIResponse, Error> {
    if is_unique_violation(&query_res) {
        return Ok(PKIResponse::Redundant);
    }

    Ok(query_res.map(|_| PKIResponse::Success)?)
}

async fn get_client() -> Result<Conn, PgError> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    let connection = connection.map(|r| {
        if let Err(e) = r {
            eprintln!("connection error: {}", e);
        }
    });
    tokio::spawn(connection);

    Ok(Conn(client))
}

type RawPkMeta = (
    Vec<u8>,
    Vec<u8>,
    i64,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
    Option<i64>,
);

type RawKeyAndMeta = (
    Vec<u8>,
    Vec<u8>,
    i64,
    Vec<u8>,
    Option<i64>,
    Option<Vec<u8>>,
    Option<Vec<u8>>,
);

impl Conn {
    // TODO read about postgres performance
    pub async fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error> {
        let client = get_client().await?;
        let stmt = client
            .prepare_typed(include_str!("sql/device_exists.sql"), &[Type::BYTEA])
            .await?;

        let row = client.query_one(&stmt, &[&pk.as_ref()]).await?;

        Ok(row.get(0))
    }

        pub async fn add_prekeys(&mut self, pres: &[sealed::PublicKey]) -> Result<Vec<PKIResponse>, Error> {
            let mut client = get_client().await?;
            let tx = client.transaction().await?;
    
            let stmt = tx.prepare_typed(include_str!("sql/add_prekey.sql"), &[BYTEA, BYTEA]).await?;
    
            pres.iter().map(|pre| {
                        stmt.execute( 
                        let res = diesel::insert_into(prekeys)
                            .values((
                                signing_key.eq(pre.signed_by().as_ref()),
                                sealed_key.eq(serde_cbor::to_vec(&pre)?),
                            ))
                            .execute(&self.0);
    
                        unique_violation_to_redundant(res)
                    })
                    .collect()
            })
        

    //pub async fn pop_prekeys(
    //    &mut self,
    //    keys: &[sig::PublicKey],
    //) -> Result<Vec<Option<sealed::PublicKey>>, Error> {
    //    let builder = self.build_transaction().deferrable();

    //    builder.run(|| {
    //        keys.iter()
    //            .map(|k| {
    //                // TODO I think this can be simplified using a RETURNING clause but diesel
    //                // doesn't support this syntax ¯\_(ツ)_/¯, fix it when we switch to tokio-postgres
    //                let raw_pk: Option<Vec<u8>> = prekeys::table
    //                    .filter(prekeys::signing_key.eq(k.as_ref()))
    //                    .select(prekeys::sealed_key)
    //                    .limit(1)
    //                    .get_result(&self.0)
    //                    .optional()?;

    //                match raw_pk {
    //                    Some(raw) => {
    //                        diesel::delete(
    //                            prekeys::table
    //                                .filter(prekeys::signing_key.eq(k.as_ref()))
    //                                .filter(prekeys::sealed_key.eq(&raw)),
    //                        )
    //                        .execute(&self.0)?;

    //                        Ok(Some(serde_cbor::from_slice(raw.as_slice())?))
    //                    }
    //                    None => Ok(None),
    //                }
    //            })
    //            .collect()
    //    })
    //}

    //pub async fn register_user(
    //    &mut self,
    //    user_id: UserId,
    //    key: Signed<sig::PublicKey>,
    //) -> Result<register::Res, Error> {
    //    let builder = self.build_transaction().deferrable();

    //    let query = userkeys::table.filter(userkeys::user_id.eq(user_id.as_str()));

    //    builder.run(|| {
    //        if select(exists(query)).get_result(&self.0)? {
    //            return Ok(register::Res::UIDTaken);
    //        }

    //        diesel::insert_into(keys::table)
    //            .values((
    //                keys::key.eq(key.data().as_ref()),
    //                keys::signed_by.eq(key.signed_by().as_ref()),
    //                keys::ts.eq(key.timestamp().0),
    //                keys::signature.eq(key.sig().as_ref()),
    //            ))
    //            .execute(&self.0)?;

    //        diesel::insert_into(userkeys::table)
    //            .values((
    //                userkeys::user_id.eq(user_id.as_str()),
    //                userkeys::key.eq(key.data().as_ref()),
    //            ))
    //            .execute(&self.0)?;

    //        return Ok(register::Res::Success);
    //    })
    //}

    //pub async fn add_key(&mut self, new_key: Signed<sig::PublicKey>) -> Result<PKIResponse, Error> {
    //    let builder = self.build_transaction().deferrable();

    //    builder.run(|| {
    //        let user_id: String = match keys::table
    //            .filter(keys::key.eq(new_key.signed_by().as_ref()))
    //            .filter(keys::dep_signature.is_null())
    //            .filter(keys::dep_signed_by.is_null())
    //            .filter(keys::dep_ts.is_null())
    //            .inner_join(userkeys::table)
    //            .select(userkeys::user_id)
    //            .get_result(&self.0)
    //            .optional()?
    //        {
    //            None => {
    //                // TODO test this branch
    //                return Ok(PKIResponse::DeadKey);
    //            }
    //            Some(uid) => uid,
    //        };

    //        let res = diesel::insert_into(keys::table)
    //            .values((
    //                keys::key.eq(new_key.data().as_ref()),
    //                keys::signed_by.eq(new_key.signed_by().as_ref()),
    //                keys::ts.eq(new_key.timestamp().0),
    //                keys::signature.eq(new_key.sig().as_ref()),
    //            ))
    //            .execute(&self.0);

    //        unique_violation_to_redundant(res)?;

    //        let res = diesel::insert_into(userkeys::table)
    //            .values((
    //                userkeys::user_id.eq(user_id.as_str()),
    //                userkeys::key.eq(new_key.data().as_ref()),
    //            ))
    //            .execute(&self.0);

    //        unique_violation_to_redundant(res)
    //    })
    //}

    //pub async fn read_key(&mut self, key_arg: sig::PublicKey) -> Result<sig::PKMeta, Error> {
    //    let (signed_by, sig, ts, dep_signed_by, dep_signature, dep_ts): RawPkMeta = keys::table
    //        .filter(keys::key.eq(key_arg.as_ref()))
    //        .select((
    //            keys::signed_by,
    //            keys::signature,
    //            keys::ts,
    //            keys::dep_signed_by,
    //            keys::dep_signature,
    //            keys::dep_ts,
    //        ))
    //        .get_result(self.deref_mut())?;

    //    let sig = sig::Signature::from_slice(sig.as_slice()).ok_or(InvalidSig)?;
    //    let signed_by = sig::PublicKey::from_slice(signed_by.as_slice()).ok_or(InvalidKey)?;
    //    let sig_meta = SigMeta::new(sig, signed_by, ts.into());

    //    let dep_is_some = (&dep_signature, &dep_ts, &dep_signed_by) != (&None, &None, &None);

    //    let dep_sig_meta = if dep_is_some {
    //        let dep_ts = dep_ts.ok_or(MissingData)?.into();

    //        let dep_signed_by =
    //            sig::PublicKey::from_slice(&dep_signed_by.ok_or(MissingData)?).ok_or(InvalidKey)?;

    //        let dep_signature =
    //            sig::Signature::from_slice(&dep_signature.ok_or(MissingData)?).ok_or(InvalidSig)?;

    //        Some(SigMeta::new(dep_signature, dep_signed_by, dep_ts))
    //    } else {
    //        None
    //    };

    //    Ok(sig::PKMeta::new(sig_meta, dep_sig_meta))
    //}

    //pub async fn deprecate_key(
    //    &mut self,
    //    signed_key: Signed<sig::PublicKey>,
    //) -> Result<PKIResponse, Error> {
    //    use crate::schema::keys::dsl::*;

    //    let (data, meta) = signed_key.split();

    //    let to_dep = keys
    //        .filter(key.eq(data.as_ref()))
    //        .filter(dep_ts.is_null())
    //        .filter(dep_signature.is_null())
    //        .filter(dep_signed_by.is_null());

    //    let signer_key = meta.signed_by();
    //    let signed_by_filter = keys
    //        .filter(key.eq(signer_key.as_ref()))
    //        .filter(dep_ts.is_null())
    //        .filter(dep_signature.is_null())
    //        .filter(dep_signed_by.is_null());

    //    let builder = self.build_transaction().deferrable();

    //    builder.run(|| {
    //        if !select(exists(signed_by_filter)).get_result(&self.0)? {
    //            return Ok(PKIResponse::DeadKey);
    //        }

    //        let num_updated = update(to_dep)
    //            .set((
    //                dep_ts.eq(meta.timestamp().0),
    //                dep_signed_by.eq(meta.signed_by().as_ref()),
    //                dep_signature.eq(meta.sig().as_ref()),
    //            ))
    //            .execute(&self.0)?;

    //        if num_updated != 1 {
    //            return Ok(PKIResponse::Redundant);
    //        }

    //        Ok(PKIResponse::Success)
    //    })
    //}

    //pub async fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error> {
    //    use crate::schema::userkeys::dsl::*;

    //    let query = userkeys.filter(user_id.eq(uid.as_str()));

    //    Ok(select(exists(query)).get_result(self.deref_mut())?)
    //}

    //pub async fn key_is_valid(&mut self, key_arg: sig::PublicKey) -> Result<bool, Error> {
    //    use crate::schema::keys::dsl::*;

    //    let query = keys
    //        .filter(key.eq(key_arg.as_ref()))
    //        .filter(dep_ts.is_null())
    //        .filter(dep_signed_by.is_null())
    //        .filter(dep_signature.is_null());

    //    Ok(select(exists(query)).get_result(self.deref_mut())?)
    //}

    //pub async fn valid_keys(&mut self, uid: &UserId) -> Result<Vec<sig::PublicKey>, Error> {
    //    let keys: Vec<Vec<u8>> = userkeys::table
    //        .filter(userkeys::user_id.eq(uid.as_str()))
    //        .inner_join(keys::table)
    //        .filter(keys::dep_ts.is_null())
    //        .filter(keys::dep_signed_by.is_null())
    //        .filter(keys::dep_signature.is_null())
    //        .select(keys::key)
    //        .get_results(self.deref_mut())?;

    //    keys.iter()
    //        .map(|raw| sig::PublicKey::from_slice(raw).ok_or(Error::InvalidKey))
    //        .collect()
    //}

    //pub async fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error> {
    //    let keys: Vec<RawKeyAndMeta> = userkeys::table
    //        .filter(userkeys::user_id.eq(uid.as_str()))
    //        .inner_join(keys::table)
    //        .select((
    //            keys::key,
    //            keys::signed_by,
    //            keys::ts,
    //            keys::signature,
    //            keys::dep_ts,
    //            keys::dep_signed_by,
    //            keys::dep_signature,
    //        ))
    //        .get_results(self.deref_mut())?;

    //    let meta_inner: Result<BTreeMap<sig::PublicKey, sig::PKMeta>, Error> = keys
    //        .into_iter()
    //        .map(
    //            |(
    //                key,
    //                signed_by,
    //                creation_ts,
    //                signature,
    //                deprecation_ts,
    //                dep_signed_by,
    //                dep_signature,
    //            )| {
    //                let key = sig::PublicKey::from_slice(&key).ok_or(InvalidKey)?;
    //                let signed_by = sig::PublicKey::from_slice(&signed_by).ok_or(InvalidKey)?;
    //                let timestamp = creation_ts.into();
    //                let signature = sig::Signature::from_slice(&signature).ok_or(InvalidSig)?;

    //                let dep_is_some = deprecation_ts.is_some()
    //                    || dep_signed_by.is_some()
    //                    || dep_signature.is_some();

    //                let dep_sig_meta = if dep_is_some {
    //                    let dep_sig =
    //                        sig::Signature::from_slice(&dep_signature.ok_or(MissingData)?)
    //                            .ok_or(InvalidSig)?;

    //                    let dep_signed_by =
    //                        sig::PublicKey::from_slice(&dep_signed_by.ok_or(MissingData)?)
    //                            .ok_or(InvalidKey)?;

    //                    let dep_ts = deprecation_ts.ok_or(MissingData)?.into();

    //                    Some(SigMeta::new(dep_sig, dep_signed_by, dep_ts))
    //                } else {
    //                    None
    //                };

    //                let sig_meta = SigMeta::new(signature, signed_by, timestamp);
    //                let pkmeta = sig::PKMeta::new(sig_meta, dep_sig_meta);
    //                Ok((key, pkmeta))
    //            },
    //        )
    //        .collect();

    //    Ok(UserMeta { keys: meta_inner? })
    //}

    //pub async fn add_pending<'a, M: Iterator<Item = &'a Push>>(
    //    &mut self,
    //    key_arg: Vec<sig::PublicKey>,
    //    msgs: M,
    //) -> Result<(), Error> {
    //    let builder = self.build_transaction().deferrable();

    //    let key_arg: Vec<_> = key_arg
    //        .into_iter()
    //        .map(|k| k.as_ref().to_vec()) // borrow checker appeasement
    //        .map(|k| pending::key.eq(k))
    //        .collect();

    //    builder.run(|| {
    //        for msg in msgs {
    //            let push_row_id: i64 = {
    //                use crate::schema::pushes::dsl::*;

    //                let push_timestamp = msg.timestamp;
    //                let push_vec = serde_cbor::to_vec(msg)?;
    //                insert_into(pushes)
    //                    .values((push_data.eq(push_vec), push_ts.eq(push_timestamp.0)))
    //                    .returning(push_id)
    //                    .get_result(&self.0)?
    //            };

    //            use crate::schema::pending::dsl::*;

    //            let keys: Vec<_> = key_arg
    //                .iter()
    //                .map(|k| (k, push_id.eq(push_row_id)))
    //                .collect();

    //            insert_into(pending).values(keys).execute(&self.0)?;
    //        }
    //        Ok(())
    //    })
    //}

    //pub async fn get_pending(&mut self, key: sig::PublicKey, limit: u32) -> Result<Vec<Push>, Error> {
    //    let pushes: Vec<Vec<u8>> = pending::table
    //        .inner_join(pushes::table)
    //        .filter(pending::key.eq(key.as_ref()))
    //        .select(pushes::push_data)
    //        .order((pushes::push_ts.asc(), pushes::push_id.asc()))
    //        .limit(limit as i64)
    //        .get_results(self.deref_mut())?;

    //    let mut out = Vec::with_capacity(pushes.len());

    //    for p in pushes.into_iter() {
    //        out.push(serde_cbor::from_slice(&p)?);
    //    }

    //    Ok(out)
    //}

    //pub async fn expire_pending(&mut self, key: sig::PublicKey, limit: u32) -> Result<(), Error> {
    //    let push_ids = pending::table
    //        .inner_join(pushes::table)
    //        .filter(pending::key.eq(key.as_ref()))
    //        .select(pushes::push_id)
    //        .order((pushes::push_ts.asc(), pushes::push_id.asc()))
    //        .limit(limit as i64);

    //    delete(pushes::table.filter(pushes::push_id.eq_any(push_ids))).execute(self.deref_mut())?;

    //    Ok(())
    //}
}

#[cfg(test)]
mod tests;
