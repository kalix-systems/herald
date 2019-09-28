use super::*;
use super::*;
use crate::message_status;
use crate::types;
use crate::womp;
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
#[serial]
fn add_get_delete() {
    Database::reset_all().expect(womp!());

    let pending = get_pending().expect(womp!());
    assert_eq!(pending.len(), 0);

    let conv_id = [0; 32].into();

    crate::conversation::add_conversation(Some(&conv_id), None).expect(womp!());

    let msg = types::cmessages::Ack {
        of: [1; 32].into(),
        stat: types::MessageReceiptStatus::NoAck,
    };

    let body = ConversationMessageBody::Ack(msg);

    add_to_pending(conv_id, &body).expect(womp!());
    let pending = get_pending().expect(womp!());
    assert_eq!(pending.len(), 1);

    remove_pending(pending[0].0).expect(womp!());

    let pending = get_pending().expect(womp!());
    assert_eq!(pending.len(), 0);
}
