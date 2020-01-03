use super::*;
use crate::{db::Database, message::InboundMessageBuilder, user::UserBuilder};
use platform_dirs::pictures_dir;
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
fn add_conversation() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    // test without id
    super::ConversationBuilder::new()
        .add_db(&mut conn)
        .expect(womp!("failed to create conversation"));

    let conversation_id = ConversationId::from([0; 32]);
    // test with id
    assert_eq!(conversation_id, {
        let mut bld = super::ConversationBuilder::new();
        bld.conversation_id(conversation_id);
        bld.add_db(&mut conn)
            .expect(womp!("failed to create conversation"))
            .meta
            .conversation_id
    });

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id([1; 32].into())
        .title("el groupo".to_owned());
    bld.add_db(&mut conn)
        .expect(womp!("failed to create conversation"));

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id([2; 32].into())
        .title("el groupo".to_owned());
    bld.add_db(&mut conn)
        .expect(womp!("failed to create conversation"));
}

#[test]
fn add_and_get() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let author = "Hello".try_into().unwrap();
    UserBuilder::new(author).add_db(&mut conn).expect(womp!());

    let conversation = ConversationId::from([0; 32]);

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conversation);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    let mid1 = [1; 32].try_into().expect(womp!());

    let mut builder1 = InboundMessageBuilder::default();
    builder1
        .id(mid1)
        .author(author)
        .timestamp(Time::now())
        .conversation_id(conversation)
        .body("1".try_into().expect(womp!()));
    builder1.store_db(&mut conn).expect(womp!());

    let mid2 = [2; 32].try_into().expect(womp!());
    let mut builder2 = InboundMessageBuilder::default();
    builder2
        .id(mid2)
        .author(author)
        .timestamp(Time::now())
        .conversation_id(conversation)
        .body("2".try_into().expect(womp!()));
    builder2.store_db(&mut conn).expect(womp!());

    let msgs = crate::message::db::conversation_messages(&conn, &conversation)
        .expect(womp!("Failed to get conversation"));

    assert_eq!(msgs.len(), 2);
}

#[test]
fn conversation_message_meta() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let receiver = crate::user::db::test_user(&mut conn, "receiver");

    let conv = receiver.pairwise_conversation;

    let mut builder = InboundMessageBuilder::default();
    let msg_id = [0; 32].into();
    builder
        .id(msg_id)
        .author(receiver.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .body("hi".try_into().expect(womp!()));

    let stored_meta = builder.store_db(&mut conn).expect(womp!()).expect(womp!());

    let meta = crate::message::db::conversation_message_meta(&conn, &conv)
        .expect(womp!("unable to get message"));

    assert_eq!(stored_meta.time.insertion, meta[0].insertion_time);
}

#[test]
fn matches() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    // test without id
    let mut bld = super::ConversationBuilder::new();
    bld.title("title".into());

    let conv_id = bld
        .add_db(&mut conn)
        .expect(womp!("failed to create conversation"))
        .meta
        .conversation_id;

    let conv = db::meta(&conn, &conv_id).expect(womp!());

    let pattern = search_pattern::SearchPattern::new_normal("titl".into()).expect(womp!());

    let bad_pattern = search_pattern::SearchPattern::new_normal("tilt".into()).expect(womp!());

    assert_eq!(conv.matches(&pattern), true);

    assert_eq!(conv.matches(&bad_pattern), false);
}

#[test]
#[serial(fs)]
fn set_prof_pic() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let conv_id = ConversationId::from([0; 32]);

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conv_id);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    let test_picture = "test_resources/maryland.png";

    super::db::set_picture(
        &conn,
        &conv_id,
        Some(image_utils::ProfilePicture::autocrop(
            test_picture.to_owned(),
        )),
    )
    .expect(womp!("failed to set picture"));

    std::fs::remove_dir_all(pictures_dir()).expect(womp!());
}

#[test]
fn set_muted_test() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let conv_id = ConversationId::from([0; 32]);

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conv_id);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    super::db::set_muted(&conn, &conv_id, true).expect(womp!("Unable to set mute"));

    let meta = super::db::meta(&conn, &conv_id).expect(womp!("failed to get meta"));

    assert_eq!(meta.muted, true);

    super::db::set_muted(&conn, &conv_id, false).expect(womp!("Unable to set mute"));

    let meta = super::db::meta(&conn, &conv_id).expect(womp!("failed to get meta"));

    assert_eq!(meta.muted, false);
}

#[test]
fn set_get_meta() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conv_id);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    super::db::set_color(&conn, &conv_id, 1).expect(womp!("Failed to set color"));

    super::db::set_title(&conn, &conv_id, Some("title")).expect(womp!("Failed to set title"));

    let conv_meta = super::db::meta(&conn, &conv_id).expect(womp!("Failed to get metadata"));

    assert_eq!(conv_meta.conversation_id, conv_id);
    assert_eq!(conv_meta.title.expect("failed to get title"), "title");
    assert_eq!(conv_meta.color, 1);

    let conv_id2 = ConversationId::from([1; 32]);

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conv_id2).title("hello".to_owned());
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    let all_meta = super::db::all_meta(&conn).expect(womp!("Failed to get all metadata"));

    assert_eq!(all_meta.len(), 3);
}

