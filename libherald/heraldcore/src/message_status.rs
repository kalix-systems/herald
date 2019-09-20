use crate::{
    db::{DBTable, Database},
    errors::HErr,
    types::*,
};
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
pub(crate) struct MessageStatus {}

//pub(crate) fn delete_by_conversation(conversation: ConversationId) -> Result<(), HErr> {
//    let db = Database::get()?;
//    db.execute(
//        include_str!("sql/message_status/delete_by_conversation.sql"),
//        params![conversation],
//    )?;
//    Ok(())
//}

pub(crate) fn delete_by_conversation_tx(
    tx: &rusqlite::Transaction,
    conversation: ConversationId,
) -> Result<(), HErr> {
    tx.execute(
        include_str!("sql/message_status/delete_by_conversation.sql"),
        params![conversation],
    )?;
    Ok(())
}

pub(crate) fn set_message_status(
    msg_id: MsgId,
    conversation: ConversationId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/message_status/set_message_status.sql"),
        params![msg_id, conversation, receipt_status],
    )?;
    Ok(())
}

impl DBTable for MessageStatus {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/message_status/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/message_status/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/message_status/drop_table.sql"), NO_PARAMS)?;
        tx.execute(
            include_str!("sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{conversation::Conversations, message::add_message};
    use serial_test_derive::serial;

    use crate::womp;

    #[test]
    #[serial]
    fn create_drop_exists() {
        Database::reset_all().expect(womp!());
        // drop twice, it shouldn't panic on multiple drops
        MessageStatus::drop_table().expect(womp!());
        MessageStatus::drop_table().expect(womp!());

        MessageStatus::create_table().expect(womp!());
        assert!(MessageStatus::exists().expect(womp!()));
        MessageStatus::create_table().expect(womp!());
        assert!(MessageStatus::exists().expect(womp!()));
        MessageStatus::drop_table().expect(womp!());
        assert!(!MessageStatus::exists().expect(womp!()));

        Database::reset_all().expect(womp!());

        MessageStatus::create_table().expect(womp!());

        MessageStatus::reset().expect(womp!());
    }

    #[test]
    #[serial]
    fn message_send_status_updates() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        let conversation_id = [0; 32].into();

        let conv_handle = Conversations::new();

        conv_handle
            .add_conversation(Some(&conversation_id), None)
            .expect(womp!());

        crate::contact::ContactBuilder::new(author.into())
            .add()
            .expect(womp!());
        conv_handle
            .add_member(&conversation_id, author)
            .expect(womp!());

        let (msg_id, _) = add_message(None, author, &conversation_id, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        set_message_status(msg_id, conversation_id, MessageReceiptStatus::Read).expect(womp!());
    }
}
