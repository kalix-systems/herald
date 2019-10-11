use super::*;
use crate::{conversation::ConversationBuilder, womp};
use serial_test_derive::serial;

#[test]
#[serial]
fn blockstore() {
    Database::reset_all().expect(womp!());
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

    let blockhash11 = BlockHash::from_slice(vec![11; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let blockhash12 = BlockHash::from_slice(vec![12; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let chainkey11 = ChainKey::from_slice(vec![11; CHAINKEY_BYTES].as_slice()).expect(womp!());
    let chainkey12 = ChainKey::from_slice(vec![12; CHAINKEY_BYTES].as_slice()).expect(womp!());

    let blockhash21 = BlockHash::from_slice(vec![21; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let blockhash22 = BlockHash::from_slice(vec![22; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let chainkey21 = ChainKey::from_slice(vec![21; CHAINKEY_BYTES].as_slice()).expect(womp!());
    let chainkey22 = ChainKey::from_slice(vec![22; CHAINKEY_BYTES].as_slice()).expect(womp!());

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
