use crate::prelude::*;
use dotenv::dotenv;
use futures::FutureExt;
use herald_common::*;
use std::{
    env,
    ops::{Deref, DerefMut},
};
use tokio_postgres::{types::Type, Client, Error as PgError, NoTls};

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
    let (client, connection) = tokio_postgres::connect(&database_url(), NoTls).await?;

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

    pub async fn add_prekeys(
        &mut self,
        pres: &[sealed::PublicKey],
    ) -> Result<Vec<PKIResponse>, Error> {
        let client = get_client().await?;

        let stmt = client
            .prepare_typed(
                include_str!("sql/add_prekey.sql"),
                &[Type::BYTEA, Type::BYTEA],
            )
            .await?;

        let mut out = Vec::with_capacity(pres.len());
        for pre in pres {
            let res = client
                .execute(
                    &stmt,
                    &[&pre.signed_by().as_ref(), &serde_cbor::to_vec(&pre)?],
                )
                .await;

            out.push(unique_violation_to_redundant(res)?);
        }

        Ok(out)
    }

    pub async fn pop_prekeys(
        &mut self,
        keys: &[sig::PublicKey],
    ) -> Result<Vec<Option<sealed::PublicKey>>, Error> {
        let client = get_client().await?;

        let stmt = client
            .prepare_typed(include_str!("sql/pop_prekey.sql"), &[Type::BYTEA])
            .await?;

        let mut prekeys = Vec::with_capacity(keys.len());

        for k in keys {
            let val: Option<Vec<u8>> = client.query_one(&stmt, &[&k.as_ref()]).await?.get(0);
            let prekey = match val {
                Some(val) => serde_cbor::from_slice(&val)?,
                None => None,
            };

            prekeys.push(prekey);
        }

        Ok(prekeys)
    }

    pub async fn register_user(
        &mut self,
        user_id: UserId,
        key: Signed<sig::PublicKey>,
    ) -> Result<register::Res, Error> {
        let mut client = get_client().await?;
        let tx = client.transaction().await?;

        let exists_stmt = tx
            .prepare_typed(include_str!("sql/user_exists.sql"), &[Type::TEXT])
            .await?;

        if tx
            .query_one(&exists_stmt, &[&user_id.as_str()])
            .await?
            .get(0)
        {
            return Ok(register::Res::UIDTaken);
        }

        let add_key_stmt = tx
            .prepare_typed(
                include_str!("sql/add_key.sql"),
                &[Type::BYTEA, Type::BYTEA, Type::INT8, Type::BYTEA],
            )
            .await?;

        tx.execute(
            &add_key_stmt,
            &[
                &key.data().as_ref(),
                &key.signed_by().as_ref(),
                &key.timestamp().0,
                &key.sig().as_ref(),
            ],
        )
        .await?;

        let add_user_key_stmt = tx
            .prepare_typed(
                include_str!("sql/add_user_key.sql"),
                &[Type::TEXT, Type::BYTEA],
            )
            .await?;

        tx.execute(
            &add_user_key_stmt,
            &[&user_id.as_str(), &key.data().as_ref()],
        )
        .await?;
        tx.commit().await?;

        return Ok(register::Res::Success);
    }

    pub async fn add_key(&mut self, key: Signed<sig::PublicKey>) -> Result<PKIResponse, Error> {
        let mut client = get_client().await?;
        let tx = client.transaction().await?;

        let user_id_stmt = tx
            .prepare_typed(include_str!("sql/get_user_id_by_key.sql"), &[Type::BYTEA])
            .await?;

        let user_id = match tx
            .query_one(&user_id_stmt, &[&key.signed_by().as_ref()])
            .await?
            .get::<_, Option<String>>(0)
        {
            Some(uid) => uid,
            None => {
                return Ok(PKIResponse::DeadKey);
            }
        };

        let add_key_stmt = tx
            .prepare_typed(
                include_str!("sql/add_key.sql"),
                &[Type::BYTEA, Type::BYTEA, Type::INT8, Type::BYTEA],
            )
            .await?;

        let res = tx
            .execute(
                &add_key_stmt,
                &[
                    &key.data().as_ref(),
                    &key.signed_by().as_ref(),
                    &key.timestamp().0,
                    &key.sig().as_ref(),
                ],
            )
            .await;

        unique_violation_to_redundant(res)?;

        let add_user_key_stmt = tx
            .prepare_typed(
                include_str!("sql/add_user_key.sql"),
                &[Type::TEXT, Type::BYTEA],
            )
            .await?;

        let res = tx
            .execute(
                &add_user_key_stmt,
                &[&user_id.as_str(), &key.data().as_ref()],
            )
            .await;

        tx.commit().await?;
        unique_violation_to_redundant(res)
    }

    pub async fn read_key(&mut self, key: sig::PublicKey) -> Result<sig::PKMeta, Error> {
        let client = get_client().await?;
        let stmt = client
            .prepare_typed(include_str!("sql/get_pk_meta.sql"), &[Type::BYTEA])
            .await?;

        let (signed_by, sig, ts, dep_signed_by, dep_signature, dep_ts): RawPkMeta = {
            let row = client.query_one(&stmt, &[&key.as_ref()]).await?;

            (
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4),
                row.get(5),
            )
        };

        let sig = sig::Signature::from_slice(sig.as_slice()).ok_or(InvalidSig)?;
        let signed_by = sig::PublicKey::from_slice(signed_by.as_slice()).ok_or(InvalidKey)?;
        let sig_meta = SigMeta::new(sig, signed_by, ts.into());

        let dep_is_some = dep_ts.is_some() || dep_signed_by.is_some() || dep_signature.is_some();

        let dep_sig_meta = if dep_is_some {
            let dep_ts = dep_ts.ok_or(MissingData)?.into();

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

    pub async fn deprecate_key(
        &mut self,
        signed_key: Signed<sig::PublicKey>,
    ) -> Result<PKIResponse, Error> {
        let (data, meta) = signed_key.split();
        let signer_key = meta.signed_by();

        let mut client = get_client().await?;
        let tx = client.transaction().await?;

        let signer_key_exists_stmt = tx
            .prepare_typed(include_str!("sql/key_is_valid.sql"), &[Type::BYTEA])
            .await?;

        let signer_key_exists: bool = tx
            .query_one(&signer_key_exists_stmt, &[&signer_key.as_ref()])
            .await?
            .get(0);

        if !signer_key_exists {
            return Ok(PKIResponse::DeadKey);
        }

        let dep_stmt = tx
            .prepare_typed(
                include_str!("sql/deprecate_key.sql"),
                &[Type::INT8, Type::BYTEA, Type::BYTEA, Type::BYTEA],
            )
            .await?;

        let num_updated = tx
            .execute(
                &dep_stmt,
                &[
                    &meta.timestamp().0,
                    &signer_key.as_ref(),
                    &meta.sig().as_ref(),
                    &data.as_ref(),
                ],
            )
            .await?;

        if num_updated != 1 {
            return Ok(PKIResponse::Redundant);
        }

        tx.commit().await?;
        Ok(PKIResponse::Success)
    }

    pub async fn user_exists(&mut self, uid: &UserId) -> Result<bool, Error> {
        let client = get_client().await?;
        let stmt = client
            .prepare_typed(include_str!("sql/user_exists.sql"), &[Type::TEXT])
            .await?;

        let row = client.query_one(&stmt, &[&uid.as_str()]).await?;

        Ok(row.get(0))
    }

    pub async fn key_is_valid(&mut self, key: sig::PublicKey) -> Result<bool, Error> {
        let client = get_client().await?;
        let stmt = client
            .prepare_typed(include_str!("sql/key_is_valid.sql"), &[Type::BYTEA])
            .await?;

        let row = client.query_one(&stmt, &[&key.as_ref()]).await?;

        Ok(row.get(0))
    }

    pub async fn get_pending(
        &mut self,
        key: sig::PublicKey,
        limit: u32,
    ) -> Result<Vec<Push>, Error> {
        let text = format!(include_str!("sql/get_pending.sql"), limit = limit);
        let client = get_client().await?;
        let stmt = client.prepare_typed(&text, &[Type::BYTEA]).await?;

        let rows = client.query(&stmt, &[&key.as_ref()]).await?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let p: Vec<u8> = row.get(0);
            out.push(serde_cbor::from_slice(&p)?);
        }

        Ok(out)
    }

    pub async fn expire_pending(&mut self, key: sig::PublicKey, limit: u32) -> Result<(), Error> {
        let text = format!(include_str!("sql/expire_pending.sql"), limit = limit);
        let client = get_client().await?;

        let stmt = client
            .prepare_typed(&text, &[Type::BYTEA])
            .await?;
        client.execute(&stmt, &[&key.as_ref()]).await?;

        Ok(())
    }

    pub async fn valid_keys(&mut self, uid: &UserId) -> Result<Vec<sig::PublicKey>, Error> {
        let client = get_client().await?;
        let stmt = client
            .prepare_typed(
                include_str!("sql/get_valid_keys_by_user_id.sql"),
                &[Type::TEXT],
            )
            .await?;

        client
            .query(&stmt, &[&uid.as_str()])
            .await?
            .into_iter()
            .map(|row| {
                let raw = row.get(0);
                sig::PublicKey::from_slice(raw).ok_or(Error::InvalidKey)
            })
            .collect()
    }

    pub async fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error> {
        let client = get_client().await?;

        let stmt = client
            .prepare_typed(include_str!("sql/read_meta.sql"), &[Type::TEXT])
            .await?;

        let keys: Vec<RawKeyAndMeta> = client
            .query(&stmt, &[&uid.as_str()])
            .await?
            .into_iter()
            .map(|row| {
                (
                    row.get(0),
                    row.get(1),
                    row.get(2),
                    row.get(3),
                    row.get(4),
                    row.get(5),
                    row.get(6),
                )
            })
            .collect();

        let meta_inner: Result<BTreeMap<sig::PublicKey, sig::PKMeta>, Error> = keys
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
                    let timestamp = creation_ts.into();
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

                        let dep_ts = deprecation_ts.ok_or(MissingData)?.into();

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

    pub async fn add_pending<'a, M: Iterator<Item = &'a Push>>(
        &mut self,
        keys: Vec<sig::PublicKey>,
        msgs: M,
    ) -> Result<(), Error> {
        let mut client = get_client().await?;
        let tx = client.transaction().await?;

        let push_stmt = tx
            .prepare_typed(include_str!("sql/add_push.sql"), &[Type::BYTEA, Type::INT8])
            .await?;

        let pending_stmt = tx
            .prepare_typed(
                include_str!("sql/add_pending.sql"),
                &[Type::BYTEA, Type::INT8],
            )
            .await?;

        for msg in msgs {
            let push_row_id: i64 = {
                let push_timestamp = msg.timestamp;
                let push_vec = serde_cbor::to_vec(msg)?;

                tx.query_one(&push_stmt, &[&push_vec, &push_timestamp.0])
                    .await?
                    .get(0)
            };

            for k in keys.iter() {
                tx.execute(&pending_stmt, &[&k.as_ref(), &push_row_id])
                    .await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
