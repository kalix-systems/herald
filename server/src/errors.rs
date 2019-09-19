use herald_common::UserId;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Redis(redis::RedisError),
    Cbor(serde_cbor::Error),
    TransportError(herald_common::TransportError),
    InvalidSig,
    InvalidKey,
    MissingData,
    CommandFailed,
    BadData,
    UnknownUser(UserId),
    CatchupFailed,
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
from_fn!(Error, redis::RedisError, Error::Redis);
from_fn!(Error, serde_cbor::Error, Error::Cbor);
from_fn!(Error, herald_common::TransportError, Error::TransportError);
