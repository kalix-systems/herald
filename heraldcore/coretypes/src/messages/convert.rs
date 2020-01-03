use super::*;
use crate::conversation::settings::SettingsUpdate;
use coremacros::from_fn;
use herald_common::Time;
use json::JsonValue;
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
    pub fn from_parts<T>(
        body: Option<MessageBody>,
        attachments: Option<AttachmentMeta>,
        op: ReplyId,
        aux: Option<T>,
    ) -> Item
    where
        T: Into<AuxItem>,
    {
        match (body, attachments, aux) {
            (_, _, Some(aux)) => Item::Aux(aux.into()),
            (body, attachments, None) => Item::Plain(PlainItem {
                body,
                attachments: attachments.unwrap_or_default(),
                op,
            }),
        }
    }
}

impl Reactions {
    pub fn from_vec(reactions: Vec<Reaction>) -> Option<Self> {
        // early return
        if reactions.is_empty() {
            return None;
        }

        // temporary collection
        let mut buckets = HashMap::<ReactContent, Vec<(Time, UserId)>>::new();

        // insertion reactionary information for each reaction, indexed by reaction content
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

        // collect into a vector
        let mut content = buckets.into_iter().collect::<Vec<_>>();

        // sort
        content.sort_unstable_by(|(_, a), (_, b)| {
            let a_min = a
                .iter()
                .map(|(t, _)| t)
                .min()
                .copied()
                // this should be covered by the early return and the compiler should be able
                // to optimize this out, but let's be safe
                .unwrap_or_else(|| Time::from(std::i64::MIN));
            let b_min = &b
                .iter()
                .map(|(t, _)| t)
                .min()
                .copied()
                .unwrap_or_else(|| Time::from(std::i64::MIN));

            a_min.cmp(b_min)
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

from_fn!(AuxItem, NewMembers, AuxItem::NewMembers);
from_fn!(
    AuxItem,
    crate::conversation::settings::SettingsUpdate,
    AuxItem::GroupSettings
);

from_fn!(Item, AuxItem, Item::Aux);
from_fn!(Item, PlainItem, Item::Plain);

impl From<AuxItem> for JsonValue {
    fn from(item: AuxItem) -> Self {
        use SettingsUpdate::*;
        let code = item.code();

        match item {
            AuxItem::GroupSettings(settings) => match settings {
                Expiration(period) => {
                    json::object! {
                        "code" => code,
                        "content" => period as u8,
                    }
                }
                Title(title) => {
                    json::object! {
                        "code" => code,
                        "content" => title,
                    }
                }
                Picture(path) => {
                    json::object! {
                        "code" => code,
                        "content" => path,
                    }
                }
            },
            AuxItem::NewMembers(members) => {
                json::object! {
                    "code" => code,
                    "content" => members.0.into_iter().map(|u| u.to_string()).collect::<Vec<_>>(),
                }
            }
        }
    }
}
