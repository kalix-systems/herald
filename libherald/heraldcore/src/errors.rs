use herald_common::{serde_cbor, TransportError};
use image;
use regex;
use std::{fmt, sync::PoisonError};

#[derive(Debug)]
pub enum HErr {
    HeraldError(String),
    DatabaseError(rusqlite::Error),
    MutexError(String),
    InvalidUserId(String),
    InvalidMessageId,
    InvalidConversationId,
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
    ImageError(image::ImageError),
    RegexError(regex::Error),
    CborError(serde_cbor::Error),
    TransportError(TransportError),
    LoginError,
    RegistrationError,
    MissingFields,
}

impl fmt::Display for HErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HErr::*;
        match self {
            DatabaseError(e) => write!(f, "Database Error: {}", e),
            HeraldError(s) => write!(f, "Herald Error: {}", s),
            MutexError(s) => write!(f, "Mutex Error: {}", s),
            InvalidUserId(s) => write!(f, "InvalidUserId: {}", s),
            IoError(e) => write!(f, "IoError: {}", e),
            ImageError(s) => write!(f, "ImageError: {}", s),
            Utf8Error(e) => write!(f, "Utf8Error error: {}", e),
            CborError(e) => write!(f, "CborError error: {}", e),
            TransportError(s) => write!(f, "TransportError: {}", s),
            RegexError(e) => write!(f, "RegexError: {}", e),
            InvalidMessageId => write!(f, "InvalidMessageId"),
            InvalidConversationId => write!(f, "InvalidConversationId"),
            LoginError => write!(f, "LoginError"),
            RegistrationError => write!(f, "RegistrationError"),
            MissingFields => write!(f, "MissingFields"),
        }
    }
}

impl std::error::Error for HErr {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use HErr::*;
        Some(match self {
            DatabaseError(e) => e,
            IoError(e) => e,
            ImageError(s) => s,
            Utf8Error(s) => s,
            CborError(e) => e,
            TransportError(s) => s,
            RegexError(e) => e,
            _ => return None,
        })
    }
}

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

impl From<std::io::Error> for HErr {
    fn from(e: std::io::Error) -> Self {
        HErr::IoError(e)
    }
}

impl From<image::ImageError> for HErr {
    fn from(e: image::ImageError) -> Self {
        use image::ImageError;
        match e {
            ImageError::IoError(e) => e.into(),
            e => HErr::ImageError(e),
        }
    }
}

impl From<regex::Error> for HErr {
    fn from(e: regex::Error) -> Self {
        HErr::RegexError(e)
    }
}

impl From<std::ffi::OsString> for HErr {
    fn from(e: std::ffi::OsString) -> Self {
        HErr::HeraldError(format!("Bad path: {:?}", e))
    }
}

impl From<serde_cbor::Error> for HErr {
    fn from(e: serde_cbor::Error) -> Self {
        HErr::CborError(e)
    }
}

impl From<TransportError> for HErr {
    fn from(e: TransportError) -> Self {
        HErr::TransportError(e)
    }
}

impl From<std::str::Utf8Error> for HErr {
    fn from(e: std::str::Utf8Error) -> Self {
        HErr::Utf8Error(e)
    }
}
