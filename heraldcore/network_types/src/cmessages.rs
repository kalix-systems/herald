use chainmail::block::Block;
use coretypes::ids::ConversationId;
use herald_common::*;

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
