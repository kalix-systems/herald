use crate::{
    db::{DBTable, Database},
    errors::HErr,
    message::Message,
    types::*,
    utils,
};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
/// Conversations
pub struct Conversations {
    db: Database,
}

impl Conversations {
    /// Creates `Conversations`
    pub fn new() -> Result<Self, HErr> {
        Ok(Self {
            db: Database::get()?,
        })
    }
}

/// Conversation metadata.
pub struct ConversationMeta {
    /// Conversation id
    pub conversation_id: ConversationId,
    /// Conversation title
    pub title: Option<String>,
    /// Conversation picture
    pub picture: Option<String>,
    /// Conversation color,
    pub color: u32,
    /// Indicates whether the conversation is muted
    pub muted: bool,
}

impl ConversationMeta {
    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(ConversationMeta {
            conversation_id: row.get(0)?,
            title: row.get(1)?,
            picture: row.get(2)?,
            color: row.get(3)?,
            muted: row.get(4)?,
        })
    }
}

/// Conversation
pub struct Conversation {
    /// Messages
    pub messages: Vec<Message>,

    /// User ID's of conversation members
    pub members: Vec<UserId>,

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

/// Adds a conversation to the database
pub(crate) fn add_conversation(
    db: &rusqlite::Connection,
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

    db.execute(
        include_str!("sql/conversation/add_conversation.sql"),
        params![id, title, color],
    )?;

    Ok(id)
}

/// Deletes all messages in a conversation.
pub(crate) fn delete_conversation(
    db: &Database,
    conversation_id: &ConversationId,
) -> Result<(), HErr> {
    db.execute(
        include_str!("sql/message/delete_conversation.sql"),
        &[conversation_id],
    )?;
    Ok(())
}

/// Get all messages in a conversation.
pub(crate) fn conversation_messages(
    db: &Database,
    conversation_id: &ConversationId,
) -> Result<Vec<Message>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/message/get_conversation_messages.sql"))?;
    let res = stmt.query_map(&[conversation_id], Message::from_db)?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Get conversation metadata
pub(crate) fn meta(
    db: &Database,
    conversation_id: &ConversationId,
) -> Result<ConversationMeta, HErr> {
    Ok(db.query_row(
        include_str!("sql/conversation/get_conversation_meta.sql"),
        params![conversation_id],
        ConversationMeta::from_db,
    )?)
}

/// Get metadata of all conversations
pub(crate) fn all_meta(db: &Database) -> Result<Vec<ConversationMeta>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/conversation/all_meta.sql"))?;
    let res = stmt.query_map(NO_PARAMS, ConversationMeta::from_db)?;

    let mut meta = Vec::new();
    for data in res {
        meta.push(data?);
    }

    Ok(meta)
}

/// Gets the members of a conversation.
pub(crate) fn members(
    db: &Database,
    conversation_id: &ConversationId,
) -> Result<Vec<UserId>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/members/get_conversation_members.sql"))?;
    let res = stmt.query_map(params![conversation_id], |row| row.get(0))?;

    let mut members = Vec::new();
    for member in res {
        members.push(member?);
    }

    Ok(members)
}

/// Get conversation
pub(crate) fn conversation(
    db: &Database,
    conversation_id: &ConversationId,
) -> Result<Conversation, HErr> {
    let messages = conversation_messages(&db, conversation_id)?;
    let meta = meta(&db, conversation_id)?;
    let members = members(&db, conversation_id)?;

    Ok(Conversation {
        meta,
        members,
        messages,
    })
}

impl Conversations {
    /// Adds a conversation to the database
    pub fn add_conversation(
        &self,
        conversation_id: Option<&ConversationId>,
        title: Option<&str>,
    ) -> Result<ConversationId, HErr> {
        add_conversation(&self.db, conversation_id, title)
    }

    /// Returns metadata of all conversations
    pub fn all_meta(&self) -> Result<Vec<ConversationMeta>, HErr> {
        all_meta(&self.db)
    }

