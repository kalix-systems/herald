use crate::{db::Database, errors::HErr, types::ConversationId};
use chainmail::{block::*, errors::ChainError};
use rusqlite::params;
use std::collections::BTreeSet;

fn store_key(
    db: &rusqlite::Connection,
    cid: ConversationId,
    hash: BlockHash,
    key: ChainKey,
) -> Result<Vec<Block>, HErr> {
    db.execute(
        include_str!("sql/store_key.sql"),
        params![cid, hash.as_ref(), key.as_ref()],
    )?;

    // TODO return real vector
    Ok(vec![])
}

fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    tx: rusqlite::Transaction,
    cid: ConversationId,
    blocks: I,
) -> Result<(), HErr> {
    // references to transaction need to be dropped before committing
    {
        let mut mark_stmt = tx.prepare(include_str!("sql/mark_used.sql"))?;

        for block in blocks {
            mark_stmt.execute(params![cid, block.as_ref()])?;
        }
    }

    tx.commit()?;

    Ok(())
}

fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
    db: &rusqlite::Connection,
    cid: ConversationId,
    blocks: I,
) -> Result<FoundKeys, HErr> {
    let mut stmt = db.prepare(include_str!("sql/get_keys.sql"))?;

    let mut keys: BTreeSet<ChainKey> = BTreeSet::new();
    let mut missing: Vec<BlockHash> = Vec::new();

    for block in blocks.copied() {
        match stmt.query_row(params![cid, block.as_ref()], |row| {
            row.get::<_, Option<Vec<u8>>>(0)
        })? {
            Some(k) => {
                keys.insert(ChainKey::from_slice(k.as_slice()).unwrap());
            }
            None => {
                missing.push(block);
            }
        }
    }

    Ok(if !missing.is_empty() {
        FoundKeys::Missing(missing)
    } else {
        FoundKeys::Found(keys)
    })
}

// this should *not* mark keys as used
fn get_unused(
    db: &rusqlite::Connection,
    cid: ConversationId,
) -> Result<Vec<(BlockHash, ChainKey)>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/get_unused.sql"))?;

    let results = stmt.query_map(params![cid], |row| {
        Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
    })?;

    let mut pairs = vec![];

    for res in results {
        let (raw_hash, raw_key) = res?;
        pairs.push((
            BlockHash::from_slice(raw_hash.as_slice()).ok_or(ChainError::BlockStoreCorrupted)?,
            ChainKey::from_slice(raw_key.as_slice()).ok_or(ChainError::BlockStoreCorrupted)?,
        ));
    }

    Ok(pairs)
}

impl BlockStore for ConversationId {
    type Error = HErr;

    // stores a key, does not mark key as used
    fn store_key(&mut self, hash: BlockHash, key: ChainKey) -> Result<Vec<Block>, Self::Error> {
        let db = Database::get()?;
        // modify this to remove missing block dependencies
        store_key(&db, *self, hash, key)
        // get all blocks that no longer have dependencies
    }

    // we'll want to implement some kind of gc strategy to collect keys marked used
    // if they're less than an hour old we should keep them, otherwise delete
    // could run on a schedule, or every time we call get_unused, or something else
    fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
        &mut self,
        blocks: I,
    ) -> Result<(), Self::Error> {
        let mut db = Database::get()?;
        // do this all in a transaction
        let tx = db.transaction()?;

        mark_used(tx, *self, blocks)
    }

    // this should *not* mark keys as used
    fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Result<FoundKeys, Self::Error> {
        let db = Database::get()?;
        get_keys(&db, *self, blocks)
    }

    // this should *not* mark keys as used
    fn get_unused(&self) -> Result<Vec<(BlockHash, ChainKey)>, HErr> {
        let db = Database::get()?;
        get_unused(&db, *self)
    }

    fn add_pending(&self, _block: Block, _awaiting: Vec<BlockHash>) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests;
