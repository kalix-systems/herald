use crate::{channel_recv_err, db::Database, errors::HErr, loc, types::*, utils};
use chrono::{DateTime, TimeZone, Utc};
use herald_common::*;
use rusqlite::params;
use std::collections::HashMap;
use std::path::PathBuf;

/// Message attachments
pub mod attachments;
use attachments::*;

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
    /// Indicates whether the message has attachments
    pub has_attachments: bool,
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
            has_attachments: row.get(8)?,
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
    /// Attachments
    pub attachments: Vec<PathBuf>,
    /// Whether to treat the value as markdown
    pub parse_markdown: bool,
}

/// Values `OutboundMessageBuilder`'s `store_and_send` function
/// can pass into the callback.
#[allow(clippy::large_enum_variant)]
pub enum StoreAndSend {
    /// The message being stored and sent
    Msg(Message),
    /// An error accompanied by the line number it occured on
    Error {
        /// The error
        error: HErr,
        /// The line number the error occured on
        line_number: u32,
    },
    /// A signal that the process has completed successfully
    Done(MsgId),
}

impl OutboundMessageBuilder {
    /// Set conversation id
    pub fn conversation_id(&mut self, conversation_id: ConversationId) -> &mut Self {
        self.conversation.replace(conversation_id);
        self
    }

    /// Set body
    pub fn body(&mut self, body: MessageBody) -> &mut Self {
        self.body.replace(body);
        self
    }

    /// Set the id of the message being replied to, if this message is a reply
    pub fn replying_to(&mut self, op_msg_id: Option<MsgId>) -> &mut Self {
        self.op = op_msg_id;
        self
    }

    /// Add attachment
    pub fn add_attachment(&mut self, path: PathBuf) -> &mut Self {
        self.attachments.push(path);
        self
    }

    /// Parses the text as markdown, if possible, rendering it to HTML
    pub fn parse_markdown(&mut self) -> &mut Self {
        if let Some(body) = &self.body {
            if let Ok(md) = body.parse_markdown() {
                self.body.replace(md);
            }
        }
        self
    }

