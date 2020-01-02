use super::*;
use json::JsonValue;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A change in the settings for a conversation
pub enum SettingsUpdate {
    /// Expiring messages setting
    Expiration(ExpirationPeriod),
    /// The title of the conversation
    Title(Option<String>),
    /// The color of the conversation
    Color(u32),
    /// Group picture
    Picture(Option<String>),
}

impl SettingsUpdate {
    fn code(&self) -> u8 {
        use SettingsUpdate::*;
        match self {
            Expiration(_) => 0,
            Title(_) => 1,
            Color(_) => 2,
            Picture(_) => 3,
        }
    }
}

impl FromSql for SettingsUpdate {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        kson::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for SettingsUpdate {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(kson::to_vec(self))))
    }
}

impl From<SettingsUpdate> for JsonValue {
    fn from(update: SettingsUpdate) -> Self {
        use SettingsUpdate::*;
        let code = update.code();

        match update {
            Expiration(period) => {
                json::object! {
                    "code" => code,
                    "content" => period as u8,
                }
            }
            Title(title) => {
                json::object! {
                    "code" => code,
                    "content" => title,
                }
            }
            Color(color) => {
                json::object! {
                    "code" => code,
                    "content" => color,
                }
            }
            Picture(path) => {
                json::object! {
                    "code" => code,
                    "content" => path,
                }
            }
        }
    }
}
