use async_trait::*;
use futures::{join, FutureExt, Stream, StreamExt, TryStreamExt};
use herald_common::*;
use parking_lot::Mutex;
use server_errors::Error;
use std::convert::TryFrom;

use tokio_postgres::{
    types::{ToSql, Type},
    Client, Error as PgError, NoTls,
};

mod imp;
mod macros;
mod pool;
mod trait_def;
pub use pool::*;
pub use trait_def::ServerStore;

pub enum PushedTo {
    PushedTo {
        devs: Vec<sig::PublicKey>,
        push_id: i64,
    },
    Missing(SingleRecip),
}

#[async_trait]
impl ServerStore for Conn {
    async fn get_sigchain(
        &mut self,
        user: UserId,
    ) -> Result<Option<sig::SigChain>, Error> {
        let (stmt, dep_stmt) = join!(
            self.prepare_typed(sql!("key_creations"), types![TEXT]),
            self.prepare_typed(sql!("key_deprecations"), types![TEXT])
        );
        let (stmt, dep_stmt) = (stmt?, dep_stmt?);

        let (rows, dep_rows) = {
            let conn = &self;
            let stmt = &stmt;
            let dep_stmt = &dep_stmt;

            join!(
                async move { conn.query(stmt, params![user.as_str()]).await },
                async move { conn.query(dep_stmt, params![user.as_str()]).await }
            )
        };

        let (rows, dep_rows) = (rows?, dep_rows?);

        let mut rows = rows.into_iter();

        let get_inner_meta = |row| {
            let signed_by = row.get("key");
            let sig = row.get("inner_signature");
            let timestamp: i64 = row.get("inner_ts");

            Ok(SigMeta::new(
                sig::Signature::from_slice(sig).ok_or(Error::InvalidSig)?,
                sig::PublicKey::from_slice(signed_by).ok_or(Error::InvalidKey)?,
                Time::from(timestamp),
            ))
        };

        let get_meta = |row| {
            let sig = row.get("signature");
            let signed_by = row.get("signed_by");
            let timestamp: i64 = row.get("timestamp");

            SigMeta::new(
                sig::Signature::from_slice(sig).ok_or(Error::InvalidSig)?,
                sig::PublicKey::from_slice(signed_by).ok_or(Error::InvalidKey)?,
                Time::from(timestamp),
            )
            .into()
        };

        let initial = match rows.next() {
            None => return Ok(None),
            Some(initial) => {
                let meta = get_inner_meta(initial)?;
                (user, meta).into()
            }
        };

        let mut sig_chain: Vec<Signed<sig::SigUpdate>> =
            Vec::with_capacity(rows.len() + dep_rows.len());

        for row in rows {
            let inner_meta: SigMeta = get_inner_meta(row)?;
            let inner = sig::SigUpdate::Endorse((user, inner_meta).into());

            let meta: SigMeta = get_meta(row)?;
            let update = Signed::from((inner, meta));

            sig_chain.push(update);
        }

        for row in dep_rows {
            let pk = sig::PublicKey::from_slice(row.get("key")).ok_or(Error::InvalidKey)?;
            let meta = get_meta(row)?;
            let update = Signed::from((sig::SigUpdate::Deprecate(pk), meta));

            sig_chain.push(update);
        }

        sig_chain.sort_unstable_by(|a, b| a.timestamp().cmp(b.timestamp()));

        Ok(Some(sig::SigChain { initial, sig_chain }))
    }

    async fn recip_exists(
        &mut self,
        recip: Recip,
    ) -> Result<bool, Error> {
        use Recip::*;

        match recip {
            One(single) => self.one_recip_exists(single).await,
            Many(recips) => self.many_recip_exists(recips).await,
        }
    }

