use crate::db::Database;
use crate::errors::HErr;
use herald_common::{sign, Signed};
use rusqlite::params;

pub(crate) fn add_key(k: Signed<sign::PublicKey>) -> Result<(), HErr> {
    let (key, meta) = k.split();
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/add_key.sql"))?;

    stmt.execute(params![
        key.as_ref(),
        meta.signed_by().as_ref(),
        meta.timestamp().timestamp(),
        meta.sig().as_ref()
    ])?;
    Ok(())
}

pub(crate) fn key_deprecated(k: Signed<sign::PublicKey>) -> Result<(), HErr> {
    let (key, meta) = k.split();
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/deprecate_key.sql"))?;

    stmt.execute(params![
        key.as_ref(),
        meta.signed_by().as_ref(),
        meta.timestamp().timestamp(),
        meta.sig().as_ref()
    ])?;

    Ok(())
}

#[cfg(test)]
mod tests;
