use herald_common::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("Kson: {0}")]
    Kson(#[from] kson::prelude::KsonError),
    #[error("Bad signature")]
    BadSignature,
    #[error("Bad key")]
    BadKey,
    #[error("Invalid user id")]
    InvalidUserId(#[from] InvalidUserId),
}
