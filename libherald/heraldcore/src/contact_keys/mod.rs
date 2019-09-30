use crate::db::Database;
use crate::errors::HErr;
use herald_common::{sign, Signed, UserId};
use rusqlite::params;

pub(crate) fn add_keys(uid: UserId, keys: &[Signed<sign::PublicKey>]) -> Result<(), HErr> {
    let mut db = Database::get()?;

    let tx = db.transaction()?;

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
                meta.timestamp().timestamp(),
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub(crate) fn get_valid_keys(uid: UserId) -> Result<Vec<sign::PublicKey>, HErr> {
    unimplemented!();
}

pub(crate) fn get_deprecated_keys(uid: UserId) -> Result<Vec<sign::PublicKey>, HErr> {
    unimplemented!();
}

pub(crate) fn deprecate_keys(uid: UserId, keys: &[Signed<sign::PublicKey>]) -> Result<(), HErr> {
    let mut db = Database::get()?;
    let tx = db.transaction()?;

    {
        let mut stmt = tx.prepare(include_str!("sql/deprecate_key.sql"))?;

        for k in keys {
            let (key, meta) = k.split();

            stmt.execute(params![
                key.as_ref(),
                meta.signed_by().as_ref(),
                meta.sig().as_ref(),
                meta.timestamp().timestamp()
            ])?;
        }
    }

    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests;
