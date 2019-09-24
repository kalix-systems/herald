use crate::{
    db::{DBTable, Database},
    errors::HErr,
    types::*,
};
use herald_common::UserId;
use rusqlite::{params, NO_PARAMS};

/// Conversation members
#[derive(Default)]
pub struct Members;

/// Add a user with `member_id` to the conversation with `conversation_id`.
pub fn add_member(conversation_id: &ConversationId, member_id: UserId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/members/add_member.sql"),
        params![conversation_id, member_id],
    )?;
    Ok(())
}

/// Remove a user with `member_id` to the conversation with `conversation_id`.
pub fn remove_member(conversation_id: &ConversationId, member_id: UserId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/members/remove_member.sql"),
        params![conversation_id, member_id],
    )?;
    Ok(())
}

/// Gets the members of a conversation.
pub fn members(conversation_id: &ConversationId) -> Result<Vec<UserId>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/members/get_conversation_members.sql"))?;
    let res = stmt.query_map(params![conversation_id], |row| row.get(0))?;

    let mut members = Vec::new();
    for member in res {
        members.push(member?);
    }

    Ok(members)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::Database, womp};
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_drop_exists_reset() {
        Database::reset_all().expect(womp!());
        // drop twice, it shouldn't panic on multiple drops
        Members::drop_table().expect(womp!());
        Members::drop_table().expect(womp!());

        Members::create_table().expect(womp!());
        assert!(Members::exists().expect(womp!()));
        Members::create_table().expect(womp!());
        assert!(Members::exists().expect(womp!()));
        Members::drop_table().expect(womp!());
        assert!(!Members::exists().expect(womp!()));

        Database::reset_all().expect(womp!());

        Members::create_table().expect(womp!());
        Members::reset().expect(womp!());
    }
}
