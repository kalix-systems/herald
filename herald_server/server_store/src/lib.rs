use futures::{try_join, FutureExt, Stream, StreamExt, TryStreamExt};
use herald_common::*;
use parking_lot::Mutex;
use server_errors::Error;
use std::convert::TryFrom;

use tokio_postgres::{
    types::{ToSql, Type},
    Client, Error as PgError, NoTls,
};

type Res<T> = Result<T, Error>;

mod macros;
mod pending;
mod pool;
mod prekeys;
mod recip_exists;
mod sigchain;
pub use pool::*;

impl Conn {
    pub async fn user_of(
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

    pub async fn add_to_group<Users: Stream<Item = UserId> + Send + Unpin>(
        &mut self,
        users: Users,
        conv: ConversationId,
    ) -> Result<add_to_group::Res, Error> {
        let tx = self.transaction().await?;

        let (insert_stmt, exists_stmt) = try_join!(
            tx.prepare_typed(sql!("add_to_group"), types![BYTEA, TEXT]),
            tx.prepare_typed(sql!("user_exists"), types![TEXT])
        )?;

        let res: Result<(), Res<add_to_group::Res>> = users
            .map(Ok::<UserId, Res<add_to_group::Res>>)
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

    pub async fn leave_group<Convs: Stream<Item = ConversationId> + Send>(
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

    pub async fn new_user(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<register::Res, Error> {
        let (user_id, meta) = init.split();

        let tx = self.transaction().await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

        if tx
            .query_one(&exists_stmt, params![user_id.as_str()])
            .await?
            .get::<_, bool>(0)
        {
            return Ok(register::Res::UserAlreadyClaimed);
        }

        let add_key_stmt = tx
            .prepare_typed(sql!("add_key"), types![BYTEA, BYTEA, INT8])
            .await?;

        tx.execute(
            &add_key_stmt,
            params![
                meta.signed_by().as_ref(),
                meta.sig().as_ref(),
                meta.timestamp().as_i64(),
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
        Ok(())
    }

    pub async fn reset_all(&mut self) -> Result<(), Error> {
        let tx = self.transaction().await?;

        // drop
        tx.batch_execute(include_str!("../schema/down.sql")).await?;

        // create
        tx.batch_execute(include_str!("../schema/up.sql")).await?;
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
pub(crate) mod tests;
