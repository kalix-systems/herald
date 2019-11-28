use bytes::*;
use coremacros::{abort_err, from_fn};
use coretypes::ids::ConversationId;
use herald_common::*;
use lazy_static::*;
use parking_lot::Mutex;
use platform_dirs::DB_DIR;
use rusqlite::params;

#[derive(Debug)]
pub enum ChainKeysError {
    Db(rusqlite::Error),
    DecryptionFailed,
    StoreCorrupted,
    Kson(KsonError),
    NoneError(Location),
}

from_fn!(ChainKeysError, rusqlite::Error, ChainKeysError::Db);

// FIXME initialization can be done more cleanly
// A lot of this is redundant
lazy_static! {
    static ref CK_CONN: Mutex<rusqlite::Connection> = {
        let path = DB_DIR.join("ck.sqlite3");
        let mut conn = abort_err!(rusqlite::Connection::open(path));
        let tx = abort_err!(conn.transaction());
        abort_err!(tx.execute_batch(include_str!("sql/create.sql")));
        abort_err!(tx.commit());
        Mutex::new(conn)
    };
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
    cipher: kdf_ratchet::Cipher,
) -> Result<Option<Decrypted>, ChainKeysError> {
    db::with_tx(move |tx| tx.open_msg(cid, cipher))
}

pub fn seal_msg(
    cid: ConversationId,
    ad: Bytes,
    msg: BytesMut,
) -> Result<kdf_ratchet::Cipher, ChainKeysError> {
    db::with_tx(move |tx| tx.seal_msg(cid, ad, msg))
}

pub fn store_state(
    cid: ConversationId,
    ratchet: &kdf_ratchet::RatchetState,
) -> Result<(), ChainKeysError> {
    db::with_tx(move |tx| {
        tx.store_ratchet_state(cid, ratchet)?;
        Ok(())
    })
}
