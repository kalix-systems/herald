use herald_common::*;
use herald_ids::ConversationId;

#[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq)]
/// A message sent to a specific user.
pub enum UserMessage {
    /// A contact request
    Req(UserReq),
}

#[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq)]
/// A contact request.
pub struct UserReq {
    /// The proposed conversation id.
    pub cid: ConversationId,
}
