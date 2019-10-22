use super::*;
use crate::{contact::ContactBuilder, conversation::ConversationBuilder, womp};
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
#[serial]
fn add_remove_member() {
    let mut conn = Database::in_memory().expect(womp!());

    let uid = "Hello".try_into().expect(womp!());
    ContactBuilder::new(uid).add_db(&mut conn).expect(womp!());

    let cid = ConversationBuilder::new().add_db(&mut conn).expect(womp!());

    let mems = db::members(&conn, &cid).expect(womp!());
    assert!(mems.is_empty());

    db::add_member(&conn, &cid, uid).expect(womp!());
    let mems = db::members(&conn, &cid).expect(womp!());
    assert_eq!(mems.len(), 1);

    db::remove_member(&conn, &cid, uid).expect(womp!());
    let mems = db::members(&conn, &cid).expect(womp!());
    assert!(mems.is_empty());
}

#[test]
#[serial]
fn add_tx() {
    let mut conn = Database::in_memory().expect(womp!());

    let uid1 = "Hello".try_into().expect(womp!());
    ContactBuilder::new(uid1).add_db(&mut conn).expect(womp!());

    let uid2 = "World".try_into().expect(womp!());
    ContactBuilder::new(uid2).add_db(&mut conn).expect(womp!());

    let cid = ConversationBuilder::new().add_db(&mut conn).expect(womp!());

    let tx = conn.transaction().expect(womp!());

    db::add_members_with_tx(&tx, cid, &[uid1, uid2]).expect(womp!());
    tx.commit().expect(womp!());

    let mems = db::members(&conn, &cid).expect(womp!());
    assert_eq!(mems.len(), 2);
}
