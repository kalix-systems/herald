use super::*;
use crate::{conversation::ConversationBuilder, user::UserBuilder};
use std::convert::TryInto;

#[test]
fn add_remove_member() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let uid = "Hello".try_into().expect(womp!());
    UserBuilder::new(uid).add_db(&mut conn).expect(womp!());

    let meta = ConversationBuilder::new()
        .add_db(&mut conn)
        .expect(womp!())
        .meta;
    let cid = meta.conversation_id;

    let mems = db::members(&conn, &cid).expect(womp!());
    assert_eq!(mems.len(), 1);

    db::add_member(&conn, &cid, &uid).expect(womp!());
    let mems = db::members(&conn, &cid).expect(womp!());
    assert_eq!(mems.len(), 2);

    db::remove_member(&conn, &cid, uid).expect(womp!());
    let mems = db::members(&conn, &cid).expect(womp!());
    assert_eq!(mems.len(), 1);
}

#[test]
fn shared_conversations() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let uid1 = "Hello".try_into().expect(womp!());
    UserBuilder::new(uid1).add_db(&mut conn).expect(womp!());

    let shared = db::shared_conversations(&conn, &uid1).expect(womp!());

    assert_eq!(shared.len(), 0);
}

#[test]
fn add_tx() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let uid1 = "Hello".try_into().expect(womp!());
    UserBuilder::new(uid1).add_db(&mut conn).expect(womp!());

    let uid2 = "World".try_into().expect(womp!());
    UserBuilder::new(uid2).add_db(&mut conn).expect(womp!());

    let meta = ConversationBuilder::new()
        .add_db(&mut conn)
        .expect(womp!())
        .meta;
    let cid = meta.conversation_id;

    let tx = conn.transaction().expect(womp!());

    db::add_members_with_tx(&tx, cid, &[uid1, uid2]).expect(womp!());
    tx.commit().expect(womp!());

    let mems = db::members(&conn, &cid).expect(womp!());
    assert_eq!(mems.len(), 3);
}