#[test]
fn add_remove_member() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id1 = "id1".try_into().unwrap();
    let id2 = "id2".try_into().unwrap();

    let conv_id = ConversationId::from([0; 32]);

    UserBuilder::new(id1)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id1"));

    UserBuilder::new(id2)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id2"));

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conv_id);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    crate::members::db::add_member(&conn, &conv_id, id1).expect(womp!("failed to add member"));

    crate::members::db::add_member(&conn, &conv_id, id2).expect(womp!("failed to add member"));

    let members =
        crate::members::db::members(&conn, &conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 3);

    crate::members::db::remove_member(&conn, &conv_id, id2)
        .expect(womp!("failed to remove member"));

    let members =
        crate::members::db::members(&conn, &conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 2);
}

#[test]
fn delete_message() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let author = "Hello".try_into().unwrap();
    UserBuilder::new(author).add_db(&mut conn).expect(womp!());

    let conversation = ConversationId::from([0; 32]);

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conversation);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    let mid = [0; 32].into();
    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid)
        .author(author)
        .conversation_id(conversation)
        .timestamp(Time::now())
        .body("1".try_into().expect(womp!()));
    builder.store_db(&mut conn).expect(womp!());

    crate::message::db::delete_message(&conn, &mid).expect(womp!());

    assert!(
        crate::message::db::conversation_messages(&conn, &conversation)
            .expect(womp!())
            .is_empty()
    );
}

#[test]
fn delete_conversation() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let author = "Hello".try_into().unwrap();
    UserBuilder::new(author).add_db(&mut conn).expect(womp!());

    let conversation = [0; 32].into();

    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conversation);
    bld.add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    let mid1 = [1; 32].into();
    let mut builder1 = InboundMessageBuilder::default();
    builder1
        .id(mid1)
        .author(author)
        .timestamp(Time::now())
        .conversation_id(conversation)
        .body("1".try_into().expect(womp!()));
    builder1.store_db(&mut conn).expect(womp!());

    let mid2 = [2; 32].into();
    let mut builder2 = InboundMessageBuilder::default();
    builder2
        .id(mid2)
        .author(author)
        .timestamp(Time::now())
        .conversation_id(conversation)
        .body("2".try_into().expect(womp!()));
    builder2.store_db(&mut conn).expect(womp!());

    crate::message::db::delete_conversation(&conn, &conversation).expect(womp!());

    assert!(
        crate::message::db::conversation_messages(&conn, &conversation)
            .expect(womp!())
            .is_empty()
    );
}

#[test]
fn pairwise_cids() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let uid1 = "Hello".try_into().expect(womp!());
    let u1 = UserBuilder::new(uid1).add_db(&mut conn).expect(womp!()).0;
    let uid2 = "World".try_into().expect(womp!());
    let u2 = UserBuilder::new(uid2).add_db(&mut conn).expect(womp!()).0;

    let uid3 = "GoodMorning".try_into().expect(womp!());
    UserBuilder::new(uid3).add_db(&mut conn).expect(womp!());

    let cids = db::get_pairwise_conversations(&conn, &[uid1, uid2]).expect(womp!());

    assert_eq!(cids.len(), 2);
    assert_eq!(cids[0], u1.pairwise_conversation);
    assert_eq!(cids[1], u2.pairwise_conversation);
}

#[test]
fn conversation_set_expiration() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    let mut conv = super::ConversationBuilder::new();
    conv.conversation_id(conv_id);

    conv.add_db(&mut conn)
        .expect(womp!("Failed to add conversation"));

    let meta = super::db::meta(&conn, &conv_id).expect(womp!("Failed to get metadata"));
    assert_eq!(meta.expiration_period, ExpirationPeriod::OneYear);

    super::db::set_expiration_period(&conn, &conv_id, ExpirationPeriod::ThirtySeconds)
        .expect(womp!("Failed to set expiration period"));

    let meta = super::db::meta(&conn, &conv_id).expect(womp!("Failed to get metadata"));
    assert_eq!(meta.expiration_period, ExpirationPeriod::ThirtySeconds);
}

#[test]
fn conversation_order() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    // this is our time resolution
    std::thread::sleep(std::time::Duration::from_millis(2));

    let conv_id1 = ConversationId::from([0; 32]);
    let author = "Hello".try_into().unwrap();

    // adds a pairwise conversation
    UserBuilder::new(author).add_db(&mut conn).expect(womp!()).0;

    // adds a group conversation
    let mut bld = super::ConversationBuilder::new();
    bld.conversation_id(conv_id1);
    bld.add_db(&mut conn)
        .expect(womp!("failed to add conversation"));

    // this is our time resolution
    std::thread::sleep(std::time::Duration::from_millis(2));

    // make conv_id1 the last active
    let mut builder = InboundMessageBuilder::default();
    let mid = [1; 32].into();
    builder
        .id(mid)
        .author(author)
        .conversation_id(conv_id1)
        .timestamp(Time::now())
        .body("1".try_into().expect(womp!()));
    builder.store_db(&mut conn).expect(womp!());

    let meta = super::db::all_meta(&conn).expect(womp!("Failed to get metadata"));
    assert_eq!(meta.len(), 3);
    assert_eq!(meta[0].conversation_id, conv_id1);
}
