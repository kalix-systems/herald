use super::*;
use crate::{
    chainkeys::DecryptionResult, config::*, errors::HErr::*, message::attachments::Attachment,
};
use chainmail::{block::*, errors::ChainError::CryptoError};
use std::{convert::AsRef, fmt, time::Duration};

/// Types relevant to [`ConversationMessage`]s
pub(crate) mod cmessages;
/// Types associated with [`DeviceMessage`]s
pub(crate) mod dmessages;

#[derive(Clone, Copy, Debug, Ser, De, Eq, PartialEq, Hash)]
/// In order to support expiring messages, it is necessary to indicate
/// that a message is a reply without necessarily knowing
pub enum ReplyId {
    /// Not a reply
    None,
    /// It is a reply, but the original message could not be located
    Dangling,
    /// The message id is known
    Known(MsgId),
}

impl ReplyId {
    /// Indicates whether `ReplyId` is `None`
    pub fn is_none(&self) -> bool {
        self == &ReplyId::None
    }

    /// Indicates whether `ReplyId` is `Dangling`
    pub fn is_dangling(&self) -> bool {
        self == &ReplyId::Dangling
    }

    /// Indicates whether `ReplyId` is `Known`
    pub fn is_known(&self) -> bool {
        if let ReplyId::Known(_) = self {
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    pub(crate) fn unwrap(self) -> MsgId {
        match self {
            ReplyId::Known(mid) => mid,
            ReplyId::Dangling => panic!("Tried to unwrap `Dangling` `ReplyId`"),
            ReplyId::None => panic!("Tried to unwrap `None` `ReplyId`"),
        }
    }
}

impl From<Option<MsgId>> for ReplyId {
    fn from(maybe_mid: Option<MsgId>) -> Self {
        match maybe_mid {
            Some(mid) => ReplyId::Known(mid),
            None => ReplyId::None,
        }
    }
}

impl From<(Option<MsgId>, bool)> for ReplyId {
    fn from(val: (Option<MsgId>, bool)) -> Self {
        match val {
            (Some(mid), true) => ReplyId::Known(mid),
            (None, true) => ReplyId::Dangling,
            _ => ReplyId::None,
        }
    }
}

#[derive(Clone, Copy, Debug, Ser, De, Eq, PartialEq, Hash)]
#[repr(u8)]
/// Expiration period for messages
pub enum ExpirationPeriod {
    /// Messages never expire
    Never = 0,
    /// Messages expire after one minute
    OneMinute = 1,
    /// Messages expire after one hour
    OneHour = 2,
    /// Messages expire after one day
    OneDay = 3,
    /// Message expire after one week
    OneWeek = 4,
    /// Messages expire after one month
    OneMonth = 5,
    /// Messages expire after one year
    OneYear = 6,
}

const MIN_SECS: u64 = 60;
const HOUR_SECS: u64 = MIN_SECS * 60;
const DAY_SECS: u64 = HOUR_SECS * 24;
const WEEK_SECS: u64 = DAY_SECS * 7;
const MONTH_SECS: u64 = WEEK_SECS * 4;
const YEAR_SECS: u64 = WEEK_SECS * 52;

impl ExpirationPeriod {
    /// Converts an `ExpirationPeriod` to a `Duration`
    pub fn into_duration(self) -> Option<Duration> {
        use ExpirationPeriod::*;
        match self {
            OneMinute => Some(Duration::from_secs(MIN_SECS)),
            OneHour => Some(Duration::from_secs(HOUR_SECS)),
            OneDay => Some(Duration::from_secs(DAY_SECS)),
            OneWeek => Some(Duration::from_secs(WEEK_SECS)),
            OneMonth => Some(Duration::from_secs(MONTH_SECS)),
            OneYear => Some(Duration::from_secs(YEAR_SECS)),
            Never => None,
        }
    }

    /// Converts an `ExpirationPeriod` to milliseconds
    pub fn into_millis(self) -> Option<Time> {
        match self.into_duration() {
            Some(d) => Some((d.as_millis() as i64).into()),
            None => None,
        }
    }
}

impl From<u8> for ExpirationPeriod {
    fn from(val: u8) -> Self {
        use ExpirationPeriod::*;
        match val {
            0 => Never,
            1 => OneMinute,
            2 => OneHour,
            3 => OneDay,
            4 => OneWeek,
            5 => OneMonth,
            6 => OneYear,
            _ => Self::default(),
        }
    }
}

impl Default for ExpirationPeriod {
    fn default() -> Self {
        ExpirationPeriod::OneYear
    }
}

impl FromSql for ExpirationPeriod {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        kson::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for ExpirationPeriod {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(
            kson::to_vec(self).map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?,
        )))
    }
}

