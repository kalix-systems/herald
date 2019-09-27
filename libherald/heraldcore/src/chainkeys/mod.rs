use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use chainmail::{block::*, errors::Error as ChainError};
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

#[derive(Default)]
pub(crate) struct ChainKeys {
    db: Database,
}

impl DBTable for ChainKeys {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("../sql/chainkeys/create_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("../sql/chainkeys/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("../sql/chainkeys/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("../sql/chainkeys/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("../sql/chainkeys/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

fn store_key(db: &rusqlite::Connection, hash: BlockHash, key: ChainKey) -> Result<(), HErr> {
    db.execute(
        include_str!("../sql/chainkeys/store_key.sql"),
        params![hash.as_ref(), key.as_ref()],
    )?;

    Ok(())
}

fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    tx: rusqlite::Transaction,
    blocks: I,
) -> Result<(), ChainError> {
    // references to transaction needs to be dropped before committing
    {
        let mut mark_stmt = tx
            .prepare(include_str!("../sql/chainkeys/mark_used.sql"))
            .map_err(|_| ChainError::BlockStoreUnavailable)?;

        let mut key_used_stmt = tx
            .prepare(include_str!("../sql/chainkeys/get_key_used_status.sql"))
            .map_err(|_| ChainError::BlockStoreUnavailable)?;

        for block in blocks {
            if key_used_stmt
                .query_row(params![block.as_ref()], |row| Ok(row.get::<_, bool>(0)?))
                // return a `MissingKeys` error if the query result is empty
                .map_err(|_| ChainError::MissingKeys)?
            {
                // if the key is already marked used, return a `RedundantMark` error
                return Err(ChainError::RedundantMark);
            }

            // otherwise we can mark the key as unused
            mark_stmt
                .execute(params![block.as_ref()])
                .map_err(|_| ChainError::BlockStoreUnavailable)?;
        }
    }

    tx.commit().map_err(|_| ChainError::BlockStoreUnavailable)?;

    Ok(())
}

fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
    db: &rusqlite::Connection,
    blocks: I,
) -> Option<BTreeSet<ChainKey>> {
    let mut stmt = db
        .prepare(include_str!("../sql/chainkeys/get_keys.sql"))
        .ok()?;

    let mut keys: BTreeSet<ChainKey> = BTreeSet::new();

    for block in blocks.map(|block| block.as_ref()) {
        let key: Vec<u8> = stmt.query_row(params![block], |row| row.get(0)).ok()?;
        keys.insert(ChainKey::from_slice(key.as_slice())?);
    }

    Some(keys)
}

// this should *not* mark keys as used
fn get_unused(db: &rusqlite::Connection) -> Result<Vec<(BlockHash, ChainKey)>, ChainError> {
    let mut stmt = db
        .prepare(include_str!("../sql/chainkeys/get_unused.sql"))
        .map_err(|_| ChainError::BlockStoreUnavailable)?;

    let results = stmt
        .query_map(NO_PARAMS, |row| {
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

impl BlockStore for ChainKeys {
    // stores a key, does not mark key as used
    fn store_key(&mut self, hash: BlockHash, key: ChainKey) -> Result<(), ChainError> {
        store_key(&self.db, hash, key).map_err(|_| ChainError::BlockStoreUnavailable)
    }

    // we'll want to implement some kind of gc strategy to collect keys marked used
    // if they're less than an hour old we should keep them, otherwise delete
    // could run on a schedule, or every time we call get_unused, or something else
    fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
        &mut self,
        blocks: I,
    ) -> Result<(), ChainError> {
        // do this all in a transaction
        let tx = self
            .db
            .transaction()
            .map_err(|_| ChainError::BlockStoreUnavailable)?;

        mark_used(tx, blocks)
    }

    // this should *not* mark keys as used
    fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Option<BTreeSet<ChainKey>> {
        get_keys(&self.db, blocks)
    }

    // this should *not* mark keys as used
    fn get_unused(&self) -> Result<Vec<(BlockHash, ChainKey)>, ChainError> {
        get_unused(&self.db)
    }
}

#[cfg(test)]
mod tests;