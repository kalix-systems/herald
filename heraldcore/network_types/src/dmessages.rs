use channel_ratchet::*;
use herald_common::*;
use herald_ids::ConversationId;
use kcl::box_;

#[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq)]
/// A message sent to a specific device.
pub struct DeviceMessage {
    /// The sender of the message
    pub from: GlobalId,
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`DeviceMessageBody`]
    pub content: Bytes,
    pub tag: box_::Tag,
    /// The prekey used to encrypt the message.
    /// If none, the message was encrypted with this device's public signing key treated as an
    /// encryption key.
    /// Until we've implemented prekey infrastructure, this will always be `None`.
    pub prekey: Option<box_::PublicKey>,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A contact request.
pub struct UserReq {
    /// The genesis block for the conversation.
    pub ratchet: RatchetState,
    /// The proposed conversation id.
    pub cid: ConversationId,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Types of device message.
pub enum DeviceMessageBody {
    /// A contact request
    Req(UserReq),
}
