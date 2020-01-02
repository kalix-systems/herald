use super::*;

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
            1 => Ok(Deleted),
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
            1 => Ok(Deleted),
            unknown => Err(Error::UnknownStatus(unknown)),
        }
    }
}
