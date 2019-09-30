use crate::db::Database;
use crate::errors::HErr;
use crate::types::{ConversationId, ConversationMessageBody};
use rusqlite::{params, NO_PARAMS};
use serde_cbor;

/// Adds message to pending messages in database
pub(crate) fn add_to_pending(
    cid: ConversationId,
    content: &ConversationMessageBody,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/add_to_pending.sql"),
        params![cid, content],
    )?;
    Ok(())
}

pub(crate) fn get_pending() -> Result<Vec<(i64, ConversationId, ConversationMessageBody)>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_pending.sql"))?;

    let ret = stmt
        .query_map(NO_PARAMS, |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .map(|triple| Ok(triple?))
        .collect();

    ret
}

pub(crate) fn remove_pending(tag: i64) -> Result<(), HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/remove_pending.sql"))?;
    stmt.execute(params![tag])?;
    Ok(())
}

#[cfg(test)]
mod tests;
