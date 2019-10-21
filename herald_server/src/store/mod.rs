use crate::prelude::*;
use dotenv::dotenv;
use futures::FutureExt;
use herald_common::*;
use std::{
    env,
    ops::{Deref, DerefMut},
};
use tokio_postgres::{types::Type, Client, Error as PgError, NoTls};

macro_rules! sql {
    ($path: literal) => {
        include_str!(concat!("sql/", $path, ".sql"))
    };
}

macro_rules! types {
    ($($typ: ident,)+) => (types!($($typ),+));

    ( $($typ:ident),* ) => {
        &[$(Type::$typ, )*]
    }
}

macro_rules! params {
    ($($val:expr,)+) => (params!($($val),+));

    ( $($val:expr),* ) => {
        &[$(&$val, )*]
    }
}

fn database_url() -> String {
    dotenv().expect("Invalid dotenv");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct Conn(pub Client);

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

fn dep_check(
    ts: Option<i64>,
    signed_by: Option<&[u8]>,
    signature: Option<&[u8]>,
) -> Result<Option<SigMeta>, Error> {
    use sig::*;
    if dep_is_some(ts, signed_by, signature) {
        let ts = ts.ok_or(MissingData)?.into();

        let signed_by = PublicKey::from_slice(signed_by.ok_or(MissingData)?).ok_or(InvalidKey)?;

        let signature = Signature::from_slice(signature.ok_or(MissingData)?).ok_or(InvalidSig)?;

        Ok(Some(SigMeta::new(signature, signed_by, ts)))
    } else {
        Ok(None)
    }
}

fn dep_is_some(ts: Option<i64>, signed_by: Option<&[u8]>, sig: Option<&[u8]>) -> bool {
    ts.is_some() || signed_by.is_some() || sig.is_some()
}

pub async fn get_client() -> Result<Conn, PgError> {
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

type RawPkMeta<'a> = (
    &'a [u8],
    &'a [u8],
    i64,
    Option<&'a [u8]>,
    Option<&'a [u8]>,
    Option<i64>,
);

impl Conn {
    pub async fn device_exists(&mut self, pk: &sign::PublicKey) -> Result<bool, Error> {
        let stmt = self
            .prepare_typed(sql!("device_exists"), types![BYTEA])
            .await?;

        let row = self.query_one(&stmt, params![pk.as_ref()]).await?;

        Ok(row.get(0))
    }

    pub async fn add_prekeys(
        &mut self,
        pres: &[sealed::PublicKey],
    ) -> Result<Vec<PKIResponse>, Error> {
        let stmt = self
            .prepare_typed(sql!("add_prekey"), types![BYTEA, BYTEA])
            .await?;

        let mut out = Vec::with_capacity(pres.len());

        for pre in pres {
            let res = self
                .execute(
                    &stmt,
                    params![pre.signed_by().as_ref(), serde_cbor::to_vec(&pre)?],
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
        let stmt = self
            .prepare_typed(sql!("pop_prekey"), types![BYTEA])
            .await?;

        let mut prekeys = Vec::with_capacity(keys.len());

        for k in keys {
            let prekey = match self
                .query(&stmt, params![k.as_ref()])
                .await?
                .into_iter()
                .next()
            {
                Some(row) => {
                    let val = row.get::<_, &[u8]>(0);
                    serde_cbor::from_slice(val)?
                }
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
        let tx = self.transaction().await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

        if tx
            .query_one(&exists_stmt, params![user_id.as_str()])
            .await?
            .get(0)
        {
            return Ok(register::Res::UIDTaken);
        }

        let add_key_stmt = tx
            .prepare_typed(sql!("add_key"), types![BYTEA, BYTEA, INT8, BYTEA])
            .await?;

        tx.execute(
            &add_key_stmt,
            params![
                key.data().as_ref(),
                key.signed_by().as_ref(),
                key.timestamp().0,
                key.sig().as_ref(),
            ],
        )
        .await?;

        let add_user_key_stmt = tx
            .prepare_typed(sql!("add_user_key"), types![TEXT, BYTEA])
            .await?;

        tx.execute(
            &add_user_key_stmt,
            params![user_id.as_str(), key.data().as_ref()],
        )
        .await?;
        tx.commit().await?;

        Ok(register::Res::Success)
    }

    pub async fn add_key(&mut self, key: Signed<sig::PublicKey>) -> Result<PKIResponse, Error> {
        let tx = self.transaction().await?;

        let user_id_stmt = tx
            .prepare_typed(sql!("get_user_id_by_key"), types![BYTEA])
            .await?;

        let user_id = match tx
            .query_one(&user_id_stmt, params![key.signed_by().as_ref()])
            .await?
            .get::<_, Option<String>>(0)
        {
            Some(uid) => uid,
            None => {
                return Ok(PKIResponse::DeadKey);
            }
        };

        let add_key_stmt = tx
            .prepare_typed(sql!("add_key"), types![BYTEA, BYTEA, INT8, BYTEA])
            .await?;

        let res = tx
            .execute(
                &add_key_stmt,
                params![
                    key.data().as_ref(),
                    key.signed_by().as_ref(),
                    key.timestamp().0,
                    key.sig().as_ref(),
                ],
            )
            .await;

        unique_violation_to_redundant(res)?;

        let add_user_key_stmt = tx
            .prepare_typed(sql!("add_user_key"), types![TEXT, BYTEA])
            .await?;

        let res = tx
            .execute(
                &add_user_key_stmt,
                params![user_id.as_str(), key.data().as_ref()],
            )
            .await;

        tx.commit().await?;
        unique_violation_to_redundant(res)
    }

    pub async fn read_key(&mut self, key: sig::PublicKey) -> Result<sig::PKMeta, Error> {
        use sig::*;

        let stmt = self
            .prepare_typed(sql!("get_pk_meta"), types![BYTEA])
            .await?;

        let row = self.query_one(&stmt, params![key.as_ref()]).await?;

        let (signed_by, sig, ts, dep_signed_by, dep_signature, dep_ts): RawPkMeta = (
            row.get(0),
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
        );

        let sig = Signature::from_slice(sig).ok_or(InvalidSig)?;
        let signed_by = PublicKey::from_slice(signed_by).ok_or(InvalidKey)?;
        let sig_meta = SigMeta::new(sig, signed_by, ts.into());

        let dep_sig_meta = dep_check(dep_ts, dep_signed_by, dep_signature)?;

        Ok(PKMeta::new(sig_meta, dep_sig_meta))
    }

    pub async fn deprecate_key(
        &mut self,
        signed_key: Signed<sig::PublicKey>,
    ) -> Result<PKIResponse, Error> {
        let (data, meta) = signed_key.split();
        let signer_key = meta.signed_by();

        let tx = self.transaction().await?;

        let signer_key_exists_stmt = tx
            .prepare_typed(sql!("key_is_valid"), types![BYTEA])
            .await?;

        let signer_key_exists: bool = tx
            .query_one(&signer_key_exists_stmt, params![signer_key.as_ref()])
            .await?
            .get(0);

        if !signer_key_exists {
            return Ok(PKIResponse::DeadKey);
        }

        let dep_stmt = tx
            .prepare_typed(sql!("deprecate_key"), types![INT8, BYTEA, BYTEA, BYTEA])
            .await?;

        let num_updated = tx
            .execute(
                &dep_stmt,
                params![
                    meta.timestamp().0,
                    signer_key.as_ref(),
                    meta.sig().as_ref(),
                    data.as_ref(),
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
        let stmt = self
            .prepare_typed(sql!("user_exists"), types![TEXT])
            .await?;

        let row = self.query_one(&stmt, params![uid.as_str()]).await?;

        Ok(row.get(0))
    }

    pub async fn key_is_valid(&mut self, key: sig::PublicKey) -> Result<bool, Error> {
        let stmt = self
            .prepare_typed(sql!("key_is_valid"), types![BYTEA])
            .await?;

        let row = self.query_one(&stmt, params![key.as_ref()]).await?;

        Ok(row.get(0))
    }

    pub async fn get_pending(
        &mut self,
        key: sig::PublicKey,
        limit: u32,
    ) -> Result<Vec<Push>, Error> {
        let text = format!(sql!("get_pending"), limit = limit);
        let stmt = self.prepare_typed(&text, types![BYTEA]).await?;

        let rows = self.query(&stmt, params![key.as_ref()]).await?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let p: &[u8] = row.get(0);
            out.push(serde_cbor::from_slice(p)?);
        }

        Ok(out)
    }

    pub async fn expire_pending(&mut self, key: sig::PublicKey, limit: u32) -> Result<(), Error> {
        let text = format!(sql!("expire_pending"), limit = limit);

        let stmt = self.prepare_typed(&text, types![BYTEA]).await?;
        self.execute(&stmt, params![key.as_ref()]).await?;

        Ok(())
    }

    pub async fn valid_keys(&mut self, uid: &UserId) -> Result<Vec<sig::PublicKey>, Error> {
        let stmt = self
            .prepare_typed(sql!("get_valid_keys_by_user_id"), types![TEXT])
            .await?;

        self.query(&stmt, params![uid.as_str()])
            .await?
            .into_iter()
            .map(|row| {
                let raw = row.get(0);
                sig::PublicKey::from_slice(raw).ok_or(Error::InvalidKey)
            })
            .collect()
    }

    pub async fn read_meta(&mut self, uid: &UserId) -> Result<UserMeta, Error> {
        let stmt = self.prepare_typed(sql!("read_meta"), types![TEXT]).await?;

        let meta_inner: Result<BTreeMap<sig::PublicKey, sig::PKMeta>, Error> = self
            .query(&stmt, params![uid.as_str()])
            .await?
            .into_iter()
            .map(|row| {
                let key = row.get::<_, &[u8]>(0);
                let signed_by = row.get::<_, &[u8]>(1);
                let creation_ts = row.get::<_, i64>(2);
                let signature = row.get::<_, &[u8]>(3);
                let deprecation_ts = row.get::<_, Option<i64>>(4);
                let dep_signed_by = row.get::<_, Option<&[u8]>>(5);
                let dep_signature = row.get::<_, Option<&[u8]>>(6);

                let key = sig::PublicKey::from_slice(key).ok_or(InvalidKey)?;
                let signed_by = sig::PublicKey::from_slice(signed_by).ok_or(InvalidKey)?;
                let timestamp = creation_ts.into();
                let signature = sig::Signature::from_slice(signature).ok_or(InvalidSig)?;

                let dep_sig_meta = dep_check(deprecation_ts, dep_signed_by, dep_signature)?;
                let sig_meta = SigMeta::new(signature, signed_by, timestamp);
                let pkmeta = sig::PKMeta::new(sig_meta, dep_sig_meta);
                Ok((key, pkmeta))
            })
            .collect();

        Ok(UserMeta { keys: meta_inner? })
    }

    pub async fn add_pending<'a, M: Iterator<Item = &'a Push>>(
        &mut self,
        keys: Vec<sig::PublicKey>,
        msgs: M,
    ) -> Result<(), Error> {
        let tx = self.transaction().await?;

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
            .await?;

        let pending_stmt = tx
            .prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
            .await?;

        for msg in msgs {
            let push_row_id: i64 = {
                let push_timestamp = msg.timestamp;
                let push_vec = serde_cbor::to_vec(msg)?;

                tx.query_one(&push_stmt, params![push_vec, push_timestamp.0])
                    .await?
                    .get(0)
            };

            for k in keys.iter() {
                tx.execute(&pending_stmt, params![k.as_ref(), &push_row_id])
                    .await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn setup(&mut self) -> Result<(), Error> {
        // create
        self.batch_execute(include_str!(
            "../../migrations/2019-09-21-221007_herald/up.sql"
        ))
        .await?;
        Ok(())
    }

    pub async fn reset_all(&mut self) -> Result<(), Error> {
        let tx = self.transaction().await?;

        // drop
        tx.batch_execute(include_str!(
            "../../migrations/2019-09-21-221007_herald/down.sql"
        ))
        .await?;

        // create
        tx.batch_execute(include_str!(
            "../../migrations/2019-09-21-221007_herald/up.sql"
        ))
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
