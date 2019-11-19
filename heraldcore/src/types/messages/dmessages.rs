use super::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A contact request.
pub struct UserReq {
    /// The genesis block for the conversation.
    pub gen: Genesis,
    /// The proposed conversation id.
    pub cid: ConversationId,
}
