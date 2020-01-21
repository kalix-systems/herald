use crate::{db::Database, errors::HErr, types::*};
pub use coretypes::conversation::*;
use herald_common::*;
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
    todo!();
    //use channel_ratchet::*;
    //use std::fs;

    //let Conversation { members, meta } = conversation;

    //let ConversationMeta {
    //    title,
    //    conversation_id: cid,
    //    picture: picture_path,
    //    expiration_period,
    //    ..
    //} = meta;

    //let ratchet = RatchetState::new();
    //chainkeys::store_state(cid, &ratchet)?;

    //let picture = match picture_path {
    //    Some(path) => Some(fs::read(path)?),
    //    None => None,
    //};

    //let pairwise = get_pairwise_conversations(&members)?;

    //let body = ConversationMessage::AddedToConvo {
    //    info: Box::new(cmessages::AddedToConvo {
    //        members,
    //        cid,
    //        title,
    //        expiration_period,
    //        picture,
    //    }),

    //    ratchet,
    //};

    //for pw_cid in pairwise {
    //    crate::network::send_cmessage(pw_cid, &body)?;
    //}

    //Ok(())
}

/// Get conversation metadata
pub fn meta(conversation_id: &ConversationId) -> Result<ConversationMeta, HErr> {
    let db = Database::get()?;
    db::meta(&db, conversation_id)
}

/// Sets title for a conversation
pub fn set_title(
    conversation_id: &ConversationId,
    title: Option<String>,
) -> Result<(), HErr> {
    let mut db = Database::get()?;
    let update = settings::db::update_title(&db, title.clone(), conversation_id)?;
    let (mid, expiration) = crate::message::db::outbound_aux(
        &mut db,
        settings::SettingsUpdate::Title(title),
        conversation_id,
    )?;

    crate::network::send_group_settings_message(mid, *conversation_id, expiration, update)?;
    Ok(())
}

/// Sets picture for a conversation
pub fn set_picture(
    conversation_id: &ConversationId,
    picture: Option<image_utils::ProfilePicture>,
) -> Result<Option<String>, HErr> {
    let mut db = Database::get()?;

    let (update, path) = settings::db::update_picture(&db, picture, conversation_id)?;

    let (mid, expiration) = crate::message::db::outbound_aux(
        &mut db,
        settings::SettingsUpdate::Picture(path.clone()),
        conversation_id,
    )?;

    crate::network::send_group_settings_message(mid, *conversation_id, expiration, update)?;

    Ok(path)
}

/// Sets expiration period for a conversation
pub fn set_expiration_period(
    conversation_id: &ConversationId,
    expiration_period: ExpirationPeriod,
) -> Result<(), HErr> {
    let mut db = Database::get()?;

    let update = settings::db::update_expiration(&db, expiration_period, conversation_id)?;
    let (mid, expiration) = crate::message::db::outbound_aux(
        &mut db,
        settings::SettingsUpdate::Expiration(expiration_period),
        conversation_id,
    )?;

    crate::network::send_group_settings_message(mid, *conversation_id, expiration, update)?;
    Ok(())
}

/// Sets muted status of a conversation
pub fn set_muted(
    conversation_id: &ConversationId,
    muted: bool,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_muted(&db, conversation_id, muted)
}

/// Sets archive status of a conversation
pub fn set_status(
    conversation_id: &ConversationId,
    status: Status,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_status(&db, conversation_id, status)
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

/// Returns user id associated with a pairwise conversation
pub fn associated_user(cid: &ConversationId) -> Result<Option<UserId>, HErr> {
    let conn = Database::get()?;
    Ok(db::associated_user(&conn, cid)?)
}

#[cfg(test)]
mod tests;
