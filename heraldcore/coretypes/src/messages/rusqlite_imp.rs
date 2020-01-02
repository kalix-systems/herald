use super::*;
use crate::conversation::settings::SettingsUpdate;
use json::JsonValue;
use rusqlite::{
    types,
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};

impl ToSql for MessageBody {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.as_slice())))
    }
}

impl FromSql for MessageBody {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        value
            .as_str()?
            .to_owned()
            .try_into()
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for MessageSendStatus {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl FromSql for MessageSendStatus {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for MessageReceiptStatus {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl FromSql for MessageReceiptStatus {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl AuxItem {
    fn code(&self) -> u8 {
        use SettingsUpdate::*;

        match self {
            AuxItem::GroupSettings(settings) => match settings {
                Expiration(_) => 0,
                Title(_) => 1,
                Color(_) => 2,
                Picture(_) => 3,
            },
            AuxItem::NewMembers(_) => 4,
        }
    }
}

impl FromSql for AuxItem {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        kson::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for AuxItem {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(kson::to_vec(self))))
    }
}

impl From<AuxItem> for JsonValue {
    fn from(item: AuxItem) -> Self {
        use SettingsUpdate::*;
        let code = item.code();

        match item {
            AuxItem::GroupSettings(settings) => match settings {
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
            },
            AuxItem::NewMembers(members) => {
                json::object! {
                    "code" => code,
                    "content" => members.0.into_iter().map(|u| u.to_string()).collect::<Vec<_>>(),
                }
            }
        }
    }
}
