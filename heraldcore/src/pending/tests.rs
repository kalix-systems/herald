use super::*;
use crate::{types, womp};

#[test]
fn add_get_delete() {
    let mut conn = Database::in_memory().expect(womp!());

    let pending = db::get_pending(&conn).expect(womp!());
    assert_eq!(pending.len(), 0);

    let conv_id = [0; 32].into();

    crate::conversation::ConversationBuilder::new()
        .conversation_id(conv_id)
        .add_db(&mut conn)
        .expect(womp!());

    let msg = types::cmessages::Ack {
        of: [1; 32].into(),
        stat: types::MessageReceiptStatus::NoAck,
    };

    let body = ConversationMessageBody::Ack(msg);

    db::add_to_pending(&conn, conv_id, &body).expect(womp!());
    let pending = db::get_pending(&conn).expect(womp!());
    assert_eq!(pending.len(), 1);

    db::remove_pending(&conn, pending[0].0).expect(womp!());

    let pending = db::get_pending(&conn).expect(womp!());
    assert_eq!(pending.len(), 0);
}
