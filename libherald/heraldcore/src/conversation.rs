use crate::{
    db::{DBTable, Database},
    errors::HErr,
    message::Message,
    types::*,
    utils,
};
use chrono::{DateTime, Utc};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};
use std::convert::TryInto;

#[derive(Default)]
/// Conversations
pub struct Conversations {}

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
    /// Indicates whether the conversation is a canonical pairwise conversation
    pub pairwise: bool,
}

impl ConversationMeta {
    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(ConversationMeta {
            conversation_id: row.get(0)?,
            title: row.get(1)?,
            picture: row.get(2)?,
            color: row.get(3)?,
            muted: row.get(4)?,
            pairwise: row.get(5)?,
        })
    }

    /// Matches contact's text fields against a [`SearchPattern`]
    pub fn matches(&self, pattern: &crate::utils::SearchPattern) -> bool {
        match self.title.as_ref() {
            Some(name) => pattern.is_match(name),
            None => false,
        }
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
pub(crate) fn add_conversation_db(
    conversation_id: Option<&ConversationId>,
    title: Option<&str>,
    pairwise: bool,
) -> Result<ConversationId, HErr> {
    let db = Database::get()?;
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
        params![id, title, color, pairwise],
    )?;

    Ok(id)
}

/// Adds a non-pairwise conversation
pub fn add_conversation(
    conversation_id: Option<&ConversationId>,
    title: Option<&str>,
) -> Result<ConversationId, HErr> {
    add_conversation_db(conversation_id, title, false)
}

/// Adds a conversation to the database
pub(crate) fn add_conversation_with_tx(
    tx: &rusqlite::Transaction,
    conversation_id: Option<&ConversationId>,
    title: Option<&str>,
    pairwise: bool,
) -> Result<ConversationId, HErr> {
    let id = match conversation_id {
        Some(id) => id.to_owned(),
        None => {
            let rand_array = utils::rand_id();
            ConversationId::from(rand_array)
        }
    };

    let color = crate::utils::id_to_color(&id);

    tx.execute(
        include_str!("sql/conversation/add_conversation.sql"),
        params![id, title, color, pairwise],
    )?;

    Ok(id)
}

/// Deletes all messages in a conversation.
pub fn delete_conversation(conversation_id: &ConversationId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/message/delete_conversation.sql"),
        &[conversation_id],
    )?;
    Ok(())
}

/// Get all messages in a conversation.
pub fn conversation_messages(conversation_id: &ConversationId) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/message/conversation_messages.sql"))?;
    let res = stmt.query_map(&[conversation_id], Message::from_db)?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Get all messages in a conversation.
pub fn conversation_messages_since(
    conversation_id: &ConversationId,
    since: DateTime<Utc>,
) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/message/conversation_messages_since.sql"))?;
    let res = stmt.query_map(
        params![conversation_id, since.timestamp()],
        Message::from_db,
    )?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Get conversation metadata
pub fn meta(conversation_id: &ConversationId) -> Result<ConversationMeta, HErr> {
    let db = Database::get()?;
    Ok(db.query_row(
        include_str!("sql/conversation/get_conversation_meta.sql"),
        params![conversation_id],
        ConversationMeta::from_db,
    )?)
}

/// Sets color for a conversation
pub fn set_color(conversation_id: &ConversationId, color: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/conversation/update_color.sql"),
        params![color, conversation_id],
    )?;
    Ok(())
}

/// Sets muted status of a conversation
pub fn set_muted(conversation_id: &ConversationId, muted: bool) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/conversation/update_muted.sql"),
        params![muted, conversation_id],
    )?;
    Ok(())
}

/// Sets title for a conversation
pub fn set_title(conversation_id: &ConversationId, title: Option<&str>) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("sql/conversation/update_title.sql"),
        params![title, conversation_id],
    )?;
    Ok(())
}

/// Sets picture for a conversation
pub fn set_picture(
    conversation_id: &ConversationId,
    picture: Option<&str>,
    old_pic: Option<&str>,
) -> Result<(), HErr> {
    use crate::image_utils;
    let path = match picture {
        Some(path) => Some(
            image_utils::save_profile_picture(
                format!("{:x?}", conversation_id.as_slice()).as_str(),
                path,
                old_pic,
            )?
            .into_os_string()
            .into_string()?,
        ),
        None => {
            if let Some(old) = old_pic {
                std::fs::remove_file(old).ok();
            }
            None
        }
    };

    let db = Database::get()?;
    db.execute(
        include_str!("sql/conversation/update_picture.sql"),
        params![path, conversation_id],
    )?;

    Ok(())
}

