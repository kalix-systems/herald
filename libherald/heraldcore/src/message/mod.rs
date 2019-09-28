use crate::{db::Database, errors::HErr, types::*, utils};
use chrono::{DateTime, TimeZone, Utc};
use herald_common::*;
use rusqlite::params;

/// Message
#[derive(Clone)]
pub struct Message {
    /// Local message id
    pub message_id: MsgId,
    /// Author user id
    pub author: UserId,
    /// Recipient user id
    pub conversation: ConversationId,
    /// Body of message
    pub body: String,
    /// Time the message was sent (if outbound) or received at the server (if inbound).
    pub timestamp: DateTime<Utc>,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
    /// Send status
    pub send_status: Option<MessageSendStatus>,
    /// Receipts
    pub receipts: Option<Vec<(UserId, MessageReceiptStatus)>>,
}

impl Message {
    pub(crate) fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Message {
            message_id: row.get(0)?,
            author: row.get(1)?,
            conversation: row.get(2)?,
            body: row.get(3)?,
            op: row.get(4)?,
            timestamp: Utc
                .timestamp_opt(row.get(5)?, 0)
                .single()
                .unwrap_or_else(Utc::now),
            receipts: None,
            send_status: row.get(6)?,
        })
    }
}

/// Adds a message to the database.
pub fn add_message(
    msg_id: Option<MsgId>,
    author: UserId,
    conversation_id: &ConversationId,
    body: &str,
    timestamp: Option<DateTime<Utc>>,
    op: &Option<MsgId>,
) -> Result<(MsgId, DateTime<Utc>), HErr> {
    let timestamp = timestamp.unwrap_or_else(Utc::now);

    let msg_id = msg_id.unwrap_or_else(|| utils::rand_id().into());
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/message/add.sql"),
        params![
            msg_id,
            author,
            conversation_id,
            body,
            timestamp.timestamp(),
            op,
        ],
    )?;
    Ok((msg_id, timestamp))
}

/// Get message by message id
pub fn get_message(msg_id: &MsgId) -> Result<Message, HErr> {
    let db = Database::get()?;
    Ok(db.query_row(
        include_str!("../sql/message/get_message.sql"),
        params![msg_id],
        Message::from_db,
    )?)
}
/// Sets the message status of an item in the database
pub fn update_send_status(msg_id: MsgId, status: MessageSendStatus) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/message/update_send_status.sql"),
        params![status, msg_id],
    )?;
    Ok(())
}

/// Deletes a message
pub fn delete_message(id: &MsgId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/message/delete_message.sql"),
        params![id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests;
