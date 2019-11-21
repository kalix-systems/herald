use bytes::*;
use coremacros::*;
use coretypes::ids::ConversationId;
use herald_common::*;
use lazy_static::*;
use parking_lot::Mutex;
use platform_dirs::DB_DIR;
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

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

pub struct Decrypted {
    pub ad: Bytes,
    pub pt: BytesMut,
}

pub fn open_msg(
    cid: ConversationId,
    cipher: channel_ratchet::Cipher,
) -> Result<Option<Decrypted>, ChainKeysError> {
    db::with_tx(move |tx| {
        use channel_ratchet::DecryptionResult::*;

        let res = if let Some(k) = tx.get_derived_key(cid, cipher.ix) {
            let r0 = cipher.open_with(k);
            if let Success { .. } = &r0 {
                tx.mark_used(cid, cipher.ix)?;
            }
            r0
        } else {
            let mut state = tx.get_ratchet_state(cid)?;
            state.open(cipher);
            tx.store_ratchet_state(cid, &state)?;
        };

        match res {
            Success { extra_keys, ad, pt } => {
                if let Some((ix, _)) = extra_keys.last() {
                    tx.mark_used(cid, *ix)?;
                }

                for (ix, key) in extra_keys {
                    tx.store_derived_key(cid, ix, key)?;
                }

                Ok(Some(Decrypted { ad, pt }))
            }
            Failed { extra_keys } => {
                for (ix, key) in extra_keys {
                    tx.store_derived_key(cid, ix, key)?;
                }

                Ok(None)
            }
            // TODO: include these fields in error msg
            IndexTooHigh { .. } => Err(ChainKeysError::StoreCorrupted),
        }
    })
}

pub fn seal_msg(
    cid: ConversationId,
    ad: Bytes,
    msg: BytesMut,
) -> Result<Cipher, ChainKeysError> {
    db::with_tx(move |tx| {
        let mut ratchet = tx.get_ratchet_state(cid)?;
        let (ix, key, cipher) = ratchet.seal(ad, msg).destruct();
        tx.store_derived_key(cid, ix, key)?;
        tx.mark_used(cid, ix)?;
        tx.store_ratchet_state(cid, &ratchet)?;
        Ok(cipher)
    })
}
