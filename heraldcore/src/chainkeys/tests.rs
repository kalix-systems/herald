use super::*;
use crate::womp;
use herald_common::*;
use serial_test_derive::*;

impl ConversationId {
    async fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Result<FoundKeys, HErr> {
        let mut db = get_conn().await?;
        get_keys(&mut db, *self, blocks)
    }
}

async fn reset() {
    let mut conn = get_conn().await.expect(womp!());
    let tx = conn.transaction().expect(womp!());
    tx.execute_batch(include_str!("sql/drop.sql"))
        .expect(womp!());
    tx.execute_batch(include_str!("sql/create.sql"))
        .expect(womp!());
    tx.commit().expect(womp!());
}

fn in_memory() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().expect(womp!());
    conn.execute_batch(include_str!("sql/create.sql"))
        .expect(womp!());
    conn
}

#[tokio::test]
async fn raw_pending() {
    let mut conn = in_memory();
    let cid = ConversationId::from(crate::utils::rand_id());

    let mut tx = conn.transaction().expect(womp!());

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
        &mut tx,
        dummy_signer_bytes1.to_vec(),
        dummy_block_bytes1.to_vec(),
    )
    .expect(womp!());

    raw_add_block_dependencies(&mut tx, block_id1, vec![blockhash1.as_ref()].into_iter())
        .expect(womp!());

    let block_id2 = raw_add_pending_block(
        &mut tx,
        dummy_signer_bytes2.to_vec(),
        dummy_block_bytes2.to_vec(),
    )
    .expect(womp!());
    raw_add_block_dependencies(
        &mut tx,
        block_id2,
        vec![blockhash1.as_ref(), blockhash2.as_ref()].into_iter(),
    )
    .expect(womp!());

    let block_id3 = raw_add_pending_block(
        &mut tx,
        dummy_signer_bytes3.to_vec(),
        dummy_block_bytes3.to_vec(),
    )
    .expect(womp!());

    raw_add_block_dependencies(
        &mut tx,
        block_id3,
        vec![blockhash1.as_ref(), blockhash2.as_ref()].into_iter(),
    )
    .expect(womp!());

    // free dummy_block_bytes1
    raw_store_key(&mut tx, cid, blockhash1.as_ref(), chainkey1.as_ref()).expect(womp!());
    raw_remove_block_dependencies(&mut tx, blockhash1.as_ref()).expect(womp!());

    let blocks = raw_pop_unblocked_blocks(&mut tx).expect(womp!());
    assert_eq!(blocks.len(), 1);
    assert_eq!(
        blocks[0],
        (dummy_signer_bytes1.to_vec(), dummy_block_bytes1.to_vec())
    );

    // free dummy_block_bytes2 and dummy_block_bytes3
    raw_store_key(&mut tx, cid, blockhash2.as_ref(), chainkey2.as_ref()).expect(womp!());
    raw_remove_block_dependencies(&mut tx, blockhash2.as_ref()).expect(womp!());

    let blocks = raw_pop_unblocked_blocks(&mut tx).expect(womp!());
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

#[tokio::test]
#[serial(CK)]
async fn channel_keys() {
    reset().await;

    let kp = herald_common::sig::KeyPair::gen_new();

    let cid1 = ConversationId::from(crate::utils::rand_id());
    let cid2 = ConversationId::from(crate::utils::rand_id());

    assert_ne!(cid1, cid2);

    let g1 = Genesis::new(kp.secret_key());
    let g2 = Genesis::new(kp.secret_key());

    assert_ne!(g1, g2);

    let h1 = g1.compute_hash().expect("failed to compute hash for g1");
    let h2 = g2.compute_hash().expect("failed to compute hash for g2");

    assert_ne!(h1, h2);

    cid1.store_genesis(&g1)
        .await
        .expect("failed to store genesis block for cid1");
    cid2.store_genesis(&g2)
        .await
        .expect("failed to store genesis block for cid2");

    let u1 = cid1
        .get_unused()
        .await
        .expect("failed to get unused keys for cid1");
    let u2 = cid2
        .get_unused()
        .await
        .expect("failed to get unused keys for cid2");

    assert_eq!(u1, vec![(h1, g1.root().clone())]);
    assert_eq!(u2, vec![(h2, g2.root().clone())]);

    assert_ne!(u1, u2);

    let ck1 = cid1
        .get_channel_key()
        .await
        .expect("failed to ge channel key for cid1");

    let ck2 = cid2
        .get_channel_key()
        .await
        .expect("failed to ge channel key for cid2");

    assert_eq!(&ck1, g1.channel_key());
    assert_eq!(&ck2, g2.channel_key());

    assert_ne!(ck1, ck2);
}

#[tokio::test]
#[serial(CK)]
async fn blockstore() {
    reset().await;

    let cid1 = ConversationId::from(crate::utils::rand_id());
    let cid2 = ConversationId::from(crate::utils::rand_id());

    let blockhash11 = BlockHash::from_slice(&[11; BLOCKHASH_BYTES]).expect(womp!());
    let blockhash12 = BlockHash::from_slice(&[12; BLOCKHASH_BYTES]).expect(womp!());
    let chainkey11 = ChainKey::from_slice(&[11; CHAINKEY_BYTES]).expect(womp!());
    let chainkey12 = ChainKey::from_slice(&[12; CHAINKEY_BYTES]).expect(womp!());

    let blockhash21 = BlockHash::from_slice(&[21; BLOCKHASH_BYTES]).expect(womp!());
    let blockhash22 = BlockHash::from_slice(&[22; BLOCKHASH_BYTES]).expect(womp!());
    let chainkey21 = ChainKey::from_slice(&[21; CHAINKEY_BYTES]).expect(womp!());
    let chainkey22 = ChainKey::from_slice(&[22; CHAINKEY_BYTES]).expect(womp!());

    // cid1 keys
    cid1.store_key(blockhash11, &chainkey11)
        .await
        .expect(womp!());
    cid1.store_key(blockhash12, &chainkey12)
        .await
        .expect(womp!());

    // cid2 keys
    cid2.store_key(blockhash21, &chainkey21)
        .await
        .expect(womp!());
    cid2.store_key(blockhash22, &chainkey22)
        .await
        .expect(womp!());

    // cid1 known keys
    let known_keys1: BTreeSet<ChainKey> = vec![(&chainkey11).clone(), (&chainkey12).clone()]
        .into_iter()
        .collect();

    let keys1 = cid1
        .get_keys(vec![blockhash11, blockhash12].iter())
        .await
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
        .await
        .expect(womp!());

    match keys2 {
        FoundKeys::Found(keys) => {
            assert_eq!(keys.len(), 2);
            assert_eq!(known_keys2, keys);
        }
        _ => panic!(),
    }

    // cid1 mark used
    cid1.mark_used(vec![blockhash11].iter())
        .await
        .expect(womp!());

    let unused1: Vec<_> = cid1
        .get_unused()
        .await
        .expect(womp!())
        .into_iter()
        .collect();

    assert_eq!(unused1.len(), 1);
    assert_eq!(unused1, vec![(blockhash12, chainkey12)]);

    // cid2 mark used
    cid2.mark_used(vec![blockhash21].iter())
        .await
        .expect(womp!());

    let unused2: Vec<_> = cid2
        .get_unused()
        .await
        .expect(womp!())
        .into_iter()
        .collect();

    assert_eq!(unused2.len(), 1);
    assert_eq!(unused2, vec![(blockhash22, chainkey22)]);
}
