use herald_common::UserId;

// TODO: have fewer of these
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Cbor(serde_cbor::Error),
    InvalidSig,
    InvalidKey,
    MissingData,
    CommandFailed,
    BadData,
    RedundantDeprecation,
    PgError(tokio_postgres::Error),
    UnknownUser(UserId),
    CatchupFailed,
    LoginFailed,
    RegistrationFailed,
    BadSessionType(u8),
    TimedOut(tokio::timer::timeout::Elapsed),
    SendFailed(tokio::sync::mpsc::error::UnboundedSendError),
    StreamDied,
}

pub use Error::*;

macro_rules! from_fn {
    ($to:ty, $from:ty, $fn:expr) => {
        impl From<$from> for $to {
            fn from(f: $from) -> $to {
                $fn(f)
            }
        }
    };
}

from_fn!(Error, std::io::Error, Error::IO);
from_fn!(Error, tokio_postgres::Error, Error::PgError);
from_fn!(Error, serde_cbor::Error, Error::Cbor);
from_fn!(Error, tokio::timer::timeout::Elapsed, TimedOut);
from_fn!(
    Error,
    tokio::sync::mpsc::error::UnboundedSendError,
    SendFailed
);
