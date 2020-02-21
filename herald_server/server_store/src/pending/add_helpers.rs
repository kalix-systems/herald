use super::*;
use tokio_postgres::Transaction as Tx;

pub(crate) async fn one_key(
    tx: &mut Tx<'_>,
    key: &sig::PublicKey,
    msg: &Bytes,
    tag: PushTag,
    timestamp: Time,
    GlobalId { uid, did }: GlobalId,
) -> Res<PushedTo> {
    if *key == did {
        return Ok(PushedTo::NoRecipients);
    }

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
        .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8, TEXT, BYTEA])
        .await?;

    let push_row_id: i64 = tx
        .query_one(
            &push_stmt,
            params![
                msg.as_ref(),
                kson::to_vec(&tag),
                timestamp.as_i64(),
                uid.as_str(),
                did.as_ref()
            ],
        )
        .await?
        .get(0);

    let pending_stmt = tx
        .prepare_typed(sql!("add_pending"), types![BYTEA, INT8])
        .await?;

    tx.execute(&pending_stmt, params![key.as_ref(), push_row_id])
        .await?;

    Ok(PushedTo::PushedTo {
        devs: vec![*key],
        push_id: push_row_id,
    })
}

pub(crate) async fn one_user(
    tx: &mut Tx<'_>,
    uid: &UserId,
    msg: &Bytes,
    tag: PushTag,
    timestamp: Time,
    GlobalId { uid: from, did }: GlobalId,
) -> Res<PushedTo> {
    let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

    if !tx
        .query_one(&exists_stmt, params![uid.as_str()])
        .await?
        .get::<_, bool>(0)
    {
        return Ok(PushedTo::Missing(SingleRecip::User(*uid)));
    }

    let push_stmt = tx
        .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8, TEXT, BYTEA])
        .await?;

    let push_row_id: i64 = tx
        .query_one(
            &push_stmt,
            params![
                msg.as_ref(),
                kson::to_vec(&tag),
                timestamp.as_i64(),
                from.as_str(),
                did.as_ref()
            ],
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
                let key = sig::PublicKey::from_slice(&key).ok_or(Error::InvalidKey)?;

                if key == did {
                    return Ok(());
                }

                tx.execute(pending_stmt, params![key.as_ref(), push_row_id])
                    .await?;
                pushed_to.lock().await.push(key);
                Ok(())
            }
        })
        .await?;

    Ok(PushedTo::PushedTo {
        devs: pushed_to.into_inner(),
        push_id: push_row_id,
    })
}

pub(crate) async fn many_users(
    tx: &mut Tx<'_>,
    uids: &[UserId],
    msg: &Bytes,
    tag: PushTag,
    timestamp: Time,
    GlobalId { uid, did }: GlobalId,
) -> Res<PushedTo> {
    let push_stmt = tx
        .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8, TEXT, BYTEA])
        .await?;

    let push_row_id: i64 = tx
        .query_one(
            &push_stmt,
            params![
                msg.as_ref(),
                kson::to_vec(&tag),
                timestamp.as_i64(),
                uid.as_str(),
                did.as_ref()
            ],
        )
        .await?
        .get(0);

    let pushed_to = Mutex::new(Vec::new());

    let (keys_stmt, pending_stmt, exists_stmt) = try_join!(
        tx.prepare_typed(sql!("valid_user_keys"), types![TEXT]),
        tx.prepare_typed(sql!("add_pending"), types![BYTEA, INT8]),
        tx.prepare_typed(sql!("user_exists"), types![TEXT]),
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
                    let key = sig::PublicKey::from_slice(&key).ok_or(Error::InvalidKey)?;
                    if key == did {
                        return Ok(());
                    }
                    tx.execute(pending_stmt, params![key.as_ref(), push_row_id])
                        .await?;

                    pushed_to.lock().await.push(key);

                    Ok(())
                }
            })
            .await?;
    }

    Ok(PushedTo::PushedTo {
        devs: pushed_to.into_inner(),
        push_id: push_row_id,
    })
}

pub(crate) async fn many_keys(
    tx: &mut Tx<'_>,
    keys: &[sig::PublicKey],
    msg: &Bytes,
    tag: PushTag,
    timestamp: Time,
    GlobalId { uid, did }: GlobalId,
) -> Res<PushedTo> {
    let push_stmt = tx
        .prepare_typed(sql!("add_push"), types![BYTEA, BYTEA, INT8, TEXT, BYTEA])
        .await?;

    let push_row_id: i64 = tx
        .query_one(
            &push_stmt,
            params![
                msg.as_ref(),
                kson::to_vec(&tag),
                timestamp.as_i64(),
                uid.as_str(),
                did.as_ref()
            ],
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
        if *key == did {
            continue;
        }

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

    Ok(PushedTo::PushedTo {
        devs: pushed_to,
        push_id: push_row_id,
    })
}
