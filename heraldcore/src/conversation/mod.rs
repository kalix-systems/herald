use crate::{db::Database, errors::HErr, message::Message, types::*, utils};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

pub(crate) mod db;
/// Functionality related to changes in conversation settings
pub mod settings;
mod types;
pub use types::*;
mod builder;
pub use builder::*;

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

    /// Matches conversation's text fields against a [`SearchPattern`]
    pub fn matches(&self, pattern: &search_pattern::SearchPattern) -> bool {
        match self.title.as_ref() {
            Some(name) => pattern.is_match(name),
            None => false,
        }
    }
}

/// Conversation
#[derive(Clone)]
pub struct Conversation {
    /// User ID's of conversation members
    pub members: Vec<UserId>,

    /// Conversation metadata
    pub meta: ConversationMeta,
}

impl Conversation {
    /// Starts the conversation, sending it to the proposed members.
    pub fn start(self) -> Result<(), HErr> {
        use chainmail::block::*;
        use std::fs;
        let Self { members, meta } = self;

        let ConversationMeta {
            title,
            conversation_id: cid,
            picture: picture_path,
            expiration_period,
            ..
        } = meta;

        let kp = crate::config::keypair()?;
        let gen = Genesis::new(kp.secret_key());
        chainkeys::store_genesis(&cid, &gen)?;

        let picture = match picture_path {
            Some(path) => Some(fs::read(path)?),
            None => None,
        };

        let pairwise = get_pairwise_conversations(&members)?;

        let body = ConversationMessageBody::AddedToConvo(Box::new(cmessages::AddedToConvo {
            members,
            gen,
            cid,
            title,
            expiration_period,
            picture,
        }));

        for pw_cid in pairwise {
            crate::network::send_cmessage(pw_cid, &body)?;
        }

        Ok(())
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
) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::set_picture(&db, conversation_id, picture)
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
