use chainmail::block::{Block, Genesis};
use coretypes::{
    attachments::Attachment,
    conversation,
    ids::*,
    messages::{MessageBody, MessageReceiptStatus},
};
use herald_common::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// The body of a [`ConversationMessage`]
pub enum ConversationMessageBody {
    /// A new key
    NewKey(NewKey),
    /// A key to be marked as deprecated
    DepKey(DepKey),
    /// Members just added to a conversation
    NewMembers(NewMembers),
    /// A message a user receives upon being added to a conversation
    AddedToConvo(Box<AddedToConvo>),
    /// An acknowledgement of a contact request.
    UserReqAck(UserReqAck),
    /// A normal message.
    Msg(Msg),
    /// An acknowledgement of a normal message.
    Ack(Ack),
    /// An update to the conversation settings
    Settings(conversation::settings::SettingsUpdate),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A conversation message
pub struct ConversationMessage {
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`ConversationMessageBody`].
    pub body: chainmail::block::Block,
    /// Conversation the message is associated with
    pub cid: ConversationId,
    /// Who supposedly sent the message
    pub from: GlobalId,
}

impl ConversationMessage {
    /// Raw body of the message
    pub fn body(&self) -> &Block {
        &self.body
    }

    /// `ConversationId` associated with the message
    pub fn cid(&self) -> ConversationId {
        self.cid
    }

    /// The device the message claims to be from.
    pub fn from(&self) -> GlobalId {
        self.from
    }
}

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
    /// The conversation's picture (as bytes)
    pub picture: Option<Vec<u8>>,
    /// The conversation's initial expiration period
    pub expiration_period: coretypes::conversation::ExpirationPeriod,
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
