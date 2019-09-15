use crate::{
    db::{DBTable, Database},
    errors::HErr,
    types::*,
};
use herald_common::UserIdRef;
use rusqlite::{params, NO_PARAMS};

/// Conversation members
#[derive(Default)]
pub struct Members;

impl Members {
    /// Add a user with `member_id` to the conversation with `conversation_id`.
    pub fn add_member(conversation_id: &ConversationId, member_id: UserIdRef) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/members/add_member.sql"),
            params![conversation_id.as_slice(), member_id],
        )?;
        Ok(())
    }

    /// Remove a user with `member_id` to the conversation with `conversation_id`.
    pub fn remove_member(
        conversation_id: &ConversationId,
        member_id: UserIdRef,
    ) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/members/remove_member.sql"),
            params![conversation_id.as_slice(), member_id],
        )?;
        Ok(())
    }
}

impl DBTable for Members {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/members/create_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/members/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/members/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/members/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/members/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}
