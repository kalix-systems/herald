use crate::{
    db::{DBTable, Database},
    errors::HErr,
    types::*,
};
use herald_common::UserIdRef;
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
pub(crate) struct MessageStatus;

impl MessageStatus {
    pub fn set_message_status(
        msg_id: MsgId,
        user_id: UserIdRef,
        receipt_status: MessageReceiptStatus,
    ) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message_status/set_message_status.sql"),
            params![msg_id.as_slice(), user_id, receipt_status as u8],
        )?;
        Ok(())
    }
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
    use crate::{conversation::Conversations, message::Messages};
    use serial_test_derive::serial;

    use womp::*;

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
    }

    #[test]
    #[serial]
    fn message_send_status_updates() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        let conversation_id = [0; 32].into();

        Conversations::add_conversation(Some(&conversation_id), None).expect(womp!());
        crate::contact::ContactBuilder::new(author.into())
            .add()
            .expect(womp!());
        crate::members::Members::add_member(&conversation_id, author).expect(womp!());

        let (msg_id, _) = Messages::add_message(None, author, &conversation_id, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        MessageStatus::set_message_status(msg_id, author, MessageReceiptStatus::Read)
            .expect(womp!());
    }
}
