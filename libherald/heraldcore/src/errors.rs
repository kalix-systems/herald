use std::{fmt, sync::PoisonError};

#[derive(Debug)]
pub enum HErr {
    HeraldError(String),
    DatabaseError(rusqlite::Error),
    MutexError(String),
}

impl fmt::Display for HErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HErr::*;
        match self {
            DatabaseError(e) => write!(f, "Database Error: {}", e),
            HeraldError(s) => write!(f, "Herald Error: {}", s),
            MutexError(s) => write!(f, "Mutex Error: {}", s),
        }
    }
}

impl std::error::Error for HErr {}

impl<T> From<PoisonError<T>> for HErr {
    fn from(e: PoisonError<T>) -> Self {
        HErr::MutexError(e.to_string())
    }
}

impl From<rusqlite::Error> for HErr {
    fn from(e: rusqlite::Error) -> Self {
        HErr::DatabaseError(e)
    }
}
