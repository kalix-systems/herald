use super::*;
use herald_common::{Time, UserId};
use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq)]
pub(super) enum SearchChanged {
    Changed,
    NotChanged,
}

pub(super) struct SearchMachine {
    pub(super) pattern: SearchPattern,
    pub(super) active: bool,
    pub(super) cursor: Option<usize>,
}

impl SearchMachine {
    pub(super) fn new() -> Self {
        Self {
            pattern: abort_err!(SearchPattern::new_normal("".into())),
            active: false,
            cursor: None,
        }
    }

    pub(super) fn is_regex(&self) -> bool {
        match self.pattern {
            SearchPattern::Normal { .. } => false,
            SearchPattern::Regex { .. } => true,
        }
    }

    pub(super) fn set_regex(&mut self, use_regex: bool) -> Result<SearchChanged, HErr> {
        match (use_regex, self.is_regex()) {
            (true, false) => {
                self.pattern.regex_mode()?;
            }
            (false, true) => {
                self.pattern.normal_mode()?;
            }
            _ => {
                return Ok(SearchChanged::NotChanged);
            }
        }
        Ok(SearchChanged::Changed)
    }
}

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
    pub(super) matched: bool,
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

#[derive(Clone, PartialEq, Eq)]
/// A thin wrapper around a `MsgId`
pub(super) struct Message {
    pub(super) msg_id: MsgId,
    pub(super) insertion_time: Time,
}

impl Message {
    pub(super) fn split_msg(msg: Msg, save_status: SaveStatus) -> (Message, MsgData) {
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
            matched: true,
        };

        let message = Message {
            msg_id: message_id,
            insertion_time: time.insertion,
        };

        (message, data)
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
