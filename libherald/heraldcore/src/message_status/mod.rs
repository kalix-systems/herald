use crate::{
    db::{DBTable, Database},
    errors::HErr,
    types::*,
};
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
pub(crate) struct MessageStatus {}

pub(crate) fn delete_by_conversation_tx(
    tx: &rusqlite::Transaction,
    conversation: ConversationId,
) -> Result<(), HErr> {
    tx.execute(
        include_str!("../sql/message_status/delete_by_conversation.sql"),
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
        include_str!("../sql/message_status/set_message_status.sql"),
        params![msg_id, conversation, receipt_status],
    )?;
    Ok(())
}

impl DBTable for MessageStatus {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("../sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("../sql/message_status/drop_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("../sql/message_status/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(
            include_str!("../sql/message_status/drop_table.sql"),
            NO_PARAMS,
        )?;
        tx.execute(
            include_str!("../sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
