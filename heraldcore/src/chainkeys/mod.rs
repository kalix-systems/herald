use crate::{db::Database, errors::HErr, types::ConversationId};
use chainmail::{block::*, errors::ChainError};
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

fn store_key(
    tx: &rusqlite::Transaction,
    cid: ConversationId,
    hash: BlockHash,
    key: ChainKey,
) -> Result<Vec<Block>, HErr> {
    // store key
    let mut store_stmt = tx.prepare(include_str!("sql/store_key.sql"))?;
    store_stmt.execute(params![cid, hash.as_ref(), key.as_ref()])?;

    // remove key as blocking dependency
    let mut remove_deps_stmt = tx.prepare(include_str!("sql/remove_block_dependencies.sql"))?;
    remove_deps_stmt.execute(params![hash.as_ref()])?;

    // get blocks that are now available
    let mut get_blocks_stmt = tx.prepare(include_str!("sql/get_unblocked_blocks.sql"))?;

    let mut blocks: Vec<Block> = Vec::new();

    for res in get_blocks_stmt.query_map(NO_PARAMS, |row| row.get::<_, Vec<u8>>(0))? {
        let block_bytes = res?;
        let block = serde_cbor::from_slice(&block_bytes)?;
        blocks.push(block);
    }

    Ok(blocks)
}

fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    tx: &rusqlite::Transaction,
    cid: ConversationId,
    blocks: I,
) -> Result<(), HErr> {
    let mut mark_stmt = tx.prepare(include_str!("sql/mark_used.sql"))?;

    for block in blocks {
        mark_stmt.execute(params![cid, block.as_ref()])?;
    }

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

    fn store_key(&mut self, hash: BlockHash, key: ChainKey) -> Result<Vec<Block>, Self::Error> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        let blocks = store_key(&tx, *self, hash, key);
        tx.commit()?;

        blocks
    }

    // TODO GC strategy
    fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
        &mut self,
        blocks: I,
    ) -> Result<(), Self::Error> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;

        mark_used(&tx, *self, blocks)?;
        tx.commit()?;
        Ok(())
    }

    fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Result<FoundKeys, Self::Error> {
        let db = Database::get()?;
        get_keys(&db, *self, blocks)
    }

    fn get_unused(&self) -> Result<Vec<(BlockHash, ChainKey)>, HErr> {
        let db = Database::get()?;
        get_unused(&db, *self)
    }

    fn add_pending(&self, block: Block, awaiting: Vec<BlockHash>) -> Result<(), Self::Error> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        let mut pending_blocks_stmt = tx.prepare(include_str!("sql/add_pending_block.sql"))?;

        let block_bytes = serde_cbor::to_vec(&block)?;

        pending_blocks_stmt.execute(params![block_bytes])?;

        let block_id = tx.last_insert_rowid();
        let mut block_dep_stmt = tx.prepare(include_str!("sql/add_block_dependency.sql"))?;

        for parent_hash in awaiting {
            block_dep_stmt.execute(params![block_id, parent_hash.as_ref()])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
