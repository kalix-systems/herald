use super::*;
pub use coretypes::conversation::settings::*;

/// Applies settings update to the conversation with the id `cid`.
pub fn apply(update: &SettingsUpdate, cid: &ConversationId) -> Result<(), HErr> {
    let conn = Database::get()?;
    db::apply(&conn, update, cid)
}

/// Sends the update to the conversation
pub fn send_update(update: SettingsUpdate, cid: &ConversationId) -> Result<(), HErr> {
    use crate::network::send_conversation_settings_update;

    send_conversation_settings_update(*cid, update)
}

pub(crate) mod db {
    use super::*;

    pub(crate) fn apply(
        conn: &rusqlite::Connection,
        update: &SettingsUpdate,
        cid: &ConversationId,
    ) -> Result<(), HErr> {
        use crate::conversation::db::*;
        use SettingsUpdate::*;

        match update {
            Expiration(period) => Ok(set_expiration_period(&conn, cid, *period)?),
            Title(title) => Ok(set_title(&conn, cid, title.as_ref().map(String::as_str))?),
            Color(color) => Ok(set_color(&conn, cid, *color)?),
        }
    }
}
