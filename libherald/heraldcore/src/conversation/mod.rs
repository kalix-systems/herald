use crate::{db::Database, errors::HErr, message::Message, types::*, utils};
use chrono::{DateTime, Utc};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
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
        include_str!("../sql/conversation/add_conversation.sql"),
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
        include_str!("../sql/conversation/add_conversation.sql"),
        params![id, title, color, pairwise],
    )?;

    Ok(id)
}

/// Deletes all messages in a conversation.
pub fn delete_conversation(conversation_id: &ConversationId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/message/delete_conversation.sql"),
        &[conversation_id],
    )?;
    Ok(())
}

/// Get all messages in a conversation.
pub fn conversation_messages(conversation_id: &ConversationId) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("../sql/message/conversation_messages.sql"))?;
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
    let mut stmt = db.prepare(include_str!(
        "../sql/message/conversation_messages_since.sql"
    ))?;
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
        include_str!("../sql/conversation/get_conversation_meta.sql"),
        params![conversation_id],
        ConversationMeta::from_db,
    )?)
}

/// Sets color for a conversation
pub fn set_color(conversation_id: &ConversationId, color: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/conversation/update_color.sql"),
        params![color, conversation_id],
    )?;
    Ok(())
}

/// Sets muted status of a conversation
pub fn set_muted(conversation_id: &ConversationId, muted: bool) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/conversation/update_muted.sql"),
        params![muted, conversation_id],
    )?;
    Ok(())
}

/// Sets title for a conversation
pub fn set_title(conversation_id: &ConversationId, title: Option<&str>) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/conversation/update_title.sql"),
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
        include_str!("../sql/conversation/update_picture.sql"),
        params![path, conversation_id],
    )?;

    Ok(())
}

/// Get metadata of all conversations
pub fn all_meta() -> Result<Vec<ConversationMeta>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("../sql/conversation/all_meta.sql"))?;
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

#[cfg(test)]
mod tests;
