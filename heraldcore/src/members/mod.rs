use crate::{db::Database, errors::HErr, types::*};
use herald_common::UserId;
use rusqlite::params;

/// db mod
pub mod db;

/// Add a user with `member_id` to the conversation with `conversation_id`.
pub fn add_member(conversation_id: &ConversationId, member_id: UserId) -> Result<(), HErr> {
    let db = Database::get()?;
    db::add_member_db(&db, conversation_id, member_id)
}

/// Remove a user with `member_id` to the conversation with `conversation_id`.
pub fn remove_member(conversation_id: &ConversationId, member_id: UserId) -> Result<(), HErr> {
    let db = Database::get()?;
    db::remove_member_db(&db, conversation_id, member_id)
}

/// Gets the members of a conversation.
pub fn members(conversation_id: &ConversationId) -> Result<Vec<UserId>, HErr> {
    let db = Database::get()?;
    db::members_db(&db, conversation_id)
}

#[cfg(test)]
mod tests;
