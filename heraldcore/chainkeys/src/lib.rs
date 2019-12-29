use bytes::*;
use coremacros::{exit_err, from_fn};
use herald_common::*;
use herald_ids::ConversationId;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use platform_dirs::db_dir;
use rusqlite::params;

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
        let mut conn = exit_err!(rusqlite::Connection::open(path));
        let tx = exit_err!(conn.transaction());

        exit_err!(tx.execute_batch(include_str!("sql/create.sql")));
        exit_err!(tx.commit());

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
    cipher: channel_ratchet::Cipher,
) -> Result<Option<Decrypted>, ChainKeysError> {
    db::with_tx(move |tx| tx.open_msg(cid, cipher))
}

pub fn seal_msg(
    cid: ConversationId,
    ad: Bytes,
    msg: BytesMut,
) -> Result<channel_ratchet::Cipher, ChainKeysError> {
    db::with_tx(move |tx| tx.seal_msg(cid, ad, msg))
}

pub fn store_state(
    cid: ConversationId,
    ratchet: &channel_ratchet::RatchetState,
) -> Result<(), ChainKeysError> {
    db::with_tx(move |tx| {
        tx.store_ratchet_state(cid, ratchet)?;
        Ok(())
    })
}
