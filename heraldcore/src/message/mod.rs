use crate::{db::Database, errors::HErr, types::*, utils};
use chrono::{DateTime, TimeZone, Utc};
use herald_common::*;
use rusqlite::params;
use std::collections::HashMap;

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
    pub send_status: MessageSendStatus,
    /// Receipts
    pub receipts: Option<HashMap<UserId, MessageReceiptStatus>>,
}

impl Message {
    pub(crate) fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let receipts = match row.get::<_, Option<Vec<u8>>>(6)? {
            Some(data) => serde_cbor::from_slice(&data).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Blob,
                    Box::new(e),
                )
            })?,
            None => None,
        };

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
            receipts,
            send_status: row.get(7)?,
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
    send_status: Option<MessageSendStatus>,
    op: &Option<MsgId>,
) -> Result<(MsgId, DateTime<Utc>), HErr> {
    let timestamp = timestamp.unwrap_or_else(Utc::now);

    let msg_id = msg_id.unwrap_or_else(|| utils::rand_id().into());
    let mut db = Database::get()?;
    let tx = db.transaction()?;
    tx.execute(
        include_str!("sql/add.sql"),
        params![
            msg_id,
            author,
            conversation_id,
            body,
            send_status.unwrap_or(MessageSendStatus::NoAck),
            timestamp.timestamp(),
        ],
    )?;

    tx.execute(
        include_str!("../conversation/sql/update_last_active.sql"),
        params![Utc::now().timestamp(), conversation_id],
    )?;

    if let Some(op) = op {
        tx.execute(include_str!("sql/add_reply.sql"), params![msg_id, op])?;
    }
    tx.commit()?;
    Ok((msg_id, timestamp))
}

/// Get message by message id
pub fn get_message(msg_id: &MsgId) -> Result<Message, HErr> {
    let db = Database::get()?;
    Ok(db.query_row(
        include_str!("sql/get_message.sql"),
        params![msg_id],
        Message::from_db,
    )?)
}

/// Sets the message status of an item in the database
pub fn update_send_status(msg_id: MsgId, status: MessageSendStatus) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/update_send_status.sql"),
        params![status, msg_id],
    )?;
    Ok(())
}

pub(crate) fn add_receipt(
    msg_id: MsgId,
    of: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let mut db = Database::get()?;
    let tx = db.transaction()?;

    let mut get_stmt = tx.prepare(include_str!("sql/get_receipts.sql"))?;
    let receipts: Option<HashMap<UserId, MessageReceiptStatus>> = {
        let res = get_stmt.query_row(params![msg_id], |row| row.get::<_, Option<Vec<u8>>>(0))?;
        match res {
            Some(data) => Some(serde_cbor::from_slice(&data)?),
            None => None,
        }
    };

    let mut receipts = receipts.unwrap_or_default();
    receipts.insert(of, receipt_status);
    let data = serde_cbor::to_vec(&receipts)?;

    let mut put_stmt = tx.prepare(include_str!("sql/set_receipts.sql"))?;
    put_stmt.execute(params![data, msg_id])?;

    Ok(())
}

/// Gets messages by `MessageSendStatus`
pub fn by_send_status(send_status: MessageSendStatus) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/by_send_status.sql"))?;
    let res = stmt.query_map(&[send_status], Message::from_db)?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Deletes a message
pub fn delete_message(id: &MsgId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(include_str!("sql/delete_message.sql"), params![id])?;
    Ok(())
}

#[cfg(test)]
mod tests;
