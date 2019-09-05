use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::{ToSql, NO_PARAMS};

#[derive(Default)]
/// Conversations
pub struct Conversations;

impl DBTable for Conversations {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/conversation/create_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/conversation/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/conversation/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }
}

/// Conversation members
#[derive(Default)]
pub struct Members;

impl Members {
    /// Add a user with `member_id` to the conversation with `conversation_id`.
    pub fn add_member(conversation_id: &str, member_id: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/conversation/members/add_member.sql"),
            &[conversation_id, member_id],
        )?;
        Ok(())
    }
}

impl DBTable for Members {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/conversation/members/create_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/conversation/members/drop_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/conversation/members/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }
}

/// Conversation metadata.
pub struct ConversationMeta {
    /// Conversation id
    pub conversation_id: String,
    /// User ID's of conversation members
    pub members: Vec<String>,
}
