use super::*;
use crate::womp;
use serial_test_derive::serial;

#[test]
#[serial]
fn create_drop_exists() {
    Database::reset_all().expect(womp!());
    // drop twice, it shouldn't panic on multiple drops
    ChainKeys::drop_table().expect(womp!());
    ChainKeys::drop_table().expect(womp!());

    ChainKeys::create_table().expect(womp!());
    assert!(ChainKeys::exists().expect(womp!()));
    ChainKeys::create_table().expect(womp!());
    assert!(ChainKeys::exists().expect(womp!()));
    ChainKeys::drop_table().expect(womp!());
    assert!(!ChainKeys::exists().expect(womp!()));
    ChainKeys::reset().expect(womp!());
}

#[test]
#[serial]
fn blockstore() {
    Database::reset_all().expect(womp!());
    let mut handle = ChainKeys::default();

    let blockhash1 = BlockHash::from_slice(vec![1; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let blockhash2 = BlockHash::from_slice(vec![2; BLOCKHASH_BYTES].as_slice()).expect(womp!());
    let chainkey1 = ChainKey::from_slice(vec![1; CHAINKEY_BYTES].as_slice()).expect(womp!());
    let chainkey2 = ChainKey::from_slice(vec![2; CHAINKEY_BYTES].as_slice()).expect(womp!());

    handle
        .store_key(blockhash1, (&chainkey1).clone())
        .expect(womp!());
    handle
        .store_key(blockhash2, (&chainkey2).clone())
        .expect(womp!());

    let known_keys: BTreeSet<ChainKey> = vec![(&chainkey1).clone(), (&chainkey2).clone()]
        .into_iter()
        .collect();

    let keys = handle
        .get_keys(vec![blockhash1, blockhash2].iter())
        .expect(womp!());

    assert_eq!(keys.len(), 2);
    assert_eq!(known_keys, keys);

    handle.mark_used(vec![blockhash1].iter()).expect(womp!());

    let unused: Vec<_> = handle.get_unused().expect(womp!()).into_iter().collect();
    assert_eq!(unused.len(), 1);
    assert_eq!(unused, vec![(blockhash2, chainkey2)]);

    assert!(handle.mark_used(vec![blockhash1].iter()).is_err());
}
