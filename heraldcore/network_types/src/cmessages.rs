use coretypes::{
    attachments::Attachment,
    conversation,
    ids::*,
    messages::{MessageBody, MessageReceiptStatus},
};
use herald_common::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A message in a conversation
pub enum ConversationMessage {
    /// Telling everyone else that you're leaving a conversation
    Leave,
    /// Members just added to a conversation
    NewMembers(NewMembers),
    /// A normal message.
    Msg(Msg),
    /// An acknowledgement of a normal message.
    Ack(Ack),
    /// An update to the conversation settings
    Settings(conversation::settings::SettingsUpdate),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Members that have just been added to a conversation.
pub struct NewMembers(pub Vec<UserId>);

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A normal message to the conversation.
pub struct Msg {
    /// The message id. Globally unique.
    pub mid: MsgId,
    /// The content of the message.
    pub content: Message,
    /// The message id of the message being replied to, if this
    /// message is a reply.
    pub op: Option<MsgId>,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Variants of messages.
pub struct Message {
    /// Body of the message
    pub body: Option<MessageBody>,
    /// Attachments
    pub attachments: Vec<Attachment>,
    /// Expiration time of the message
    pub expiration: Option<Time>,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement that a message was received.
pub struct Ack {
    /// The message id.
    pub of: MsgId,
    /// The receipt status of the message.
    pub stat: MessageReceiptStatus,
}
