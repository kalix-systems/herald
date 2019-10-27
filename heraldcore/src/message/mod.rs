use crate::{db::Database, errors::HErr, types::*, utils};
use herald_common::*;
use rusqlite::params;
use std::collections::HashMap;
use std::path::PathBuf;

/// Message attachments
pub mod attachments;

pub(crate) mod db;
use attachments::*;

#[derive(Clone, Copy, Debug)]
/// Time data relating to messages
pub struct MessageTime {
    /// The `Time` the message reached the server, if applicable.
    pub server: Option<Time>,
    /// The `Time` the message was saved on this device
    pub insertion: Time,
    /// The `Time` the message will expire, if applicable
    pub expiration: Option<Time>,
}

/// Message
#[derive(Clone, Debug)]
pub struct Message {
    /// Local message id
    pub message_id: MsgId,
    /// Author user id
    pub author: UserId,
    /// Recipient user id
    pub conversation: ConversationId,
    /// Body of message
    pub body: Option<MessageBody>,
    /// Message time information
    pub time: MessageTime,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
    /// Send status
    pub send_status: MessageSendStatus,
    /// Receipts
    pub receipts: HashMap<UserId, MessageReceiptStatus>,
    /// Indicates whether the message has attachments
    pub has_attachments: bool,
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
#[derive(Debug)]
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
    /// A signal that the message has been stored successfully
    StoreDone(MsgId),
    /// A signal that the message has been sent
    SendDone(MsgId),
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
        callback: F,
    ) -> Result<(), HErr> {
        let db = Database::get()?;
        self.store_and_send_db(db, callback)
    }

    #[cfg(test)]
    pub(crate) fn store_and_send_blocking(self) -> Result<Message, HErr> {
        let db = Database::get()?;
        self.store_and_send_blocking_db(db)
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
    timestamp: Option<Time>,
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

    pub(crate) fn timestamp(&mut self, ts: Time) -> &mut Self {
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
        let mut db = Database::get()?;
        self.store_db(&mut db)
    }
}

/// Get message by message id
pub fn get_message(msg_id: &MsgId) -> Result<Message, HErr> {
    let db = Database::get()?;
    db::get_message(&db, msg_id)
}

/// Sets the message status of an item in the database
pub fn update_send_status(msg_id: MsgId, status: MessageSendStatus) -> Result<(), HErr> {
    let db = Database::get()?;
    db::update_send_status(&db, msg_id, status)
}

/// Get message read receipts by message id
pub fn get_message_receipts(msg_id: &MsgId) -> Result<HashMap<UserId, MessageReceiptStatus>, HErr> {
    let db = Database::get()?;
    Ok(db::get_receipts(&db, msg_id)?)
}

pub(crate) fn add_receipt(
    msg_id: MsgId,
    recip: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let mut db = Database::get()?;
    db::add_receipt(&mut db, msg_id, recip, receipt_status)
}

/// Gets messages by `MessageSendStatus`
pub fn by_send_status(send_status: MessageSendStatus) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    db::by_send_status(&db, send_status)
}

/// Deletes a message
pub fn delete_message(id: &MsgId) -> Result<(), HErr> {
    let db = Database::get()?;
    db::delete_message(&db, id)
}

#[cfg(test)]
mod tests;
