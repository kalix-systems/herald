use super::*;

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

impl ToSql for MessageReceiptStatus {
    fn to_sql(&self) -> Result<types::ToSqlOutput, rusqlite::Error> {
        use types::*;

        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
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

mod cmessages {
    use super::*;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct NewKey(sig::PublicKey);
    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct DepKey(sig::PublicKey);
    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct NewMembers(Vec<UserId>);
    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct AddedToConvo {
        members: Vec<UserId>,
        meta: crate::conversation::ConversationMeta,
    }
    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct ContactReqAck(bool);
    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct Msg {
        mid: MsgId,
        from: UserId,
        content: Message,
    }
    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub enum Message {
        Text(String),
        Blob(Bytes),
        Ack(MessageReceiptStatus),
    }
}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum ConversationMessageBody {
    NewKey(cmessages::NewKey),
    DepKey(cmessages::DepKey),
    NewMembers(cmessages::NewMembers),
    AddedToConvo(cmessages::AddedToConvo),
    ContactReqAck(cmessages::ContactReqAck),
    Msg(cmessages::Msg),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct ConversationMessage {
    pub body: ConversationMessageBody,
    /// Conversation the message is associated with
    pub cid: ConversationId,
    // TODO: block caching
    // pub bid: UQ,
}

mod dmessages {
    use super::*;

    #[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct ContactReq {
        uid: UserId,
        cid: ConversationId,
    }
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToDevice {
    ContactReq(dmessages::ContactReq),
}