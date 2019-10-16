use crate::{db::Database, errors::HErr, types::*, utils};
use chrono::{DateTime, TimeZone, Utc};
use herald_common::*;
use rusqlite::params;
use std::collections::HashMap;
// use std::path::PathBuf;

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
    pub body: Option<MessageBody>,
    /// Time the message was sent (if outbound) or received at the server (if inbound).
    pub timestamp: DateTime<Utc>,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
    /// Send status
    pub send_status: MessageSendStatus,
    /// Receipts
    pub receipts: HashMap<UserId, MessageReceiptStatus>,
}

impl Message {
    pub(crate) fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let data = row.get::<_, Vec<u8>>(6)?;
        let receipts = serde_cbor::from_slice(&data).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Blob, Box::new(e))
        })?;

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

#[derive(Default)]
/// Builder for storing outbound messages
pub struct OutboundMessageBuilder {
    /// Recipient user id
    pub conversation: Option<ConversationId>,
    /// Body of message
    pub body: Option<MessageBody>,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
}

impl OutboundMessageBuilder {
    /// Set conversation id
    pub fn conversation_id(mut self, conversation_id: ConversationId) -> Self {
        self.conversation.replace(conversation_id);
        self
    }

    /// Set body
    pub fn body(mut self, body: MessageBody) -> Self {
        self.body.replace(body);
        self
    }

    /// Set the id of the message being replied to, if this message is a reply
    pub fn replying_to(mut self, op_msg_id: MsgId) -> Self {
        self.op.replace(op_msg_id);
        self
    }

    /// Store the message
    pub fn store(self) -> Result<Message, HErr> {
        let Self {
            conversation,
            body,
            op,
        } = self;

        use MissingOutboundMessageField::*;
        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id: MsgId = utils::rand_id().into();
        let timestamp = Utc::now();
        let author = crate::config::Config::static_id()?;
        let send_status = MessageSendStatus::NoAck;

        let receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::default();
        let receipts_bytes = serde_cbor::to_vec(&receipts)?;

        let mut db = Database::get()?;
        let tx = db.transaction()?;

        tx.execute(
            include_str!("sql/add.sql"),
            params![
                msg_id,
                author,
                conversation_id,
                body,
                send_status,
                receipts_bytes,
                timestamp.timestamp(),
            ],
        )?;

        tx.execute(
            include_str!("../conversation/sql/update_last_active.sql"),
            params![timestamp.timestamp(), conversation_id],
        )?;

        if let Some(op) = op {
            tx.execute(include_str!("sql/add_reply.sql"), params![msg_id, op])?;
        }

        tx.commit()?;

        Ok(Message {
            message_id: msg_id,
            author,
            body,
            op,
            conversation: conversation_id,
            timestamp,
            send_status,
            receipts: receipts,
        })
    }
}

#[derive(Default)]
pub(crate) struct InboundMessageBuilder {
    /// Local message id
    pub message_id: Option<MsgId>,
    /// Author user id
    pub author: Option<UserId>,
    /// Recipient user id
    pub conversation: Option<ConversationId>,
    /// Body of message
    pub body: Option<MessageBody>,
    /// Time the message was sent (if outbound) or received at the server (if inbound).
    pub timestamp: Option<DateTime<Utc>>,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
}

impl InboundMessageBuilder {
    pub fn id(&mut self, msg_id: MsgId) -> &mut Self {
        self.message_id.replace(msg_id);
        self
    }

    pub fn author(&mut self, uid: UserId) -> &mut Self {
        self.author.replace(uid);
        self
    }

    pub fn conversation_id(&mut self, conversation_id: ConversationId) -> &mut Self {
        self.conversation.replace(conversation_id);
        self
    }

    pub fn body(&mut self, body: MessageBody) -> &mut Self {
        self.body.replace(body);
        self
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp.replace(ts);
        self
    }

    pub fn replying_to(&mut self, op_msg_id: MsgId) -> &mut Self {
        self.op.replace(op_msg_id);
        self
    }

    pub fn store(self) -> Result<(), HErr> {
        let Self {
            message_id,
            author,
            conversation,
            body,
            timestamp,
            op,
        } = self;

        use MissingInboundMessageField::*;

        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id = message_id.ok_or(MissingMessageId)?;
        let timestamp = timestamp.ok_or(MissingTimestamp)?;
        let author = author.ok_or(MissingAuthor)?;

        // this can be inferred from the fact that this message was received
        let send_status = MessageSendStatus::Ack;

        let mut db = Database::get()?;
        let tx = db.transaction()?;

        let receipts = get_receipts(&tx, msg_id)?.unwrap_or_default();
        let receipts = serde_cbor::to_vec(&receipts)?;

        tx.execute(
            include_str!("sql/add.sql"),
            params![
                msg_id,
                author,
                conversation_id,
                body,
                send_status,
                receipts,
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

        Ok(())
    }
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

/// Get message read receipts by message id
pub fn get_message_receipts(msg_id: &MsgId) -> Result<HashMap<UserId, MessageReceiptStatus>, HErr> {
    let db = Database::get()?;
    get_message_receipts_db(&db, msg_id)
}

/// Get message read receipts by message id
pub(crate) fn get_message_receipts_db(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<HashMap<UserId, MessageReceiptStatus>, HErr> {
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;
    let receipts: HashMap<UserId, MessageReceiptStatus> = {
        let data = get_stmt.query_row(params![msg_id], |row| row.get::<_, Vec<u8>>(0))?;
        serde_cbor::from_slice(&data)?
    };
    Ok(receipts)
}

pub(crate) fn get_receipts(
    conn: &rusqlite::Connection,
    msg_id: MsgId,
) -> Result<Option<HashMap<UserId, MessageReceiptStatus>>, HErr> {
    dbg!();
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;
    dbg!();

    let mut res = get_stmt.query(params![msg_id])?;

    match res.next()? {
        Some(row) => match row.get::<_, Option<Vec<u8>>>(0)? {
            Some(data) => Ok(Some(serde_cbor::from_slice(&data)?)),
            None => Ok(None),
        },
        None => Ok(None),
    }
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
            let mut receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::new();
            receipts.insert(recip, receipt_status);
            let data = serde_cbor::to_vec(&receipts)?;

            let mut put_pending_stmt = tx.prepare(include_str!("sql/add_pending_receipt.sql"))?;
            dbg!();
            put_pending_stmt.execute(params![msg_id, data])?;
            dbg!();
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

#[allow(unused)]
/// Testing utility
pub(crate) fn test_outbound_text(msg: &str, conv: ConversationId) -> (MsgId, DateTime<Utc>) {
    use crate::womp;
    use std::convert::TryInto;
    let out = OutboundMessageBuilder::default()
        .conversation_id(conv)
        .body("test".try_into().expect(womp!()))
        .store()
        .expect(womp!());
    (out.message_id, out.timestamp)
}

#[cfg(test)]
mod tests;
