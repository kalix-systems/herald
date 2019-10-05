use super::*;
use crate::{contact::ContactBuilder, message::add_message};
use serial_test_derive::serial;
use std::convert::TryInto;

use crate::womp;

#[test]
#[serial]
fn message_send_status_updates() {
    Database::reset_all().expect(womp!());

    let author = "Hello".try_into().expect(womp!());

    let receiver = "World".try_into().expect(womp!());
    ContactBuilder::new(receiver).add().expect(womp!());

    let conversation_id = [0; 32].into();

    crate::conversation::add_conversation(Some(&conversation_id), None).expect(womp!());

    crate::contact::ContactBuilder::new(author)
        .add()
        .expect(womp!());
    crate::members::add_member(&conversation_id, author).expect(womp!());

    let (msg_id, _) = add_message(None, author, &conversation_id, "1", None, None, &None)
        .expect(womp!("Failed to add first message"));

    add_receipt(msg_id, receiver, MessageReceiptStatus::Read).expect(womp!());
}
