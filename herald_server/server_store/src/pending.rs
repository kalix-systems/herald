use super::*;
use crate::slice_iter;

#[derive(Debug, PartialEq)]
pub enum PushedTo {
    PushedTo {
        devs: Vec<sig::PublicKey>,
        push_id: i64,
    },
    Missing(SingleRecip),
}

#[cfg(test)]
impl PushedTo {
    fn is_missing(&self) -> bool {
        match self {
            PushedTo::Missing(_) => true,
            _ => false,
        }
    }
}

impl Conn {
    pub async fn get_pending(
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

    pub async fn del_pending<S: Stream<Item = i64> + Send>(
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

    // should be done transactionally, returns Missing(r) for the first missing recip r
    // only adds to pending when it finds all devices
    pub async fn add_to_pending_and_get_valid_devs(
        &mut self,
        recip: &Recip,
        Push {
            tag,
            timestamp,
            msg,
        }: &Push,
    ) -> Result<PushedTo, Error> {
        use Recip::*;

        match recip {
            One(single) => {
                use SingleRecip::*;
                match single {
                    Group(cid) => self.one_group(cid, msg, tag, timestamp).await,
                    User(uid) => self.one_user(uid, msg, tag, timestamp).await,
                    Key(key) => self.one_key(key, msg, tag, timestamp).await,
                }
            }
            Many(recips) => {
                use Recips::*;

                match recips {
                    Groups(cids) => self.many_groups(cids, msg, tag, timestamp).await,
                    Users(uids) => self.many_users(uids, msg, tag, timestamp).await,
                    Keys(keys) => self.many_keys(keys, msg, tag, timestamp).await,
                }
            }
        }
    }

    pub(crate) async fn one_key(
        &mut self,
        key: &sig::PublicKey,
        msg: &Bytes,
        tag: &PushTag,
        timestamp: &Time,
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

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8])
            .await?;

        let push_row_id: i64 = tx
            .query_one(
                &push_stmt,
                params![msg.as_ref(), kson::to_vec(tag), timestamp.as_i64()],
            )
            .await?
            .get(0);

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

    pub(crate) async fn one_user(
        &mut self,
        uid: &UserId,
        msg: &Bytes,
        tag: &PushTag,
        timestamp: &Time,
    ) -> Result<PushedTo, Error> {
        let tx = self.transaction().await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

        if !tx
            .query_one(&exists_stmt, params![uid.as_str()])
            .await?
            .get::<_, bool>(0)
        {
            return Ok(PushedTo::Missing(SingleRecip::User(*uid)));
        }

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8])
            .await?;

        let push_row_id: i64 = tx
            .query_one(
                &push_stmt,
                params![msg.as_ref(), kson::to_vec(tag), timestamp.as_i64()],
            )
            .await?
            .get(0);

        let (keys_stmt, pending_stmt) = try_join!(
            tx.prepare_typed(sql!("valid_user_keys"), types![TEXT]),
            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
        )?;

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

