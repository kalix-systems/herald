use chainmail::errors::ChainError;
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
    WebsocketError(websocket::result::WebSocketError),
    /// Unexpected `None`
    NoneError,
    /// Error from `chainmail`
    ChainError(ChainError),
    /// Malformed path
    BadPath(std::ffi::OsString),
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
            CborError(e) => write!(f, "CborError error: {}", e),
            BadPath(s) => write!(f, "Bad path: {:?}", s),
            RegexError(e) => write!(f, "RegexError: {}", e),
            InvalidMessageId => write!(f, "InvalidMessageId"),
            InvalidConversationId => write!(f, "InvalidConversationId"),
            LazyPondError => write!(f, "LazyPondError"),
            ChainError(e) => write!(f, "ChainError: {}", e),
            LoginError => write!(f, "LoginError"),
            RegistrationError => write!(f, "RegistrationError"),
            MissingFields => write!(f, "MissingFields"),
            WebsocketError(e) => write!(f, "WebsocketError: {}", e),
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
            CborError(e) => e,
            RegexError(e) => e,
            WebsocketError(e) => e,
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

macro_rules! herr {
    ($from:ty, $fn:ident) => {
        from_fn!(HErr, $from, HErr::$fn);
    };
}

herr!(ChainError, ChainError);
herr!(rusqlite::Error, DatabaseError);
herr!(std::io::Error, IoError);
herr!(serde_cbor::Error, CborError);
herr!(websocket::result::WebSocketError, WebsocketError);
herr!(regex::Error, RegexError);
herr!(std::ffi::OsString, BadPath);

impl From<LazyError> for HErr {
    fn from(_: LazyError) -> Self {
        HErr::LazyPondError
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
