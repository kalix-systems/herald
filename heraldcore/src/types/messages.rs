use super::*;
use crate::{config::*, errors::HErr::*};
use std::convert::AsRef;

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
/// Send status of a message
pub enum MessageSendStatus {
    /// No ack from server
    NoAck = 0,
    /// Acknowledged by server
    Ack = 1,
    /// The message has timed-out.
    Timeout = 2,
}

impl TryFrom<u8> for MessageSendStatus {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoAck),
            1 => Ok(Self::Ack),
            2 => Ok(Self::Timeout),
            i => Err(i),
        }
    }
}

impl ToSql for MessageSendStatus {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl FromSql for MessageSendStatus {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl std::convert::TryFrom<i64> for MessageSendStatus {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n
                .try_into()
                .map_err(|n| HErr::HeraldError(format!("Unknown status {}", n))),
            Err(_) => Err(HErr::HeraldError(format!("Unknown status {}", n))),
        }
    }
}

impl Serialize for MessageSendStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for MessageSendStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::deserialize(d)?;
        u.try_into().map_err(|u| {
            Error::invalid_value(
                Unexpected::Unsigned(u64::from(u)),
                &format!("expected a value between {} and {}", 0, 2).as_str(),
            )
        })
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
/// Receipt status of a message
pub enum MessageReceiptStatus {
    /// Not acknowledged
    NoAck = 0,
    /// Received by user
    Received = 1,
    /// Read by the recipient
    Read = 2,
    /// The user has read receipts turned off
    AckTerminal = 3,
}

impl TryFrom<u8> for MessageReceiptStatus {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoAck),
            1 => Ok(Self::Received),
            2 => Ok(Self::Read),
            3 => Ok(Self::AckTerminal),
            i => Err(i),
        }
    }
}

impl std::convert::TryFrom<i64> for MessageReceiptStatus {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n
                .try_into()
                .map_err(|n| HErr::HeraldError(format!("Unknown status {}", n))),
            Err(_) => Err(HErr::HeraldError(format!("Unknown status {}", n))),
        }
    }
}

impl ToSql for MessageReceiptStatus {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl FromSql for MessageReceiptStatus {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl Serialize for MessageReceiptStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for MessageReceiptStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::deserialize(d)?;
        u.try_into().map_err(|u| {
            Error::invalid_value(
                Unexpected::Unsigned(u64::from(u)),
                &format!("expected a value between {} and {}", 0, 3).as_str(),
            )
        })
    }
}

/// Types relevant to [`ConversationMessage`]s
pub mod cmessages {
    use super::*;

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
    /// An acknowledgement of a contact request, with a bool to indicate whether the
    /// request was accepted.
    pub struct ContactReqAck(pub bool);
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
    pub enum Message {
        /// A text message.
        Text(String),
        /// A blob message, e.g., an attachment.
        Blob(Bytes),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    /// An acknowledgement that a message was received.
    pub struct Ack {
        /// The message id.
        pub of: MsgId,
        /// The receipt status of the message.
        pub stat: MessageReceiptStatus,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// The body of a [`ConversationMessage`]
pub enum ConversationMessageBody {
    /// A new key
    NewKey(cmessages::NewKey),
    /// A key to be marked as deprecated
    DepKey(cmessages::DepKey),
    /// Members just added to a conversation
    NewMembers(cmessages::NewMembers),
    /// A message a user receives upon being added to a conversation
    AddedToConvo(cmessages::AddedToConvo),
    /// An acknowledgement of a contact request.
    ContactReqAck(cmessages::ContactReqAck),
    /// A normal message.
    Msg(cmessages::Msg),
    /// An acknowledgement of a normal message.
    Ack(cmessages::Ack),
}

impl FromSql for ConversationMessageBody {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        serde_cbor::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for ConversationMessageBody {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(
            serde_cbor::to_vec(self)
                .map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?,
        )))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A conversation message
pub struct ConversationMessage {
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`ConversationMessageBody`].
    body: chainmail::block::Block,
    /// Conversation the message is associated with
    cid: ConversationId,
    /// Who supposedly sent the message
    from: GlobalId,
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

