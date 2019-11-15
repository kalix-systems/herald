use crate::{ffi, interface::*, ret_err, ret_none, shared::SingletonBus, toasts::new_msg_toast};
use herald_common::UserId;
use heraldcore::{
    abort_err,
    config::Config,
    conversation,
    errors::HErr,
    message::{
        self, Message as Msg, MessageBody, MessageReceiptStatus, MessageSendStatus, MessageTime,
        ReplyId,
    },
    types::*,
    utils::SearchPattern,
    NE,
};
use im::vector::Vector;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

mod search;
use search::*;
mod container;
use container::*;
mod imp;
pub(crate) mod shared;
pub(crate) mod types;
use shared::*;
use types::*;

type Emitter = MessagesEmitter;
type List = MessagesList;

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    local_id: UserId,
    conversation_id: Option<ConversationId>,
    container: Container,
    search: SearchMachine,
}

impl MessagesTrait for Messages {
    fn new(emit: Emitter, model: List) -> Self {
        Messages {
            model,
            emit,
            container: Container::default(),
            conversation_id: None,
            local_id: abort_err!(Config::static_id()),
            search: SearchMachine::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        let last = self.container.last_msg()?;

        if last.author == self.local_id {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    fn last_status(&self) -> Option<u32> {
        self.container
            .last_msg()?
            .receipts
            .values()
            .map(|status| *status as u32)
            .max()
    }

    fn last_body(&self) -> Option<&str> {
        Some(self.container.last_msg()?.body.as_ref()?.as_str())
    }

    fn last_epoch_timestamp_ms(&self) -> Option<i64> {
        Some(self.container.last_msg()?.time.insertion.0)
    }

    /// Returns index of a message given its id.
    fn index_by_id(&self, msg_id: ffi::MsgIdRef) -> u64 {
        let ret_val = std::u32::MAX as u64;

        let msg_id = ret_err!(msg_id.try_into(), ret_val);

        ret_none!(self.container.index_of(msg_id), ret_val) as u64
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

            let container = ret_err!(Container::new(conversation_id));

            if container.is_empty() {
                return;
            }

            self.model
                .begin_insert_rows(0, container.len().saturating_sub(1));
            self.container = container;
            self.model.end_insert_rows();
            self.emit_last_changed();
        }
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|c| c.as_slice())
    }

    fn data_saved(&self, row_index: usize) -> Option<bool> {
        Some(self.container.msg_data(row_index)?.save_status == SaveStatus::Saved)
    }

    fn author(&self, row_index: usize) -> Option<ffi::UserIdRef> {
        Some(self.container.msg_data(row_index)?.author.as_str())
    }

    fn body(&self, row_index: usize) -> Option<&str> {
        Some(self.container.msg_data(row_index)?.body.as_ref()?.as_str())
    }

    fn message_id(&self, row_index: usize) -> Option<ffi::MsgIdRef> {
        Some(self.container.get(row_index)?.msg_id.as_slice())
    }

    fn has_attachments(&self, row_index: usize) -> Option<bool> {
        Some(self.container.msg_data(row_index)?.has_attachments)
    }

    fn receipt_status(&self, row_index: usize) -> Option<u32> {
        Some(
            self.container
                .msg_data(row_index)?
                .receipts
                .values()
                .map(|r| *r as u32)
                .max()
                .unwrap_or(MessageReceiptStatus::NoAck as u32),
        )
    }

    fn op(&self, row_index: usize) -> Option<ffi::MsgIdRef> {
        match self.container.msg_data(row_index)?.op {
            ReplyId::Known(ref mid) => Some(mid.as_slice()),
            _ => None,
        }
    }

    fn is_reply(&self, row_index: usize) -> Option<bool> {
        Some(!self.container.msg_data(row_index)?.op.is_none())
    }

    fn is_head(&self, row_index: usize) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if row_index == 0 {
            return Some(true);
        }

        // other cases
        let (msg, prev) = (
            self.container.msg_data(row_index)?,
            self.container.msg_data(row_index - 1)?,
        );

        Some(!msg.same_flurry(prev))
    }

