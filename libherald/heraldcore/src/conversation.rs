use crate::{
    db::{DBTable, Database},
    errors::HErr,
    message::Message,
    utils,
};
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
/// Conversations
pub struct Conversations;

impl Conversations {
    /// Adds a conversation to the database
    pub fn add_conversation(
        conversation_id: Option<&[u8]>,
        title: Option<&str>,
    ) -> Result<Vec<u8>, HErr> {
        let id = match conversation_id {
            Some(id) => {
                if id.len() != utils::RAND_ID_LEN {
                    return Err(HErr::HeraldError(format!(
                        "IDs should have {} bytes, but {} bytes were found",
                        utils::RAND_ID_LEN,
                        id.len()
                    )));
                }
                id.to_owned()
            }
            None => {
                let rand_array = utils::rand_id();
                rand_array.to_vec()
            }
        };

        let color = crate::utils::id_to_color(&id);
        let db = Database::get()?;

        db.execute(
            include_str!("sql/conversation/add_conversation.sql"),
            params![id, title, color],
        )?;

        Ok(id)
    }

    /// Deletes all messages in a conversation.
    pub fn delete_conversation(conversation_id: &[u8]) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/delete_conversation.sql"),
            &[conversation_id],
        )?;
        Ok(())
    }

    /// Get all messages with `user_id` as author or recipient.
    pub fn get_conversation(id: &[u8]) -> Result<Vec<Message>, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/message/get_conversation.sql"))?;
        let res = stmt.query_map(&[id], Message::from_db)?;

        let mut msgs = Vec::new();
        for msg in res {
            msgs.push(msg?);
        }

        Ok(msgs)
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

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/conversation/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/conversation/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
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
    use crate::{contact::Contacts, db::Database, message::Messages};
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
        Database::reset_all().expect(womp!());

        // test without id
        Conversations::add_conversation(None, None).expect(womp!("failed to create conversation"));

        // test with id
        assert_eq!(
            vec![0; 32],
            Conversations::add_conversation(Some(&[0; 32]), None)
                .expect(womp!("failed to create conversation"))
        );

        Conversations::add_conversation(Some(&[1; 32]), Some("el groupo"))
            .expect(womp!("failed to create conversation"));

        Conversations::add_conversation(Some(&[2; 32]), Some("el groupo"))
            .expect(womp!("failed to create conversation"));
    }

    #[test]
    #[serial]
    fn add_and_get() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = [0; 32];
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        Messages::add_message(None, author, &conversation, "1", None, None)
            .expect(womp!("Failed to add first message"));

        Messages::add_message(None, author, &conversation, "2", None, None)
            .expect(womp!("Failed to add second message"));

        let msgs = Conversations::get_conversation(&conversation)
            .expect(womp!("Failed to get conversation"));

        assert_eq!(msgs.len(), 2);
    }

    #[test]
    #[serial]
    fn delete_message() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = [0; 32];
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let (msg_id, _) = Messages::add_message(None, author, &conversation, "1", None, None)
            .expect(womp!("Failed to add first message"));

        Messages::delete_message(msg_id.as_slice()).expect(womp!());

        assert!(Conversations::get_conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }

    #[test]
    #[serial]
    fn delete_conversation() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = [0; 32];
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let author = "Hello";
        let conversation = [0; 32];
        Messages::add_message(None, author, &conversation, "1", None, None)
            .expect(womp!("Failed to add first message"));
        Messages::add_message(None, author, &conversation, "1", None, None)
            .expect(womp!("Failed to add second message"));

        Conversations::delete_conversation(&conversation).expect(womp!());

        assert!(Conversations::get_conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }
}
