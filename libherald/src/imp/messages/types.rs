use super::*;
use herald_common::{Time, UserId};
use std::{cmp::Ordering, collections::HashMap};

const FLURRY_FUZZ: i64 = 5 * 60_000;

#[derive(Clone, Debug)]
pub(super) struct MsgData {
    pub(super) author: UserId,
    pub(super) body: Option<MessageBody>,
    pub(super) time: MessageTime,
    pub(super) op: ReplyId,
    pub(super) receipts: HashMap<UserId, MessageReceiptStatus>,
    pub(super) has_attachments: bool,
    pub(super) save_status: SaveStatus,
    pub(super) send_status: MessageSendStatus,
    pub(super) match_status: MatchStatus,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum MatchStatus {
    NotMatched = 0,
    Matched = 1,
    Focused = 2,
}

impl MatchStatus {
    pub(super) fn is_match(self) -> bool {
        self == MatchStatus::Matched || self == MatchStatus::Focused
    }
}

impl MsgData {
    pub(super) fn same_flurry(&self, rhs: &Self) -> bool {
        (self.author == rhs.author)
            && (self.time.insertion.0 - rhs.time.insertion.0).abs() < FLURRY_FUZZ
    }

    pub(super) fn matches(&self, pattern: &heraldcore::utils::SearchPattern) -> bool {
        match self.body.as_ref() {
            Some(body) => pattern.is_match(body.as_str()),
            None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SaveStatus {
    Saved,
    Unsaved,
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// A thin wrapper around a `MsgId`
pub(super) struct Message {
    pub(super) msg_id: MsgId,
    pub(super) insertion_time: Time,
}

impl Message {
    pub(super) fn split_msg(msg: Msg, save_status: SaveStatus) -> (Self, MsgData) {
        let Msg {
            message_id,
            author,
            body,
            time,
            op,
            receipts,
            has_attachments,
            send_status,
            ..
        } = msg;

        let data = MsgData {
            author,
            receipts,
            body,
            op,
            has_attachments,
            time,
            send_status,
            save_status,
            match_status: MatchStatus::NotMatched,
        };

        let message = Message {
            msg_id: message_id,
            insertion_time: time.insertion,
        };

        (message, data)
    }

    pub(super) fn from_msg_id(msg_id: MsgId, container: &Container) -> Option<Self> {
        let insertion_time = container.get_data(&msg_id)?.time.insertion;

        Some(Self {
            msg_id,
            insertion_time,
        })
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.insertion_time.0.partial_cmp(&rhs.insertion_time.0)
    }
}

impl Ord for Message {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.partial_cmp(rhs) {
            Some(ord) => ord,
            None => self.msg_id.cmp(&rhs.msg_id),
        }
    }
}
