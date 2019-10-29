use crate::{ffi, interface::*, ret_err, ret_none, shared::SingletonBus, toasts::new_msg_toast};
use herald_common::UserId;
use heraldcore::{
    abort_err,
    config::Config,
    conversation,
    errors::HErr,
    message::{self, Message as Msg},
    types::*,
    NE,
};
use im_rc::vector::Vector;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

mod imp;
pub(crate) mod shared;
pub(crate) mod types;
use shared::*;
use types::*;

type Emitter = MessagesEmitter;
type List = MessagesList;

const UNKNOWN_BODY: &str = "Unknown message";
const UNKNOWN_AUTHOR: &str = "Unknown";

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    list: Vector<Message>,
    map: HashMap<MsgId, MsgData>,
    local_id: UserId,
    conversation_id: Option<ConversationId>,
}

impl MessagesTrait for Messages {
    fn new(emit: Emitter, model: List) -> Self {
        Messages {
            list: Vector::new(),
            map: HashMap::new(),
            model,
            emit,
            conversation_id: None,
            local_id: abort_err!(Config::static_id()),
        }
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        let last = self.last_msg()?;

        if last.author == self.local_id {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    fn last_status(&self) -> Option<u32> {
        self.last_msg()?
            .receipts
            .iter()
            .map(|(_, status)| *status as u32)
            .max()
    }

    fn last_body(&self) -> Option<&str> {
        Some(self.last_msg()?.body.as_ref()?.as_str())
    }

    fn last_epoch_timestamp_ms(&self) -> Option<i64> {
        Some(self.last_msg()?.time.insertion.0)
    }

    /// Returns index of a message given its id.
    fn index_by_id(&self, msg_id: ffi::MsgIdRef) -> u64 {
        let ret_val = std::u32::MAX as u64;

        let msg_id = ret_err!(msg_id.try_into(), ret_val);

        ret_none!(self.index_of(msg_id), ret_val) as u64
    }

    fn set_conversation_id(&mut self, conversation_id: Option<ffi::ConversationIdRef>) {
        if let (Some(id), None) = (conversation_id, self.conversation_id) {
            let conversation_id = ret_err!(ConversationId::try_from(id));

            EMITTERS.insert(conversation_id, self.emit().clone());
            // remove left over channel from previous session
            RXS.remove(&conversation_id);
            TXS.remove(&conversation_id);

            self.conversation_id = Some(conversation_id);
            self.emit.conversation_id_changed();

            let messages: Vector<Message> =
                ret_err!(conversation::conversation_messages(&conversation_id))
                    .into_iter()
                    .map(|m| {
                        let (message, data) = Message::split_msg(m, SaveStatus::Saved);
                        self.map.insert(message.msg_id, data);
                        message
                    })
                    .collect();

            if messages.is_empty() {
                return;
            }

            self.model
                .begin_insert_rows(0, messages.len().saturating_sub(1));
            self.list = messages;
            self.model.end_insert_rows();
            self.emit_last_changed();
        }
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|c| c.as_slice())
    }

    fn data_saved(&self, row_index: usize) -> Option<bool> {
        Some(self.msg_data(row_index)?.save_status == SaveStatus::Saved)
    }

    fn author(&self, row_index: usize) -> Option<ffi::UserIdRef> {
        Some(self.msg_data(row_index)?.author.as_str())
    }

    fn body(&self, row_index: usize) -> Option<&str> {
        Some(self.msg_data(row_index)?.body.as_ref()?.as_str())
    }

    fn message_id(&self, row_index: usize) -> Option<ffi::MsgIdRef> {
        Some(self.list.get(row_index)?.msg_id.as_slice())
    }

    fn has_attachments(&self, row_index: usize) -> Option<bool> {
        Some(self.msg_data(row_index)?.has_attachments)
    }

    fn receipt_status(&self, row_index: usize) -> Option<u32> {
        Some(
            self.msg_data(row_index)?
                .receipts
                .values()
                .map(|r| *r as u32)
                .max()
                .unwrap_or(MessageReceiptStatus::NoAck as u32),
        )
    }

    fn message_body_by_id(&self, msg_id: ffi::MsgIdRef) -> String {
        if msg_id == &ffi::NULL_MSG_ID {
            return UNKNOWN_BODY.to_owned();
        }

        let msg_id = ret_err!(MsgId::try_from(msg_id), UNKNOWN_BODY.to_owned());

        match self.map.get(&msg_id) {
            Some(msg) => match msg.body.as_ref() {
                Some(body) => body.to_string(),
                None => UNKNOWN_BODY.to_owned(),
            },
            None => UNKNOWN_BODY.to_owned(),
        }
    }

    fn message_author_by_id(&self, msg_id: ffi::MsgIdRef) -> ffi::UserId {
        if msg_id == &ffi::NULL_MSG_ID {
            return UNKNOWN_AUTHOR.to_owned();
        }

        let msg_id = ret_err!(MsgId::try_from(msg_id), ffi::NULL_USER_ID.to_string());

        match self.map.get(&msg_id) {
            Some(msg) => msg.author.to_string(),
            None => UNKNOWN_AUTHOR.to_owned(),
        }
    }

    fn op(&self, row_index: usize) -> Option<ffi::MsgIdRef> {
        match self.msg_data(row_index)?.op {
            ReplyId::Known(ref mid) => Some(mid.as_slice()),
            _ => None,
        }
    }

    fn is_reply(&self, row_index: usize) -> Option<bool> {
        Some(!self.msg_data(row_index)?.op.is_none())
    }

    fn is_head(&self, row_index: usize) -> Option<bool> {
        if self.list.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if row_index == 0 {
            return Some(true);
        }

        // other cases
        let (msg, prev) = (self.msg_data(row_index)?, self.msg_data(row_index - 1)?);

        Some(!msg.same_flurry(prev))
    }

    fn is_tail(&self, row_index: usize) -> Option<bool> {
        if self.list.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if row_index == self.list.len().saturating_sub(1) {
            return Some(true);
        }

        // other cases
        let (msg, succ) = (self.msg_data(row_index)?, self.msg_data(row_index + 1)?);

        Some(!msg.same_flurry(succ))
    }

    fn epoch_timestamp_ms(&self, row_index: usize) -> Option<i64> {
        Some(self.list.get(row_index)?.insertion_time.0)
    }

    fn expiration_timestamp_ms(&self, row_index: usize) -> Option<i64> {
        Some(self.msg_data(row_index)?.time.expiration?.0)
    }

    fn server_timestamp_ms(&self, row_index: usize) -> Option<i64> {
        Some(self.msg_data(row_index)?.time.server?.0)
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let ix = row_index as usize;

        let id = ret_none!(self.list.get(ix), false).msg_id;

        ret_err!(message::delete_message(&id), false);

        self.raw_list_remove(ix, &id);

        true
    }

    /// Deletes all messages in the current conversation.
    fn clear_conversation_history(&mut self) -> bool {
        let id = ret_none!(self.conversation_id, false);

        ret_err!(conversation::delete_conversation(&id), false);

        self.model
            .begin_remove_rows(0, self.list.len().saturating_sub(1));
        self.list = Vector::new();
        self.map = HashMap::new();
        self.model.end_remove_rows();

        self.emit_last_changed();
        true
    }

    fn can_fetch_more(&self) -> bool {
        let conv_id = match &self.conversation_id {
            Some(cid) => cid,
            None => return false,
        };

        let rx = match RXS.get(conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return false,
        };

        !rx.is_empty()
    }

    /// Polls for updates
    fn fetch_more(&mut self) {
        let conv_id = ret_none!(self.conversation_id);

        let rx = match RXS.get(&conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return,
        };

        for update in rx.try_iter() {
            match update {
                MsgUpdate::Msg(mid) => {
                    let new = ret_err!(message::get_message(&mid));

                    new_msg_toast(&new);

                    ret_err!(self.raw_insert(new, SaveStatus::Saved));
                }
                MsgUpdate::FullMsg(msg) => {
                    ret_err!(self.raw_insert(msg, SaveStatus::Unsaved));
                }
                MsgUpdate::Receipt(mid) => {
                    let mut msg = match self.map.get_mut(&mid) {
                        None => {
                            // This can (possibly) happen if the message
                            // was deleted between the receipt
                            // being received over the network
                            // and this part of the code.
                            continue;
                        }
                        Some(msg) => msg,
                    };

                    // NOTE: If this fails, there is a bug somewhere
                    // in libherald.
                    //
                    // It is probably trivial, but may reflect something
                    // deeply wrong with our understanding of the program's
                    // concurrency.
                    let ix = ret_none!(self
                        .list
                        .iter()
                        // search backwards,
                        // it's probably fairly recent
                        .rposition(|m| m.msg_id == mid));

                    let receipts = ret_err!(message::get_message_receipts(&mid));
                    msg.receipts = receipts;

                    self.model.data_changed(ix, ix);
                }
                MsgUpdate::StoreDone(mid) => {
                    let data = ret_none!(self.map.get_mut(&mid));

                    data.save_status = SaveStatus::Saved;
                    let ix = ret_none!(self
                        .list
                        .iter()
                        // search backwards,
                        // it's probably fairly recent
                        .rposition(|m| m.msg_id == mid));
                    self.model.data_changed(ix, ix);
                }
                MsgUpdate::ExpiredMessages(mids) => {
                    for mid in mids {
                        if let Some(ix) = self.index_of(mid) {
                            self.raw_list_remove(ix, &mid);
                        }
                    }
                    // TODO update messages list upon expiration
                }
            }
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
