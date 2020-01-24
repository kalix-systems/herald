use coretypes::messages::NewMembers;
use coretypes::{
    conversation::ExpirationPeriod,
    messages::{MessageBody, ReactContent, ReceiptStatus},
};
use herald_attachments::Attachment;
use herald_common::*;
use herald_ids::*;

mod rusqlite_imp;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub struct NetMsg {
    pub cid: ConversationId,
    pub sub: Substance,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Substance {
    /// A normal message.
    Msg(Msg),

    /// An acknowledgement of a normal message.
    Receipt(Receipt),

    /// A message reaction
    Reaction(Reaction),

    /// The sender's profile has changed
    ProfileChanged(ProfileChanged),

    /// Typing notification. Includes the time the notification was sent
    Typing(Time),

    /// Initializes a conversation
    Init(ConversationInit),

    /// A change in conversation membership
    Membership(Membership),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum ConversationInit {
    Pairwise,

    Group {
        members: Vec<UserId>,
        title: Option<String>,
        picture: Option<Vec<u8>>,
        expiration_period: ExpirationPeriod,
    },
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A normal message to the conversation.
pub struct Msg {
    /// The message id. Globally unique.
    pub mid: MsgId,
    /// The content of the message.
    pub content: MsgContent,
    /// Expiration time of the message
    pub expiration: Option<Time>,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A normal message to the conversation.
pub enum MsgContent {
    Normal(Message),
    GroupSettings(GroupSettingsUpdate),
    NewMembers(NewMembers),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum GroupSettingsUpdate {
    /// Expiring messages setting
    Expiration(ExpirationPeriod),
    /// The title of the group
    Title(Option<String>),
    /// The group picture, as a buffer
    Picture(Option<Vec<u8>>),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Normal message.
pub struct Message {
    /// Body of the message
    pub body: Option<MessageBody>,
    /// Attachments
    pub attachments: Vec<Attachment>,
    /// The message id of the message being replied to, if this
    /// message is a reply.
    pub op: Option<MsgId>,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement that a message was received.
pub struct Receipt {
    /// The message id.
    pub of: MsgId,
    /// The receipt status of the message.
    pub stat: ReceiptStatus,
}

/// A message reaction
#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub struct Reaction {
    /// The message being reacted to
    pub msg_id: MsgId,
    /// The text of the receipt
    pub react_content: ReactContent,
    /// Whether this is a removal or addition
    pub remove: bool,
}

/// A change in conversation membership
#[derive(Ser, De, Clone, PartialEq, Eq, Debug)]
pub struct Membership {
    /// Conversation that changed
    cid: ConversationId,
    /// The change
    change: MembershipUpdate,
}

/// A change in conversation membership
#[derive(Ser, De, Clone, PartialEq, Eq, Debug)]
pub enum MembershipUpdate {
    /// Members have been added
    Added {
        members: Vec<UserId>,
        added_by: UserId,
    },
    /// A member has left
    Left(UserId),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Update to the current user's profile
pub enum ProfileChanged {
    Color(u32),
    Picture(Option<Vec<u8>>),
    DisplayName(Option<String>),
}
