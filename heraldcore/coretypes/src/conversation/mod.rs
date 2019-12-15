use crate::ids::ConversationId;
use herald_common::*;
use rusqlite::{
    types,
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};
use std::time::Duration;

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
    /// Indicates whether the conversation is a canonical pairwise conversation
    pub pairwise: bool,
    /// Last notable activity
    pub last_active: Time,
    /// Time until message expiration
    pub expiration_period: ExpirationPeriod,
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

const MIN_SECS: u64 = 60;
const THIRTY_MIN_SECS: u64 = MIN_SECS * 30;
const HOUR_SECS: u64 = MIN_SECS * 60;
const TWELVE_HOUR_SECS: u64 = HOUR_SECS * 12;
const DAY_SECS: u64 = HOUR_SECS * 24;
const WEEK_SECS: u64 = DAY_SECS * 7;
const MONTH_SECS: u64 = DAY_SECS * 30;
const YEAR_SECS: u64 = DAY_SECS * 365;

const THIRTY_SEC: Duration = Duration::from_secs(30);
const MIN: Duration = Duration::from_secs(MIN_SECS);
const THIRTY_MIN: Duration = Duration::from_secs(THIRTY_MIN_SECS);
const HOUR: Duration = Duration::from_secs(HOUR_SECS);
const TWELVE_HOUR: Duration = Duration::from_secs(TWELVE_HOUR_SECS);
const DAY: Duration = Duration::from_secs(DAY_SECS);
const WEEK: Duration = Duration::from_secs(WEEK_SECS);
const MONTH: Duration = Duration::from_secs(MONTH_SECS);
const YEAR: Duration = Duration::from_secs(YEAR_SECS);

impl ExpirationPeriod {
    /// Converts an `ExpirationPeriod` to a `Duration`
    pub fn into_duration(self) -> Option<Duration> {
        use ExpirationPeriod::*;
        match self {
            ThirtySeconds => Some(THIRTY_SEC),
            OneMinute => Some(MIN),
            ThirtyMinutes => Some(THIRTY_MIN),
            OneHour => Some(HOUR),
            TwelveHours => Some(TWELVE_HOUR),
            OneDay => Some(DAY),
            OneWeek => Some(WEEK),
            OneMonth => Some(MONTH),
            OneYear => Some(YEAR),
            Never => None,
        }
    }

    /// Converts an `ExpirationPeriod` to milliseconds
    pub fn into_millis(self) -> Option<Time> {
        match self.into_duration() {
            Some(d) => Some((d.as_millis() as i64).into()),
            None => None,
        }
    }
}

impl Default for ExpirationPeriod {
    fn default() -> Self {
        ExpirationPeriod::OneYear
    }
}

impl FromSql for ExpirationPeriod {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        kson::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for ExpirationPeriod {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(kson::to_vec(self))))
    }
}

impl From<u8> for ExpirationPeriod {
    fn from(val: u8) -> Self {
        use ExpirationPeriod::*;
        match val {
            0 => Never,
            1 => ThirtySeconds,
            2 => OneMinute,
            3 => ThirtyMinutes,
            4 => OneHour,
            5 => TwelveHours,
            6 => OneDay,
            7 => OneWeek,
            8 => OneMonth,
            9 => OneYear,
            _ => Self::default(),
        }
    }
}
