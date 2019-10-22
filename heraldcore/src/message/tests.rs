use super::*;
use serial_test_derive::serial;
use std::convert::TryInto;

use crate::{config::test_config, womp};

/// Testing utility
fn test_outbound_text(msg: &str, conv: ConversationId) -> (MsgId, Time) {
    let conn = Database::get().expect(womp!());
    db::test_outbound_text(conn, msg, conv)
}

#[test]
fn delete_get_message() {
    let mut conn = Database::in_memory().expect(womp!());

    let receiver = crate::contact::db::test_contact(&mut conn, "receiver");

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
fn reply() {
    let mut conn = Database::in_memory().expect(womp!());

    let author = "Hello".try_into().unwrap();
    crate::contact::ContactBuilder::new(author)
        .add_db(&mut conn)
        .expect(womp!());

    let conversation = [1; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conversation)
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

    let reply = db::get_message(&conn, &mid2).unwrap();

    assert_eq!(reply.op.unwrap(), mid1);
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
    let mut conn = Database::in_memory().expect(womp!());

    let receiver = crate::contact::db::test_contact(&mut conn, "receiver");

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

    let receipts = db::get_receipts(&conn, msg_id)
        .expect(womp!())
        .expect(womp!());

    let receipt = receipts.get(&receiver.id).expect(womp!());
    assert_eq!(*receipt, MessageReceiptStatus::Received);

    db::add_receipt(&mut conn, msg_id, receiver.id, MessageReceiptStatus::Read).expect(womp!());

    let receipts = db::get_receipts(&conn, msg_id)
        .expect(womp!())
        .expect(womp!());
    let receipt = receipts.get(&receiver.id).expect(womp!());
    assert_eq!(*receipt, MessageReceiptStatus::Read);
}

#[test]
fn receipt_before_message() {
    use crate::contact::ContactBuilder;

    let mut conn = Database::in_memory().expect(womp!());

    let author = "Hello".try_into().expect(womp!());

    let receiver = "World".try_into().expect(womp!());
    ContactBuilder::new(receiver)
        .add_db(&mut conn)
        .expect(womp!());

    let conversation_id = [0; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conversation_id)
        .add_db(&mut conn)
        .expect(womp!());

    crate::contact::ContactBuilder::new(author)
        .add_db(&mut conn)
        .expect(womp!());
    crate::members::db::add_member(&conn, &conversation_id, author).expect(womp!());

    let msg_id = [1; 32].into();
    db::add_receipt(&mut conn, msg_id, receiver, MessageReceiptStatus::Read).expect(womp!());

    let mut builder = InboundMessageBuilder::default();
    builder
        .id(msg_id)
        .conversation_id(conversation_id)
        .timestamp(Time::now())
        .body("1".try_into().expect(womp!()))
        .author(author);
    builder.store_db(&mut conn).expect(womp!());

    let msg = db::get_message(&conn, &msg_id).expect(womp!());

    assert_eq!(
        msg.receipts.get(&receiver).expect(womp!()),
        &MessageReceiptStatus::Read
    );
}
