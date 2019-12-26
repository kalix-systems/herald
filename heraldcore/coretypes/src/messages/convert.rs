use super::*;
use std::convert::TryFrom;

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

#[derive(Debug, Clone, Copy)]
pub enum Error {
    SendStatus(i64),
    ReceiptStatus(i64),
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        out: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        use Error::*;
        match self {
            SendStatus(n) => write!(
                out,
                "Unknown message send status: found {}, expected 0, 1, or 2",
                n
            ),
            ReceiptStatus(n) => write!(
                out,
                "Unknown message receipt status: found {}, expected 0, 1, 2, or 3",
                n
            ),
        }
    }
}

impl std::error::Error for Error {}

impl TryFrom<u8> for MessageSendStatus {
    type Error = u8;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::NoAck),
            1 => Ok(Self::Ack),
            2 => Ok(Self::Timeout),
            i => Err(i),
        }
    }
}

impl std::convert::TryFrom<i64> for MessageSendStatus {
    type Error = Error;

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::NoAck),
            1 => Ok(Self::Ack),
            2 => Ok(Self::Timeout),
            i => Err(Error::SendStatus(i as i64)),
        }
    }
}

impl TryFrom<u8> for MessageReceiptStatus {
    type Error = u8;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::Nil),
            1 => Ok(Self::Received),
            2 => Ok(Self::Read),
            i => Err(i),
        }
    }
}

impl std::convert::TryFrom<i64> for MessageReceiptStatus {
    type Error = Error;

    fn try_from(n: i64) -> Result<Self, Error> {
        match n {
            0 => Ok(Self::Nil),
            1 => Ok(Self::Received),
            2 => Ok(Self::Read),
            i => Err(Error::ReceiptStatus(i)),
        }
    }
}

impl TryFrom<Vec<Reaction>> for Reactions {
    type Error = ();

    fn try_from(mut reactions: Vec<Reaction>) -> Result<Self, Self::Error> {
        let mid = match reactions.first() {
            Some(first) => first.mid,
            None => return Err(()),
        };

        reactions.sort_unstable_by(|a, b| a.time.cmp(&b.time));

        let mut content = Vec::new();

        Ok(Self { mid, content })
    }
}
