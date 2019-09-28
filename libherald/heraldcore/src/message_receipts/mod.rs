use crate::{db::Database, errors::HErr, types::*};
use herald_common::UserId;
use rusqlite::params;

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
pub(crate) fn add_receipt(
    msg_id: MsgId,
    of: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/set_message_status.sql"),
        params![msg_id, of, receipt_status],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests;
