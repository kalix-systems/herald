use super::*;

impl Status {
    pub fn from_u8(s: u8) -> Option<Self> {
        match s {
            0 => Some(Status::Active),
            1 => Some(Status::Archived),
            _ => None,
        }
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

impl FromSql for Status {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        kson::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(kson::to_vec(self))))
    }
}

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
