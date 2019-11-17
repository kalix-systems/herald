use super::*;
use rusqlite::{
    types,
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(u8)]
/// Expiration period for messages
pub enum ExpirationPeriod {
    /// Messages never expire
    Never = 0,
    /// Messages expire after one minute
    OneMinute = 1,
    /// Messages expire after one hour
    OneHour = 2,
    /// Messages expire after one day
    OneDay = 3,
    /// Message expire after one week
    OneWeek = 4,
    /// Messages expire after one month
    OneMonth = 5,
    /// Messages expire after one year
    OneYear = 6,
}

const MIN_SECS: u64 = 60;
const HOUR_SECS: u64 = MIN_SECS * 60;
const DAY_SECS: u64 = HOUR_SECS * 24;
const WEEK_SECS: u64 = DAY_SECS * 7;
const MONTH_SECS: u64 = DAY_SECS * 30;
const YEAR_SECS: u64 = DAY_SECS * 365;

const MIN: Duration = Duration::from_secs(MIN_SECS);
const HOUR: Duration = Duration::from_secs(HOUR_SECS);
const DAY: Duration = Duration::from_secs(DAY_SECS);
const WEEK: Duration = Duration::from_secs(WEEK_SECS);
const MONTH: Duration = Duration::from_secs(MONTH_SECS);
const YEAR: Duration = Duration::from_secs(YEAR_SECS);

impl ExpirationPeriod {
    /// Converts an `ExpirationPeriod` to a `Duration`
    pub fn into_duration(self) -> Option<Duration> {
        use ExpirationPeriod::*;
        match self {
            OneMinute => Some(MIN),
            OneHour => Some(HOUR),
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
        serde_cbor::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for ExpirationPeriod {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(
            serde_cbor::to_vec(self)
                .map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?,
        )))
    }
}

impl From<u8> for ExpirationPeriod {
    fn from(val: u8) -> Self {
        use ExpirationPeriod::*;
        match val {
            0 => Never,
            1 => OneMinute,
            2 => OneHour,
            3 => OneDay,
            4 => OneWeek,
            5 => OneMonth,
            6 => OneYear,
            _ => Self::default(),
        }
    }
}
