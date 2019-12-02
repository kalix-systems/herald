use crate::{config, db::Database, errors::HErr, message::Message, types::*};
pub use coretypes::conversation::*;
use herald_common::*;
use network_types::{amessages::*, cmessages::*, dmessages::*};
use rusqlite::{params, NO_PARAMS};

pub(crate) mod db;
/// Functionality related to changes in conversation settings
pub mod settings;
mod types;
pub use types::*;
mod builder;
pub use builder::*;

/// Starts the conversation, sending it to the proposed members.
pub fn start(conversation: Conversation) -> Result<(), HErr> {
    use kdf_ratchet::*;
    use std::fs;

    let Conversation { members, meta } = conversation;

    let ConversationMeta {
        title,
        conversation_id: cid,
        picture: picture_path,
        expiration_period,
        ..
    } = meta;

    let ratchet = RatchetState::gen_new();
    let pk = *config::keypair()?.public_key();
    let gen = 0;
    chainkeys::store_state(cid, pk, gen, &ratchet)?;

    let ratchets = vec![(pk, gen, ratchet)];

    let picture = match picture_path {
        Some(path) => Some(fs::read(path)?),
        None => None,
    };

    let body = AuxMessage::AddedToConvo(Box::new(AddedToConvo {
        members: members.clone(),
        ratchets,
        cid,
        title,
        expiration_period,
        picture,
    }));

    for member in members {
        crate::network::send_amessage(member, &body)?;
    }

    Ok(())
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
pub fn set_color(
    conversation_id: &ConversationId,
    color: u32,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_color(&db, conversation_id, color)
}

/// Sets muted status of a conversation
pub fn set_muted(
    conversation_id: &ConversationId,
    muted: bool,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_muted(&db, conversation_id, muted)
}

/// Sets title for a conversation
pub fn set_title(
    conversation_id: &ConversationId,
    title: Option<&str>,
) -> Result<(), HErr> {
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
