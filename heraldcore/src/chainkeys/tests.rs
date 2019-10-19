use super::*;
use crate::{conversation::ConversationBuilder, womp};
use serial_test_derive::serial;

fn reset() {
    let mut conn = CK_CONN.lock();
    let tx = conn.transaction().expect(womp!());
    tx.execute_batch(include_str!("sql/drop.sql"))
        .expect(womp!());
    tx.execute_batch(include_str!("sql/create.sql"))
        .expect(womp!());
    tx.commit().expect(womp!());
}

#[test]
#[serial]
fn raw_pending() {
    reset();

    let cid = ConversationId::from(crate::utils::rand_id());

    ConversationBuilder::new()
        .conversation_id(cid)
        .add()
        .expect(womp!());

    let mut conn = CK_CONN.lock();
    let tx = conn.transaction().expect(womp!());

    let blockhash1 = BlockHash::from_slice(&[1; BLOCKHASH_BYTES]).expect(womp!());
    let blockhash2 = BlockHash::from_slice(&[2; BLOCKHASH_BYTES]).expect(womp!());
    let chainkey1 = ChainKey::from_slice(&[1; CHAINKEY_BYTES]).expect(womp!());
    let chainkey2 = ChainKey::from_slice(&[2; CHAINKEY_BYTES]).expect(womp!());

    let dummy_block_bytes1 = &[0; 32];
    let dummy_signer_bytes1 = &[0; 32];
    let dummy_block_bytes2 = &[1; 32];
    let dummy_signer_bytes2 = &[1; 32];
    let dummy_block_bytes3 = &[2; 32];
    let dummy_signer_bytes3 = &[2; 32];

    let block_id1 = raw_add_pending_block(
        &tx,
        dummy_signer_bytes1.to_vec(),
        dummy_block_bytes1.to_vec(),
    )
    .expect(womp!());

    raw_add_block_dependencies(&tx, block_id1, vec![blockhash1.as_ref()].into_iter())
        .expect(womp!());

    let block_id2 = raw_add_pending_block(
        &tx,
        dummy_signer_bytes2.to_vec(),
        dummy_block_bytes2.to_vec(),
    )
    .expect(womp!());
    raw_add_block_dependencies(
        &tx,
        block_id2,
        vec![blockhash1.as_ref(), blockhash2.as_ref()].into_iter(),
    )
    .expect(womp!());

    let block_id3 = raw_add_pending_block(
        &tx,
        dummy_signer_bytes3.to_vec(),
        dummy_block_bytes3.to_vec(),
    )
    .expect(womp!());

    raw_add_block_dependencies(
        &tx,
        block_id3,
        vec![blockhash1.as_ref(), blockhash2.as_ref()].into_iter(),
    )
    .expect(womp!());

    // free dummy_block_bytes1
    raw_store_key(&tx, cid, blockhash1.as_ref(), chainkey1.as_ref()).expect(womp!());
    raw_remove_block_dependencies(&tx, blockhash1.as_ref()).expect(womp!());

    let blocks = raw_pop_unblocked_blocks(&tx).expect(womp!());
    assert_eq!(blocks.len(), 1);
    assert_eq!(
        blocks[0],
        (dummy_signer_bytes1.to_vec(), dummy_block_bytes1.to_vec())
    );

    // free dummy_block_bytes2 and dummy_block_bytes3
    raw_store_key(&tx, cid, blockhash2.as_ref(), chainkey2.as_ref()).expect(womp!());
    raw_remove_block_dependencies(&tx, blockhash2.as_ref()).expect(womp!());

    let blocks = raw_pop_unblocked_blocks(&tx).expect(womp!());
    assert_eq!(blocks.len(), 2);
    assert_eq!(
        blocks[0],
        (dummy_signer_bytes2.to_vec(), dummy_block_bytes2.to_vec())
    );
    assert_eq!(
        blocks[1],
        (dummy_signer_bytes3.to_vec(), dummy_block_bytes3.to_vec())
    );

    tx.commit().expect(womp!());
}

#[test]
#[serial]
fn blockstore() {
    reset();

    let mut cid1 = ConversationId::from(crate::utils::rand_id());
    let mut cid2 = ConversationId::from(crate::utils::rand_id());

    ConversationBuilder::new()
        .conversation_id(cid1)
        .add()
        .expect(womp!());
    ConversationBuilder::new()
        .conversation_id(cid2)
        .add()
        .expect(womp!());

    let blockhash11 = BlockHash::from_slice(&[11; BLOCKHASH_BYTES]).expect(womp!());
    let blockhash12 = BlockHash::from_slice(&[12; BLOCKHASH_BYTES]).expect(womp!());
    let chainkey11 = ChainKey::from_slice(&[11; CHAINKEY_BYTES]).expect(womp!());
    let chainkey12 = ChainKey::from_slice(&[12; CHAINKEY_BYTES]).expect(womp!());

    let blockhash21 = BlockHash::from_slice(&[21; BLOCKHASH_BYTES]).expect(womp!());
    let blockhash22 = BlockHash::from_slice(&[22; BLOCKHASH_BYTES]).expect(womp!());
    let chainkey21 = ChainKey::from_slice(&[21; CHAINKEY_BYTES]).expect(womp!());
    let chainkey22 = ChainKey::from_slice(&[22; CHAINKEY_BYTES]).expect(womp!());

    // cid1 keys
    cid1.store_key(blockhash11, (&chainkey11).clone())
        .expect(womp!());
    cid1.store_key(blockhash12, (&chainkey12).clone())
        .expect(womp!());

    // cid2 keys
    cid2.store_key(blockhash21, (&chainkey21).clone())
        .expect(womp!());
    cid2.store_key(blockhash22, (&chainkey22).clone())
        .expect(womp!());

    // cid1 known keys
    let known_keys1: BTreeSet<ChainKey> = vec![(&chainkey11).clone(), (&chainkey12).clone()]
        .into_iter()
        .collect();

    let keys1 = cid1
        .get_keys(vec![blockhash11, blockhash12].iter())
        .expect(womp!());

    match keys1 {
        FoundKeys::Found(keys) => {
            assert_eq!(keys.len(), 2);
            assert_eq!(known_keys1, keys);
        }
        _ => panic!(),
    }

    // cid2 known keys
    let known_keys2: BTreeSet<ChainKey> = vec![(&chainkey21).clone(), (&chainkey22).clone()]
        .into_iter()
        .collect();

    let keys2 = cid2
        .get_keys(vec![blockhash21, blockhash22].iter())
        .expect(womp!());

    match keys2 {
        FoundKeys::Found(keys) => {
            assert_eq!(keys.len(), 2);
            assert_eq!(known_keys2, keys);
        }
        _ => panic!(),
    }

    // cid1 mark used
    cid1.mark_used(vec![blockhash11].iter()).expect(womp!());

    let unused1: Vec<_> = cid1.get_unused().expect(womp!()).into_iter().collect();

    assert_eq!(unused1.len(), 1);
    assert_eq!(unused1, vec![(blockhash12, chainkey12)]);

    // cid2 mark used
    cid2.mark_used(vec![blockhash21].iter()).expect(womp!());

    let unused2: Vec<_> = cid2.get_unused().expect(womp!()).into_iter().collect();

    assert_eq!(unused2.len(), 1);
    assert_eq!(unused2, vec![(blockhash22, chainkey22)]);
}