    /// Stores and sends the message
    pub fn store_and_send<F: FnMut(StoreAndSend) + Send + 'static>(
        self,
        mut callback: F,
    ) -> Result<(), HErr> {
        let Self {
            conversation,
            mut body,
            op,
            attachments,
            parse_markdown,
        } = self;

        use MissingOutboundMessageField::*;

        if parse_markdown {
            body = match body {
                Some(body) => Some(body.parse_markdown()?),
                None => None,
            };
        }

        if attachments.is_empty() && body.is_none() {
            return Err(MissingBody.into());
        }

        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id: MsgId = utils::rand_id().into();
        let timestamp = Utc::now();
        let author = crate::config::Config::static_id()?;
        let send_status = MessageSendStatus::NoAck;

        let receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::default();
        let receipts_bytes = serde_cbor::to_vec(&receipts)?;
        let has_attachments = !attachments.is_empty();

        let msg = Message {
            message_id: msg_id,
            author,
            body: (&body).clone(),
            op,
            conversation: conversation_id,
            timestamp,
            send_status,
            receipts,
            has_attachments,
        };

        callback(StoreAndSend::Msg(msg));

        macro_rules! e {
            ($res: expr) => {
                match $res {
                    Ok(val) => val,
                    Err(e) => {
                        callback(StoreAndSend::Error {
                            error: e.into(),
                            line_number: line!(),
                        });
                        return;
                    }
                }
            };
        }

        std::thread::Builder::new().spawn(move || {
            let attachments: Result<Vec<Attachment>, HErr> = attachments
                .into_iter()
                .map(|path| {
                    let attach: Attachment = Attachment::new(&path)?;

                    attach.save()?;

                    Ok(attach)
                })
                .collect();
            let attachments = e!(attachments);

            let mut db = e!(Database::get());
            let tx = e!(db.transaction());

            e!(tx.execute(
                include_str!("sql/add.sql"),
                params![
                    msg_id,
                    author,
                    conversation_id,
                    body,
                    send_status,
                    receipts_bytes,
                    has_attachments,
                    timestamp.timestamp(),
                ],
            ));

            e!(tx.execute(
                include_str!("../conversation/sql/update_last_active.sql"),
                params![timestamp.timestamp(), conversation_id],
            ));

            if let Some(op) = op {
                e!(tx.execute(include_str!("sql/add_reply.sql"), params![msg_id, op]));
            }

            if !attachments.is_empty() {
                e!(attachments::add_db(
                    &tx,
                    &msg_id,
                    attachments.iter().map(|a| a.hash_dir())
                ));
            }

            e!(tx.commit());

            let content = cmessages::Message { body, attachments };
            let msg = cmessages::Msg {
                mid: msg_id,
                content,
                op,
            };
            e!(crate::network::send_normal_message(conversation_id, msg));

            callback(StoreAndSend::Done(msg_id));
        })?;

        Ok(())
    }

    // NOTE: This function should probably remain only public to the crate.
    pub(crate) fn store_and_send_blocking(self) -> Result<Message, HErr> {
        use crossbeam_channel::*;

        let (tx, rx) = bounded(2);
        self.store_and_send(move |m| {
            tx.send(m)
                .unwrap_or_else(|_| panic!("Send error at {}", loc!()));
        })?;

        let out = match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::Msg(msg) => msg,
            // TODO use line number
            StoreAndSend::Error { error, .. } => return Err(error),
            StoreAndSend::Done(msg_id) => {
                panic!("Unexpected `Done` variant with {:?}", msg_id);
            }
        };

        match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::Done(_) => Ok(out),
            StoreAndSend::Error { error, line_number } => {
                panic!("error {} at {}", error, line_number);
            }
            StoreAndSend::Msg(_) => {
                panic!("Message should not be sent twice");
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct InboundMessageBuilder {
    /// Local message id
    message_id: Option<MsgId>,
    /// Author user id
    author: Option<UserId>,
    /// Recipient user id
    conversation: Option<ConversationId>,
    /// Body of message
    body: Option<MessageBody>,
    /// Time the message was sent (if outbound) or received at the server (if inbound).
    timestamp: Option<DateTime<Utc>>,
    /// Message id of the message being replied to
    op: Option<MsgId>,
    attachments: Vec<attachments::Attachment>,
}

impl InboundMessageBuilder {
    pub(crate) fn id(&mut self, msg_id: MsgId) -> &mut Self {
        self.message_id.replace(msg_id);
        self
    }

    pub(crate) fn author(&mut self, uid: UserId) -> &mut Self {
        self.author.replace(uid);
        self
    }

    pub(crate) fn conversation_id(&mut self, conversation_id: ConversationId) -> &mut Self {
        self.conversation.replace(conversation_id);
        self
    }

    pub(crate) fn body(&mut self, body: MessageBody) -> &mut Self {
        self.body.replace(body);
        self
    }

    pub(crate) fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp.replace(ts);
        self
    }

    pub(crate) fn replying_to(&mut self, op_msg_id: MsgId) -> &mut Self {
        self.op.replace(op_msg_id);
        self
    }

    pub(crate) fn attachments(&mut self, attachments: Vec<attachments::Attachment>) -> &mut Self {
        self.attachments = attachments;
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
            attachments,
        } = self;

        use MissingInboundMessageField::*;

        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id = message_id.ok_or(MissingMessageId)?;
        let timestamp = timestamp.ok_or(MissingTimestamp)?;
        let author = author.ok_or(MissingAuthor)?;

        let res: Result<Vec<PathBuf>, HErr> = attachments.into_iter().map(|a| a.save()).collect();
        let attachment_paths = res?;
        let has_attachments = !attachment_paths.is_empty();

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
                has_attachments,
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

        if has_attachments {
            attachments::add_db(&tx, &msg_id, attachment_paths.iter().map(|p| p.as_path()))?;
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
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;

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
            put_pending_stmt.execute(params![msg_id, data])?;
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

    let mut builder = OutboundMessageBuilder::default();
    builder.conversation_id(conv).body(
        "test"
            .try_into()
            .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!())),
    );
    let out = builder
        .store_and_send_blocking()
        .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!()));

    (out.message_id, out.timestamp)
}

#[cfg(test)]
mod tests;
