//! Core logic for herald client.

#![warn(missing_docs)]
#![allow(warnings)]

/// Chainmail support
mod chainkeys;
/// User configuration
pub mod config;
/// Functions and data structures related to contacts.
pub mod contact;
/// Contact keys
mod contact_keys;
/// Conversations
pub mod conversation;
/// Wrapper around database.
pub mod db;
/// Errors
mod errors;
/// Image processing
pub(crate) mod image_utils;
/// Members of conversations
pub mod members;
/// Functions and data structures related to messages.
pub mod message;
/// message status
mod message_status;
/// Networking
// pub mod network;
/// Utils
pub mod utils;
/// Tokio re-export
pub use tokio;
/// Types
pub mod types;
pub use chrono;
