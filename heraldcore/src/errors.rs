use chainkeys::ChainKeysError;
use coremacros::from_fn;
use coretypes::{
    ids::InvalidRandomIdLength,
    messages::{EmptyMessageBody, MissingInboundMessageField, MissingOutboundMessageField},
};
use herald_common::*;
use image;
use location::Location;
use std::fmt;

#[derive(Debug)]
/// Error variants
pub enum HErr {
    // TODO: replace all instances of this with enum branches
    /// Uncategorized error.
    HeraldError(String),
    /// Database error.
    DatabaseError(rusqlite::Error),
    /// Invalid `ConversationId` or `MsgId`
    BadRandomId(InvalidRandomIdLength),
    /// Missing fields when sending a message
    MissingOutboundMessageField(MissingOutboundMessageField),
    /// Missing fields when storing a received a message
    MissingInboundMessageField(MissingInboundMessageField),
    /// IO Error
    IoError(std::io::Error),
    /// Error processing images
    ImageError(image::ImageError),
    /// Error compiling regex
    RegexError(search_pattern::SearchPatternError),
    /// Deserialization error
    KsonError(KsonError),
    /// Global ID was either already active or involved a nonexistent user
    GIDSpecFailed(login::SignAsResponse),
    /// Failed to sign in - either signature or timestamp was invalid
    SignInFailed(login::LoginTokenResponse),
    /// An HTTP request was dropped
    /// Websocket issue
    WebsocketError(websocket::result::WebSocketError),
    /// Unexpected `None`
    NoneError(Location),
    /// An error occured sending a value through a channel
    ChannelSendError(Location),
    /// An error occured receiving a value from a channel
    ChannelRecvError(Location),
    /// Error from `chainkeys`
    ChainError(ChainKeysError),
    /// Malformed path
    BadPath(std::ffi::OsString),
    /// Attachments error
    Attachment(coretypes::attachments::Error),
    /// An empty message body,
    EmptyMessageBody,
    /// Invalid `UserId`
    InvalidUserId,
}

impl fmt::Display for HErr {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        use HErr::*;
        match self {
            DatabaseError(e) => write!(f, "Database Error: {}", e),
            HeraldError(s) => write!(f, "Herald Error: {}", s),
            InvalidUserId => write!(f, "InvalidUserId"),
            IoError(e) => write!(f, "IoError: {}", e),
            ImageError(s) => write!(f, "ImageError: {}", s),
            KsonError(e) => write!(f, "KsonError error: {}", e),
            BadPath(s) => write!(f, "Bad path: {:?}", s),
            RegexError(e) => write!(f, "RegexError: {}", e),
            // FIXME ChainError could have a good display implementation
            ChainError(e) => write!(f, "ChainError: {:?}", e),
            GIDSpecFailed(lt) => write!(f, "GIDSpecFailed: {:?}", lt),
            SignInFailed(lt) => write!(f, "SignInFailed: {:?}", lt),
            WebsocketError(e) => write!(f, "WebsocketError: {}", e),
            MissingOutboundMessageField(missing) => write!(f, "{}", missing),
            MissingInboundMessageField(missing) => write!(f, "{}", missing),
            NoneError(location) => write!(f, "Unexpected none at {}", location),
            ChannelSendError(location) => write!(f, "Channel send error at {}", location),
            ChannelRecvError(location) => write!(f, "Channel receive error at {}", location),
            BadRandomId(e) => write!(f, "{}", e),
            Attachment(e) => write!(f, "{}", e),
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
            // KsonError(e) => e,
            RegexError(e) => e,
            WebsocketError(e) => e,
            _ => return None,
        })
    }
}

macro_rules! herr {
    ($from:ty, $fn:ident) => {
        from_fn!(HErr, $from, HErr::$fn);
    };
}

herr!(MissingOutboundMessageField, MissingOutboundMessageField);
herr!(MissingInboundMessageField, MissingInboundMessageField);
herr!(ChainKeysError, ChainError);
herr!(rusqlite::Error, DatabaseError);
herr!(std::io::Error, IoError);
herr!(KsonError, KsonError);
herr!(websocket::result::WebSocketError, WebsocketError);
herr!(search_pattern::SearchPatternError, RegexError);
herr!(std::ffi::OsString, BadPath);
herr!(coretypes::attachments::Error, Attachment);

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
/// Creates a `ChannelSendError`
macro_rules! channel_send_err {
    () => {{
        use herald_common::loc;
        $crate::errors::HErr::ChannelSendError(loc!())
    }};
}

#[macro_export]
/// Creates a `ChannelRecvError`
macro_rules! channel_recv_err {
    () => {{
        use herald_common::loc;
        $crate::errors::HErr::ChannelRecvError(loc!())
    }};
}

/// Returns a `NoneError` annotated with the current file and line number.
#[macro_export]
macro_rules! NE {
    () => {{
        use herald_common::loc;
        $crate::errors::HErr::NoneError(loc!())
    }};
}
