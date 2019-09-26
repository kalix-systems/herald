use herald_common::UserId;

// TODO: have fewer of these
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Cbor(serde_cbor::Error),
    Warp(warp::Error),
    TransportError(herald_common::TransportError),
    InvalidSig,
    InvalidKey,
    MissingData,
    CommandFailed,
    BadData,
    RedundantDeprecation,
    DieselError(diesel::result::Error),
    UnknownUser(UserId),
    CatchupFailed,
    LoginFailed,
    RegistrationFailed,
    BadSessionType(u8),
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
from_fn!(Error, diesel::result::Error, Error::DieselError);
from_fn!(Error, serde_cbor::Error, Error::Cbor);
from_fn!(Error, herald_common::TransportError, Error::TransportError);
from_fn!(Error, warp::Error, Error::Warp);
