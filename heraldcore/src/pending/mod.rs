use crate::{
    db::Database,
    errors::HErr,
    types::{ConversationId, ConversationMessageBody},
};
use rusqlite::{params, NO_PARAMS};
mod db;

/// Adds message to pending messages in database
pub(crate) fn add_to_pending(
    cid: ConversationId,
    content: &ConversationMessageBody,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::add_to_pending(&db, cid, content)
}

pub(crate) fn get_pending() -> Result<Vec<(i64, ConversationId, ConversationMessageBody)>, HErr> {
    let db = Database::get()?;
    db::get_pending(&db)
}

pub(crate) fn remove_pending(tag: i64) -> Result<(), HErr> {
    let db = Database::get()?;
    db::remove_pending(&db, tag)
}

#[cfg(test)]
mod tests;
