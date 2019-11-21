use crate::{db::Database, errors::HErr};
use herald_common::{sign, Signed, UserId};
use rusqlite::params;

mod db;

pub(crate) fn add_keys(
    uid: UserId,
    keys: &[Signed<sign::PublicKey>],
) -> Result<(), HErr> {
    let mut db = Database::get()?;

    db::add_keys(&mut db, uid, keys)
}

#[allow(unused)]
pub(crate) fn get_valid_keys(uid: UserId) -> Result<Vec<sign::PublicKey>, HErr> {
    let db = Database::get()?;
    db::get_valid_keys(&db, uid)
}

#[allow(unused)]
pub(crate) fn get_deprecated_keys(uid: UserId) -> Result<Vec<sign::PublicKey>, HErr> {
    let db = Database::get()?;
    db::get_deprecated_keys(&db, uid)
}

pub(crate) fn deprecate_keys(keys: &[Signed<sign::PublicKey>]) -> Result<(), HErr> {
    let mut db = Database::get()?;
    db::deprecate_keys(&mut db, keys)
}

#[cfg(test)]
mod tests;