/// Get metadata of all conversations
pub fn all_meta() -> Result<Vec<ConversationMeta>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/conversation/all_meta.sql"))?;
    let res = stmt.query_map(NO_PARAMS, ConversationMeta::from_db)?;

    let mut meta = Vec::new();
    for data in res {
        meta.push(data?);
    }

    Ok(meta)
}

/// Get conversation
pub fn conversation(conversation_id: &ConversationId) -> Result<Conversation, HErr> {
    let messages = conversation_messages(conversation_id)?;
    let meta = meta(conversation_id)?;
    let members = crate::members::members(conversation_id)?;

    Ok(Conversation {
        meta,
        members,
        messages,
    })
}

/// Adds a conversation to the database
pub(crate) fn add_pairwise_conversation(
    tx: &rusqlite::Transaction,
    conversation_id: Option<&ConversationId>,
    title: Option<&str>,
) -> Result<ConversationId, HErr> {
    add_conversation_with_tx(tx, conversation_id, title, false)
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
    use crate::{contact::ContactBuilder, db::Database, womp};
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_drop_exists_reset() {
        Database::reset_all().expect(womp!());
        // drop twice, it shouldn't panic on multiple drops
        Conversations::drop_table().expect(womp!());
        Conversations::drop_table().expect(womp!());

        Conversations::create_table().expect(womp!());
        assert!(Conversations::exists().expect(womp!()));
        Conversations::create_table().expect(womp!());
        assert!(Conversations::exists().expect(womp!()));
        Conversations::drop_table().expect(womp!());
        assert!(!Conversations::exists().expect(womp!()));

        Database::reset_all().expect(womp!());

        Conversations::create_table().expect(womp!());
        Conversations::reset().expect(womp!());
        assert!(Conversations::exists().expect(womp!()));
    }

    #[test]
    #[serial]
    fn conv_id_length() {
        Database::reset_all().expect(womp!());
        super::add_conversation(None, None).expect(womp!("failed to create conversation"));

        let all_meta = super::all_meta().expect(womp!("failed to get data"));

        assert_eq!(all_meta[0].conversation_id.into_array().len(), 32);
    }

    #[test]
    #[serial]
    fn add_conversation() {
        Database::reset_all().expect(womp!());

        // test without id
        super::add_conversation(None, None).expect(womp!("failed to create conversation"));

        let conversation_id = ConversationId::from([0; 32]);
        // test with id
        assert_eq!(
            conversation_id,
            super::add_conversation(Some(&conversation_id), None)
                .expect(womp!("failed to create conversation"))
        );

        super::add_conversation(Some(&[1; 32].into()), Some("el groupo"))
            .expect(womp!("failed to create conversation"));

        super::add_conversation(Some(&[2; 32].into()), Some("el groupo"))
            .expect(womp!("failed to create conversation"));
    }

    #[test]
    #[serial]
    fn add_and_get() {
        Database::reset_all().expect(womp!());

        let author = "Hello".try_into().unwrap();
        ContactBuilder::new(author).add().expect(womp!());

        let conversation = ConversationId::from([0; 32]);

        super::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        crate::message::add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        crate::message::add_message(None, author, &conversation, "2", None, &None)
            .expect(womp!("Failed to add second message"));

        let msgs = super::conversation(&conversation).expect(womp!("Failed to get conversation"));

        assert_eq!(msgs.len(), 2);
    }

    #[test]
    #[serial]
    fn matches() {
        Database::reset_all().expect(womp!());

        // test without id
        let conv_id = super::add_conversation(None, Some("title"))
            .expect(womp!("failed to create conversation"));

        let conv = meta(&conv_id).expect(womp!());

        let pattern = utils::SearchPattern::new_normal("titl".into()).expect(womp!());

        let bad_pattern = utils::SearchPattern::new_normal("tilt".into()).expect(womp!());

        assert_eq!(conv.matches(&pattern), true);

        assert_eq!(conv.matches(&bad_pattern), false);
    }

    #[test]
    #[serial]
    fn set_prof_pic() {
        Database::reset_all().expect(womp!());
        let conv_id = ConversationId::from([0; 32]);

        super::add_conversation(Some(&conv_id), None)
            .expect(womp!("Failed to create conversation"));

        let test_picture = "test_resources/maryland.png";

        super::set_picture(&conv_id, Some(&test_picture), None)
            .expect(womp!("failed to set picture"));

        std::fs::remove_dir_all("profile_pictures").expect(womp!());
    }

    #[test]
    #[serial]
    fn set_muted_test() {
        Database::reset_all().expect(womp!());
        let conv_id = ConversationId::from([0; 32]);

        super::add_conversation(Some(&conv_id), None)
            .expect(womp!("Failed to create conversation"));

        super::set_muted(&conv_id, true).expect(womp!("Unable to set mute"));

        let meta = super::meta(&conv_id).expect(womp!("failed to get meta"));

        assert_eq!(meta.muted, true);

        super::set_muted(&conv_id, false).expect(womp!("Unable to set mute"));

        let meta = super::meta(&conv_id).expect(womp!("failed to get meta"));

        assert_eq!(meta.muted, false);
    }

    #[test]
    #[serial]
    fn set_get_meta() {
        Database::reset_all().expect(womp!());

        let conv_id = ConversationId::from([0; 32]);

        super::add_conversation(Some(&conv_id), None)
            .expect(womp!("Failed to create conversation"));

        super::set_color(&conv_id, 1).expect(womp!("Failed to set color"));

        super::set_title(&conv_id, Some("title")).expect(womp!("Failed to set title"));

        let conv_meta = super::meta(&conv_id).expect(womp!("Failed to get metadata"));

        assert_eq!(conv_meta.conversation_id, conv_id);
        assert_eq!(conv_meta.title.expect("failed to get title"), "title");
        assert_eq!(conv_meta.color, 1);

        let conv_id2 = ConversationId::from([1; 32]);

        super::add_conversation(Some(&conv_id2), Some("hello"))
            .expect(womp!("Failed to create conversation"));

        let all_meta = super::all_meta().expect(womp!("Failed to get all metadata"));

        assert_eq!(all_meta.len(), 2);

        assert_eq!(all_meta[1].conversation_id, conv_id2);
    }

    #[test]
    #[serial]
    fn conv_messages_since() {
        Database::reset_all().expect(womp!());

        let contact = "contact".try_into().unwrap();
        ContactBuilder::new(contact).add().expect(womp!());

        let conv_id = ConversationId::from([0; 32]);

        super::add_conversation(Some(&conv_id), None).expect(womp!("Failed to make conversation"));

        crate::message::add_message(None, contact, &conv_id, "1", None, &None)
            .expect(womp!("Failed to make message"));
        let timestamp = chrono::Utc::now();

        assert!(conversation_messages_since(&conv_id, timestamp)
            .expect(womp!())
            .is_empty());
    }

    #[test]
    #[serial]
    fn add_remove_member() {
        Database::reset_all().expect(womp!());

        let id1 = "id1".try_into().unwrap();
        let id2 = "id2".try_into().unwrap();

        let conv_id = ConversationId::from([0; 32]);

        ContactBuilder::new(id1)
            .add()
            .expect(womp!("Failed to add id1"));

        ContactBuilder::new(id2)
            .add()
            .expect(womp!("Failed to add id2"));

        super::add_conversation(Some(&conv_id), None)
            .expect(womp!("Failed to create conversation"));

        crate::members::add_member(&conv_id, id1).expect(womp!("failed to add member"));

        crate::members::add_member(&conv_id, id2).expect(womp!("failed to add member"));

        let members = crate::members::members(&conv_id).expect(womp!("failed to get members"));

        assert_eq!(members.len(), 2);

        crate::members::remove_member(&conv_id, id2).expect(womp!("failed to remove member"));

        let members = crate::members::members(&conv_id).expect(womp!("failed to get members"));

        assert_eq!(members.len(), 1);
    }

    #[test]
    #[serial]
    fn delete_message() {
        Database::reset_all().expect(womp!());

        let author = "Hello".try_into().unwrap();
        ContactBuilder::new(author).add().expect(womp!());

        let conversation = ConversationId::from([0; 32]);

        super::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let (msg_id, _) =
            crate::message::add_message(None, author, &conversation, "1", None, &None)
                .expect(womp!("Failed to add first message"));

        crate::message::delete_message(&msg_id).expect(womp!());

        assert!(super::conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }

    #[test]
    #[serial]
    fn delete_conversation() {
        Database::reset_all().expect(womp!());

        let author = "Hello".try_into().unwrap();
        ContactBuilder::new(author).add().expect(womp!());

        let conversation = [0; 32].into();

        super::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        crate::message::add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        crate::message::add_message(None, author, &conversation, "1", None, &None)
            .expect(womp!("Failed to add second message"));

        super::delete_conversation(&conversation).expect(womp!());

        assert!(super::conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }
}
