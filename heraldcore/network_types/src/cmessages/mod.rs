use coretypes::{
    conversation,
    messages::{MessageBody, ReactContent, ReceiptStatus},
};
use herald_attachments::Attachment;
use herald_common::*;
use herald_ids::*;

mod content;
pub use content::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A message in a conversation
pub enum ConversationMessage {
    /// A message a user receives upon being added to a conversation
    AddedToConvo { info: Box<AddedToConvo> },
    /// User content
    Message(Content),
}
