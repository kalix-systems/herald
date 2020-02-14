use super::*;
use coremacros::from_fn;
use coretypes::messages::{
    EmptyMessageBody, MissingInboundMessageField, MissingOutboundMessageField,
};
use herald_common::*;
use herald_ids::*;
use location::Location;
use std::fmt;
use thiserror::*;

#[derive(Debug, Error)]
/// Error variants
pub enum HErr {
    #[error("Herald error: {0}")]
    /// Uncategorized error.
    HeraldError(String),
    #[error("Database error: {0}")]
    /// Database error.
    DatabaseError(#[from] rusqlite::Error),
    #[error("Invalid ID: {0}")]
    /// Invalid `ConversationId` or `MsgId`
    BadRandomId(#[from] InvalidRandomIdLength),
    #[error("Missing outbound message field: {0}")]
    /// Missing fields when sending a message
    MissingOutboundMessageField(#[from] MissingOutboundMessageField),
    #[error("Missing inbound message field: {0}")]
    /// Missing fields when storing a received a message
    MissingInboundMessageField(#[from] MissingInboundMessageField),
    #[error("IO error: {0}")]
    /// IO Error
    IoError(#[from] std::io::Error),
    #[error("ImageError: {0}")]
    /// Error processing images
    ImageError(#[source] image_utils::ImageError),
    #[error("Invalid regex: {0}")]
    /// Error compiling regex
    RegexError(#[from] search_pattern::SearchPatternError),
    #[error("Kson: {0}")]
    /// Deserialization error
    KsonError(#[from] KsonError),
    #[error("Login failed: bad sig")]
    LoginChallengeFailed,
    #[error("Login failed: invalid claim {0:?}")]
    LoginClaimFailed(protocol::auth::login_types::ClaimResponse),
    #[error("Websocket error: {0}")]
    /// An HTTP request was dropped
    /// Websocket issue
    WebsocketError(#[from] websocket::result::WebSocketError),
    #[error("Unexpected None at {0}")]
    /// Unexpected `None`
    NoneError(Location),
    #[error("Channel send failed at {0}")]
    /// An error occured sending a value through a channel
    ChannelSendError(Location),
    #[error("Channel recv failed at {0}")]
    /// An error occured receiving a value from a channel
    ChannelRecvError(Location),
    #[error("Malformed path: {0:?}")]
    /// Malformed path
    BadPath(std::ffi::OsString),
    #[error("Attachment error: {0}")]
    /// Attachments error
    Attachment(#[from] herald_attachments::Error),
    #[error("Message body cannot be empty: {0}")]
    /// An empty message body,
    EmptyMessageBody(EmptyMessageBody),
    #[error("Invalid socket addr")]
    /// Bad socket address
    BadSocketAddr(#[from] std::net::AddrParseError),
}

impl From<image_utils::ImageError> for HErr {
    fn from(e: image_utils::ImageError) -> Self {
        use image_utils::ImageError;
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
        use ::herald_common::loc;
        $crate::errors::HErr::ChannelSendError(loc!())
    }};
}

#[macro_export]
/// Creates a `ChannelRecvError`
macro_rules! channel_recv_err {
    () => {{
        use ::herald_common::loc;
        $crate::errors::HErr::ChannelRecvError(loc!())
    }};
}

/// Returns a `NoneError` annotated with the current file and line number.
#[macro_export]
macro_rules! NE {
    () => {{
        use ::herald_common::loc;
        $crate::errors::HErr::NoneError(loc!())
    }};
}
