use crate::container::*;
use herald_common::{Time, UserId};
pub use heraldcore::{
    message::{
        Message as Msg, MessageBody, MessageReceiptStatus, MessageSendStatus, MessageTime, ReplyId,
    },
    types::MsgId,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
use unicode_segmentation::UnicodeSegmentation;

const FLURRY_FUZZ: i64 = 5 * 60_000;

#[derive(Clone, Debug)]
pub struct MsgData {
    pub author: UserId,
    pub body: Option<MessageBody>,
    pub time: MessageTime,
    pub op: ReplyId,
    pub receipts: HashMap<UserId, MessageReceiptStatus>,
    pub attachments: heraldcore::message::attachments::AttachmentMeta,
    pub send_status: MessageSendStatus,
    pub match_status: MatchStatus,
    pub replies: HashSet<MsgId>,
    pub search_buf: Option<String>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchStatus {
    NotMatched = 0,
    Matched = 1,
    Focused = 2,
}

impl MatchStatus {
    pub fn is_match(self) -> bool {
        self == MatchStatus::Matched || self == MatchStatus::Focused
    }
}

impl MsgData {
    pub fn same_flurry(
        &self,
        rhs: &Self,
    ) -> bool {
        (self.author == rhs.author) && self.time.insertion.within(FLURRY_FUZZ, rhs.time.insertion)
    }

    pub fn matches(
        &self,
        pattern: &search_pattern::SearchPattern,
    ) -> bool {
        match self.body.as_ref() {
            Some(body) => pattern.is_match(body.as_str()),
            None => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// A thin wrapper around a `MsgId`
pub struct Message {
    pub msg_id: MsgId,
    pub insertion_time: Time,
}

pub fn split_msg(msg: Msg) -> (Message, MsgData) {
    let Msg {
        message_id,
        author,
        body,
        time,
        op,
        receipts,
        attachments,
        send_status,
        replies,
        ..
    } = msg;

    let data = MsgData {
        author,
        receipts,
        body,
        op,
        attachments,
        time,
        send_status,
        match_status: MatchStatus::NotMatched,
        replies,
        search_buf: None,
    };

    let message = Message {
        msg_id: message_id,
        insertion_time: time.insertion,
    };

    (message, data)
}

pub fn from_msg_id(
    msg_id: MsgId,
    container: &Container,
) -> Option<Message> {
    let insertion_time = container.get_data(&msg_id)?.time.insertion;

    Some(Message {
        msg_id,
        insertion_time,
    })
}

#[repr(u8)]
pub enum ReplyType {
    None = 0,
    Dangling = 1,
    Known = 2,
}

pub fn reply_type(reply_id: &ReplyId) -> ReplyType {
    match reply_id {
        ReplyId::None => ReplyType::None,
        ReplyId::Dangling => ReplyType::Dangling,
        ReplyId::Known(_) => ReplyType::Known,
    }
}

impl PartialOrd for Message {
    fn partial_cmp(
        &self,
        rhs: &Self,
    ) -> Option<Ordering> {
        self.insertion_time.partial_cmp(&rhs.insertion_time)
    }
}

impl Ord for Message {
    fn cmp(
        &self,
        rhs: &Self,
    ) -> Ordering {
        match self.partial_cmp(rhs) {
            Some(ord) => ord,
            None => self.msg_id.cmp(&rhs.msg_id),
        }
    }
}

pub struct Elider {
    line_count: usize,
    char_count: usize,
    char_per_line: usize,
}

impl Default for Elider {
    fn default() -> Self {
        let line_count = 30;
        let char_per_line = 25;
        let char_count = line_count * char_per_line;

        Self {
            line_count,
            char_per_line,
            char_count,
        }
    }
}

impl Elider {
    pub fn set_line_count(
        &mut self,
        line_count: usize,
    ) {
        self.line_count = line_count
    }

    pub fn set_char_count(
        &mut self,
        char_count: usize,
    ) {
        self.char_count = char_count
    }

    pub fn set_char_per_line(
        &mut self,
        char_per_line: usize,
    ) {
        self.char_per_line = char_per_line;
    }

    pub fn elided_body(
        &self,
        body: &MessageBody,
    ) -> String {
        let char_count = UnicodeSegmentation::graphemes(body.as_str(), true).count();

        let line_count = body.as_str().lines().count();

        if char_count < self.char_count && line_count < self.line_count {
            return body.to_string();
        }

        let chars_to_take = self.char_count.min(self.line_count * self.char_per_line);

        UnicodeSegmentation::graphemes(body.as_str(), true)
            .take(chars_to_take)
            .collect()
    }
}
