use crate::{db::Database, errors::HErr, image_utils, types::*};
pub use coretypes::conversation::Conversation;
use herald_common::*;
use herald_user::{User, UserStatus, UserType};
use rusqlite::{params, NO_PARAMS};

pub(crate) mod db;

/// Gets a user's name by their `id`.
pub fn name(id: UserId) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::name(&db, id)
}

/// Change name of user by their `id`
pub fn set_name(
    id: UserId,
    name: &str,
) -> Result<(), HErr> {
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
    profile_picture: Option<image_utils::ProfilePicture>,
) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::set_profile_picture(&db, id, profile_picture)
}

/// Sets a user's color
pub fn set_color(
    id: UserId,
    color: u32,
) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_color(&db, id, color)
}

/// Indicates whether user exists
pub fn user_exists(id: UserId) -> Result<bool, HErr> {
    let db = Database::get()?;
    db::user_exists(&db, id)
}

/// Sets user status
pub fn set_status(
    id: UserId,
    status: UserStatus,
) -> Result<(), HErr> {
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
    pub fn name(
        mut self,
        name: String,
    ) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the color of the user being built.
    pub fn color(
        mut self,
        color: u32,
    ) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the status of the user being built.
    pub fn status(
        mut self,
        status: UserStatus,
    ) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the pairwise conversation id of the user being built.
    pub fn pairwise_conversation(
        mut self,
        pairwise_conversation: ConversationId,
    ) -> Self {
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

#[cfg(test)]
mod tests;
