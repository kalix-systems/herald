use super::*;

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
