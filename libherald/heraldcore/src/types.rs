#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
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
                Unexpected::Unsigned(u as u64),
                &format!("expected a value between {} and {}", 0, 2).as_str(),
            )
        })
    }
}

// the network status of a message
#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
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
                Unexpected::Unsigned(u as u64),
                &format!("expected a value between {} and {}", 0, 3).as_str(),
            )
        })
    }
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct ClientMessageAck {
    pub update_code: MessageReceiptStatus,
    pub message_id: MsgId,
}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub struct MessageReceipt {
    pub update_code: MessageReceiptStatus,
    pub message_id: MsgId,
}

/// This type gets serialized into raw bytes and sent to the server
/// Then it is deserialized again on the client side to implement
/// control flow for the frontend.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToPeer {
    // TODO: replace this with an &str
    Message(String),
    Ack(ClientMessageAck),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Message(String),
    Ack(ClientMessageAck),
    Receipt(MessageReceiptStatus),
}
