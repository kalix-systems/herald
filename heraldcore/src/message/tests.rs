use super::*;
use serial_test_derive::serial;
use std::convert::TryInto;

use crate::{config::test_config, womp};

#[test]
#[serial]
fn delete_get_message() {
    Database::reset_all().expect(womp!());

    let conf = test_config();
    let conv_id = conf.nts_conversation;

    let (msg_id, _) = test_outbound_text("test", conv_id);

    let message = super::get_message(&msg_id).expect(womp!("unable to get message"));

    assert_eq!(message.body.expect(womp!()).as_str(), "test");

    super::delete_message(&msg_id).expect(womp!("failed to delete message"));

    assert!(super::get_message(&msg_id).is_err());
}

#[test]
#[serial]
fn reply() {
    Database::reset_all().expect(womp!());
    let conf = test_config();
    let conv_id = conf.nts_conversation;

    let (op, _) = test_outbound_text("test", conv_id);

    let mut builder = OutboundMessageBuilder::default();

    builder
        .replying_to(Some(op))
        .body("1".try_into().expect(womp!()))
        .conversation_id(conv_id);

    let reply = builder.store_and_send_blocking().unwrap();

    assert_eq!(reply.op.unwrap(), op);
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
#[serial]
fn message_receipt_status_updates() {
    use crate::contact::test_contact;

    Database::reset_all().expect(womp!());
    test_config();

    let receiver = test_contact("receiver");

    let conv = receiver.pairwise_conversation;

    let (msg_id, _) = test_outbound_text("msg", conv);

    add_receipt(msg_id, receiver.id, MessageReceiptStatus::Read).expect(womp!());
}

#[test]
#[serial]
fn receipt_before_message() {
    use crate::contact::ContactBuilder;

    Database::reset_all().expect(womp!());

    let author = "Hello".try_into().expect(womp!());

    let receiver = "World".try_into().expect(womp!());
    ContactBuilder::new(receiver).add().expect(womp!());

    let conversation_id = [0; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conversation_id)
        .add()
        .expect(womp!());

    crate::contact::ContactBuilder::new(author)
        .add()
        .expect(womp!());
    crate::members::add_member(&conversation_id, author).expect(womp!());

    let msg_id = [1; 32].into();
    add_receipt(msg_id, receiver, MessageReceiptStatus::Read).expect(womp!());

    let mut builder = InboundMessageBuilder::default();
    builder
        .id(msg_id)
        .conversation_id(conversation_id)
        .timestamp(Time::now())
        .body("1".try_into().expect(womp!()))
        .author(author);
    builder.store().expect(womp!());

    let msg = get_message(&msg_id).expect(womp!());

    assert_eq!(
        msg.receipts.get(&receiver).expect(womp!()),
        &MessageReceiptStatus::Read
    );
}