    fn is_tail(&self, row_index: usize) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if row_index == self.container.len().saturating_sub(1) {
            return Some(true);
        }

        // other cases
        let (msg, succ) = (
            self.container.msg_data(row_index)?,
            self.container.msg_data(row_index + 1)?,
        );

        Some(!msg.same_flurry(succ))
    }

    fn epoch_timestamp_ms(&self, row_index: usize) -> Option<i64> {
        Some(self.container.get(row_index)?.insertion_time.0)
    }

    fn expiration_timestamp_ms(&self, row_index: usize) -> Option<i64> {
        Some(self.container.msg_data(row_index)?.time.expiration?.0)
    }

    fn server_timestamp_ms(&self, row_index: usize) -> Option<i64> {
        Some(self.container.msg_data(row_index)?.time.server?.0)
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let ix = row_index as usize;

        let id = ret_none!(self.container.get(ix), false).msg_id;

        ret_err!(message::delete_message(&id), false);

        self.raw_list_remove(ix);

        true
    }

    /// Deletes all messages in the current conversation.
    fn clear_conversation_history(&mut self) -> bool {
        let id = ret_none!(self.conversation_id, false);

        ret_err!(conversation::delete_conversation(&id), false);

        self.model
            .begin_remove_rows(0, self.container.len().saturating_sub(1));
        self.container = Default::default();
        self.model.end_remove_rows();

        self.emit_last_changed();
        self.emit.is_empty_changed();
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
                    ret_err!(self.raw_insert(*msg, SaveStatus::Unsaved));
                }
                MsgUpdate::Receipt(mid) => {
                    ret_err!(self.container.handle_receipt(mid, &mut self.model));
                }
                MsgUpdate::StoreDone(mid) => {
                    ret_none!(self.container.handle_store_done(mid, &mut self.model));
                }
                MsgUpdate::ExpiredMessages(mids) => {
                    for mid in mids {
                        if let Some(ix) = self.container.index_of(mid) {
                            self.raw_list_remove(ix);
                        }
                    }
                }
            }
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.container.len()
    }

    fn search_pattern(&self) -> &str {
        self.search.pattern.raw()
    }

    fn set_search_pattern(&mut self, pattern: String) {
        if pattern.is_empty() {
            self.clear_search();
            return;
        }

        if ret_err!(self.search.set_pattern(pattern, &mut self.emit)).changed() {
            self.search.matches = self
                .container
                .apply_search(&self.search, &mut self.model, &mut self.emit)
                .unwrap_or_default();
        }
    }

    /// Indicates whether regex search is activated
    fn search_regex(&self) -> bool {
        self.search.is_regex()
    }

    /// Sets search mode
    fn set_search_regex(&mut self, use_regex: bool) {
        if ret_err!(self.search.set_regex(use_regex)).changed() {
            self.emit.search_regex_changed();
            self.search.matches = self
                .container
                .apply_search(&self.search, &mut self.model, &mut self.emit)
                .unwrap_or_default();
        }
    }

    /// Indicates whether search is active
    fn search_active(&self) -> bool {
        self.search.active
    }

    /// Turns search on or off
    fn set_search_active(&mut self, active: bool) {
        self.search.active = active;
        self.emit.search_active_changed();
    }

    /// Clears search
    fn clear_search(&mut self) {
        self.container.clear_search(&mut self.model);
        self.search.clear_search(&mut self.emit);
    }

    fn search_num_matches(&self) -> u64 {
        self.search.num_matches() as u64
    }

    fn next_search_match(&mut self) -> i64 {
        match self.search.next(&self.container) {
            Some(Match { mid }) => self
                .container
                .index_of(mid)
                .map(|ix| ix as i64)
                .unwrap_or(-1),
            None => -1,
        }
    }

    fn prev_search_match(&mut self) -> i64 {
        match self.search.prev(&self.container) {
            Some(Match { mid }) => self
                .container
                .index_of(mid)
                .map(|ix| ix as i64)
                .unwrap_or(-1),
            None => -1,
        }
    }
}
