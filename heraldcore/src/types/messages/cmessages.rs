use super::*;
use crate::message::{MessageBody, MessageReceiptStatus};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A new, signed key.
pub struct NewKey(pub Signed<sig::PublicKey>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A key that is to be marked as deprecated.
pub struct DepKey(pub Signed<sig::PublicKey>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Members that have just been added to a conversation.
pub struct NewMembers(pub Vec<UserId>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A message received by a user when they are addeded to a conversation.
pub struct AddedToConvo {
    /// The current members in that conversation.
    pub members: Vec<UserId>,
    /// The [`ConversationId`]
    pub cid: ConversationId,
    /// The conversation's title.
    pub title: Option<String>,
    /// The genesis block for the new conversation
    pub gen: Genesis,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement of a user request, with a bool to indicate whether the
/// request was accepted.
pub struct UserReqAck(pub bool);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Variants of messages.
pub struct Message {
    /// Body of the message
    pub body: Option<MessageBody>,
    /// Attachments
    pub attachments: Vec<Attachment>,
    /// Expiration time of the message
    pub expiration: Option<Time>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement that a message was received.
pub struct Ack {
    /// The message id.
    pub of: MsgId,
    /// The receipt status of the message.
    pub stat: MessageReceiptStatus,
}
