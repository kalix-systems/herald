use super::*;
use crate::{contact::ContactBuilder, conversation::ConversationBuilder, womp};
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
#[serial]
fn add_remove_member() {
    Database::reset_all().expect(womp!());

    let uid = "Hello".try_into().expect(womp!());
    ContactBuilder::new(uid).add().expect(womp!());

    let cid = ConversationBuilder::new().add().expect(womp!());

    let mems = members(&cid).expect(womp!());
    assert!(mems.is_empty());

    add_member(&cid, uid).expect(womp!());
    let mems = members(&cid).expect(womp!());
    assert_eq!(mems.len(), 1);

    remove_member(&cid, uid).expect(womp!());
    let mems = members(&cid).expect(womp!());
    assert!(mems.is_empty());
}

#[test]
#[serial]
fn add_tx() {
    Database::reset_all().expect(womp!());

    let uid1 = "Hello".try_into().expect(womp!());
    ContactBuilder::new(uid1).add().expect(womp!());

    let uid2 = "World".try_into().expect(womp!());
    ContactBuilder::new(uid2).add().expect(womp!());

    let cid = ConversationBuilder::new().add().expect(womp!());

    let mut db = Database::get().expect(womp!());
    let tx = db.transaction().expect(womp!());

    add_members_with_tx(&tx, cid, &[uid1, uid2]).expect(womp!());
    tx.commit().expect(womp!());

    let mems = members(&cid).expect(womp!());
    assert_eq!(mems.len(), 2);
}
