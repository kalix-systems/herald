use super::*;
impl Conn {
    pub async fn add_to_group<Users: Stream<Item = UserId> + Send + Unpin>(
        &mut self,
        users: Users,
        conv: ConversationId,
    ) -> Res<add_to_group::Res> {
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
    ) -> Res<leave_groups::Res> {
        let leave_stmt = self
            .prepare_typed(sql!("leave_group"), types![TEXT, BYTEA])
            .await?;

        let exists_stmt = self
            .prepare_typed(sql!("group_exists"), types![BYTEA])
            .await?;

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
}
