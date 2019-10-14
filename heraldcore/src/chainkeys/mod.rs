use crate::{db::Database, errors::HErr, types::ConversationId};
use chainmail::{block::*, errors::ChainError};
use herald_common::GlobalId;
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

type RawBlock = Vec<u8>;
type RawSigner = Vec<u8>;

fn raw_store_key(
    tx: &rusqlite::Transaction,
    cid: ConversationId,
    hash_bytes: &[u8],
    key_bytes: &[u8],
) -> Result<(), HErr> {
    let mut store_stmt = tx.prepare(include_str!("sql/store_key.sql"))?;
    store_stmt.execute(params![cid, hash_bytes, key_bytes])?;
    Ok(())
}

fn raw_remove_block_dependencies(
    tx: &rusqlite::Transaction,
    hash_bytes: &[u8],
) -> Result<(), HErr> {
    let mut remove_deps_stmt = tx.prepare(include_str!("sql/remove_block_dependencies.sql"))?;
    remove_deps_stmt.execute(params![hash_bytes])?;
    Ok(())
}

fn raw_pop_unblocked_blocks(
    tx: &rusqlite::Transaction,
) -> Result<Vec<(RawBlock, RawSigner)>, HErr> {
    let mut get_blocks_stmt = tx.prepare(include_str!("sql/get_unblocked_blocks.sql"))?;

    let res = get_blocks_stmt
        .query_map(NO_PARAMS, |row| {
            Ok((row.get::<_, RawBlock>(0)?, row.get::<_, RawSigner>(1)?))
        })?
        .map(|res| {
            let (block_bytes, signer_bytes) = res?;
            Ok((block_bytes, signer_bytes))
        })
        .collect();

    let mut stmt = tx.prepare(include_str!("sql/remove_pending_blocks.sql"))?;
    stmt.execute(NO_PARAMS)?;

    res
}

fn store_key(
    tx: &rusqlite::Transaction,
    cid: ConversationId,
    hash: BlockHash,
    key: ChainKey,
) -> Result<Vec<(Block, GlobalId)>, HErr> {
    // store key
    raw_store_key(&tx, cid, hash.as_ref(), key.as_ref())?;

    // remove key as blocking dependency
    raw_remove_block_dependencies(&tx, hash.as_ref())?;

    // get blocks that are now available
    raw_pop_unblocked_blocks(&tx)?
        .into_iter()
        .map(|(block_bytes, signer_bytes)| {
            Ok((
                serde_cbor::from_slice(&block_bytes)?,
                serde_cbor::from_slice(&signer_bytes)?,
            ))
        })
        .collect()
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
        match stmt
            .query_map(params![cid, block.as_ref()], |row| row.get::<_, Vec<u8>>(0))?
            .next()
        {
            Some(k) => {
                keys.insert(ChainKey::from_slice(k?.as_slice()).unwrap());
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

fn raw_add_pending_block(
    tx: &rusqlite::Connection,
    signer_bytes: Vec<u8>,
    block_bytes: Vec<u8>,
) -> Result<i64, HErr> {
    let mut pending_blocks_stmt = tx.prepare(include_str!("sql/add_pending_block.sql"))?;

    pending_blocks_stmt.execute(params![signer_bytes, block_bytes])?;

    Ok(tx.last_insert_rowid())
}

fn raw_add_block_dependencies<'a, I: Iterator<Item = &'a [u8]>>(
    tx: &rusqlite::Connection,
    block_id: i64,
    parent_hashes_bytes: I,
) -> Result<(), HErr> {
    let mut block_dep_stmt = tx.prepare(include_str!("sql/add_block_dependency.sql"))?;

    for hash_bytes in parent_hashes_bytes {
        block_dep_stmt.execute(params![block_id, hash_bytes])?;
    }
    Ok(())
}

impl BlockStore for ConversationId {
    type Error = HErr;
    type Signer = GlobalId;

    fn store_key(
        &mut self,
        hash: BlockHash,
        key: ChainKey,
    ) -> Result<Vec<(Block, Self::Signer)>, Self::Error> {
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

    fn add_pending(
        &self,
        signer: &Self::Signer,
        block: Block,
        awaiting: Vec<BlockHash>,
    ) -> Result<(), Self::Error> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;

        let block_bytes = serde_cbor::to_vec(&block)?;
        let signer_bytes = serde_cbor::to_vec(&signer)?;

        let block_id = raw_add_pending_block(&tx, signer_bytes, block_bytes)?;

        raw_add_block_dependencies(&tx, block_id, awaiting.iter().map(|hash| hash.as_ref()))?;

        tx.commit()?;

        Ok(())
    }
}

// TODO: make this actually work

// fn raw_del_key(
//     tx: &rusqlite::Transaction,
//     cid: ConversationId,
//     hash_bytes: &[u8],
// ) -> Result<(), HErr> {
//     let mut store_stmt = tx.prepare(include_str!("sql/del_key.sql"))?;
//     store_stmt.execute(params![cid, hash_bytes])?;
//     Ok(())
// }

// pub(crate) fn del_key(cid: ConversationId, hash: BlockHash) -> Result<(), HErr> {
//     let mut db = Database::get()?;
//     let tx = db.transaction()?;
//     raw_del_key(&tx, cid, hash.as_ref())?;
//     Ok(())
// }

#[cfg(test)]
mod tests;
