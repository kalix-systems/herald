use super::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A change in the settings for a conversation
pub enum SettingsUpdate {
    /// Expiring messages setting
    Expiration(ExpirationPeriod),
    /// The title of the conversation
    Title(Option<String>),
    /// Group picture
    Picture(Option<String>),
}
