use coretypes::ids::*;
use herald_common::*;
use kdf_ratchet::RatchetState;
use std::collections::HashMap;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum AuxMessage {
    /// A new key
    NewKey(NewKey),
    /// A deprecated key
    DepKey(DepKey),
    /// Update ratchets for certain conversations
    NewRatchets(NewRatchets),
    /// A message a user receives upon being added to a conversation
    AddedToConvo(Box<AddedToConvo>),
    /// An acknowledgement of a contact request.
    UserReqAck(UserReqAck),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A new, signed key.
pub struct NewKey(pub Signed<sig::PublicKey>);

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A signed key to be deprecated
pub struct DepKey(pub Signed<sig::PublicKey>);

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// New ratchets to associate with conversations
pub struct NewRatchets(pub HashMap<ConversationId, RatchetState>);

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A message received by a user when they are addeded to a conversation.
pub struct AddedToConvo {
    /// The current ratchet states in the conversation
    pub ratchets: HashMap<sig::PublicKey, RatchetState>,
    /// The members of the conversation
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

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement of a user request, with a bool to indicate whether the
/// request was accepted.
pub struct UserReqAck(pub bool);
