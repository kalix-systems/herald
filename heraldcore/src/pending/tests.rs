use super::*;
use crate::{message::ReceiptStatus, types};

#[test]
fn add_get_delete() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let pending = db::get_pending(&conn).expect(womp!());
    assert_eq!(pending.len(), 0);

    let conv_id = [0; 32].into();

    let mut builder = crate::conversation::ConversationBuilder::new();
    builder.conversation_id(conv_id);
    builder.add_db(&mut conn).expect(womp!());

    let msg = types::cmessages::Receipt {
        of: [1; 32].into(),
        stat: ReceiptStatus::Received,
    };

    let body = ConversationMessage::Message(network_types::cmessages::Content::Receipt(msg));

    db::add_to_pending(&conn, conv_id, &body).expect(womp!());
    let pending = db::get_pending(&conn).expect(womp!());
    assert_eq!(pending.len(), 1);

    db::remove_pending(&conn, pending[0].0).expect(womp!());

    let pending = db::get_pending(&conn).expect(womp!());
    assert_eq!(pending.len(), 0);
}
