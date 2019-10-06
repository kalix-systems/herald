use herald_common::serde_cbor;
use image;
use lazy_pond::LazyError;
use regex;
use std::fmt;

#[derive(Debug)]
/// Error variants
pub enum HErr {
    // TODO: replace all instances of this with enum branches
    /// Uncategorized error.
    HeraldError(String),
    /// Database error.
    DatabaseError(rusqlite::Error),
    /// Error from connection pool
    LazyPondError,
    /// Invalid `UserId`
    InvalidUserId,
    /// Invalid `MsgId`
    InvalidMessageId,
    /// Invalid `ConversationId`
    InvalidConversationId,
    /// IO Error
    IoError(std::io::Error),
    /// Error processing images
    ImageError(image::ImageError),
    /// Error compiling regex
    RegexError(regex::Error),
    /// Serialization or deserialization
    /// error
    CborError(serde_cbor::Error),
    /// An issue occurred at login
    LoginError,
    /// An issue occurred at registration
    RegistrationError,
    /// Failed to find expected
    MissingFields,
    /// An HTTP request was dropped
    /// Websocket issue
    TungsteniteError(tungstenite::Error),
    /// Unexpected `None`
    NoneError,
}

impl fmt::Display for HErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HErr::*;
        match self {
            DatabaseError(e) => write!(f, "Database Error: {}", e),
            HeraldError(s) => write!(f, "Herald Error: {}", s),
            InvalidUserId => write!(f, "InvalidUserId"),
            IoError(e) => write!(f, "IoError: {}", e),
            ImageError(s) => write!(f, "ImageError: {}", s),
            // Utf8Error(e) => write!(f, "Utf8Error error: {}", e),
            CborError(e) => write!(f, "CborError error: {}", e),
            RegexError(e) => write!(f, "RegexError: {}", e),
            InvalidMessageId => write!(f, "InvalidMessageId"),
            InvalidConversationId => write!(f, "InvalidConversationId"),
            LazyPondError => write!(f, "LazyPondError"),
            LoginError => write!(f, "LoginError"),
            RegistrationError => write!(f, "RegistrationError"),
            MissingFields => write!(f, "MissingFields"),
            //RequestDropped => write!(f, "RequestDropped"),
            TungsteniteError(e) => write!(f, "TungsteniteError: {}", e),
            NoneError => write!(f, "Unexpected none"),
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
            //Utf8Error(s) => s,
            CborError(e) => e,
            RegexError(e) => e,
            TungsteniteError(e) => e,
            _ => return None,
        })
    }
}

macro_rules! from_fn {
    ($to:ty, $from:ty, $fn:expr) => {
        impl From<$from> for $to {
            fn from(f: $from) -> $to {
                $fn(f)
            }
        }
    };
}

// TODO: replace these

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

impl From<LazyError> for HErr {
    fn from(_: LazyError) -> Self {
        HErr::LazyPondError
    }
}

from_fn!(HErr, tungstenite::Error, HErr::TungsteniteError);
