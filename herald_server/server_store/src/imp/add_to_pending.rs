use super::*;
use crate::slice_iter;

macro_rules! sql {
    ($path: literal) => {
        include_str!(concat!("../sql/", $path, ".sql"))
    };
}

impl Conn {
    pub(crate) async fn one_group(
        &mut self,
        cid: &ConversationId,
        msg: &Push,
    ) -> Result<PushedTo, Error> {
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

                    pushed_to
                        .lock()
                        .push(sig::PublicKey::from_slice(&key).ok_or(Error::InvalidKey)?);

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

    pub(crate) async fn one_user(
        &mut self,
        uid: &UserId,
        msg: &Push,
    ) -> Result<PushedTo, Error> {
        let tx = self.transaction().await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![BYTEA]).await?;

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
                    pushed_to
                        .lock()
                        .push(sig::PublicKey::from_slice(&key).ok_or(Error::InvalidKey)?);
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

    pub(crate) async fn one_key(
        &mut self,
        key: &sig::PublicKey,
        msg: &Push,
    ) -> Result<PushedTo, Error> {
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

    pub(crate) async fn many_groups(
        &mut self,
        cids: &[ConversationId],
        msg: &Push,
    ) -> Result<PushedTo, Error> {
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

        let (keys_stmt, pending_stmt, exists_stmt) = (keys_stmt?, pending_stmt?, exists_stmt?);

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
                .try_for_each_concurrent(Some(10), |row| {
                    let key = row.get::<_, Vec<u8>>(0);

                    async move {
                        tx.execute(pending_stmt, params![key, push_row_id])
                            .await
                            .map_err(Error::PgError)?;

                        pushed_to
                            .lock()
                            .push(sig::PublicKey::from_slice(&key).ok_or(Error::InvalidKey)?);

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

    pub(crate) async fn many_users(
        &mut self,
        uids: &[UserId],
        msg: &Push,
    ) -> Result<PushedTo, Error> {
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

        let (keys_stmt, pending_stmt, exists_stmt) = (keys_stmt?, pending_stmt?, exists_stmt?);

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

                        pushed_to
                            .lock()
                            .push(sig::PublicKey::from_slice(&key).ok_or(Error::InvalidKey)?);

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

    pub(crate) async fn many_keys(
        &mut self,
        keys: &[sig::PublicKey],
        msg: &Push,
    ) -> Result<PushedTo, Error> {
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
