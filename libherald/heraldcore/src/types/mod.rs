use crate::errors::HErr;
use herald_common::*;
use rusqlite::types::{self, FromSql, FromSqlError, FromSqlResult, ToSql};
use std::convert::{TryFrom, TryInto};

mod messages;
pub use messages::*;

/// Lenght of randomly generated unique ids
pub const UID_LEN: usize = 32;

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
/// Message ID
pub struct MsgId([u8; UID_LEN]);

#[derive(Default, Hash, Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
/// Conversation ID
pub struct ConversationId([u8; UID_LEN]);

impl MsgId {
    /// Converts [`MsgId`] to `Vec<u8>`
    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Converts [`MsgId`] into a fixed length array.
    pub fn into_array(self) -> [u8; UID_LEN] {
        self.0
    }

    /// [`MsgId`] as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0 as &[u8]
    }
}

impl FromSql for MsgId {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        MsgId::try_from(value.as_blob()?).map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for MsgId {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;
        Ok(ToSqlOutput::Borrowed(ValueRef::Blob(self.as_slice())))
    }
}

impl From<[u8; UID_LEN]> for MsgId {
    fn from(arr: [u8; UID_LEN]) -> Self {
        Self(arr)
    }
}

impl TryFrom<Vec<u8>> for MsgId {
    type Error = HErr;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(HErr::InvalidMessageId)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.into_iter().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}

impl TryFrom<&[u8]> for MsgId {
    type Error = HErr;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(HErr::InvalidMessageId)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.iter().copied().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}

impl FromSql for ConversationId {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        ConversationId::try_from(value.as_blob()?).map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for ConversationId {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;
        Ok(ToSqlOutput::Borrowed(ValueRef::Blob(self.as_slice())))
    }
}

impl ConversationId {
    /// Converts [`ConversationId`] to `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Converts [`ConversationId`] into a fixed length array.
    pub fn into_array(self) -> [u8; UID_LEN] {
        self.0
    }

    /// [`ConversationId`] as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0 as &[u8]
    }
}

impl From<[u8; UID_LEN]> for ConversationId {
    fn from(arr: [u8; UID_LEN]) -> Self {
        Self(arr)
    }
}

impl TryFrom<Vec<u8>> for ConversationId {
    type Error = HErr;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(HErr::InvalidConversationId)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.into_iter().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}

impl TryFrom<&[u8]> for ConversationId {
    type Error = HErr;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        if val.len() != UID_LEN {
            Err(HErr::InvalidConversationId)
        } else {
            let mut buf = [0u8; UID_LEN];

            for (ix, n) in val.iter().copied().enumerate() {
                buf[ix] = n;
            }

            Ok(Self(buf))
        }
    }
}
