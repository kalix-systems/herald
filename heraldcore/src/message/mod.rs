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

    let mut db = Database::get()?;
    let tx = db.transaction()?;

    let (msg_id, receipts) = match msg_id {
        Some(mid) => {
            let receipts = get_pending_receipts(&tx, mid)?;

            delete_pending_receipts(&tx, mid)?;

            (mid, receipts)
        }
        None => {
            let empty_receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::new();
            (utils::rand_id().into(), empty_receipts)
        }
    };

    let receipts = serde_cbor::to_vec(&receipts)?;

    tx.execute(
        include_str!("sql/add.sql"),
        params![
            msg_id,
            author,
            conversation_id,
            body,
            send_status.unwrap_or(MessageSendStatus::NoAck),
            receipts,
            timestamp.timestamp(),
        ],
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

pub(crate) fn get_receipts(
    conn: &rusqlite::Connection,
    msg_id: MsgId,
) -> Result<Option<HashMap<UserId, MessageReceiptStatus>>, HErr> {
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;

    let mut res = get_stmt.query(params![msg_id])?;

    match res.next()? {
        Some(row) => {
            let data = row.get::<_, Vec<u8>>(0)?;
            Ok(Some(serde_cbor::from_slice(&data)?))
        }
        None => return Ok(None),
    }
}

pub(crate) fn get_pending_receipts(
    conn: &rusqlite::Connection,
    msg_id: MsgId,
) -> Result<HashMap<UserId, MessageReceiptStatus>, HErr> {
    let mut get_pending_receipts_stmt =
        conn.prepare(include_str!("sql/get_pending_receipts.sql"))?;

    let receipts: Result<HashMap<UserId, MessageReceiptStatus>, rusqlite::Error> =
        get_pending_receipts_stmt
            .query_map(params![msg_id], |row| {
                Ok((
                    row.get::<_, UserId>(0)?,
                    row.get::<_, MessageReceiptStatus>(1)?,
                ))
            })?
            .collect();

    Ok(receipts?)
}

pub(crate) fn delete_pending_receipts(
    conn: &rusqlite::Connection,
    msg_id: MsgId,
) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/delete_pending_receipts.sql"))?;
    stmt.execute(params![msg_id])?;
    Ok(())
}

pub(crate) fn add_receipt(
    msg_id: MsgId,
    recip: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let mut db = Database::get()?;
    let tx = db.transaction()?;

    // check if message exists yet
    let mut receipts = get_receipts(&tx, msg_id)?;

    match receipts {
        Some(ref mut receipts) => {
            // update receipts

            match receipts.get(&recip) {
                Some(old_status) => {
                    if (*old_status as u8) < (receipt_status as u8) {
                        receipts.insert(recip, receipt_status);
                    }
                }
                None => {
                    receipts.insert(recip, receipt_status);
                }
            }

            receipts.insert(recip, receipt_status);
            let data = serde_cbor::to_vec(&receipts)?;

            let mut put_stmt = tx.prepare(include_str!("sql/set_receipts.sql"))?;
            put_stmt.execute(params![data, msg_id])?;
        }
        None => {
            let mut put_pending_stmt = tx.prepare(include_str!("sql/add_pending_receipt.sql"))?;
            put_pending_stmt.execute(params![msg_id, recip, receipt_status])?;
        }
    }

    tx.commit()?;

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