    /// Seals the messages.
    pub fn seal(
        // note: this is only mut because BlockStore thinks it should be
        mut cid: ConversationId,
        content: &ConversationMessageBody,
    ) -> Result<ConversationMessage, HErr> {
        let cbytes = serde_cbor::to_vec(content)?;
        let kp = Config::static_keypair()?;
        let from = Config::static_gid()?;
        let body = dbg!(cid.seal_block(kp.secret_key(), cbytes))?;
        Ok(ConversationMessage { cid, from, body })
    }

    /// Opens the message.
    pub fn open(self) -> Result<ConversationMessageBody, HErr> {
        let ConversationMessage {
            mut cid,
            from,
            body,
        } = self;
        match dbg!(cid.open_block(&from, body))? {
            DecryptionResult::Success(bvec, u) => {
                assert!(u.is_empty());
                Ok(dbg!(serde_cbor::from_slice(&bvec))?)
            }
            DecryptionResult::Pending => unimplemented!(),
        }
    }
}

/// Types associated with [`DeviceMessage`]s
pub mod dmessages {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    /// A contact request.
    pub struct ContactReq {
        /// The genesis block for the conversation.
        pub gen: Genesis,
        /// The proposed conversation id.
        pub cid: ConversationId,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Types of device message.
pub enum DeviceMessageBody {
    /// A contact request
    ContactReq(dmessages::ContactReq),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
/// A message sent to a specific device.
pub struct DeviceMessage {
    /// The sender of the message
    from: GlobalId,
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`DeviceMessageBody`]
    content: Vec<u8>,
    nonce: box_::Nonce,
    tag: box_::Tag,
    /// The prekey used to encrypt the message.
    /// If none, the message was encrypted with this device's public signing key treated as an
    /// encryption key.
    /// Until we've implemented prekey infrastructure, this will always be `None`.
    prekey: Option<box_::PublicKey>,
}

fn spk_to_epk(pk: &sig::PublicKey) -> Result<box_::PublicKey, HErr> {
    let mut pkbuf = [0u8; box_::PUBLICKEYBYTES];
    let ret = unsafe {
        libsodium_sys::crypto_sign_ed25519_pk_to_curve25519(
            pkbuf.as_mut_ptr(),
            pk.as_ref().as_ptr(),
        )
    };
    if ret == 0 {
        Ok(box_::PublicKey(pkbuf))
    } else {
        Err(HeraldError(
            "failed to convert ed25519 public key to x25519 public key".into(),
        ))
    }
}
fn ssk_to_esk(sk: &sign::SecretKey) -> Result<box_::SecretKey, HErr> {
    let mut skbuf = [0u8; box_::SECRETKEYBYTES];
    let ret = unsafe {
        libsodium_sys::crypto_sign_ed25519_sk_to_curve25519(
            skbuf.as_mut_ptr(),
            sk.as_ref().as_ptr(),
        )
    };
    if ret == 0 {
        Ok(box_::SecretKey(skbuf))
    } else {
        Err(HeraldError(
            "failed to convert ed25519 public key to x25519 public key".into(),
        ))
    }
}

impl DeviceMessage {
    pub(crate) fn seal(
        to: &sig::PublicKey,
        content: &DeviceMessageBody,
    ) -> Result<DeviceMessage, HErr> {
        let mut content = serde_cbor::to_vec(content)?;

        let pk = spk_to_epk(to)?;

        let kp = Config::static_keypair()?;
        let sk = ssk_to_esk(kp.secret_key())?;

        let nonce = box_::gen_nonce();

        let tag = box_::seal_detached(&mut content, &nonce, &pk, &sk);

        Ok(DeviceMessage {
            from: Config::static_gid()?,
            content,
            nonce,
            tag,
            prekey: None,
        })
    }

    pub(crate) fn open(self) -> Result<(GlobalId, DeviceMessageBody), HErr> {
        // TODO: remove this, handle prekey
        assert!(self.prekey.is_none());

        let DeviceMessage {
            from,
            mut content,
            nonce,
            tag,
            prekey: _,
        } = self;

        let pk = spk_to_epk(&from.did)?;

        let kp = Config::static_keypair()?;
        let sk = ssk_to_esk(kp.secret_key())?;

        box_::open_detached(&mut content, &tag, &nonce, &pk, &sk)
            .map_err(|_| HeraldError("Failed to decrypt message to device".into()))?;

        let dm = serde_cbor::from_slice(&content)?;

        Ok((from, dm))
    }
}
