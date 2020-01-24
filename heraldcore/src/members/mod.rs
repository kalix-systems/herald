use crate::{db::Database, errors::HErr, types::*};
use coremacros::{from_fn, w};
use herald_common::UserId;
use network_types::Membership;
use rusqlite::params;

from_fn!(
    crate::Notification,
    Membership,
    crate::Notification::Membership
);

pub(crate) mod db;

/// Add a user with `member_id` to the conversation with `conversation_id`.
pub fn add_member(
    conversation_id: &ConversationId,
    member_id: &UserId,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::add_member(&db, conversation_id, member_id)
}

pub(crate) fn add_members(
    cid: &ConversationId,
    members: &[UserId],
) -> Result<(), HErr> {
    let mut db = Database::get()?;

    let tx = db.transaction()?;
    for member in members {
        w!(db::add_member(&tx, &cid, member));
    }
    tx.commit()?;

    Ok(())
}

/// Remove a user with `member_id` to the conversation with `conversation_id`.
pub fn remove_member(
    conversation_id: &ConversationId,
    member_id: UserId,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::remove_member(&db, conversation_id, member_id)
}

/// Gets the members of a conversation.
pub fn members(conversation_id: &ConversationId) -> Result<Vec<UserId>, HErr> {
    let db = Database::get()?;
    db::members(&db, conversation_id)
}

/// Gets the conversations shared with a user
pub fn shared_conversations(user_id: &UserId) -> Result<Vec<ConversationId>, HErr> {
    let conn = Database::get()?;

    Ok(w!(db::shared_conversations(&conn, user_id)))
}

#[cfg(test)]
mod tests;
