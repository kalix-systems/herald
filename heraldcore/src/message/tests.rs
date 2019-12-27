use super::*;
use crate::config::test_config;
use serial_test_derive::serial;
use std::convert::TryInto;

/// Testing utility
fn test_outbound_text(
    msg: &str,
    conv: ConversationId,
) -> (MsgId, Time) {
    let mut conn = Database::get().expect(womp!());
    db::test_outbound_text(&mut conn, msg, conv)
}

#[test]
fn delete_get_message() {
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

    builder.store_db(&mut conn).expect(womp!());

    let message = db::get_message(&conn, &msg_id).expect(womp!("unable to get message"));

    assert_eq!(message.body.expect(womp!()).as_str(), "hi");

    db::delete_message(&conn, &msg_id).expect(womp!("failed to delete message"));

    assert!(db::get_message(&conn, &msg_id).is_err());
}

#[test]
fn message_meta() {
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

    let meta = db::message_meta(&conn, &msg_id).expect(womp!("unable to get message"));

    assert_eq!(stored_meta.time.insertion, meta.insertion_time);
}

#[test]
fn reaction() {
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

    builder.store_db(&mut conn).expect(womp!()).expect(womp!());

    assert!(db::reactions(&conn, &msg_id).expect(womp!()).is_none());

    db::add_reaction(&conn, &msg_id, &receiver.id, "++").expect(womp!());

    let reactions = db::reactions(&conn, &msg_id)
        .expect(womp!())
        .expect(womp!());
    assert_eq!(reactions.content.len(), 1);

    db::remove_reaction(&conn, &msg_id, &receiver.id, "++").expect(womp!());

    let reactions = db::reactions(&conn, &msg_id).expect(womp!());
    assert!(reactions.is_none());
}

#[test]
fn message_data() {
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

    let stored = builder.store_db(&mut conn).expect(womp!()).expect(womp!());

    let data = db::message_data(&conn, &msg_id).expect(womp!("unable to get message"));

    assert_eq!(stored.time.insertion, data.time.insertion);
}

#[test]
fn reply() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let author = "Hello".try_into().unwrap();
    crate::user::UserBuilder::new(author)
        .add_db(&mut conn)
        .expect(womp!());

    let conversation = [1; 32].into();

    let mut builder = crate::conversation::ConversationBuilder::new();
    builder.conversation_id(conversation);
    builder
        .add_db(&mut conn)
        .expect(womp!("Failed to create conversation"));

    let mid1 = [0; 32].into();
    let mid2 = [1; 32].into();
    let mut builder = InboundMessageBuilder::default();

    builder
        .id(mid1)
        .author(author)
        .conversation_id(conversation)
        .timestamp(Time::now())
        .body("1".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    let mut builder = InboundMessageBuilder::default();

    builder
        .id(mid2)
        .author(author)
        .replying_to(mid1)
        .timestamp(Time::now())
        .body("1".try_into().expect(womp!()))
        .conversation_id(conversation);

    builder.store_db(&mut conn).expect(womp!());

    let op_replies = db::replies(&conn, &mid1).expect(womp!());
    let op = db::get_message(&conn, &mid1).expect(womp!());
    assert_eq!(op.replies.len(), 1);
    assert_eq!(op_replies.len(), 1);

    assert!(op.replies.contains(&mid2));
    assert!(op_replies.contains(&mid2));

    let reply = db::get_message(&conn, &mid2).expect(womp!());

    assert_eq!(ReplyId::Known(mid1), reply.op);
}

#[test]
#[serial]
fn message_send_status_updates() {
    Database::reset_all().expect(womp!());

    let conf = test_config();
    let conv_id = conf.nts_conversation;

    let (msg_id, _) = test_outbound_text("test", conv_id);
    assert_eq!(
        super::get_message(&msg_id)
            .expect(womp!("failed to get conversation by author"))
            .send_status,
        MessageSendStatus::NoAck,
    );

    update_send_status(msg_id, MessageSendStatus::Ack).expect(womp!());

    assert_eq!(
        by_send_status(MessageSendStatus::Ack).expect(womp!())[0]
            .body
            .as_ref()
            .expect(womp!())
            .as_str(),
        "test"
    );

    assert_eq!(
        crate::conversation::conversation_messages(&conv_id)
            .expect(womp!("failed to get conversation by author"))[0]
            .send_status,
        MessageSendStatus::Ack
    );
}

#[test]
fn message_receipt_status_updates() {
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

    builder.store_db(&mut conn).expect(womp!());

    db::add_receipt(
        &mut conn,
        msg_id,
        receiver.id,
        MessageReceiptStatus::Received,
    )
    .expect(womp!());

    let receipts = db::get_receipts(&conn, &msg_id).expect(womp!());

    let receipt = receipts.get(&receiver.id).expect(womp!());
    assert_eq!(*receipt, MessageReceiptStatus::Received);

    db::add_receipt(&mut conn, msg_id, receiver.id, MessageReceiptStatus::Read).expect(womp!());

    let receipts = db::get_receipts(&conn, &msg_id).expect(womp!());
    let receipt = receipts.get(&receiver.id).expect(womp!());
    assert_eq!(*receipt, MessageReceiptStatus::Read);
}

#[test]
fn reply_to_unknown_message() {
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
        .replying_to([1; 32].into())
        .body("hi".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    let msg = db::get_message(&conn, &msg_id).expect(womp!());

    assert!(msg.op.is_dangling());
}

#[test]
fn delete_op() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let receiver = crate::user::db::test_user(&mut conn, "receiver");

    let conv = receiver.pairwise_conversation;

    let mid0 = [0; 32].into();

    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid0)
        .author(receiver.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .body("hi".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    let mid1 = [1; 32].into();

    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid1)
        .author(receiver.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .replying_to(mid0)
        .body("hi".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    let msg = db::get_message(&conn, &mid1).expect(womp!());

    assert!(msg.op.is_known());

    db::delete_message(&conn, &mid0).expect(womp!());

    let msg = db::get_message(&conn, &mid1).expect(womp!());

    assert!(msg.op.is_dangling());
}

#[test]
fn add_delete_reaction() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let mid = [0; 32].into();
    let author = crate::user::db::test_user(&mut conn, "author");

    let conv = author.pairwise_conversation;
    let mut builder = InboundMessageBuilder::default();

    builder
        .id(mid)
        .author(author.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .body("test".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    db::add_reaction(&conn, &mid, &author.id, ":)").expect(womp!("failed to add react"));

    //make sure adding the same react from the same userdoes not get registered
    db::add_reaction(&conn, &mid, &author.id, ":)".try_into().expect(womp!()))
        .expect(womp!("failed to add react"));
    let reacts = db::reactions(&conn, &mid).unwrap().expect(womp!());
    assert_eq!(reacts.content[0].content, ":)");
    assert_eq!(reacts.content.len(), 1);
    assert_eq!(reacts.content[0].reactionaries[0], author.id);
    db::remove_reaction(&conn, &mid, &author.id, ":)").expect(womp!("failed to delete react"));

    let reacts = db::reactions(&conn, &mid).expect(womp!());
    assert!(reacts.is_none());
}
