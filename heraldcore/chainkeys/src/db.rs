use super::*;
use byteorder::*;

pub(super) fn store_ratchet_state(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    state: &RatchetState,
) -> Result<(), rusqlite::Error> {
    let mut store_stmt = tx.prepare(include_str!("sql/store_ratchet_state.sql"))?;
    store_stmt.execute(params![
        cid,
        state.ix() as i64,
        state.base_key().as_ref(),
        state.ratchet_key().as_ref()
    ])?;
    Ok(())
}

pub(super) fn store_derived_keys<'a, I: IntoIterator<Item = &'a (u64, kcl::aead::Key)>>(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
    keys: I,
) -> Result<(), rusqlite::Error> {
    let mut store_stmt = tx.prepare(include_str!("sql/store_derived_key.sql"))?;
    let ts = Time::now();
    for (ix, key) in keys {
        store_stmt.execute(params![cid, &ix.to_le_bytes() as &[u8], key.as_ref(), ts.0])?;
    }
    Ok(())
}

pub(super) fn get_ratchet_state(
    tx: &mut rusqlite::Transaction,
    cid: ConversationId,
) -> Result<RatchetState, ChainKeysError> {
    let mut get_stmt = tx.prepare(include_str!("sql/get_ratchet_state.sql"))?;
    let (raw_ix, raw_base_key, raw_ratchet_key) = get_stmt
        .query_map(params![cid], |row| {
            Ok((
                row.get::<_, Vec<u8>>(0)?,
                row.get::<_, Vec<u8>>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?
        .next()
        .ok_or(ChainKeysError::NoneError(loc!()))??;

    let ix = if raw_ix.len() != 8 {
        return Err(ChainKeysError::StoreCorrupted);
    } else {
        LE::read_u64(&raw_ix)
    };

    let base_key =
        kcl::hash::Key::from_slice(&raw_base_key).ok_or(ChainKeysError::StoreCorrupted)?;

    let ratchet_key =
        RatchetKey::from_slice(&raw_ratchet_key).ok_or(ChainKeysError::StoreCorrupted)?;

    Ok(RatchetState::mk(ix, base_key, ratchet_key))
}

// pub(super) fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
//     db: &rusqlite::Connection,
//     cid: ConversationId,
//     blocks: I,
// ) -> Result<FoundKeys, rusqlite::Error> {
//     let mut stmt = db.prepare(include_str!("sql/get_keys.sql"))?;

//     let mut keys: BTreeSet<ChainKey> = BTreeSet::new();
//     let mut missing: Vec<BlockHash> = Vec::new();

//     for block in blocks {
//         match stmt
//             .query_map(params![cid, block.as_ref()], |row| row.get::<_, Vec<u8>>(0))?
//             .next()
//         {
//             Some(k) => {
//                 keys.insert(ChainKey::from_slice(k?.as_slice()).unwrap());
//             }
//             None => {
//                 missing.push(block.clone());
//             }
//         }
//     }

//     Ok(if !missing.is_empty() {
//         FoundKeys::Missing(missing)
//     } else {
//         FoundKeys::Found(keys)
//     })
// }

// pub(super) fn get_channel_key(
//     db: &rusqlite::Connection,
//     cid: ConversationId,
// ) -> Result<ChannelKey, ChainKeysError> {
//     let mut stmt = db.prepare(include_str!("sql/get_channel_key.sql"))?;

//     let raw_key = stmt
//         .query_map(params![cid], |row| row.get::<_, Vec<u8>>(0))?
//         .next()
//         .ok_or(ChainKeysError::NoneError(loc!()))??;

//     let key =
//         ChannelKey::from_slice(raw_key.as_slice()).ok_or(ChainKeysError::NoneError(loc!()))?;

//     Ok(key)
// }

// pub(super) fn get_unused(
//     db: &rusqlite::Connection,
//     cid: ConversationId,
// ) -> Result<Vec<(BlockHash, ChainKey)>, ChainKeysError> {
//     let mut stmt = db.prepare(include_str!("sql/get_unused.sql"))?;

//     let results = stmt.query_map(params![cid], |row| {
//         Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
//     })?;

//     let mut pairs = vec![];

//     for res in results {
//         let (raw_hash, raw_key) = res?;
//         pairs.push((
//             BlockHash::from_slice(raw_hash.as_slice())
//                 .ok_or(ChainMailError::BlockStoreCorrupted)?,
//             ChainKey::from_slice(raw_key.as_slice()).ok_or(ChainMailError::BlockStoreCorrupted)?,
//         ));
//     }

//     Ok(pairs)
// }

// pub(super) fn raw_add_pending_block(
//     tx: &rusqlite::Connection,
//     signer_bytes: Vec<u8>,
//     block_bytes: Vec<u8>,
// ) -> Result<i64, rusqlite::Error> {
//     let mut pending_blocks_stmt = tx.prepare(include_str!("sql/add_pending_block.sql"))?;

//     pending_blocks_stmt.execute(params![signer_bytes, block_bytes])?;

//     Ok(tx.last_insert_rowid())
// }

// pub(super) fn raw_add_block_dependencies<'a, I: Iterator<Item = &'a [u8]>>(
//     tx: &rusqlite::Connection,
//     block_id: i64,
//     parent_hashes_bytes: I,
// ) -> Result<(), rusqlite::Error> {
//     let mut block_dep_stmt = tx.prepare(include_str!("sql/add_block_dependency.sql"))?;

//     for hash_bytes in parent_hashes_bytes {
//         block_dep_stmt.execute(params![block_id, hash_bytes])?;
//     }
//     Ok(())
// }
