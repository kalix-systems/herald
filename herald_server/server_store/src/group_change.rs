use super::*;

impl Conn {
    pub async fn init_group(
        &mut self,
        user: UserId,
        conv: ConversationId,
    ) -> Res<init_group::Res> {
        let conn = &self;

        let (user_exists, group_exists) = try_join!(
            async move {
                let stmt = conn
                    .prepare_typed(sql!("user_exists"), types![TEXT])
                    .await?;
                Ok::<_, Error>(
                    conn.query_one(&stmt, params![user.as_str()])
                        .await?
                        .get::<_, bool>(0),
                )
            },
            async move {
                let stmt = conn
                    .prepare_typed(sql!("group_exists"), types![BYTEA])
                    .await?;

                Ok::<_, Error>(
                    conn.query_one(&stmt, params![conv.as_slice()])
                        .await?
                        .get::<_, bool>(0),
                )
            }
        )?;

        if group_exists {
            return Ok(init_group::Res::GroupAlreadyExists(conv));
        }

        if !user_exists {
            return Ok(init_group::Res::MissingUser(user));
        }

        let stmt = self
            .prepare_typed(sql!("add_to_group"), types![BYTEA, TEXT])
            .await?;

        self.execute(&stmt, params![conv.as_slice(), user.as_str()])
            .await?;

        Ok(init_group::Res::Success)
    }

    pub async fn user_in_group(
        &mut self,
        user: UserId,
        conv: ConversationId,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("user_in_group"), types![BYTEA, TEXT])
            .await?;

        Ok(self
            .query_one(&stmt, params![conv.as_slice(), user.as_str()])
            .await?
            .get::<_, bool>(0))
    }

    pub async fn add_to_group<Users: Stream<Item = UserId> + Send + Unpin>(
        &mut self,
        added_by: UserId,
        users: Users,
        conv: ConversationId,
    ) -> Res<add_to_group::Res> {
        let tx = self.transaction().await?;

        let (insert_stmt, exists_stmt, _) = {
            let tx = &tx;
            let res = try_join!(
                async move {
                    tx.prepare_typed(sql!("add_to_group"), types![BYTEA, TEXT])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)
                },
                async move {
                    tx.prepare_typed(sql!("user_exists"), types![TEXT])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)
                },
                async move {
                    let stmt = tx
                        .prepare_typed(sql!("user_in_group"), types![BYTEA, TEXT])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?;

                    let added_by_exists = tx
                        .query_one(&stmt, params![conv.as_slice(), added_by.as_str()])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?
                        .get::<_, bool>(0);

                    let res: Result<(), Res<add_to_group::Res>> = if added_by_exists {
                        Ok(())
                    } else {
                        Err(Ok(add_to_group::Res::AddedByMissing(added_by)))
                    };

                    res
                }
            );

            match res {
                Err(v @ Ok(_)) => return v,
                Err(e @ Err(_)) => return e,
                Ok(v) => v,
            }
        };

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
            Err(v @ Ok(_)) => return v,
            Err(Err(e)) => return Err(e),
            _ => {}
        };

        tx.commit().await?;

        Ok(add_to_group::Res::Success)
    }

    pub async fn leave_groups<Convs: Stream<Item = ConversationId> + Send>(
        &mut self,
        user: UserId,
        groups: Convs,
    ) -> Res<leave_groups::Res> {
        let conn = &self;

        let (leave_stmt, exists_stmt) = try_join!(
            conn.prepare_typed(sql!("leave_group"), types![TEXT, BYTEA]),
            conn.prepare_typed(sql!("group_exists"), types![BYTEA])
        )?;

        let uid_str: &str = user.as_str();

        let res: Result<(), Res<leave_groups::Res>> = groups
            .map(Ok::<ConversationId, Res<leave_groups::Res>>)
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

                    conn.execute(leave_stmt, params![uid_str, cid.as_slice()])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?;

                    Ok(())
                }
            })
            .await;

        match res {
            Err(v @ Ok(leave_groups::Res::Missing(_))) => return v,
            Err(Err(e)) => return Err(e),
            _ => {}
        };

        Ok(leave_groups::Res::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::get_client, w, wa};
    use futures::stream::iter;
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    #[tokio::test]
    #[serial]
    async fn add_and_leave() {
        let mut client = wa!(get_client());
        let cid = ConversationId::gen_new();

        let a_uid: UserId = "a".try_into().expect(womp!());
        let a_kp = sig::KeyPair::gen_new();
        let a_init = a_kp.sign(a_uid);

        // non-existent user can't start conversation
        match wa!(client.init_group(a_uid, cid)) {
            init_group::Res::MissingUser(missing) => {
                assert_eq!(missing, a_uid);
            }
            _ => panic!(),
        };

        assert!(!wa!(client.user_in_group(a_uid, cid)));
        wa!(client.new_user(a_init));

        // you can't read a non-existent conversation
        match wa!(client.leave_groups(a_uid, iter(vec![cid]))) {
            leave_groups::Res::Missing(missing) => {
                assert_eq!(missing, cid);
            }
            _ => panic!(),
        };

        // but now it should succeed
        assert_eq!(wa!(client.init_group(a_uid, cid)), init_group::Res::Success);
        assert!(wa!(client.user_in_group(a_uid, cid)));

        // a group can't initialized twice
        match wa!(client.init_group(a_uid, cid)) {
            init_group::Res::GroupAlreadyExists(conv) => {
                assert_eq!(cid, conv);
            }
            _ => panic!(),
        };

        let b_uid: UserId = "b".try_into().expect(womp!());
        let b_kp = sig::KeyPair::gen_new();
        let b_init = b_kp.sign(b_uid);

        let c_uid: UserId = "c".try_into().expect(womp!());

        let c_kp = sig::KeyPair::gen_new();
        let c_init = c_kp.sign(c_uid);

        let uids = vec![b_uid, c_uid];

        // non-existent users can't be added
        match wa!(client.add_to_group(a_uid, iter(uids.clone()), cid)) {
            add_to_group::Res::MissingUser(_) => {}
            _ => panic!(),
        };

        assert!(!wa!(client.user_in_group(b_uid, cid)));
        assert!(!wa!(client.user_in_group(c_uid, cid)));

        wa!(client.new_user(b_init));
        wa!(client.new_user(c_init));

        assert_eq!(
            wa!(client.add_to_group(a_uid, iter(uids.clone()), cid)),
            add_to_group::Res::Success
        );
        assert!(wa!(client.user_in_group(b_uid, cid)));
        assert!(wa!(client.user_in_group(c_uid, cid)));

        match wa!(client.leave_groups(a_uid, iter(vec![cid]))) {
            leave_groups::Res::Success => {}
            _ => panic!(),
        };

        assert!(!wa!(client.user_in_group(a_uid, cid)));
    }
}
