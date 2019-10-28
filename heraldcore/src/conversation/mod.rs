use crate::{db::Database, errors::HErr, message::Message, types::*, utils};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

pub(crate) mod db;

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
    /// Last notable activity
    pub last_active: Time,
    /// Time until message expiration
    pub expiration_period: ExpirationPeriod,
}

impl ConversationMeta {
    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(ConversationMeta {
            conversation_id: row.get("conversation_id")?,
            title: row.get("title")?,
            picture: row.get("picture")?,
            color: row.get("color")?,
            muted: row.get("muted")?,
            pairwise: row.get("pairwise")?,
            last_active: row.get("last_active_ts")?,
            expiration_period: row.get("expiration_period")?,
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

/// Builder for Conversations
#[derive(Default)]
pub struct ConversationBuilder {
    /// Conversation id
    conversation_id: Option<ConversationId>,
    /// Conversation title
    title: Option<String>,
    /// Conversation picture
    picture: Option<String>,
    /// Conversation color,
    color: Option<u32>,
    /// Indicates whether the conversation is muted
    muted: Option<bool>,
    /// Indicates whether the conversation is a canonical pairwise conversation
    pairwise: Option<bool>,
    /// How long until a message expires
    expiration_period: Option<ExpirationPeriod>,
}

impl ConversationBuilder {
    /// Creates new `ConversationBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets title
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title.replace(title);
        self
    }

    /// Sets conversation id
    pub fn conversation_id(&mut self, cid: ConversationId) -> &mut Self {
        self.conversation_id.replace(cid);
        self
    }

    /// Sets picture
    pub fn picture(&mut self, picture: String) -> &mut Self {
        self.picture.replace(picture);
        self
    }

    /// Sets color
    pub fn color(&mut self, color: u32) -> &mut Self {
        self.color.replace(color);
        self
    }

    /// Sets muted status
    pub fn muted(&mut self, muted: bool) -> &mut Self {
        self.muted.replace(muted);
        self
    }

    /// Sets muted status
    pub fn pairwise(&mut self, pairwise: bool) -> &mut Self {
        self.pairwise.replace(pairwise);
        self
    }

    /// Sets expiration period
    pub fn expiration_period(&mut self, expiration_period: ExpirationPeriod) -> &mut Self {
        self.expiration_period.replace(expiration_period);
        self
    }

    /// Adds conversation
    pub fn add(&mut self) -> Result<ConversationId, HErr> {
        let mut db = Database::get()?;
        self.add_db(&mut db)
    }
}

/// Deletes all messages in a conversation.
pub fn delete_conversation(conversation_id: &ConversationId) -> Result<(), HErr> {
    let db = Database::get()?;
    db::delete_conversation(&db, conversation_id)
}

/// Get all messages in a conversation.
pub fn conversation_messages(conversation_id: &ConversationId) -> Result<Vec<Message>, HErr> {
    let db = Database::get()?;
    db::conversation_messages(&db, conversation_id)
}

/// Get conversation metadata
pub fn meta(conversation_id: &ConversationId) -> Result<ConversationMeta, HErr> {
    let db = Database::get()?;
    db::meta(&db, conversation_id)
}

/// Sets color for a conversation
pub fn set_color(conversation_id: &ConversationId, color: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_color(&db, conversation_id, color)
}

/// Sets muted status of a conversation
pub fn set_muted(conversation_id: &ConversationId, muted: bool) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_muted(&db, conversation_id, muted)
}

/// Sets title for a conversation
pub fn set_title(conversation_id: &ConversationId, title: Option<&str>) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_title(&db, conversation_id, title)
}

/// Sets picture for a conversation
pub fn set_picture(
    conversation_id: &ConversationId,
    picture: Option<&str>,
    old_pic: Option<&str>,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_picture(&db, conversation_id, picture, old_pic)
}

/// Sets expiration period for a conversation
pub fn set_expiration_period(
    conversation_id: &ConversationId,
    expiration_period: ExpirationPeriod,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_expiration_period(&db, conversation_id, expiration_period)
}

/// Get metadata of all conversations
pub fn all_meta() -> Result<Vec<ConversationMeta>, HErr> {
    let db = Database::get()?;
    db::all_meta(&db)
}

/// Get pairwise conversations
pub fn get_pairwise_conversations(uids: &[UserId]) -> Result<Vec<ConversationId>, HErr> {
    let db = Database::get()?;
    db::get_pairwise_conversations(&db, uids)
}

#[cfg(test)]
mod tests;
