#[macro_use]
extern crate coremacros;

#[macro_use]
extern crate location;

use chainmail::block::*;
pub use chainmail::errors::ChainError as ChainMailError;
use coretypes::ids::ConversationId;
use herald_common::GlobalId;
use lazy_static::*;
use location::Location;
use parking_lot::Mutex;
use platform_dirs::DB_DIR;
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum ChainKeysError {
    Db(rusqlite::Error),
    Chain(ChainMailError),
    Deserialization(Location),
    Serialization(Location),
    NoneError(Location),
}

from_fn!(ChainKeysError, rusqlite::Error, ChainKeysError::Db);
from_fn!(ChainKeysError, ChainMailError, ChainKeysError::Chain);

// FIXME initialization can be done more cleanly
// A lot of this is redundant
lazy_static! {
    pub static ref CK_CONN: Mutex<rusqlite::Connection> = {
        let path = DB_DIR.join("ck.sqlite3");
        let mut conn = abort_err!(rusqlite::Connection::open(path));
        let tx = abort_err!(conn.transaction());
        abort_err!(tx.execute_batch(include_str!("sql/create.sql")));
        abort_err!(tx.commit());
        Mutex::new(conn)
    };
}

pub mod db;

pub enum FoundKeys {
    Found(BTreeSet<ChainKey>),
    Missing(Vec<BlockHash>),
}

pub enum DecryptionResult {
    Success(Vec<u8>, Vec<(Block, GlobalId)>),
    Pending,
}

type RawBlock = Vec<u8>;
type RawSigner = Vec<u8>;

#[cfg(test)]
pub(crate) fn store_key(
    cid: &ConversationId,
    hash: BlockHash,
    key: &ChainKey,
) -> Result<Vec<(Block, GlobalId)>, ChainKeysError> {
    let mut db = CK_CONN.lock();
    let mut tx = db.transaction()?;
    let blocks = db::store_key(&mut tx, *cid, hash, key)?;
    tx.commit()?;

    Ok(blocks)
}

// TODO GC strategy
#[cfg(test)]
pub(crate) fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    cid: &ConversationId,
    blocks: I,
) -> Result<(), rusqlite::Error> {
    let mut db = CK_CONN.lock();
    let mut tx = db.transaction()?;

    db::mark_used(&mut tx, *cid, blocks)?;
    tx.commit()?;
    Ok(())
}

#[allow(unused)]
// TODO use this
pub(crate) fn mark_unused(
    cid: &ConversationId,
    blocks: &BTreeSet<BlockHash>,
) -> Result<(), rusqlite::Error> {
    let mut db = CK_CONN.lock();
    let mut tx = db.transaction()?;

    db::mark_unused(&mut tx, *cid, blocks)?;
    tx.commit()?;
    Ok(())
}

pub fn get_channel_key(cid: &ConversationId) -> Result<ChannelKey, ChainKeysError> {
    let db = CK_CONN.lock();
    db::get_channel_key(&db, *cid)
}

pub fn get_unused(cid: &ConversationId) -> Result<Vec<(BlockHash, ChainKey)>, ChainKeysError> {
    let db = CK_CONN.lock();
    db::get_unused(&db, *cid)
}

pub fn open_block(
    cid: &ConversationId,
    signer: &GlobalId,
    block: Block,
) -> Result<DecryptionResult, ChainKeysError> {
    let hashes = block.parent_hashes().clone();

    let mut db = CK_CONN.lock();
    let mut tx = db.transaction()?;

    // TODO: consider storing pending for these too?
    let channel_key = db::get_channel_key(&tx, *cid)?;
    let res = match db::get_keys(&tx, *cid, block.parent_hashes().iter())? {
        FoundKeys::Found(parent_keys) => {
            let OpenData { msg, hash, key } =
                block.open(&channel_key, &signer.did, &parent_keys)?;
            let unlocked = db::store_key(&mut tx, *cid, hash, &key)?;
            db::mark_used(&mut tx, *cid, hashes.iter())?;
            DecryptionResult::Success(msg, unlocked)
        }
        FoundKeys::Missing(missing_keys) => {
            db::add_pending(&mut tx, signer, &block, &missing_keys)?;
            DecryptionResult::Pending
        }
    };

    tx.commit()?;

    Ok(res)
}

pub fn store_genesis(
    cid: &ConversationId,
    gen: &Genesis,
) -> Result<Vec<(Block, GlobalId)>, ChainKeysError> {
    let hash = gen.compute_hash().ok_or(ChainMailError::CryptoError)?;

    let mut db = CK_CONN.lock();
    let mut tx = db.transaction()?;

    db::store_channel_key(&mut tx, *cid, gen.channel_key())?;
    let out = db::store_key(&mut tx, *cid, hash, gen.root())?;

    tx.commit()?;

    Ok(out)
}

#[cfg(test)]
mod tests;
