use crate::{db::Database, errors::HErr, types::*};
use rusqlite::params;

#[derive(Default)]
pub(crate) struct MessageStatus {}

pub(crate) fn delete_by_conversation_tx(
    tx: &rusqlite::Transaction,
    conversation: ConversationId,
) -> Result<(), HErr> {
    tx.execute(
        include_str!("sql/delete_by_conversation.sql"),
        params![conversation],
    )?;
    Ok(())
}

#[allow(unused)]
pub(crate) fn set_message_status(
    msg_id: MsgId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/set_message_status.sql"),
        params![msg_id, receipt_status],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests;
