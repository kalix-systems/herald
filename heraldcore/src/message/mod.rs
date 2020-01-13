use crate::{db::Database, errors::HErr, types::*};
use coremacros::{from_fn, w};
use herald_common::*;
use rusqlite::params;
use std::{collections::HashMap, path::PathBuf};

/// Message attachments
pub mod attachments;

pub(crate) mod db;
/// Runs message garbage collection tasks such as removing expired messages
pub mod gc;
pub use coretypes::messages::*;
mod builder;
pub use builder::*;
mod search;
pub use search::{ResultBody, Search, SearchResult};

/// Outbound settings send updates
#[derive(Clone, Debug)]
pub enum OutboundAux {
    /// Render message
    Msg(Box<Message>),
    /// Finished send
    SendDone(ConversationId, MsgId),
}

from_fn!(
    crate::updates::Notification,
    OutboundAux,
    crate::updates::Notification::OutboundAux
);

/// Get message by message id
pub fn get_message(msg_id: &MsgId) -> Result<Message, HErr> {
    let db = Database::get()?;
    db::get_message(&db, msg_id)
}

/// Get message metadata by message id
pub fn message_meta(msg_id: &MsgId) -> Result<MessageMeta, HErr> {
    let db = Database::get()?;
    db::message_meta(&db, msg_id)
}

/// Get message data by message id
pub fn message_data(msg_id: &MsgId) -> Result<Option<MsgData>, HErr> {
    let db = Database::get()?;
    db::message_data(&db, msg_id)
}

/// Gets a message by message id. If the message cannot be found, it returns an option rather than
/// an error.
pub fn get_message_opt(msg_id: &MsgId) -> Result<Option<Message>, HErr> {
    let db = Database::get()?;
    db::get_message_opt(&db, msg_id)
}

/// Sets the message status of an item in the database
pub fn update_send_status(
    msg_id: MsgId,
    status: SendStatus,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::update_send_status(&db, msg_id, status)
}

/// Get message read receipts by message id
pub fn get_message_receipts(msg_id: &MsgId) -> Result<HashMap<UserId, ReceiptStatus>, HErr> {
    let db = Database::get()?;
    Ok(db::receipts::get_receipts(&db, msg_id)?)
}

/// Adds a message receipt
pub fn add_receipt(
    msg_id: MsgId,
    recip: UserId,
    receipt_status: ReceiptStatus,
) -> Result<(), HErr> {
    let db = Database::get()?;

    w!(db::receipts::add_receipt(
        &db,
        msg_id,
        recip,
        receipt_status
    ));

    Ok(())
}

/// Adds a reaction to a message
pub fn add_reaction(
    msg_id: &MsgId,
    reactionary: &UserId,
    react_content: &str,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::reactions::add_reaction(&db, msg_id, reactionary, react_content).map_err(HErr::from)
}

/// Removes a reaction from a message
pub fn remove_reaction(
    msg_id: &MsgId,
    reactionary: &UserId,
    react_content: &str,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::reactions::remove_reaction(&db, msg_id, reactionary, react_content).map_err(HErr::from)
}

/// Gets messages by `SendStatus`
pub fn by_send_status(send_status: SendStatus) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    db::by_send_status(&db, send_status)
}

/// Deletes a message
pub fn delete_message(id: &MsgId) -> Result<(), HErr> {
    let db = Database::get()?;
    db::delete_message(&db, id)
}

/// Deletes all messages in a conversation.
pub fn delete_conversation_messages(conversation_id: &ConversationId) -> Result<(), HErr> {
    let db = Database::get()?;
    db::delete_conversation(&db, conversation_id)
}

/// Get all messages in a conversation.
pub fn conversation_messages(conversation_id: &ConversationId) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    db::conversation_messages(&db, conversation_id)
}

/// Get all message metadata in a conversation.
pub fn conversation_message_meta(
    conversation_id: &ConversationId
) -> Result<Vec<crate::message::MessageMeta>, HErr> {
    let db = Database::get()?;
    db::conversation_message_meta(&db, conversation_id)
}

#[cfg(test)]
mod tests;
