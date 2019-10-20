use crate::{abort_err, errors::HErr, types::ConversationId};
use chainmail::{block::*, errors::ChainError};
use herald_common::GlobalId;
use lazy_static::*;
use parking_lot::Mutex;
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

lazy_static! {
    pub static ref CK_CONN: Mutex<rusqlite::Connection> = {
        let mut conn = abort_err!(rusqlite::Connection::open("ck.sqlite3"));
        let tx = abort_err!(conn.transaction());
        abort_err!(tx.execute_batch(include_str!("sql/create.sql")));
        abort_err!(tx.commit());
        Mutex::new(conn)
    };
}

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

fn raw_store_key(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    hash_bytes: &[u8],
    key_bytes: &[u8],
) -> Result<(), HErr> {
    let mut store_stmt = tx.prepare(include_str!("sql/store_key.sql"))?;
    store_stmt.execute(params![cid, hash_bytes, key_bytes])?;
    Ok(())
}

fn raw_remove_block_dependencies(
    tx: &mut rusqlite::Transaction,
    hash_bytes: &[u8],
) -> Result<(), HErr> {
    let mut remove_deps_stmt = tx.prepare(include_str!("sql/remove_block_dependencies.sql"))?;
    remove_deps_stmt.execute(params![hash_bytes])?;
    Ok(())
}

fn raw_pop_unblocked_blocks(
    tx: &mut rusqlite::Transaction,
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

pub fn store_key(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    hash: BlockHash,
    key: ChainKey,
) -> Result<Vec<(Block, GlobalId)>, HErr> {
    // store key
    raw_store_key(tx, cid, hash.as_ref(), key.as_ref())?;

    // remove key as blocking dependency
    raw_remove_block_dependencies(tx, hash.as_ref())?;

    // get blocks that are now available
    raw_pop_unblocked_blocks(tx)?
        .into_iter()
        .map(|(block_bytes, signer_bytes)| {
            Ok((
                serde_cbor::from_slice(&block_bytes)?,
                serde_cbor::from_slice(&signer_bytes)?,
            ))
        })
        .collect()
}

pub fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    blocks: I,
) -> Result<(), HErr> {
    let mut mark_stmt = tx.prepare(include_str!("sql/mark_used.sql"))?;

    for block in blocks {
        mark_stmt.execute(params![cid, block.as_ref()])?;
    }

    Ok(())
}

pub fn mark_unused(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    blocks: &BTreeSet<BlockHash>,
) -> Result<(), HErr> {
    let mut mark_stmt = tx.prepare(include_str!("sql/mark_unused.sql"))?;

    for block in blocks {
        mark_stmt.execute(params![cid, block.as_ref()])?;
    }

    Ok(())
}

fn get_keys(
    db: &rusqlite::Connection,
    cid: ConversationId,
    blocks: &BTreeSet<BlockHash>,
) -> Result<FoundKeys, HErr> {
    let mut stmt = db.prepare(include_str!("sql/get_keys.sql"))?;

    let mut keys: BTreeSet<ChainKey> = BTreeSet::new();
    let mut missing: Vec<BlockHash> = Vec::new();

    for block in blocks {
        match stmt
            .query_map(params![cid, block.as_ref()], |row| row.get::<_, Vec<u8>>(0))?
            .next()
        {
            Some(k) => {
                keys.insert(ChainKey::from_slice(k?.as_slice()).unwrap());
            }
            None => {
                missing.push(block.clone());
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

impl ConversationId {
    pub fn store_key(
        &self,
        hash: BlockHash,
        key: ChainKey,
    ) -> Result<Vec<(Block, GlobalId)>, HErr> {
        let mut db = CK_CONN.lock();
        let mut tx = db.transaction()?;
        let blocks = store_key(&mut tx, *self, hash, key)?;
        tx.commit()?;

        Ok(blocks)
    }

    // TODO GC strategy
    pub fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(&self, blocks: I) -> Result<(), HErr> {
        let mut db = CK_CONN.lock();
        let mut tx = db.transaction()?;

        mark_used(&mut tx, *self, blocks)?;
        tx.commit()?;
        Ok(())
    }

    pub fn mark_unused(&self, blocks: &BTreeSet<BlockHash>) -> Result<(), HErr> {
        let mut db = CK_CONN.lock();
        let mut tx = db.transaction()?;

        mark_unused(&mut tx, *self, blocks)?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_keys(&self, blocks: &BTreeSet<BlockHash>) -> Result<FoundKeys, HErr> {
        let db = CK_CONN.lock();
        get_keys(&db, *self, blocks)
    }

    pub fn get_unused(&self) -> Result<Vec<(BlockHash, ChainKey)>, HErr> {
        let db = CK_CONN.lock();
        get_unused(&db, *self)
    }

    pub fn add_pending(
        &self,
        signer: &GlobalId,
        block: Block,
        awaiting: Vec<BlockHash>,
    ) -> Result<(), HErr> {
        let mut db = CK_CONN.lock();
        let mut tx = db.transaction()?;

        let block_bytes = serde_cbor::to_vec(&block)?;
        let signer_bytes = serde_cbor::to_vec(&signer)?;

        let block_id = raw_add_pending_block(&mut tx, signer_bytes, block_bytes)?;

        raw_add_block_dependencies(&mut tx, block_id, awaiting.iter().map(|hash| hash.as_ref()))?;

        tx.commit()?;

        Ok(())
    }

    pub fn open_block(&self, signer: &GlobalId, block: Block) -> Result<DecryptionResult, HErr> {
        let hashes = block.parent_hashes().clone();
        match self.get_keys(&hashes)? {
            FoundKeys::Found(parent_keys) => {
                let OpenData { msg, hash, key } = block.open(&signer.did, &parent_keys)?;
                let unlocked = self.store_key(hash, key)?;
                self.mark_used(hashes.iter())?;
                Ok(DecryptionResult::Success(msg, unlocked))
            }
            FoundKeys::Missing(missing_keys) => {
                self.add_pending(signer, block, missing_keys)?;
                Ok(DecryptionResult::Pending)
            }
        }
    }

    pub fn store_genesis(&self, gen: &Genesis) -> Result<Vec<(Block, GlobalId)>, HErr> {
        let hash = gen.compute_hash().ok_or(ChainError::CryptoError)?;
        let key = gen.key().clone();
        self.store_key(hash, key)
    }
}

#[cfg(test)]
mod tests;