#[derive(Clone, Copy, Debug)]
/// Time data relating to messages
pub struct MessageTime {
    /// The `Time` the message reached the server, if applicable.
    pub server: Option<Time>,
    /// The `Time` the message was saved on this device
    pub insertion: Time,
    /// The `Time` the message will expire, if applicable
    pub expiration: Option<Time>,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A message body
pub struct MessageBody(String);

impl fmt::Display for MessageBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<String> for MessageBody {
    type Error = EmptyMessageBody;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(EmptyMessageBody);
        }
        Ok(Self(s))
    }
}

impl TryFrom<&str> for MessageBody {
    type Error = EmptyMessageBody;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(EmptyMessageBody);
        }
        Ok(Self(s.to_owned()))
    }
}

impl Into<String> for MessageBody {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for MessageBody {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl MessageBody {
    /// Returns `MessageBody` as `&str`
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Returns `MessageBody` as `&[u8]`
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }

    /// Parses the text as markdown, rendering it to HTML
    pub fn parse_markdown(&self) -> Result<Self, EmptyMessageBody> {
        use pulldown_cmark::{html, Parser};

        let body_str = self.as_str();

        let parser = Parser::new(body_str);
        let mut buf = String::with_capacity(body_str.len());
        html::push_html(&mut buf, parser);

        buf.try_into()
    }
}

impl ToSql for MessageBody {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.as_slice())))
    }
}

impl FromSql for MessageBody {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        value
            .as_str()?
            .to_owned()
            .try_into()
            .map_err(|_| FromSqlError::InvalidType)
    }
}

#[derive(Debug)]
/// Error returned when trying to creat an empty message body
pub struct EmptyMessageBody;

impl fmt::Display for EmptyMessageBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message bodies must have at least one character")
    }
}

impl std::error::Error for EmptyMessageBody {}

#[derive(Debug)]
/// Error returned if an inbound message is missing data
pub enum MissingInboundMessageField {
    /// Message id was missing
    MissingMessageId,
    /// Body was missing
    MissingBody,
    /// Conversation id was missing
    MissingConversationId,
    /// Timestamp was missing
    MissingTimestamp,
    /// Author was missing
    MissingAuthor,
}

impl fmt::Display for MissingInboundMessageField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingInboundMessageField::MissingMessageId => write!(f, "Message id was missing"),
            MissingInboundMessageField::MissingBody => write!(f, "Body was missing"),
            MissingInboundMessageField::MissingConversationId => {
                write!(f, "Conversation id was missing")
            }
            MissingInboundMessageField::MissingTimestamp => write!(f, "Timestamp was missing"),
            MissingInboundMessageField::MissingAuthor => write!(f, "Author was missing"),
        }
    }
}

impl std::error::Error for MissingInboundMessageField {}

#[derive(Debug)]
/// Error returned if an outbound message is missing data
pub enum MissingOutboundMessageField {
    /// Message body was missing
    MissingBody,
    /// Conversation id was missing
    MissingConversationId,
}

impl fmt::Display for MissingOutboundMessageField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingOutboundMessageField::MissingBody => write!(f, "Body was missing"),
            MissingOutboundMessageField::MissingConversationId => {
                write!(f, "Conversation id was missing")
            }
        }
    }
}

impl std::error::Error for MissingOutboundMessageField {}

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

impl Ser for MessageSendStatus {
    fn ser(&self, s: &mut Serializer) {
        (*self as u8).ser(s)
    }
}

