use herald_common::*;
use herald_ids::ConversationId;
use std::convert::TryInto;

mod error;
pub use error::Error;
mod convert;

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
    /// The user is deleted
    Deleted = 1,
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
