#![allow(unused)]

use async_trait::*;
use futures::{join, FutureExt, Stream, StreamExt, TryStreamExt};
use herald_common::*;
use parking_lot::Mutex;
use server_errors::{Error, Error::*};
use std::convert::{TryFrom, TryInto};

use std::future::Future;
use std::pin::Pin;
use tokio_postgres::{
    types::{ToSql, Type},
    Client, Error as PgError, NoTls, Row,
};

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
        unimplemented!()
    }

    async fn recip_exists(
        &mut self,
        recip: Recip,
    ) -> Result<bool, Error> {
        use Recip::*;

        match recip {
            One(single) => {
                use SingleRecip::*;
                match single {
                    Group(cid) => {
                        let stmt = self
                            .prepare_typed(sql!("group_exists"), types![BYTEA])
                            .await?;

                        Ok(self.query_one(&stmt, params![cid.as_slice()]).await?.get(0))
                    }

                    User(uid) => {
                        let stmt = self
                            .prepare_typed(sql!("user_exists"), types![TEXT])
                            .await?;

                        Ok(self.query_one(&stmt, params![uid.as_str()]).await?.get(0))
                    }

                    Key(key) => {
                        let stmt = self
                            .prepare_typed(sql!("device_exists"), types![BYTEA])
                            .await?;

                        let row = self.query_one(&stmt, params![key.as_ref()]).await?;

                        Ok(row.get(0))
                    }
                }
            }

            Many(recips) => {
                use Recips::*;

                match recips {
                    Groups(cids) => {
                        let stmt = self
                            .prepare_typed(sql!("group_exists"), types![BYTEA])
                            .await?;

                        for cid in cids {
                            let cid_slice = cid.as_slice();
                            let row = self.query_one(&stmt, params![cid_slice]).await?;

                            if !row.get::<_, bool>(0) {
                                return Ok(false);
                            }
                        }

                        Ok(true)
                    }

                    Users(uids) => {
                        let stmt = self
                            .prepare_typed(sql!("user_exists"), types![BYTEA])
                            .await?;

                        for uid in uids {
                            let uid_str = uid.as_str();
                            let row = self.query_one(&stmt, params![uid_str]).await?;

                            if !row.get::<_, bool>(0) {
                                return Ok(false);
                            }
                        }

                        Ok(true)
                    }

                    Keys(keys) => {
                        let stmt = self
                            .prepare_typed(sql!("device_exists"), types![BYTEA])
                            .await?;

                        for key in keys {
                            let key_slice = key.as_ref();

                            let row = self.query_one(&stmt, params![key_slice]).await?;

                            if !row.get::<_, bool>(0) {
                                return Ok(false);
                            }
                        }

                        Ok(true)
                    }
                }
            }
        }
    }

    async fn add_to_sigchain(
        &mut self,
        new: Signed<sig::SigUpdate>,
    ) -> Result<PKIResponse, Error> {
        use sig::SigUpdate::*;

        let (update, meta) = new.split();

        match update {
            Endorse(signed_uid) => unimplemented!(),

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
        unimplemented!()
    }

    async fn add_to_group<Users: Stream<Item = UserId> + Send + Unpin>(
        &mut self,
        users: Users,
        conv: ConversationId,
    ) -> Result<add_to_group::Res, Error> {
        let tx = self.transaction().await?;

        let insert_stmt = tx
            .prepare_typed(sql!("add_to_group"), types![BYTEA, TEXT])
            .await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

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

                    tx.execute(insert_stmt, params![uid_str])
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
                    Group(cid) => {
                        let tx = self.transaction().await?;
                        let exists_stmt = tx
                            .prepare_typed(sql!("group_exists"), types![BYTEA])
                            .await?;

                        if !tx
                            .query_one(&exists_stmt, params![cid.as_slice()])
                            .await?
                            .get::<_, bool>(0)
                        {
                            return Ok(PushedTo::Missing(SingleRecip::Group(*cid)));
                        }

                        let push_row_id: i64 = {
                            let push_stmt = tx
                                .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
                                .await?;

                            let push_timestamp = msg.timestamp;
                            let push_vec = kson::to_vec(msg);

                            tx.query_one(&push_stmt, params![push_vec, push_timestamp.as_i64()])
                                .await?
                                .get(0)
                        };

                        let (keys_stmt, pending_stmt) = join!(
                            tx.prepare_typed(sql!("conversation_member_keys"), types![BYTEA]),
                            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
                        );
                        let (keys_stmt, pending_stmt) = (keys_stmt?, pending_stmt?);

                        let pushed_to = Mutex::new(Vec::new());

                        tx.query_raw(&keys_stmt, slice_iter(params![cid.as_slice()]))
                            .await?
                            .map_err(Error::PgError)
                            .try_for_each_concurrent(10, |row| {
                                let pending_stmt = &pending_stmt;
                                let pushed_to = &pushed_to;
                                let tx = &tx;
                                let key = row.get::<_, Vec<u8>>(0);

                                async move {
                                    tx.execute(pending_stmt, params![key, push_row_id])
                                        .await
                                        .map_err(Error::PgError)?;

                                    pushed_to.lock().push(
                                        sig::PublicKey::from_slice(&key)
                                            .ok_or(Error::InvalidKey)?,
                                    );

                                    Ok(())
                                }
                            })
                            .await?;

                        tx.commit().await?;

                        Ok(PushedTo::PushedTo {
                            devs: pushed_to.into_inner(),
                            push_id: push_row_id,
                        })
                    }

                    User(uid) => {
                        let tx = self.transaction().await?;

                        let exists_stmt =
                            tx.prepare_typed(sql!("user_exists"), types![BYTEA]).await?;

                        if !tx
                            .query_one(&exists_stmt, params![uid.as_str()])
                            .await?
                            .get::<_, bool>(0)
                        {
                            return Ok(PushedTo::Missing(SingleRecip::User(*uid)));
                        }

                        let push_row_id: i64 = {
                            let push_stmt = tx
                                .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
                                .await?;

                            let push_timestamp = msg.timestamp;
                            let push_vec = kson::to_vec(msg);

                            tx.query_one(&push_stmt, params![push_vec, push_timestamp.as_i64()])
                                .await?
                                .get(0)
                        };

                        let (keys_stmt, pending_stmt) = join!(
                            tx.prepare_typed("TODO: valid user keys", types![BYTEA]),
                            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
                        );
                        let (keys_stmt, pending_stmt) = (keys_stmt?, pending_stmt?);

                        let pushed_to = Mutex::new(Vec::new());

                        tx.query_raw(&keys_stmt, slice_iter(params![uid.as_str()]))
                            .await?
                            .map_err(Error::PgError)
                            .try_for_each_concurrent(10, |row| {
                                let pending_stmt = &pending_stmt;
                                let tx = &tx;
                                let key = row.get::<_, Vec<u8>>(0);
                                let pushed_to = &pushed_to;

                                async move {
                                    tx.execute(pending_stmt, params![key, push_row_id]).await?;
                                    pushed_to.lock().push(
                                        sig::PublicKey::from_slice(&key)
                                            .ok_or(Error::InvalidKey)?,
                                    );
                                    Ok(())
                                }
                            });

                        tx.commit().await?;

                        Ok(PushedTo::PushedTo {
                            devs: pushed_to.into_inner(),
                            push_id: push_row_id,
                        })
                    }

                    Key(key) => {
                        let tx = self.transaction().await?;
                        let exists_stmt = tx
                            .prepare_typed(sql!("device_exists"), types![BYTEA])
                            .await?;

                        if !tx
                            .query_one(&exists_stmt, params![key.as_ref()])
                            .await?
                            .get::<_, bool>(0)
                        {
                            return Ok(PushedTo::Missing(SingleRecip::Key(*key)));
                        }

                        let push_row_id: i64 = {
                            let push_stmt = tx
                                .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
                                .await?;

                            let push_timestamp = msg.timestamp;
                            let push_vec = kson::to_vec(msg);

                            tx.query_one(&push_stmt, params![push_vec, push_timestamp.as_i64()])
                                .await?
                                .get(0)
                        };

                        let pending_stmt = tx
                            .prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
                            .await?;

                        tx.execute(&pending_stmt, params![key.as_ref(), push_row_id])
                            .await?;

                        tx.commit().await?;

                        Ok(PushedTo::PushedTo {
                            devs: vec![*key],
                            push_id: push_row_id,
                        })
                    }
                }
            }
            Many(recips) => {
                use Recips::*;

                match recips {
                    Groups(cids) => {
                        let tx = self.transaction().await?;

                        let push_row_id: i64 = {
                            let push_stmt = tx
                                .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
                                .await?;

                            let push_timestamp = msg.timestamp;
                            let push_vec = kson::to_vec(msg);

                            tx.query_one(&push_stmt, params![push_vec, push_timestamp.as_i64()])
                                .await?
                                .get(0)
                        };

                        let pushed_to = Mutex::new(Vec::new());

                        let (keys_stmt, pending_stmt, exists_stmt) = join!(
                            tx.prepare_typed(sql!("conversation_member_keys"), types![BYTEA]),
                            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8]),
                            tx.prepare_typed(sql!("group_exists"), types![BYTEA])
                        );

                        let (keys_stmt, pending_stmt, exists_stmt) =
                            (keys_stmt?, pending_stmt?, exists_stmt?);

                        // TODO: process concurrently?
                        for cid in cids {
                            let pending_stmt = &pending_stmt;
                            let exists_stmt = &exists_stmt;
                            let pushed_to = &pushed_to;
                            let tx = &tx;

                            if !tx
                                .query_one(exists_stmt, params![cid.as_slice()])
                                .await?
                                .get::<_, bool>(0)
                            {
                                return Ok(PushedTo::Missing(SingleRecip::Group(*cid)));
                            }

                            tx.query_raw(&keys_stmt, slice_iter(params![cid.as_slice()]))
                                .await?
                                .map_err(Error::PgError)
                                .try_for_each_concurrent(10, |row| {
                                    let key = row.get::<_, Vec<u8>>(0);

                                    async move {
                                        tx.execute(pending_stmt, params![key, push_row_id])
                                            .await
                                            .map_err(Error::PgError)?;

                                        pushed_to.lock().push(
                                            sig::PublicKey::from_slice(&key)
                                                .ok_or(Error::InvalidKey)?,
                                        );

                                        Ok(())
                                    }
                                })
                                .await?;
                        }

                        tx.commit().await?;

                        Ok(PushedTo::PushedTo {
                            devs: pushed_to.into_inner(),
                            push_id: push_row_id,
                        })
                    }
                    Users(uids) => {
                        let tx = self.transaction().await?;

                        let push_row_id: i64 = {
                            let push_stmt = tx
                                .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
                                .await?;

                            let push_timestamp = msg.timestamp;
                            let push_vec = kson::to_vec(msg);

                            tx.query_one(&push_stmt, params![push_vec, push_timestamp.as_i64()])
                                .await?
                                .get(0)
                        };

                        let pushed_to = Mutex::new(Vec::new());

                        let (keys_stmt, pending_stmt, exists_stmt) = join!(
                            tx.prepare_typed("TODO: valid user keys", types![BYTEA]),
                            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8]),
                            tx.prepare_typed(sql!("user_exists"), types![BYTEA]),
                        );

                        let (keys_stmt, pending_stmt, exists_stmt) =
                            (keys_stmt?, pending_stmt?, exists_stmt?);

                        // TODO: process concurrently?
                        for uid in uids {
                            let pending_stmt = &pending_stmt;
                            let exists_stmt = &exists_stmt;
                            let pushed_to = &pushed_to;
                            let tx = &tx;

                            if !tx
                                .query_one(exists_stmt, params![uid.as_str()])
                                .await?
                                .get::<_, bool>(0)
                            {
                                return Ok(PushedTo::Missing(SingleRecip::User(*uid)));
                            }

                            tx.query_raw(&keys_stmt, slice_iter(params![uid.as_str()]))
                                .await?
                                .map_err(Error::PgError)
                                .try_for_each_concurrent(10, |row| {
                                    let key = row.get::<_, Vec<u8>>(0);

                                    async move {
                                        tx.execute(pending_stmt, params![key, push_row_id]).await?;

                                        pushed_to.lock().push(
                                            sig::PublicKey::from_slice(&key)
                                                .ok_or(Error::InvalidKey)?,
                                        );

                                        Ok(())
                                    }
                                })
                                .await?;
                        }

                        tx.commit().await?;

                        Ok(PushedTo::PushedTo {
                            devs: pushed_to.into_inner(),
                            push_id: push_row_id,
                        })
                    }

                    Keys(keys) => {
                        let tx = self.transaction().await?;

                        let push_row_id: i64 = {
                            let push_stmt = tx
                                .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
                                .await?;

                            let push_timestamp = msg.timestamp;
                            let push_vec = kson::to_vec(msg);

                            tx.query_one(&push_stmt, params![push_vec, push_timestamp.as_i64()])
                                .await?
                                .get(0)
                        };
                        let (pending_stmt, exists_stmt) = join!(
                            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8]),
                            tx.prepare_typed(sql!("device_exists"), types![BYTEA])
                        );

                        let (pending_stmt, exists_stmt) = (pending_stmt?, exists_stmt?);

                        let mut pushed_to = Vec::new();

                        // TODO: process concurrently?
                        for key in keys {
                            if !tx
                                .query_one(&exists_stmt, params![key.as_ref()])
                                .await?
                                .get::<_, bool>(0)
                            {
                                return Ok(PushedTo::Missing(SingleRecip::Key(*key)));
                            }

                            tx.execute(&pending_stmt, params![key.as_ref(), push_row_id])
                                .await?;

                            pushed_to.push(*key);
                        }

                        tx.commit().await?;

                        Ok(PushedTo::PushedTo {
                            devs: pushed_to,
                            push_id: push_row_id,
                        })
                    }
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
                    conn.execute(stmt, params![of.as_ref(), index]);
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

        //tx.execute(
        //    &add_key_stmt,
        //    params![
        //        key.data().as_ref(),
        //        key.signed_by().as_ref(),
        //        key.timestamp().as_i64(),
        //        key.sig().as_ref(),
        //    ],
        //)
        //.await?;

        let add_user_key_stmt = tx
            .prepare_typed(sql!("add_user_key"), types![TEXT, BYTEA])
            .await?;

        //tx.execute(
        //    &add_user_key_stmt,
        //    params![user_id.as_str(), key.data().as_ref()],
        //)
        //.await?;
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

// #[cfg(test)]
// mod tests;
//
fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)]
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}
