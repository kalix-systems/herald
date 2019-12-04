use bytes::*;
use coremacros::{abort_err, from_fn};
use coretypes::ids::ConversationId;
use herald_common::*;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use platform_dirs::db_dir;

#[derive(Debug)]
pub enum ChainKeysError {
    Db(rusqlite::Error),
    DecryptionFailed,
    StoreCorrupted,
    Kson(KsonError),
    NoneError(location::Location),
}

from_fn!(ChainKeysError, rusqlite::Error, ChainKeysError::Db);

static CK_CONN: OnceCell<Mutex<rusqlite::Connection>> = OnceCell::new();

fn ck_conn() -> &'static Mutex<rusqlite::Connection> {
    CK_CONN.get_or_init(|| {
        // FIXME initialization can be done more cleanly
        // A lot of this is redundant
        kcl::init();

        let path = db_dir().join("ck.sqlite3");
        let mut conn = abort_err!(rusqlite::Connection::open(path));
        let tx = abort_err!(conn.transaction());

        abort_err!(tx.execute_batch(include_str!("sql/create.sql")));
        abort_err!(tx.commit());

        Mutex::new(conn)
    })
}

pub mod db;
#[cfg(test)]
mod tests;

pub struct Decrypted {
    pub ad: Bytes,
    pub pt: BytesMut,
}

pub fn open_msg(
    cid: ConversationId,
    pk: sig::PublicKey,
    gen: u32,
    cipher: kdf_ratchet::Cipher,
) -> Result<Option<Decrypted>, ChainKeysError> {
    db::with_tx(move |tx| tx.open_msg(cid, pk, gen, cipher))
}

pub fn store_state(
    cid: ConversationId,
    pk: sig::PublicKey,
    gen: u32,
    ratchet: &kdf_ratchet::RatchetState,
) -> Result<(), ChainKeysError> {
    db::with_tx(|tx| {
        tx.store_ratchet_state(cid, pk, gen, ratchet)?;
        Ok(())
    })
}

pub fn store_new_state(
    cid: ConversationId,
    pk: sig::PublicKey,
    gen: u32,
    ratchet: &kdf_ratchet::RatchetState,
) -> Result<(), ChainKeysError> {
    db::with_tx(|tx| {
        tx.store_ratchet_state(cid, pk, gen, ratchet)?;
        tx.deprecate_before(cid, pk, gen)?;
        Ok(())
    })
}

pub fn deprecate_all(pk: sig::PublicKey) -> Result<(), ChainKeysError> {
    db::with_tx(|tx| {
        tx.deprecate_all(pk)?;
        Ok(())
    })
}

pub fn deprecate_all_in_convo(
    cid: ConversationId,
    pk: sig::PublicKey,
) -> Result<(), ChainKeysError> {
    db::with_tx(|tx| {
        tx.deprecate_all_in_convo(cid, pk)?;
        Ok(())
    })
}
