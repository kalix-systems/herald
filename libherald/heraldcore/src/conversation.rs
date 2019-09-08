use crate::{
    db::{DBTable, Database},
    errors::HErr,
    message::Message,
    utils,
};
use herald_common::{ConversationId, UserId};
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
/// Conversations
pub struct Conversations;

/// Conversation metadata.
pub struct ConversationMeta {
    /// Conversation id
    pub conversation_id: ConversationId,
    /// User ID's of conversation members
    pub members: Vec<UserId>,
}

/// Conversation
pub struct Conversation {
    /// Messages
    pub messages: Vec<Message>,

    /// Conversation metadata
    pub meta: ConversationMeta,
}

impl Conversation {
    /// Indicates whether conversation is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Number of messages in conversation
    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

impl Conversations {
    /// Adds a conversation to the database
    pub fn add_conversation(
        conversation_id: Option<&ConversationId>,
        title: Option<&str>,
    ) -> Result<ConversationId, HErr> {
        let id = match conversation_id {
            Some(id) => id.to_owned(),
            None => {
                let rand_array = utils::rand_id();
                ConversationId::from(rand_array)
            }
        };

        let color = crate::utils::id_to_color(&id);
        let db = Database::get()?;

        db.execute(
            include_str!("sql/conversation/add_conversation.sql"),
            params![id.as_slice(), title, color],
        )?;

        Ok(id)
    }

    /// Deletes all messages in a conversation.
    pub fn delete_conversation(conversation_id: &ConversationId) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/delete_conversation.sql"),
            &[conversation_id.as_slice()],
        )?;
        Ok(())
    }

    /// Get all messages in a conversation.
    pub fn get_conversation(conversation_id: &ConversationId) -> Result<Vec<Message>, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/message/get_conversation_messages.sql"))?;
        let res = stmt.query_map(&[conversation_id.as_slice()], Message::from_db)?;

        let mut messages = Vec::new();
        for msg in res {
            messages.push(msg?);
        }

        Ok(messages)
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

        let conversation_id = ConversationId::from([0; 32]);
        // test with id
        assert_eq!(
            conversation_id,
            Conversations::add_conversation(Some(&conversation_id), None)
                .expect(womp!("failed to create conversation"))
        );

        Conversations::add_conversation(Some(&[1; 32].into()), Some("el groupo"))
            .expect(womp!("failed to create conversation"));

        Conversations::add_conversation(Some(&[2; 32].into()), Some("el groupo"))
            .expect(womp!("failed to create conversation"));
    }

    #[test]
    #[serial]
    fn add_and_get() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = ConversationId::from([0; 32]);
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

        let conversation = ConversationId::from([0; 32]);
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let (msg_id, _) = Messages::add_message(None, author, &conversation, "1", None, None)
            .expect(womp!("Failed to add first message"));

        Messages::delete_message(&msg_id).expect(womp!());

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

        let conversation = [0; 32].into();
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

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
