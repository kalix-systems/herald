use kson::prelude::KsonError;
use thiserror::Error;

#[cfg(feature = "async")]
#[derive(Debug, Error)]
pub enum FramedError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialization: {0}")]
    Encoding(#[from] KsonError),
    #[error("Timed out: {0}")]
    TimedOut(#[from] tokio::time::Elapsed),
}

#[cfg(not(feature = "async"))]
#[derive(Debug, Error)]
pub enum FramedError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialization: {0}")]
    Encoding(#[from] KsonError),
}
