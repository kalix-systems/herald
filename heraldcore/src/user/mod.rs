use crate::{conversation::Conversation, db::Database, errors::HErr, image_utils, types::*};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};
use std::convert::TryInto;

pub(crate) mod db;

/// Gets a user's name by their `id`.
pub fn name(id: UserId) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::name(&db, id)
}

/// Change name of user by their `id`
pub fn set_name(id: UserId, name: &str) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_name(&db, id, name)
}

/// Gets a user's profile picture by their `id`.
pub fn profile_picture(id: UserId) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::profile_picture(&db, id)
}

/// Returns all members of a conversation.
pub fn conversation_members(conversation_id: &ConversationId) -> Result<Vec<User>, HErr> {
    let db = Database::get()?;
    db::conversation_members(&db, conversation_id)
}

/// Updates a user's profile picture.
pub fn set_profile_picture(
    id: UserId,
    profile_picture: Option<String>,
) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::set_profile_picture(&db, id, profile_picture)
}

/// Sets a user's color
pub fn set_color(id: UserId, color: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_color(&db, id, color)
}

/// Indicates whether user exists
pub fn user_exists(id: UserId) -> Result<bool, HErr> {
    let db = Database::get()?;
    db::user_exists(&db, id)
}

/// Sets user status
pub fn set_status(id: UserId, status: UserStatus) -> Result<(), HErr> {
    let mut db = Database::get()?;
    db::set_status(&mut db, id, status)
}

/// Gets user status
pub fn status(id: UserId) -> Result<UserStatus, HErr> {
    let db = Database::get()?;
    db::status(&db, id)
}

/// Returns all users
pub fn all() -> Result<Vec<User>, HErr> {
    let db = Database::get()?;
    db::all(&db)
}

/// Returns a single user by `user_id`
pub fn by_user_id(user_id: UserId) -> Result<User, HErr> {
    let db = Database::get()?;
    db::by_user_id(&db, user_id)
}

/// Returns all users with the specified `status`
pub fn get_by_status(status: UserStatus) -> Result<Vec<User>, HErr> {
    let db = Database::get()?;
    db::get_by_status(&db, status)
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
    /// The user is local (ie it is you)
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
    type Error = HErr;

    fn try_from(n: u8) -> Result<Self, HErr> {
        use UserType::*;
        match n {
            0 => Ok(Local),
            1 => Ok(Remote),
            unknown => Err(HErr::HeraldError(format!(
                "Unknown user status {}",
                unknown
            ))),
        }
    }
}

impl std::convert::TryFrom<i64> for UserType {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n.try_into(),
            Err(_) => Err(HErr::HeraldError(format!("Unknown user status {}", n))),
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
    type Error = HErr;

    fn try_from(n: u8) -> Result<Self, HErr> {
        use UserStatus::*;
        match n {
            0 => Ok(Active),
            1 => Ok(Archived),
            2 => Ok(Deleted),
            unknown => Err(HErr::HeraldError(format!(
                "Unknown user status {}",
                unknown
            ))),
        }
    }
}

impl std::convert::TryFrom<i64> for UserStatus {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n.try_into(),
            Err(_) => Err(HErr::HeraldError(format!("Unknown user status {}", n))),
        }
    }
}

/// Builder for `User`
pub struct UserBuilder {
    /// User id
    id: UserId,
    /// User name
    name: Option<String>,
    /// User set color for user
    color: Option<u32>,
    /// Indicates whether user is archived
    status: Option<UserStatus>,
    /// Pairwise conversation corresponding to user
    pairwise_conversation: Option<ConversationId>,
    /// Indicates that the user is the local user
    user_type: Option<UserType>,
}

impl UserBuilder {
    /// Creates new `UserBuilder`
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            name: None,
            color: None,
            status: None,
            pairwise_conversation: None,
            user_type: None,
        }
    }

    /// Sets the name of the user being built.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the color of the user being built.
    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the status of the user being built.
    pub fn status(mut self, status: UserStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the pairwise conversation id of the user being built.
    pub fn pairwise_conversation(mut self, pairwise_conversation: ConversationId) -> Self {
        self.pairwise_conversation = Some(pairwise_conversation);
        self
    }

    pub(crate) fn local(mut self) -> Self {
        self.user_type = Some(UserType::Local);
        self
    }

    /// Adds user to database
    pub fn add(self) -> Result<(User, Conversation), HErr> {
        let mut db = Database::get()?;
        self.add_db(&mut db)
    }
}

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
    pub fn matches(&self, pattern: &crate::utils::SearchPattern) -> bool {
        pattern.is_match(self.id.as_str()) || pattern.is_match(self.name.as_str())
    }

    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            profile_picture: row.get(2)?,
            color: row.get(3)?,
            status: row.get(4)?,
            pairwise_conversation: row.get(5)?,
            user_type: row.get(6)?,
        })
    }
}

#[cfg(test)]
mod tests;
