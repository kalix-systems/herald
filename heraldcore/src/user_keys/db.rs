use super::*;

pub(crate) fn add_keys(
    conn: &mut rusqlite::Connection,
    uid: UserId,
    keys: &[Signed<sign::PublicKey>],
) -> Result<(), HErr> {
    let tx = conn.transaction()?;

    // drop reference to transaction before commiting
    {
        let mut user_keys_stmt = tx.prepare(include_str!("sql/add_to_user_keys.sql"))?;

        let mut key_creations_stmt = tx.prepare(include_str!("sql/add_key.sql"))?;

        for k in keys {
            let (key, meta) = k.split();

            user_keys_stmt.execute(params![key.as_ref(), &uid])?;

            key_creations_stmt.execute(params![
                key.as_ref(),
                meta.signed_by().as_ref(),
                meta.sig().as_ref(),
                meta.timestamp(),
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub(crate) fn get_valid_keys(
    conn: &rusqlite::Connection,
    uid: UserId,
) -> Result<Vec<sign::PublicKey>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/valid_keys.sql"))?;

    let res = stmt.query_map(params![uid], |row| Ok(row.get::<_, Vec<u8>>(0)?))?;

    res.map(|res| {
        sign::PublicKey::from_slice(res?.as_slice())
            .ok_or_else(|| HErr::HeraldError("Invalid key".into()))
    })
    .collect()
}

pub(crate) fn get_deprecated_keys(
    conn: &rusqlite::Connection,
    uid: UserId,
) -> Result<Vec<sign::PublicKey>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/dep_keys.sql"))?;

    let res = stmt.query_map(params![uid], |row| Ok(row.get::<_, Vec<u8>>(0)?))?;

    res.map(|res| {
        sign::PublicKey::from_slice(res?.as_slice())
            .ok_or_else(|| HErr::HeraldError("Invalid key".into()))
    })
    .collect()
}

pub(crate) fn deprecate_keys(
    conn: &mut rusqlite::Connection,
    keys: &[Signed<sign::PublicKey>],
) -> Result<(), HErr> {
    let tx = conn.transaction()?;

    {
        let mut stmt = tx.prepare(include_str!("sql/deprecate_key.sql"))?;

        for k in keys {
            let (key, meta) = k.split();

            stmt.execute(params![
                key.as_ref(),
                meta.signed_by().as_ref(),
                meta.sig().as_ref(),
                meta.timestamp()
            ])?;
        }
    }

    tx.commit()?;
    Ok(())
}
