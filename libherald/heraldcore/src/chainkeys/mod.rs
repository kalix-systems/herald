use crate::types::ConversationId;
use crate::{db::Database, errors::HErr};
use chainmail::{block::*, errors::Error as ChainError};
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

fn store_key(
    db: &rusqlite::Connection,
    cid: ConversationId,
    hash: BlockHash,
    key: ChainKey,
) -> Result<(), HErr> {
    db.execute(
        include_str!("sql/store_key.sql"),
        params![cid, hash.as_ref(), key.as_ref()],
    )
    .unwrap();

    Ok(())
}

fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    tx: rusqlite::Transaction,
    cid: ConversationId,
    blocks: I,
) -> Result<(), ChainError> {
    // references to transaction need to be dropped before committing
    {
        let mut mark_stmt = tx
            .prepare(include_str!("sql/mark_used.sql"))
            .map_err(|_| ChainError::BlockStoreUnavailable)?;

        let mut key_used_stmt = tx
            .prepare(include_str!("sql/get_key_used_status.sql"))
            .map_err(|_| ChainError::BlockStoreUnavailable)?;

        for block in blocks {
            if key_used_stmt
                .query_row(params![cid, block.as_ref()], |row| {
                    Ok(row.get::<_, bool>(0)?)
                })
                // return a `MissingKeys` error if the query result is empty
                .map_err(|_| ChainError::MissingKeys)?
            {
                // if the key is already marked used, return a `RedundantMark` error
                return Err(ChainError::RedundantMark);
            }

            // otherwise we can mark the key as used
            mark_stmt
                .execute(params![cid, block.as_ref()])
                .map_err(|_| ChainError::BlockStoreUnavailable)?;
        }
    }

    tx.commit().map_err(|_| ChainError::BlockStoreUnavailable)?;

    Ok(())
}

fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
    db: &rusqlite::Connection,
    cid: ConversationId,
    blocks: I,
) -> Option<BTreeSet<ChainKey>> {
    let mut stmt = db.prepare(include_str!("sql/get_keys.sql")).ok()?;

    let mut keys: BTreeSet<ChainKey> = BTreeSet::new();

    for block in blocks.map(|block| block.as_ref()) {
        let key: Vec<u8> = stmt.query_row(params![cid, block], |row| row.get(0)).ok()?;
        keys.insert(ChainKey::from_slice(key.as_slice())?);
    }

    Some(keys)
}

// this should *not* mark keys as used
fn get_unused(
    db: &rusqlite::Connection,
    cid: ConversationId,
) -> Result<Vec<(BlockHash, ChainKey)>, ChainError> {
    let mut stmt = db
        .prepare(include_str!("sql/get_unused.sql"))
        .map_err(|_| ChainError::BlockStoreUnavailable)?;

    let results = stmt
        .query_map(params![cid], |row| {
            Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
        })
        .map_err(|_| ChainError::BlockStoreCorrupted)?;

    let mut pairs = vec![];

    for res in results {
        let (raw_hash, raw_key) = res.map_err(|_| ChainError::MissingKeys)?;
        pairs.push((
            BlockHash::from_slice(raw_hash.as_slice()).ok_or(ChainError::BlockStoreCorrupted)?,
            ChainKey::from_slice(raw_key.as_slice()).ok_or(ChainError::BlockStoreCorrupted)?,
        ));
    }

    Ok(pairs)
}

impl BlockStore for ConversationId {
    // stores a key, does not mark key as used
    fn store_key(&mut self, hash: BlockHash, key: ChainKey) -> Result<(), ChainError> {
        let db = Database::get().map_err(|_| ChainError::BlockStoreUnavailable)?;
        store_key(&db, *self, hash, key).map_err(|_| ChainError::BlockStoreUnavailable)
    }

    // we'll want to implement some kind of gc strategy to collect keys marked used
    // if they're less than an hour old we should keep them, otherwise delete
    // could run on a schedule, or every time we call get_unused, or something else
    fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
        &mut self,
        blocks: I,
    ) -> Result<(), ChainError> {
        let mut db = Database::get().map_err(|_| ChainError::BlockStoreUnavailable)?;
        // do this all in a transaction
        let tx = db
            .transaction()
            .map_err(|_| ChainError::BlockStoreUnavailable)?;

        mark_used(tx, *self, blocks)
    }

    // this should *not* mark keys as used
    fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Option<BTreeSet<ChainKey>> {
        let db = Database::get().ok()?;
        get_keys(&db, *self, blocks)
    }

    // this should *not* mark keys as used
    fn get_unused(&self) -> Result<Vec<(BlockHash, ChainKey)>, ChainError> {
        let db = Database::get().map_err(|_| ChainError::BlockStoreUnavailable)?;
        get_unused(&db, *self)
    }
}

#[cfg(test)]
mod tests;
