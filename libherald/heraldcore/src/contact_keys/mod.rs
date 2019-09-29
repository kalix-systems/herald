use crate::db::Database;
use crate::errors::HErr;
use herald_common::{sign, Signed, UserId};
use rusqlite::params;

pub(crate) fn add_keys(uid: UserId, keys: &[Signed<sign::PublicKey>]) -> Result<(), HErr> {
    let mut db = Database::get()?;

    let tx = db.transaction()?;

    // drop reference to transaction before commiting
    {
        let mut stmt = tx.prepare(include_str!("sql/add_key.sql"))?;

        for k in keys {
            let (key, meta) = k.split();
            stmt.execute(params![
                uid,
                key.as_ref(),
                meta.signed_by().as_ref(),
                meta.timestamp().timestamp(),
                meta.sig().as_ref()
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub(crate) fn deprecate_keys(uid: UserId, keys: &[Signed<sign::PublicKey>]) -> Result<(), HErr> {
    let mut db = Database::get()?;
    let tx = db.transaction()?;

    let mut stmt = tx.prepare(include_str!("sql/deprecate_key.sql"))?;

    for k in keys {
        let (key, meta) = k.split();

        stmt.execute(params![
            uid,
            key.as_ref(),
            meta.signed_by().as_ref(),
            meta.timestamp().timestamp(),
            meta.sig().as_ref()
        ])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
