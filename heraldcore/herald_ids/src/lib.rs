use herald_common::*;
use rusqlite::types::{self, FromSql, FromSqlError, FromSqlResult, ToSql};
use std::convert::TryFrom;

/// Wrong number of bytes
#[derive(Clone, Copy, Debug)]
pub struct InvalidRandomIdLength {
    /// Found length
    pub found: usize,
    /// Type of id
    pub variant: Variant,
}

/// Variants of `InvalidIdLenght`
#[derive(Clone, Copy, Debug)]
pub enum Variant {
    /// Message id
    Msg,
    /// Conversation id
    Conversation,
}

impl std::fmt::Display for InvalidRandomIdLength {
    fn fmt(
        &self,
        out: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self.variant {
            Variant::Msg => write!(
                out,
                "Invalid message id, expected {} bytes, found {} bytes",
                UID_LEN, self.found
            ),
            Variant::Conversation => write!(
                out,
                "Invalid conversation id, expected {} bytes, found {} bytes",
                UID_LEN, self.found
            ),
        }
    }
}

impl std::error::Error for InvalidRandomIdLength {}

/// Length of randomly generated unique ids
pub const UID_LEN: usize = 32;

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Ser, De, PartialOrd, Ord)]
/// Message ID
pub struct MsgId(pub UQ);

impl MsgId {
    /// Creates a new random MsgId
    pub fn gen_new() -> Self {
        Self(UQ::gen_new())
    }

    /// Converts [`MsgId`] to `Vec<u8>`
    pub fn to_vec(self) -> Vec<u8> {
        self.into_array().to_vec()
    }

    /// Converts [`MsgId`] into a fixed length array.
    pub fn into_array(self) -> [u8; UID_LEN] {
        (self.0).0
    }

    /// [`MsgId`] as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
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
        Self(UQ(arr))
    }
}

impl TryFrom<Vec<u8>> for MsgId {
    type Error = InvalidRandomIdLength;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        UQ::from_slice(&val)
            .ok_or(InvalidRandomIdLength {
                found: val.len(),
                variant: Variant::Msg,
            })
            .map(Self)
    }
}

impl TryFrom<&[u8]> for MsgId {
    type Error = InvalidRandomIdLength;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        UQ::from_slice(val)
            .ok_or(InvalidRandomIdLength {
                found: val.len(),
                variant: Variant::Msg,
            })
            .map(Self)
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Ser, De)]
/// Conversation ID
pub struct ConversationId(pub UQ);

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
    /// Creates a new random `ConversationId`
    pub fn gen_new() -> Self {
        Self(UQ::gen_new())
    }

    /// Converts [`ConversationId`] to `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    /// Converts [`ConversationId`] into a fixed length array.
    pub fn into_array(self) -> [u8; UID_LEN] {
        (self.0).0
    }

    /// [`ConversationId`] as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<[u8; UID_LEN]> for ConversationId {
    fn from(arr: [u8; UID_LEN]) -> Self {
        Self(UQ(arr))
    }
}

impl TryFrom<Vec<u8>> for ConversationId {
    type Error = InvalidRandomIdLength;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        UQ::from_slice(&val)
            .ok_or(InvalidRandomIdLength {
                found: val.len(),
                variant: Variant::Msg,
            })
            .map(Self)
    }
}

impl TryFrom<&[u8]> for ConversationId {
    type Error = InvalidRandomIdLength;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        UQ::from_slice(val)
            .ok_or(InvalidRandomIdLength {
                found: val.len(),
                variant: Variant::Msg,
            })
            .map(Self)
    }
}
