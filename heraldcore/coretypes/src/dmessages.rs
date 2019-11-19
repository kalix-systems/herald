use herald_common::*;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
/// A message sent to a specific device.
pub struct DeviceMessage {
    /// The sender of the message
    pub from: GlobalId,
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`DeviceMessageBody`]
    pub content: Vec<u8>,
    pub nonce: box_::Nonce,
    pub tag: box_::Tag,
    /// The prekey used to encrypt the message.
    /// If none, the message was encrypted with this device's public signing key treated as an
    /// encryption key.
    /// Until we've implemented prekey infrastructure, this will always be `None`.
    pub prekey: Option<box_::PublicKey>,
}
