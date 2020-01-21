//! Core logic for herald client.

#![allow(warnings)]
/// User configuration
pub mod config;
/// Conversations
pub mod conversation;
/// Wrapper around database.
pub mod db;
/// Errors
pub mod errors;
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
/// Notification stream
pub mod updates;
/// Functions and data structures related to users.
pub mod user;
/// User keys
mod user_keys;
/// Utils
pub mod utils;

pub use image_utils;
pub use platform_dirs::set_data_dir;
pub(crate) use updates::{err, push, Notification};

#[cfg(test)]
#[macro_use]
extern crate coremacros;
