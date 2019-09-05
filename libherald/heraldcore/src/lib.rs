//! Core logic for herald client.

#![warn(missing_docs)]

/// User configuration
pub mod config;
/// Functions and data structures related to contacts.
pub mod contact;
/// Conversations
pub mod conversation;
/// Wrapper around database.
pub mod db;
/// Errors
mod errors;
/// Image processing
pub(crate) mod image_utils;
/// Functions and data structures related to messages.
pub mod message;
/// Utils
pub mod utils;

///// Networking
//pub mod network;
