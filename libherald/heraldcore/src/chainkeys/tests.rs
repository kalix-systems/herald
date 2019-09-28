use super::*;
use crate::conversation::add_conversation;
use crate::womp;
use serial_test_derive::serial;

#[test]
#[serial]
fn blockstore() {
    Database::reset_all().expect(womp!());
    let cid1 = ConversationId::from(crate::utils::rand_id());
    let cid2 = ConversationId::from(crate::utils::rand_id());

    add_conversation(Some(&cid1), None).expect(womp!());
    add_conversation(Some(&cid2), None).expect(womp!());

    let mut handle1 = ChainKeys::new(cid1);
    let mut handle2 = ChainKeys::new(cid2);

    let blockhash11 = BlockHash::from_slice(vec![11; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let blockhash12 = BlockHash::from_slice(vec![12; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let chainkey11 = ChainKey::from_slice(vec![11; CHAINKEY_BYTES].as_slice()).expect(womp!());
    let chainkey12 = ChainKey::from_slice(vec![12; CHAINKEY_BYTES].as_slice()).expect(womp!());

    let blockhash21 = BlockHash::from_slice(vec![21; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let blockhash22 = BlockHash::from_slice(vec![22; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let chainkey21 = ChainKey::from_slice(vec![21; CHAINKEY_BYTES].as_slice()).expect(womp!());
    let chainkey22 = ChainKey::from_slice(vec![22; CHAINKEY_BYTES].as_slice()).expect(womp!());

    // handle1 keys
    handle1
        .store_key(blockhash11, (&chainkey11).clone())
        .expect(womp!());
    handle1
        .store_key(blockhash12, (&chainkey12).clone())
        .expect(womp!());

    // handle2 keys
    handle2
        .store_key(blockhash21, (&chainkey21).clone())
        .expect(womp!());
    handle2
        .store_key(blockhash22, (&chainkey22).clone())
        .expect(womp!());

    // handle1 known keys
    let known_keys1: BTreeSet<ChainKey> = vec![(&chainkey11).clone(), (&chainkey12).clone()]
        .into_iter()
        .collect();

    let keys1 = handle1
        .get_keys(vec![blockhash11, blockhash12].iter())
        .expect(womp!());

    assert_eq!(keys1.len(), 2);
    assert_eq!(known_keys1, keys1);

    // handle2 known keys
    let known_keys2: BTreeSet<ChainKey> = vec![(&chainkey21).clone(), (&chainkey22).clone()]
        .into_iter()
        .collect();

    let keys2 = handle2
        .get_keys(vec![blockhash21, blockhash22].iter())
        .expect(womp!());

    assert_eq!(keys2.len(), 2);
    assert_eq!(known_keys2, keys2);

    // handle1 mark used
    handle1.mark_used(vec![blockhash11].iter()).expect(womp!());

    let unused1: Vec<_> = handle1.get_unused().expect(womp!()).into_iter().collect();

    assert_eq!(unused1.len(), 1);
    assert_eq!(unused1, vec![(blockhash12, chainkey12)]);

    assert!(handle1.mark_used(vec![blockhash11].iter()).is_err());

    // handle2 mark used
    handle2.mark_used(vec![blockhash21].iter()).expect(womp!());

    let unused2: Vec<_> = handle2.get_unused().expect(womp!()).into_iter().collect();

    assert_eq!(unused2.len(), 1);
    assert_eq!(unused2, vec![(blockhash22, chainkey22)]);

    assert!(handle1.mark_used(vec![blockhash21].iter()).is_err());
}
