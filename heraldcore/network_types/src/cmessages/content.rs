use super::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// Members just added to a conversation
    NewMembers(NewMembers),
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

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Members that have just been added to a conversation.
pub struct NewMembers(pub Vec<UserId>);

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement of a user request, with a bool to indicate whether the
/// request was accepted.
pub struct UserReqAck(pub bool);

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
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum GroupSettingsUpdate {
    /// Expiring messages setting
    Expiration(conversation::ExpirationPeriod),
    /// The title of the group
    Title(Option<String>),
    /// The color of the group
    Color(u32),
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
    pub stat: MessageReceiptStatus,
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

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
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
}
