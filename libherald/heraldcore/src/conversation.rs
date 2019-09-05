use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::{ToSql, NO_PARAMS};

#[derive(Default)]
/// Conversations
pub struct Conversations;

impl Conversations {
    /// Adds a conversation to the database
    pub fn add_conversation(
        conversation_id: Option<&[u8]>,
        name: Option<&str>,
    ) -> Result<Vec<u8>, HErr> {
        use rand::{thread_rng, RngCore};

        let id = match conversation_id {
            Some(id) => id.to_owned(),
            None => {
                let mut rng = thread_rng();
                let mut buf = [0u8; 32];
                rng.fill_bytes(&mut buf);
                buf.to_vec()
            }
        };

        let db = Database::get()?;

        db.execute(
            include_str!("sql/conversation/add_conversation.sql"),
            &[id.to_sql()?, name.to_sql()?],
        )?;

        Ok(id)
    }
}

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
    pub fn add_member(conversation_id: &[u8], member_id: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/conversation/members/add_member.sql"),
            &[conversation_id.to_sql()?, member_id.to_sql()?],
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
    pub conversation_id: Vec<u8>,
    /// User ID's of conversation members
    pub members: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;
    use womp::*;

    #[test]
    #[serial]
    fn create_drop_exists() {
        // drop twice, it shouldn't panic on multiple drops
        Conversations::drop_table().expect(womp!());
        Conversations::drop_table().expect(womp!());

        Conversations::create_table().expect(womp!());
        assert!(Conversations::exists().expect(womp!()));
        Conversations::create_table().expect(womp!());
        assert!(Conversations::exists().expect(womp!()));
        Conversations::drop_table().expect(womp!());
        assert!(!Conversations::exists().expect(womp!()));
    }

    #[test]
    #[serial]
    fn add_conversation() {
        Conversations::reset().expect(womp!());

        // test without id
        Conversations::add_conversation(None, None).expect(womp!("failed to create conversation"));

        // test with id
        assert_eq!(
            vec![0; 32],
            Conversations::add_conversation(Some(&vec![0; 32]), None)
                .expect(womp!("failed to create conversation"))
        );

        Conversations::add_conversation(Some(&vec![1; 32]), Some("el groupo"))
            .expect(womp!("failed to create conversation"));

        Conversations::add_conversation(Some(&vec![2; 32]), Some("el groupo"))
            .expect(womp!("failed to create conversation"));
    }
}
