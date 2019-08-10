use std::{fmt, sync::PoisonError};

#[derive(Debug)]
pub enum HErr {
    HeraldError(String),
    MutexError(String),
    DatabaseError(rusqlite::Error),
}

impl fmt::Display for HErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HErr::DatabaseError(e) => write!(f, "{}", e),
            HErr::HeraldError(s) => write!(f, "{}", s),
            HErr::MutexError(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for HErr {}

impl From<rusqlite::Error> for HErr {
    fn from(e: rusqlite::Error) -> HErr {
        HErr::DatabaseError(e)
    }
}

impl<T> From<PoisonError<T>> for HErr {
    fn from(e: PoisonError<T>) -> HErr {
        HErr::MutexError(format!("{}", e))
    }
}
