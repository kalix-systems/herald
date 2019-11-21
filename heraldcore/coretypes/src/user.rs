use crate::ids::ConversationId;
use herald_common::*;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnknownStatus(i64),
    UnknownUserType(i64),
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        out: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        use Error::*;
        match self {
            UnknownStatus(n) => {
                write!(out, "Unknown user status: found {}, expected 0, 1, or 2", n)
            }
            UnknownUserType(n) => write!(out, "Unknown user type: found {}, expected 0 or 1", n),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Clone)]
/// A Herald user.
pub struct User {
    /// User id
    pub id: UserId,
    /// User name
    pub name: String,
    /// Path of profile picture
    pub profile_picture: Option<String>,
    /// User set color for user
    pub color: u32,
    /// Indicates whether user is archived
    pub status: UserStatus,
    /// Pairwise conversation corresponding to user
    pub pairwise_conversation: ConversationId,
    /// User type, local or remote
    pub user_type: UserType,
}

impl User {
    /// Returns name
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns path to profile picture
    pub fn profile_picture(&self) -> Option<&str> {
        self.profile_picture.as_ref().map(|s| s.as_ref())
    }

    /// Returns user's color
    pub fn color(&self) -> u32 {
        self.color
    }

    /// Returns user's status
    pub fn status(&self) -> UserStatus {
        self.status
    }

    /// Matches user's text fields against a [`SearchPattern`]
    pub fn matches(
        &self,
        pattern: &search_pattern::SearchPattern,
    ) -> bool {
        pattern.is_match(self.id.as_str()) || pattern.is_match(self.name.as_str())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
/// Status of the user
pub enum UserStatus {
    /// The user is active
    Active = 0,
    /// The user is archived
    Archived = 1,
    /// The user is deleted
    Deleted = 2,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
/// Type of the user
pub enum UserType {
    /// The user is local (i.e., it is you)
    Local = 0,
    /// The user is remote
    Remote = 1,
}

impl rusqlite::types::FromSql for UserType {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl rusqlite::ToSql for UserType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
        use rusqlite::types::*;
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl std::convert::TryFrom<u8> for UserType {
    type Error = Error;

    fn try_from(n: u8) -> Result<Self, Error> {
        use UserType::*;
        match n {
            0 => Ok(Local),
            1 => Ok(Remote),
            unknown => Err(Error::UnknownUserType(unknown as i64)),
        }
    }
}

impl std::convert::TryFrom<i64> for UserType {
    type Error = Error;

    fn try_from(n: i64) -> Result<Self, Error> {
        use UserType::*;
        match n {
            0 => Ok(Local),
            1 => Ok(Remote),
            unknown => Err(Error::UnknownUserType(unknown)),
        }
    }
}

impl rusqlite::types::FromSql for UserStatus {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl rusqlite::ToSql for UserStatus {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
        use rusqlite::types::*;
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl std::convert::TryFrom<u8> for UserStatus {
    type Error = Error;

    fn try_from(n: u8) -> Result<Self, Error> {
        use UserStatus::*;
        match n {
            0 => Ok(Active),
            1 => Ok(Archived),
            2 => Ok(Deleted),
            unknown => Err(Error::UnknownStatus(unknown as i64)),
        }
    }
}

impl std::convert::TryFrom<i64> for UserStatus {
    type Error = Error;

    fn try_from(n: i64) -> Result<Self, Error> {
        use UserStatus::*;
        match n {
            0 => Ok(Active),
            1 => Ok(Archived),
            2 => Ok(Deleted),
            unknown => Err(Error::UnknownStatus(unknown)),
        }
    }
}
