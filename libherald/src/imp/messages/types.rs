use super::*;
use herald_common::{Time, UserId};
use heraldcore::message::MessageTime;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug)]
pub(super) struct MsgData {
    pub(super) author: UserId,
    pub(super) body: Option<MessageBody>,
    pub(super) time: MessageTime,
    pub(super) op: Option<MsgId>,
    pub(super) receipts: HashMap<UserId, MessageReceiptStatus>,
    pub(super) has_attachments: bool,
    pub(super) save_status: SaveStatus,
    pub(super) send_status: MessageSendStatus,
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
            Some(ord) => {
                return ord;
            }
            None => self.msg_id.cmp(&rhs.msg_id),
        }
    }
}
