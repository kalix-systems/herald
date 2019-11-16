use crate::types::{EmptyMessageBody, MissingInboundMessageField, MissingOutboundMessageField};
use chainmail::errors::ChainError;
use herald_common::*;
use image;
use regex;
use std::fmt;

#[derive(Debug)]
/// A location in source code
pub struct Location {
    /// The line where the error occurred
    pub line: u32,
    /// The column where the error occurred
    pub col: u32,
    /// The file where the error occurred
    pub file: &'static str,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.file,
            line = self.line,
            column = self.file
        )
    }
}

#[derive(Debug)]
/// Error variants
pub enum HErr {
    // TODO: replace all instances of this with enum branches
    /// Uncategorized error.
    HeraldError(String),
    /// Database error.
    DatabaseError(rusqlite::Error),
    /// Invalid `UserId`
    InvalidUserId,
    /// Invalid `MsgId`
    InvalidMessageId,
    /// Invalid `ConversationId`
    InvalidConversationId,
    /// Missing fields when sending a message
    MissingOutboundMessageField(MissingOutboundMessageField),
    /// Missing fields when storing a received a message
    MissingInboundMessageField(MissingInboundMessageField),
    /// An empty message body,
    EmptyMessageBody,
    /// IO Error
    IoError(std::io::Error),
    /// Error processing images
    ImageError(image::ImageError),
    /// Error compiling regex
    RegexError(regex::Error),
    /// Serialization or deserialization
    /// error
    CborError(kson::Error),
    /// Global ID was either already active or involved a nonexistent user
    GIDSpecFailed(login::SignAsResponse),
    /// Failed to sign in - either signature or timestamp was invalid
    SignInFailed(login::LoginTokenResponse),
    /// An HTTP request was dropped
    /// Websocket issue
    WebsocketError(websocket::result::WebSocketError),
    /// Unexpected `None`
    NoneError(&'static str, u32),
    /// An error occured sending a value through a channel
    ChannelSendError(Location),
    /// An error occured receiving a value from a channel
    ChannelRecvError(Location),
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
            ChainError(e) => write!(f, "ChainError: {}", e),
            GIDSpecFailed(lt) => write!(f, "GIDSpecFailed: {:?}", lt),
            SignInFailed(lt) => write!(f, "SignInFailed: {:?}", lt),
            WebsocketError(e) => write!(f, "WebsocketError: {}", e),
            MissingOutboundMessageField(missing) => write!(f, "{}", missing),
            MissingInboundMessageField(missing) => write!(f, "{}", missing),
            NoneError(file, line) => write!(f, "Unexpected none in file {} on line {}", file, line),
            ChannelSendError(location) => write!(f, "Channel send error at {}", location),
            ChannelRecvError(location) => write!(f, "Channel receive error at {}", location),
            EmptyMessageBody => write!(f, "{}", EmptyMessageBody),
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

herr!(MissingOutboundMessageField, MissingOutboundMessageField);
herr!(MissingInboundMessageField, MissingInboundMessageField);
herr!(ChainError, ChainError);
herr!(rusqlite::Error, DatabaseError);
herr!(std::io::Error, IoError);
herr!(kson::Error, CborError);
herr!(websocket::result::WebSocketError, WebsocketError);
herr!(regex::Error, RegexError);
herr!(std::ffi::OsString, BadPath);

impl From<EmptyMessageBody> for HErr {
    fn from(_: EmptyMessageBody) -> Self {
        HErr::EmptyMessageBody
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

#[macro_export]
/// Returns the location this macro was called from
macro_rules! loc {
    () => {
        $crate::errors::Location {
            file: file!(),
            line: line!(),
            col: column!(),
        }
    };
}

#[macro_export]
/// Creates a `ChannelSendError`
macro_rules! channel_send_err {
    () => {{
        use $crate::loc;
        $crate::errors::HErr::ChannelSendError(loc!())
    }};
}

#[macro_export]
/// Creates a `ChannelRecvError`
macro_rules! channel_recv_err {
    () => {{
        use $crate::loc;
        $crate::errors::HErr::ChannelRecvError(loc!())
    }};
}

/// Returns a `NoneError` annotated with the current file and line number.
#[macro_export]
macro_rules! NE {
    () => {
        $crate::errors::HErr::NoneError(file!(), line!())
    };
}
