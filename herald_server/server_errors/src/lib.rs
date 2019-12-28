use thiserror::Error;

// TODO: have fewer of these
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Kson: {0}")]
    Kson(#[from] kson::prelude::KsonError),
    #[error("Postgres: {0}")]
    PgError(#[from] tokio_postgres::Error),
    #[error("Timeout: {0}")]
    TimedOut(#[from] tokio::time::Elapsed),
    #[error("Invalid signature")]
    InvalidSig,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Invalid key")]
    MissingData,
    #[error("Login failed")]
    LoginFailed,
    #[error("Uncategorized error. Please downcast")]
    Underscore(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Invalid user id")]
    InvalidUserId(#[from] herald_common::InvalidUserId),
}

pub use Error::*;
