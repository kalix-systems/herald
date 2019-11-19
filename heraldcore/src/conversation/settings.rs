// NOTE: This only needs to be here until the settings update enum is fleshed out
#![allow(clippy::trivially_copy_pass_by_ref)]
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A change in the settings for a conversation
pub enum SettingsUpdate {
    /// Expiring messages setting
    Expiration(ExpirationPeriod),
    /// The title of the conversation
    Title(Option<String>),
    /// The color of the conversation
    Color(u32),
}

impl SettingsUpdate {
    /// Applies settings update to the conversation with the id `cid`.
    pub fn apply(&self, cid: &ConversationId) -> Result<(), HErr> {
        let conn = Database::get()?;
        self.apply_db(&conn, cid)
    }

    /// Sends the update to the conversation
    pub fn send_update(&self, cid: &ConversationId) -> Result<(), HErr> {
        use crate::network::send_conversation_settings_update;

        send_conversation_settings_update(*cid, self.clone())
    }
}

pub(crate) mod db {
    use super::*;
    impl SettingsUpdate {
        pub(crate) fn apply_db(
            &self,
            conn: &rusqlite::Connection,
            cid: &ConversationId,
        ) -> Result<(), HErr> {
            use crate::conversation::db::*;
            use SettingsUpdate::*;
            match self {
                Expiration(period) => Ok(set_expiration_period(&conn, cid, *period)?),
                Title(title) => Ok(set_title(&conn, cid, title.as_ref().map(String::as_str))?),
                Color(color) => Ok(set_color(&conn, cid, *color)?),
            }
        }
    }
}