    pub(crate) async fn one_group(
        &mut self,
        cid: &ConversationId,
        msg: &Bytes,
        tag: &PushTag,
        timestamp: &Time,
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

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8])
            .await?;

        let push_row_id = tx
            .query_one(
                &push_stmt,
                params![msg.as_ref(), kson::to_vec(tag), timestamp.as_i64()],
            )
            .await?
            .get(0);

        let (keys_stmt, pending_stmt) = try_join!(
            tx.prepare_typed(sql!("conversation_member_keys"), types![BYTEA]),
            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
        )?;

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

    pub(crate) async fn many_groups(
        &mut self,
        cids: &[ConversationId],
        msg: &Bytes,
        tag: &PushTag,
        timestamp: &Time,
    ) -> Result<PushedTo, Error> {
        let tx = self.transaction().await?;

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, INT8])
            .await?;

        let push_row_id: i64 = tx
            .query_one(
                &push_stmt,
                params![msg.as_ref(), kson::to_vec(tag), timestamp.as_i64()],
            )
            .await?
            .get(0);

        let pushed_to = Mutex::new(Vec::new());

        let (keys_stmt, pending_stmt, exists_stmt) = try_join!(
            tx.prepare_typed(sql!("conversation_member_keys"), types![BYTEA]),
            tx.prepare_typed(sql!("add_pending"), types![BYTEA, BYTEA, INT8]),
            tx.prepare_typed(sql!("group_exists"), types![BYTEA])
        )?;

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
        msg: &Bytes,
        tag: &PushTag,
        timestamp: &Time,
    ) -> Result<PushedTo, Error> {
        let tx = self.transaction().await?;

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8])
            .await?;

        let push_row_id: i64 = tx
            .query_one(
                &push_stmt,
                params![msg.as_ref(), kson::to_vec(tag), timestamp.as_i64()],
            )
            .await?
            .get(0);

        let pushed_to = Mutex::new(Vec::new());

        let (keys_stmt, pending_stmt, exists_stmt) = try_join!(
            tx.prepare_typed(sql!("valid_user_keys"), types![TEXT]),
            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8]),
            tx.prepare_typed(sql!("user_exists"), types![BYTEA]),
        )?;

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
        msg: &Bytes,
        tag: &PushTag,
        timestamp: &Time,
    ) -> Result<PushedTo, Error> {
        let tx = self.transaction().await?;

        let push_stmt = tx
            .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8])
            .await?;

        let push_row_id: i64 = tx
            .query_one(
                &push_stmt,
                params![msg.as_ref(), kson::to_vec(tag), timestamp.as_i64()],
            )
            .await?
            .get(0);
        let (pending_stmt, exists_stmt) = try_join!(
            tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8]),
            tx.prepare_typed(sql!("device_exists"), types![BYTEA])
        )?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_client;
    use crate::{w, wa};
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    fn push() -> Push {
        Push {
            msg: Bytes::from_static(b"test"),
            timestamp: Time::now(),
            tag: PushTag::Device,
        }
    }

    #[tokio::test]
    #[serial]
    async fn one_key() {
        let mut client = wa!(get_client());

        let push = push();

        let a_uid = w!("a".try_into());
        let a_kp = sig::KeyPair::gen_new();
        let recip = Recip::One(SingleRecip::Key(*a_kp.public_key()));

        assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

        let a_init = a_kp.sign(a_uid);
        wa!(client.new_user(a_init));

        match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
            PushedTo::PushedTo { devs, .. } => {
                assert_eq!(devs, vec![*a_kp.public_key()]);
            }
            _ => panic!(),
        }
    }

    #[tokio::test]
    #[serial]
    async fn one_user() {
        let mut client = wa!(get_client());

        let push = push();

        let a_uid = w!("a".try_into());
        let a_kp = sig::KeyPair::gen_new();
        let recip = Recip::One(SingleRecip::User(a_uid));

        assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

        let a_init = a_kp.sign(a_uid);
        wa!(client.new_user(a_init));

        match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
            PushedTo::PushedTo { devs, .. } => {
                assert_eq!(devs, vec![*a_kp.public_key()]);
            }
            _ => panic!(),
        }
    }

    #[tokio::test]
    #[serial]
    async fn one_group() {
        let mut client = wa!(get_client());

        let push = push();

        let a_uid = w!("a".try_into());
        let a_kp = sig::KeyPair::gen_new();
        let cid = ConversationId::gen_new();

        let recip = Recip::One(SingleRecip::Group(cid));

        assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

        let a_init = a_kp.sign(a_uid);

        wa!(client.new_user(a_init));
        wa!(client.add_to_group(futures::stream::iter(vec![a_uid]), cid));

        match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
            PushedTo::PushedTo { devs, .. } => {
                assert_eq!(devs, vec![*a_kp.public_key()]);
            }
            _ => panic!(),
        }
    }

    #[tokio::test]
    #[serial]
    async fn many_keys() {
        let mut client = wa!(get_client());

        let push = push();

        let a_uid = w!("a".try_into());
        let a_kp = sig::KeyPair::gen_new();
        let cid = ConversationId::gen_new();

        let recip = Recip::One(SingleRecip::Group(cid));

        assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

        let a_init = a_kp.sign(a_uid);

        wa!(client.new_user(a_init));
        wa!(client.add_to_group(futures::stream::iter(vec![a_uid]), cid));

        match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
            PushedTo::PushedTo { devs, .. } => {
                assert_eq!(devs, vec![*a_kp.public_key()]);
            }
            _ => panic!(),
        }
    }

    #[tokio::test]
    #[serial]
    async fn many_users() {}

    #[tokio::test]
    #[serial]
    async fn many_groups() {}
}
