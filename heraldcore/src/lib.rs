//! Core logic for herald client.

#![warn(missing_docs)]

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
pub mod errors;
/// Image processing
pub(crate) mod image_utils;
/// Members of conversations
pub mod members;
/// Functions and data structures related to messages.
pub mod message;
/// Networking
pub mod network;
/// Pending out messages
pub mod pending;
/// Types
pub mod types;
/// Utils
pub mod utils;

mod platform_dirs;
