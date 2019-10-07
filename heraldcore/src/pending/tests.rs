use super::*;
use crate::{types, womp};
use serial_test_derive::serial;

#[test]
#[serial]
fn add_get_delete() {
    Database::reset_all().expect(womp!());

    let pending = get_pending().expect(womp!());
    assert_eq!(pending.len(), 0);

    let conv_id = [0; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conv_id)
        .add()
        .expect(womp!());

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
