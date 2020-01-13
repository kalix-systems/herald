use channel_ratchet::*;
use coretypes::{
    conversation,
    messages::{MessageBody, ReactContent, ReceiptStatus},
};
use herald_attachments::Attachment;
use herald_common::*;
use herald_ids::*;

mod content;
mod crypto;
pub use content::*;
pub use crypto::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A message in a conversation
pub enum ConversationMessage {
    /// A new key
    NewKey(NewKey),
    /// A key to be marked as deprecated
    DepKey(DepKey),
    /// A message a user receives upon being added to a conversation
    AddedToConvo {
        info: Box<AddedToConvo>,
        /// The genesis block for the new conversation
        ratchet: RatchetState,
    },
    /// User content
    Message(Content),
}
