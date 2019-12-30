use channel_ratchet::*;
use coretypes::{
    conversation,
    messages::{MessageBody, MessageReceiptStatus, ReactContent},
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
    /// Members just added to a conversation
    NewMembers(NewMembers),
    /// A message a user receives upon being added to a conversation
    AddedToConvo {
        info: AddedToConvo,
        /// The genesis block for the new conversation
        ratchet: RatchetState,
    },
    /// An acknowledgement of a contact request.
    UserReqAck(UserReqAck),
    /// A normal message.
    Msg(Msg),
    /// An acknowledgement of a normal message.
    Receipt(Receipt),
    /// A message reaction
    Reaction(Reaction),
    /// An update to the conversation settings
    Settings(conversation::settings::SettingsUpdate),
}
