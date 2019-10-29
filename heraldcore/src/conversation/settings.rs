use super::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
/// A change in the settings for a conversation
pub enum SettingsUpdate {
    /// A change in the expiring messages setting
    Expiration(ExpirationPeriod),
}

impl SettingsUpdate {
    /// Applies settings update to the conversation with the id `cid`.
    pub fn apply(&self, cid: &ConversationId) -> Result<(), HErr> {
        let conn = Database::get()?;
        self.apply_db(&conn, cid)
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
                Expiration(period) => Ok(set_expiration_period(&conn, cid, period)?),
            }
        }
    }
}
