use super::*;

pub(super) fn store_channel_key(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    channel_key: &ChannelKey,
) -> Result<(), rusqlite::Error> {
    let mut store_stmt = tx.prepare(include_str!("sql/add_channelkey.sql"))?;
    store_stmt.execute(params![cid, channel_key.as_ref()])?;
    Ok(())
}

pub(super) fn raw_store_key(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    hash_bytes: &[u8],
    key_bytes: &[u8],
) -> Result<(), rusqlite::Error> {
    let mut store_stmt = tx.prepare(include_str!("sql/store_key.sql"))?;
    store_stmt.execute(params![cid, hash_bytes, key_bytes])?;
    Ok(())
}

pub(super) fn raw_remove_block_dependencies(
    tx: &mut rusqlite::Transaction,
    hash_bytes: &[u8],
) -> Result<(), rusqlite::Error> {
    let mut remove_deps_stmt = tx.prepare(include_str!("sql/remove_block_dependencies.sql"))?;
    remove_deps_stmt.execute(params![hash_bytes])?;
    Ok(())
}

pub(super) fn raw_pop_unblocked_blocks(
    tx: &mut rusqlite::Transaction
) -> Result<Vec<(RawBlock, RawSigner)>, rusqlite::Error> {
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
    key: &ChainKey,
) -> Result<Vec<(Block, GlobalId)>, ChainKeysError> {
    // store key
    raw_store_key(tx, cid, hash.as_ref(), key.as_ref())?;

    // remove key as blocking dependency
    raw_remove_block_dependencies(tx, hash.as_ref())?;

    // get blocks that are now available
    raw_pop_unblocked_blocks(tx)?
        .into_iter()
        .map(|(block_bytes, signer_bytes)| {
            Ok((
                serde_cbor::from_slice(&block_bytes)
                    .map_err(|_| ChainKeysError::Deserialization(loc!()))?,
                serde_cbor::from_slice(&signer_bytes)
                    .map_err(|_| ChainKeysError::Deserialization(loc!()))?,
            ))
        })
        .collect()
}

pub fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    blocks: I,
) -> Result<(), rusqlite::Error> {
    let mut mark_stmt = tx.prepare(include_str!("sql/mark_used.sql"))?;

    for block in blocks {
        mark_stmt.execute(params![cid, block.as_ref()])?;
    }

    Ok(())
}

pub(crate) fn mark_unused(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    blocks: &BTreeSet<BlockHash>,
) -> Result<(), rusqlite::Error> {
    let mut mark_stmt = tx.prepare(include_str!("sql/mark_unused.sql"))?;

    for block in blocks {
        mark_stmt.execute(params![cid, block.as_ref()])?;
    }

    Ok(())
}

pub(crate) fn add_pending(
    tx: &mut rusqlite::Transaction,
    signer: &GlobalId,
    block: &Block,
    awaiting: &[BlockHash],
) -> Result<(), ChainKeysError> {
    let block_bytes =
        serde_cbor::to_vec(block).map_err(|_| ChainKeysError::Serialization(loc!()))?;
    let signer_bytes =
        serde_cbor::to_vec(signer).map_err(|_| ChainKeysError::Serialization(loc!()))?;

    let block_id = raw_add_pending_block(tx, signer_bytes, block_bytes)?;

    raw_add_block_dependencies(tx, block_id, awaiting.iter().map(BlockHash::as_ref))?;

    Ok(())
}

pub(super) fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
    db: &rusqlite::Connection,
    cid: ConversationId,
    blocks: I,
) -> Result<FoundKeys, rusqlite::Error> {
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

pub(super) fn get_channel_key(
    db: &rusqlite::Connection,
    cid: ConversationId,
) -> Result<ChannelKey, ChainKeysError> {
    let mut stmt = db.prepare(include_str!("sql/get_channel_key.sql"))?;

    let raw_key = stmt
        .query_map(params![cid], |row| row.get::<_, Vec<u8>>(0))?
        .next()
        .ok_or(ChainKeysError::NoneError(loc!()))??;

    let key =
        ChannelKey::from_slice(raw_key.as_slice()).ok_or(ChainKeysError::NoneError(loc!()))?;

    Ok(key)
}

pub(super) fn get_unused(
    db: &rusqlite::Connection,
    cid: ConversationId,
) -> Result<Vec<(BlockHash, ChainKey)>, ChainKeysError> {
    let mut stmt = db.prepare(include_str!("sql/get_unused.sql"))?;

    let results = stmt.query_map(params![cid], |row| {
        Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
    })?;

    let mut pairs = vec![];

    for res in results {
        let (raw_hash, raw_key) = res?;
        pairs.push((
            BlockHash::from_slice(raw_hash.as_slice())
                .ok_or(ChainMailError::BlockStoreCorrupted)?,
            ChainKey::from_slice(raw_key.as_slice()).ok_or(ChainMailError::BlockStoreCorrupted)?,
        ));
    }

    Ok(pairs)
}

pub(super) fn raw_add_pending_block(
    tx: &rusqlite::Connection,
    signer_bytes: Vec<u8>,
    block_bytes: Vec<u8>,
) -> Result<i64, rusqlite::Error> {
    let mut pending_blocks_stmt = tx.prepare(include_str!("sql/add_pending_block.sql"))?;

    pending_blocks_stmt.execute(params![signer_bytes, block_bytes])?;

    Ok(tx.last_insert_rowid())
}

pub(super) fn raw_add_block_dependencies<'a, I: Iterator<Item = &'a [u8]>>(
    tx: &rusqlite::Connection,
    block_id: i64,
    parent_hashes_bytes: I,
) -> Result<(), rusqlite::Error> {
    let mut block_dep_stmt = tx.prepare(include_str!("sql/add_block_dependency.sql"))?;

    for hash_bytes in parent_hashes_bytes {
        block_dep_stmt.execute(params![block_id, hash_bytes])?;
    }
    Ok(())
}