    async fn add_to_sigchain(
        &mut self,
        new: Signed<sig::SigUpdate>,
    ) -> Result<PKIResponse, Error> {
        use sig::SigUpdate::*;

        let (update, meta) = new.split();

        match update {
            Endorse(signed_uid) => {
                let (uid, inner_meta) = signed_uid.split();

                let tx = self.transaction().await?;

                let (key_created_stmt, key_deprecated_stmt) = join!(
                    tx.prepare_typed(sql!("key_is_valid"), types![BYTEA]),
                    tx.pre
                );

                let signer_key_exists_stmt = tx
                    .prepare_typed(sql!("key_is_valid"), types![BYTEA])
                    .await?;

                let signer_key_exists: bool = tx
                    .query_one(&signer_key_exists_stmt, params![meta.signed_by().as_ref()])
                    .await?
                    .get(0);

                if !signer_key_exists {
                    return Ok(PKIResponse::DeadKey);
                }

                let (user_key_stmt, endorsement_stmt) = join!(
                    tx.prepare_typed(sql!("add_user_key"), types!(TEXT, BYTEA)),
                    tx.prepare_typed(
                        sql!("add_endorsement"),
                        types!(BYTEA, BYTEA, INT8, BYTEA, BYTEA, INT8),
                    )
                );

                let (user_key_stmt, endorsement_stmt) = (user_key_stmt?, endorsement_stmt?);

                tx.execute(
                    &user_key_stmt,
                    params![uid.as_str(), inner_meta.signed_by().as_ref()],
                )
                .await?;

                tx.execute(
                    &endorsement_stmt,
                    params![
                        inner_meta.signed_by().as_ref(),
                        inner_meta.sig().as_ref(),
                        inner_meta.timestamp().as_i64(),
                        meta.signed_by().as_ref(),
                        meta.timestamp().as_i64(),
                        meta.sig().as_ref(),
                    ],
                )
                .await?;

                tx.commit().await?;

                Ok(PKIResponse::Success)
            }

            Deprecate(pk) => {
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
                            meta.timestamp().as_i64(),
                            signer_key.as_ref(),
                            meta.sig().as_ref(),
                            pk.as_ref(),
                        ],
                    )
                    .await?;

                if num_updated != 1 {
                    return Ok(PKIResponse::Redundant);
                }

                tx.commit().await?;

                Ok(PKIResponse::Success)
            }
        }
    }

    async fn user_of(
        &mut self,
        key: sig::PublicKey,
    ) -> Result<Option<UserId>, Error> {
        let stmt = self.prepare_typed(sql!("user_of"), types![BYTEA]).await?;

        Ok(self
            .query(&stmt, params![key.as_ref()])
            .await?
            .into_iter()
            .next()
            .and_then(|row: tokio_postgres::Row| -> Option<UserId> {
                let uid_str: &str = row.get(0);
                UserId::try_from(uid_str).ok()
            }))
    }

    async fn new_prekeys<Keys: Stream<Item = (Signed<Prekey>, Option<Prekey>)> + Send>(
        &mut self,
        keys: Keys,
    ) -> Result<new_prekeys::Res, Error> {
        unimplemented!()
    }

    async fn get_random_prekeys<Keys: Stream<Item = sig::PublicKey> + Send>(
        &mut self,
        keys: Keys,
    ) -> Result<Vec<(sig::PublicKey, Signed<Prekey>)>, Error> {
        let prekeys = Mutex::new(Vec::new());
        let stmt = self.prepare_typed("TODO", types![BYTEA]).await?;

        keys.map(Ok::<_, Error>)
            .try_for_each_concurrent(10, |k| {
                let conn = &self;
                async { unimplemented!() }
            })
            .await?;

        Ok(prekeys.into_inner())
    }

    async fn add_to_group<Users: Stream<Item = UserId> + Send + Unpin>(
        &mut self,
        users: Users,
        conv: ConversationId,
    ) -> Result<add_to_group::Res, Error> {
        let tx = self.transaction().await?;

        let (insert_stmt, exists_stmt) = join!(
            tx.prepare_typed(sql!("add_to_group"), types![BYTEA, TEXT]),
            tx.prepare_typed(sql!("user_exists"), types![TEXT])
        );
        let (insert_stmt, exists_stmt) = (insert_stmt?, exists_stmt?);

        let res: Result<(), Result<add_to_group::Res, Error>> = users
            .map(Ok::<UserId, Result<add_to_group::Res, Error>>)
            .try_for_each_concurrent(10, |u| {
                let tx = &tx;
                let insert_stmt = &insert_stmt;
                let exists_stmt = &exists_stmt;

                async move {
                    let uid_str: &str = u.as_str();

                    if !tx
                        .query_one(exists_stmt, params![uid_str])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?
                        .get::<_, bool>(0)
                    {
                        return Err(Ok(add_to_group::Res::MissingUser(u)));
                    }

                    tx.execute(insert_stmt, params![conv.as_slice(), uid_str])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?;

                    Ok(())
                }
            })
            .await;

        match res {
            Err(v @ Ok(add_to_group::Res::MissingUser(_))) => return v,
            Err(Err(e)) => return Err(e.into()),
            _ => {}
        };

        tx.commit().await?;

        Ok(add_to_group::Res::Success)
    }

    async fn leave_group<Convs: Stream<Item = ConversationId> + Send>(
        &mut self,
        user: UserId,
        groups: Convs,
    ) -> Result<leave_groups::Res, Error> {
        let leave_stmt = self
            .prepare_typed(sql!("leave_group"), types![TEXT, BYTEA])
            .await?;

        let exists_stmt = self
            .prepare_typed(sql!("group_exists"), types![BYTEA])
            .await?;

        let uid_str: &str = user.as_str();

        let res: Result<(), Result<leave_groups::Res, Error>> = groups
            .map(Ok::<ConversationId, Result<leave_groups::Res, Error>>)
            .try_for_each_concurrent(10, |cid| {
                let conn = &self;
                let exists_stmt = &exists_stmt;
                let leave_stmt = &leave_stmt;

                async move {
                    let cid_slice: &[u8] = cid.as_slice();

                    if !conn
                        .query_one(exists_stmt, params![cid_slice])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?
                        .get::<_, bool>(0)
                    {
                        return Err(Ok(leave_groups::Res::Missing(cid)));
                    }

                    conn.execute(leave_stmt, params![uid_str])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?;

                    Ok(())
                }
            })
            .await;

        match res {
            Err(v @ Ok(leave_groups::Res::Missing(_))) => return v,
            Err(Err(e)) => return Err(e.into()),
            _ => {}
        };

        Ok(leave_groups::Res::Success)
    }

    // should be done transactionally, returns Missing(r) for the first missing recip r
    // only adds to pending when it finds all devices
    async fn add_to_pending_and_get_valid_devs(
        &mut self,
        recip: &Recip,
        msg: &Push,
    ) -> Result<PushedTo, Error> {
        use Recip::*;

        match recip {
            One(single) => {
                use SingleRecip::*;
                match single {
                    Group(cid) => self.one_group(cid, msg).await,
                    User(uid) => self.one_user(uid, msg).await,
                    Key(key) => self.one_key(key, msg).await,
                }
            }
            Many(recips) => {
                use Recips::*;

                match recips {
                    Groups(cids) => self.many_groups(cids, msg).await,
                    Users(uids) => self.many_users(uids, msg).await,
                    Keys(keys) => self.many_keys(keys, msg).await,
                }
            }
        }
    }

    async fn get_pending(
        &mut self,
        of: sig::PublicKey,
    ) -> Result<Vec<(Push, i64)>, Error> {
        let stmt = self
            .prepare_typed(sql!("get_pending"), types![BYTEA])
            .await?;

        let rows = self.query(&stmt, params![of.as_ref()]).await?;

        let mut out = Vec::with_capacity(rows.len());

        for row in rows {
            let push: &[u8] = row.get("push_data");
            let push_id: i64 = row.get("push_id");

            out.push((kson::from_slice(push)?, push_id));
        }

        Ok(out)
    }

    async fn del_pending<S: Stream<Item = i64> + Send>(
        &mut self,
        of: sig::PublicKey,
        items: S,
    ) -> Result<(), Error> {
        let stmt = self
            .prepare_typed(sql!("expire_pending"), types![BYTEA, INT8])
            .await?;

        items
            .map(Ok::<i64, Error>)
            .try_for_each_concurrent(10, |index| {
                let conn = &self;
                let stmt = &stmt;
                let of = &of;

                async move {
                    conn.execute(stmt, params![of.as_ref(), index]).await?;
                    Ok(())
                }
            })
            .await
    }

    async fn new_user(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<register::Res, Error> {
        let (user_id, meta) = init.split();

        let tx = self.transaction().await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

        if !tx
            .query(&exists_stmt, params![user_id.as_str()])
            .await?
            .is_empty()
        {
            return Ok(register::Res::UserAlreadyClaimed);
        }

        let add_key_stmt = tx
            .prepare_typed(sql!("add_key"), types![BYTEA, BYTEA, INT8, BYTEA])
            .await?;

        tx.execute(
            &add_key_stmt,
            params![
                meta.signed_by().as_ref(),
                meta.signed_by().as_ref(),
                meta.timestamp().as_i64(),
                meta.sig().as_ref(),
            ],
        )
        .await?;

        let add_user_key_stmt = tx
            .prepare_typed(sql!("add_user_key"), types![TEXT, BYTEA])
            .await?;

        tx.execute(
            &add_user_key_stmt,
            params![user_id.as_str(), meta.signed_by().as_ref()],
        )
        .await?;
        tx.commit().await?;

        Ok(register::Res::Success)
    }
}

impl Conn {
    pub async fn setup(&mut self) -> Result<(), Error> {
        // create
        self.batch_execute(include_str!("../schema/up.sql")).await?;
        self.execute(sql!("user_exists_func"), params![]).await?;
        Ok(())
    }

    pub async fn reset_all(&mut self) -> Result<(), Error> {
        let tx = self.transaction().await?;

        // drop
        tx.batch_execute(include_str!("../schema/down.sql")).await?;

        // create
        tx.batch_execute(include_str!("../schema/up.sql")).await?;
        tx.execute(sql!("user_exists_func"), params![]).await?;
        tx.commit().await?;
        Ok(())
    }
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)]
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

#[cfg(test)]
mod tests;
