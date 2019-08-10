use std::fmt;

#[derive(Debug)]
pub enum HErr {
    HeraldError(String),
    DatabaseError(rusqlite::Error),
    InvalidString,
    NullPtr,
}

impl fmt::Display for HErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HErr::*;
        match self {
            DatabaseError(e) => write!(f, "Database Error: {}", e),
            HeraldError(s) => write!(f, "Herald Error: {}", s),
            InvalidString => write!(f, "Error: Tried to pass invalid string"),
            NullPtr => write!(f, "Error: Tried to pass null pointer"),
        }
    }
}

impl std::error::Error for HErr {}

impl From<rusqlite::Error> for HErr {
    fn from(e: rusqlite::Error) -> HErr {
        HErr::DatabaseError(e)
    }
}
