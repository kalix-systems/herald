use crate::{db::Database, errors::HErr, NE};
use herald_common::{sig, SigValid, Signed, UserId, UserMeta};
use rusqlite::params;
use std::ops::DerefMut;

mod db;

pub(crate) fn add_keys(
    uid: UserId,
    keys: &[Signed<sig::PublicKey>],
) -> Result<(), HErr> {
    let mut db = Database::get()?;

    db::add_keys(&mut db, uid, keys)
}

pub(crate) fn add_umeta(
    uid: UserId,
    meta: UserMeta,
) -> Result<(), HErr> {
    let mut db = Database::get()?;

    db::add_umeta(&mut db, uid, meta)
}

#[allow(unused)]
pub(crate) fn get_valid_keys(uid: UserId) -> Result<Vec<sig::PublicKey>, HErr> {
    let db = Database::get()?;
    db::get_valid_keys(&db, uid)
}

#[allow(unused)]
pub(crate) fn get_deprecated_keys(uid: UserId) -> Result<Vec<sig::PublicKey>, HErr> {
    let db = Database::get()?;
    db::get_deprecated_keys(&db, uid)
}

pub(crate) fn deprecate_keys(keys: &[Signed<sig::PublicKey>]) -> Result<(), HErr> {
    let mut db = Database::get()?;
    db::deprecate_keys(&mut db, keys)
}

pub(crate) fn get_user_by_key(key: &sig::PublicKey) -> Result<Option<UserId>, HErr> {
    db::get_user_by_key(Database::get()?.deref_mut(), key)
}

pub(crate) fn guard_sig_valid<T: AsRef<[u8]>>(
    uid: UserId,
    sig: &Signed<T>,
    loc: location::Location,
) -> Result<(), HErr> {
    match sig.verify_sig() {
        SigValid::Yes => {
            let u_signed_by =
                db::get_user_by_key(Database::get()?.deref_mut(), sig.signed_by())?.ok_or(NE!())?;

            if uid == u_signed_by {
                Ok(())
            } else {
                Err(HErr::HeraldError(format!(
                    "invalid signature at {} - expected signature by {}, found {}",
                    loc, uid, u_signed_by
                )))
            }
        }
        f => Err(HErr::HeraldError(format!(
            "invalid signature at {} - error was {:#?}",
            loc, f
        ))),
    }
}

#[cfg(test)]
mod tests;
