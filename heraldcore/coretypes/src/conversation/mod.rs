use herald_common::*;
use herald_ids::ConversationId;
use rusqlite::{
    types,
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};
use std::time::Duration;

mod convert;
pub mod settings;

#[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq)]
/// Conversation metadata.
pub struct ConversationMeta {
    /// Conversation id
    pub conversation_id: ConversationId,
    /// Conversation title
    pub title: Option<String>,
    /// Conversation picture
    pub picture: Option<String>,
    /// Conversation color,
    pub color: u32,
    /// Indicates whether the conversation is muted
    pub muted: bool,
    /// Associated user id if the conversation is pairwise
    pub pairwise_uid: Option<UserId>,
    /// Last notable activity
    pub last_active: Time,
    /// Time until message expiration
    pub expiration_period: ExpirationPeriod,
    /// Conversation status
    pub status: Status,
    /// Message id of the last message in the conversation
    pub last_msg_id: Option<herald_ids::MsgId>,
}

impl ConversationMeta {
    /// Matches conversation's text fields against a [`SearchPattern`]
    pub fn matches(
        &self,
        pattern: &search_pattern::SearchPattern,
    ) -> bool {
        match self.title.as_ref() {
            Some(name) => pattern.is_match(name),
            None => false,
        }
    }
}

/// Conversation
#[derive(Clone)]
pub struct Conversation {
    /// User ID's of conversation members
    pub members: Vec<UserId>,

    /// Conversation metadata
    pub meta: ConversationMeta,
}

#[derive(Clone, Copy, Debug, Ser, De, Eq, PartialEq, Hash)]
#[repr(u8)]
/// Expiration period for messages
pub enum ExpirationPeriod {
    /// Messages never expire
    Never = 0,
    /// Messages expire after 30 seconds
    ThirtySeconds = 1,
    /// Messages expire after one minute
    OneMinute = 2,
    /// Messages expire after one minute
    ThirtyMinutes = 3,
    /// Messages expire after one hour
    OneHour = 4,
    /// Messages expire after twelve hours
    TwelveHours = 5,
    /// Messages expire after one day
    OneDay = 6,
    /// Message expire after one week
    OneWeek = 7,
    /// Messages expire after one month
    OneMonth = 8,
    /// Messages expire after one year
    OneYear = 9,
}

impl Default for ExpirationPeriod {
    fn default() -> Self {
        ExpirationPeriod::OneYear
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Ser, De, Eq, PartialEq, Hash)]
pub enum Status {
    Active = 0,
    Archived = 1,
}

impl Default for Status {
    fn default() -> Self {
        Status::Active
    }
}