    /// Deletes all messages in a conversation.
    pub fn delete_conversation(&self, conversation_id: &ConversationId) -> Result<(), HErr> {
        delete_conversation(&self.db, conversation_id)
    }

    /// Get all messages in a conversation.
    pub fn conversation_messages(
        &self,
        conversation_id: &ConversationId,
    ) -> Result<Vec<Message>, HErr> {
        conversation_messages(&self.db, conversation_id)
    }

    /// Get conversation metadata
    pub fn meta(&self, conversation_id: &ConversationId) -> Result<ConversationMeta, HErr> {
        meta(&self.db, conversation_id)
    }

    /// Adds member to conversation.
    pub fn add_member(
        &self,
        conversation_id: &ConversationId,
        member_id: UserIdRef,
    ) -> Result<(), HErr> {
        crate::members::add_member(&self.db, conversation_id, member_id)
    }

    /// Removes member from conversation.
    pub fn remove_member(
        &self,
        conversation_id: &ConversationId,
        member_id: UserIdRef,
    ) -> Result<(), HErr> {
        crate::members::remove_member(&self.db, conversation_id, member_id)
    }

    /// Gets the members of a conversation.
    pub fn members(&self, conversation_id: &ConversationId) -> Result<Vec<UserId>, HErr> {
        members(&self.db, conversation_id)
    }

    /// Get conversation
    pub fn conversation(&self, conversation_id: &ConversationId) -> Result<Conversation, HErr> {
        conversation(&self.db, conversation_id)
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
    use crate::{contact::ContactBuilder, db::Database, message::Messages};
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

        let handle = Conversations::new().expect(womp!());
        // test without id
        handle
            .add_conversation(None, None)
            .expect(womp!("failed to create conversation"));

        let conversation_id = ConversationId::from([0; 32]);
        // test with id
        assert_eq!(
            conversation_id,
            handle
                .add_conversation(Some(&conversation_id), None)
                .expect(womp!("failed to create conversation"))
        );

        handle
            .add_conversation(Some(&[1; 32].into()), Some("el groupo"))
            .expect(womp!("failed to create conversation"));

        handle
            .add_conversation(Some(&[2; 32].into()), Some("el groupo"))
            .expect(womp!("failed to create conversation"));
    }

    #[test]
    #[serial]
    fn add_and_get() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        ContactBuilder::new(author.into()).add().expect(womp!());

        let conversation = ConversationId::from([0; 32]);
        let msg_handle = Messages::new().expect(womp!());
        let conv_handle = Conversations::new().expect(womp!());

        conv_handle
            .add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        msg_handle
            .add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        msg_handle
            .add_message(None, author, &conversation, "2", None, &None)
            .expect(womp!("Failed to add second message"));

        let msgs = conv_handle
            .conversation(&conversation)
            .expect(womp!("Failed to get conversation"));

        assert_eq!(msgs.len(), 2);
    }

    #[test]
    #[serial]
    fn delete_message() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        ContactBuilder::new(author.into()).add().expect(womp!());

        let conversation = ConversationId::from([0; 32]);
        let handle = Conversations::new().expect(womp!());

        handle
            .add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let msg_handle = Messages::new().expect(womp!());
        let (msg_id, _) = msg_handle
            .add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        msg_handle.delete_message(&msg_id).expect(womp!());

        assert!(handle
            .conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }

    #[test]
    #[serial]
    fn delete_conversation() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        ContactBuilder::new(author.into()).add().expect(womp!());

        let conversation = [0; 32].into();

        let handle = Conversations::new().expect(womp!());
        handle
            .add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let msg_handle = Messages::new().expect(womp!());

        msg_handle
            .add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        msg_handle
            .add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add second message"));

        handle.delete_conversation(&conversation).expect(womp!());

        assert!(handle
            .conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }
}
