//! Client logic for herald.
#![deny(missing_docs)]
#![allow(clippy::needless_return)]

/// Implementation of autogenerated traits.
pub mod imp;

/// Autogenerated by `rust-qt-binding-generator`.
#[allow(missing_docs)]
#[allow(clippy::all)]
#[allow(unused_parens)]
pub mod interface;

/// Attachments object
pub mod attachments;
/// Config object
pub mod config;
/// Conversation builder
pub mod conversation_builder;
/// Conversation content
pub mod conversation_content;
/// Conversations object
pub mod conversations;
/// Emoji Picker backend Object
pub mod emoji_picker;
/// Error queue
pub mod errors;
/// FFI type aliases.
pub mod ffi;
/// Herald state object
pub mod herald;
/// Members object
pub mod members;
/// Message search object
pub mod message_search;
/// Messages object
pub mod messages;
/// Desktop push notifications
pub mod toasts;
/// Users object
pub mod users;
/// Users search object
pub mod users_search;
/// Utils object
pub mod utils;

pub(crate) use conversation_content::content_push;
pub(crate) use herald::{push, Update};

pub(crate) trait Loadable {
    type Error;

    fn try_load(&mut self) -> Result<(), Self::Error>;

    fn loaded(&self) -> bool;
}
