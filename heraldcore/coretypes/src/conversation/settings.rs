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
