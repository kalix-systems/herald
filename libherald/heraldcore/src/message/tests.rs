use super::*;
use serial_test_derive::serial;
use std::convert::TryInto;

use crate::womp;

#[test]
#[serial]
fn delete_get_message() {
    Database::reset_all().expect(womp!());

    let conv_id = [0; 32].into();

    crate::conversation::add_conversation(Some(&conv_id), None).expect(womp!());

    let contact = "contact".try_into().unwrap();

    crate::contact::ContactBuilder::new(contact)
        .add()
        .expect(womp!());

    crate::members::add_member(&conv_id, contact).expect(womp!());

    let (msg_id, _) = super::add_message(None, contact, &conv_id, "test", None, &None)
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

    crate::conversation::add_conversation(Some(&conversation_id), None).expect(womp!());

    let author = "Hello".try_into().expect(womp!());

    crate::contact::ContactBuilder::new(author)
        .add()
        .expect(womp!());

    crate::members::add_member(&conversation_id, author).expect(womp!());

    let (msg_id, _) = super::add_message(None, author, &conversation_id, "1", None, &None)
        .expect(womp!("Failed to add first message"));

    //check message id length

    assert_eq!(msg_id.into_array().len(), 32);

    assert_eq!(
        super::get_message(&msg_id)
            .expect(womp!("failed to get conversation by author"))
            .send_status,
        None,
    );

    update_send_status(msg_id, MessageSendStatus::Ack).expect(womp!());

    assert_eq!(
        crate::conversation::conversation_messages(&conversation_id)
            .expect(womp!("failed to get conversation by author"))[0]
            .send_status,
        Some(MessageSendStatus::Ack)
    );
}
