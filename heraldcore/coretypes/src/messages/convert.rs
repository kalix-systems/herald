use super::*;
use herald_common::Time;
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

impl Item {
    pub fn from_parts(
        body: Option<MessageBody>,
        update: Option<crate::conversation::settings::SettingsUpdate>,
    ) -> Option<Item> {
        match (body, update) {
            (Some(body), None) => Item::Plain(body).into(),
            (None, Some(update)) => Item::Update(update).into(),
            _ => None,
        }
    }
}

impl Reactions {
    pub fn from_vec(reactions: Vec<Reaction>) -> Option<Self> {
        if reactions.is_empty() {
            return None;
        }

        let mut buckets = HashMap::<ReactContent, Vec<(Time, UserId)>>::new();

        for Reaction {
            reactionary,
            react_content,
            time,
        } in reactions
        {
            buckets
                .entry(react_content)
                .or_default()
                .push((time, reactionary));
        }

        let mut content = buckets.into_iter().collect::<Vec<_>>();

        content.sort_unstable_by(|(_, a), (_, b)| {
            a.iter()
                .map(|(t, _)| t)
                .min()
                .copied()
                .unwrap_or_else(|| Time::from(std::i64::MIN))
                .cmp(
                    &b.iter()
                        .map(|(t, _)| t)
                        .min()
                        .copied()
                        .unwrap_or_else(|| Time::from(std::i64::MIN)),
                )
        });

        let content = content
            .into_iter()
            .map(|(content, v)| TaggedReact {
                content,
                reactionaries: v.into_iter().map(|(_, u)| u).collect(),
            })
            .collect();

        Some(Self { content })
    }
}

impl From<TaggedReact> for json::JsonValue {
    fn from(
        TaggedReact {
            content,
            reactionaries,
        }: TaggedReact
    ) -> json::JsonValue {
        json::object! {
           "content" => content,
           "reactionaries" => reactionaries.into_iter().map(|u| u.to_string()).collect::<Vec<String>>()
        }
    }
}

impl From<Reactions> for json::JsonValue {
    fn from(Reactions { content }: Reactions) -> json::JsonValue {
        content.into()
    }
}
