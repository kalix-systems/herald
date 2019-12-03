use crate::{db::Database, errors::HErr, NE};
use herald_common::{sig, SigValid, Signed, UserId};
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

pub(crate) fn guard_sig_valid<T: AsRef<[u8]>>(
    uid: UserId,
    sig: &Signed<T>,
) -> Result<(), HErr> {
    match sig.verify_sig() {
        SigValid::Yes => {
            let u_signed_by =
                db::get_user_by_key(Database::get()?.deref_mut(), sig.signed_by())?.ok_or(NE!())?;

            if uid == u_signed_by {
                Ok(())
            } else {
                Err(HErr::HeraldError(format!(
                    "invalid signature - expected signature by {}, found {}",
                    uid, u_signed_by
                )))
            }
        }
        f => Err(HErr::HeraldError(format!("invalid signature {:#?}", f))),
    }
}

#[cfg(test)]
mod tests;