impl De for MessageSendStatus {
    fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
        let u = u8::de(d)?;
        u.try_into().map_err(|u| {
            E!(
                CustomError(format!("expected a value between {} and {}", 0, 2)),
                d.data.clone(),
                d.ix
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

impl Ser for MessageReceiptStatus {
    fn ser(&self, s: &mut Serializer) {
        (*self as u8).ser(s)
    }
}

impl De for MessageReceiptStatus {
    fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
        let u = u8::de(d)?;
        u.try_into().map_err(|u| {
            E!(
                CustomError(format!("expected a value between {} and {}", 0, 3)),
                d.data.clone(),
                d.ix
            )
        })
    }
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// The body of a [`ConversationMessage`]
pub enum ConversationMessageBody {
    /// A new key
    NewKey(cmessages::NewKey),
    /// A key to be marked as deprecated
    DepKey(cmessages::DepKey),
    /// Members just added to a conversation
    NewMembers(cmessages::NewMembers),
    /// A message a user receives upon being added to a conversation
    AddedToConvo(Box<cmessages::AddedToConvo>),
    /// An acknowledgement of a contact request.
    ContactReqAck(cmessages::ContactReqAck),
    /// A normal message.
    Msg(cmessages::Msg),
    /// An acknowledgement of a normal message.
    Ack(cmessages::Ack),
    /// An update to the conversation settings
    Settings(crate::conversation::settings::SettingsUpdate),
}

impl FromSql for ConversationMessageBody {
    fn column_result(value: types::ValueRef) -> FromSqlResult<Self> {
        kson::from_slice(value.as_blob().map_err(|_| FromSqlError::InvalidType)?)
            .map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for ConversationMessageBody {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Blob(
            kson::to_vec(self).map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?,
        )))
    }
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
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
        cid: ConversationId,
        content: &ConversationMessageBody,
    ) -> Result<(ConversationMessage, BlockHash, ChainKey), HErr> {
        let cbytes = kson::to_vec(content)?;
        let kp = Config::static_keypair()?;
        let from = Config::static_gid()?;
        let (hashes, keys) = cid.get_unused()?.into_iter().unzip();
        let channel_key = cid.get_channel_key()?;

        let SealData { block, key } =
            Block::seal(kp.secret_key(), &channel_key, &keys, hashes, cbytes).ok_or(CryptoError)?;
        let hash = block.compute_hash().ok_or(CryptoError)?;

        Ok((
            ConversationMessage {
                cid,
                from,
                body: block,
            },
            hash,
            key,
        ))
    }

    /// Opens the message.
    pub fn open(self) -> Result<Vec<(ConversationMessageBody, GlobalId)>, HErr> {
        let ConversationMessage { cid, from, body } = self;

        let mut out = Vec::new();

        let mut blocks = {
            match cid.open_block(&from, body)? {
                DecryptionResult::Success(bvec, unlocked) => {
                    out.push((kson::from_slice(&bvec)?, from));
                    unlocked
                }
                DecryptionResult::Pending => Vec::new(),
            }
        };

        while let Some((block, from)) = blocks.pop() {
            match cid.open_block(&from, block)? {
                DecryptionResult::Success(bvec, mut unlocked) => {
                    blocks.append(&mut unlocked);
                    out.push((kson::from_slice(&bvec)?, from));
                }
                DecryptionResult::Pending => {
                    panic!("this should never happen");
                }
            }
        }

        Ok(out)
    }
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// Types of device message.
pub enum DeviceMessageBody {
    /// A contact request
    ContactReq(dmessages::ContactReq),
}

#[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq)]
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
        let mut content = kson::to_vec(content)?;

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
            ..
        } = self;

        let pk = spk_to_epk(&from.did)?;

        let kp = Config::static_keypair()?;
        let sk = ssk_to_esk(kp.secret_key())?;

        box_::open_detached(&mut content, &tag, &nonce, &pk, &sk)
            .map_err(|_| HeraldError("Failed to decrypt message to device".into()))?;

        let dm = kson::from_slice(&content)?;

        Ok((from, dm))
    }
}
