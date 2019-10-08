use super::*;
use serial_test_derive::serial;
use std::convert::TryInto;

use crate::womp;

#[test]
#[serial]
fn delete_get_message() {
    Database::reset_all().expect(womp!());

    let conv_id = [0; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conv_id)
        .add()
        .expect(womp!());

    let contact = "contact".try_into().unwrap();

    crate::contact::ContactBuilder::new(contact)
        .add()
        .expect(womp!());

    crate::members::add_member(&conv_id, contact).expect(womp!());

    let (msg_id, _) = super::add_message(None, contact, &conv_id, "test", None, None, &None)
        .expect(womp!("Failed to add message"));

    let message = super::get_message(&msg_id).expect(womp!("unable to get message"));

    assert_eq!(message.body, "test");

    super::delete_message(&msg_id).expect(womp!("failed to delete message"));

    assert!(super::get_message(&msg_id).is_err());
}

#[test]
#[serial]
fn message_send_status_updates() {
    Database::reset_all().expect(womp!());

    let conversation_id = [0; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conversation_id)
        .add()
        .expect(womp!());

    let author = "Hello".try_into().expect(womp!());

    crate::contact::ContactBuilder::new(author)
        .add()
        .expect(womp!());

    crate::members::add_member(&conversation_id, author).expect(womp!());

    let (msg_id, _) = super::add_message(None, author, &conversation_id, "1", None, None, &None)
        .expect(womp!("Failed to add first message"));

    //check message id length

    assert_eq!(msg_id.into_array().len(), 32);

    assert_eq!(
        super::get_message(&msg_id)
            .expect(womp!("failed to get conversation by author"))
            .send_status,
        MessageSendStatus::NoAck,
    );

    update_send_status(msg_id, MessageSendStatus::Ack).expect(womp!());

    assert_eq!(
        by_send_status(MessageSendStatus::Ack).expect(womp!())[0].body,
        "1"
    );

    assert_eq!(
        crate::conversation::conversation_messages(&conversation_id)
            .expect(womp!("failed to get conversation by author"))[0]
            .send_status,
        MessageSendStatus::Ack
    );
}

#[test]
#[serial]
fn message_receipt_status_updates() {
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

    let (msg_id, _) = add_message(None, author, &conversation_id, "1", None, None, &None)
        .expect(womp!("Failed to add first message"));

    add_receipt(msg_id, receiver, MessageReceiptStatus::Read).expect(womp!());
}
