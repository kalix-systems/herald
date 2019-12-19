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
    #[error("Krpc: {0}")]
    Krpc(#[from] krpc::KrpcError),

    #[error("Invalid signature")]
    InvalidSig,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Invalid key")]
    MissingData,
    #[error("Login failed")]
    LoginFailed,
    #[error("Uncategorized error. Please downcast")]
    Underscore(#[from] Box<dyn std::error::Error + Send + 'static>),
}

pub use Error::*;
