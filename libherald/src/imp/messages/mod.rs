use crate::{
    ffi,
    interface::{MessagesEmitter as Emitter, MessagesList as List, MessagesTrait as Interface},
    ret_err, ret_none,
    shared::SingletonBus,
    spawn,
    toasts::new_msg_toast,
};
use herald_common::UserId;
use heraldcore::{
    config, conversation,
    errors::HErr,
    message::{self, Message as Msg, MessageBody, MessageReceiptStatus},
    types::*,
    NE,
};
use im::vector::Vector;
use search_pattern::SearchPattern;
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
use shared::*;

/// Implementation of `crate::interface::MessageBuilderTrait`.
pub mod builder;
use builder::MessageBuilder;

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    local_id: Option<UserId>,
    conversation_id: Option<ConversationId>,
    container: Container,
    search: SearchState,
    builder: MessageBuilder,
}

impl Interface for Messages {
    fn new(
        emit: Emitter,
        model: List,
        builder: MessageBuilder,
    ) -> Self {
        Messages {
            model,
            emit,
            container: Container::default(),
            conversation_id: None,
            local_id: config::id().ok(),
            search: SearchState::new(),
            builder,
        }
    }

    fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        let last = self.container.last_msg()?;

        if last.author == self.local_id? {
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
            .max()
            .map(|status| *status as u32)
    }

    fn last_body(&self) -> Option<&str> {
        Some(self.container.last_msg()?.body.as_ref()?.as_str())
    }

    fn last_time(&self) -> Option<i64> {
        Some(self.container.last_msg()?.time.insertion.0)
    }

    /// Returns index of a message given its id.
    fn index_by_id(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> u64 {
        let ret_val = std::u32::MAX as u64;

        let msg_id = ret_err!(msg_id.try_into(), ret_val);

        ret_none!(self.container.index_by_id(msg_id), ret_val) as u64
    }

    fn set_conversation_id(
        &mut self,
        conversation_id: Option<ffi::ConversationIdRef>,
    ) {
        if let (Some(id), None) = (conversation_id, self.conversation_id) {
            let conversation_id = ret_err!(ConversationId::try_from(id));

            EMITTERS.insert(conversation_id, self.emit().clone());
            // remove left over channel from previous session
            RXS.remove(&conversation_id);
            TXS.remove(&conversation_id);

            self.conversation_id = Some(conversation_id);
            self.builder.set_conversation_id(conversation_id);
            self.emit.conversation_id_changed();

            container::fill(conversation_id);
        }
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|c| c.as_slice())
    }

    fn data_saved(
        &self,
        row_index: usize,
    ) -> Option<bool> {
        Some(self.container.msg_data(row_index)?.save_status == SaveStatus::Saved)
    }

    fn author(
        &self,
        row_index: usize,
    ) -> Option<ffi::UserIdRef> {
        Some(self.container.msg_data(row_index)?.author.as_str())
    }

    fn body(
        &self,
        row_index: usize,
    ) -> Option<&str> {
        Some(self.container.msg_data(row_index)?.body.as_ref()?.as_str())
    }

    fn msg_id(
        &self,
        row_index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.container.get(row_index)?.msg_id.as_slice())
    }

    fn has_attachments(
        &self,
        row_index: usize,
    ) -> Option<bool> {
        Some(self.container.msg_data(row_index)?.has_attachments)
    }

    fn receipt_status(
        &self,
        row_index: usize,
    ) -> Option<u32> {
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

    fn match_status(
        &self,
        row_index: usize,
    ) -> Option<u8> {
        Some(self.container.msg_data(row_index)?.match_status as u8)
    }

    fn is_head(
        &self,
        row_index: usize,
    ) -> Option<bool> {
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

    fn is_tail(
        &self,
        row_index: usize,
    ) -> Option<bool> {
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

    fn insertion_time(
        &self,
        row_index: usize,
    ) -> Option<i64> {
        Some(self.container.get(row_index)?.insertion_time.0)
    }

    fn expiration_time(
        &self,
        row_index: usize,
    ) -> Option<i64> {
        Some(self.container.msg_data(row_index)?.time.expiration?.0)
    }

    fn server_time(
        &self,
        row_index: usize,
    ) -> Option<i64> {
        Some(self.container.msg_data(row_index)?.time.server?.0)
    }

    fn delete_message(
        &mut self,
        row_index: u64,
    ) -> bool {
        let ix = row_index as usize;

        let id = ret_none!(self.container.get(ix), false).msg_id;

        self.remove_helper(id, ix);
        spawn!(message::delete_message(&id), false);

        true
    }

    /// Deletes all messages in the current conversation.
    fn clear_conversation_history(&mut self) -> bool {
        let id = ret_none!(self.conversation_id, false);

        spawn!(conversation::delete_conversation(&id), false);

        self.clear_search();
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
                MsgUpdate::NewMsg(new) => {
                    new_msg_toast(&new);

                    ret_err!(self.insert_helper(*new, SaveStatus::Saved));
                }
                MsgUpdate::BuilderMsg(msg) => {
                    ret_err!(self.insert_helper(*msg, SaveStatus::Unsaved));
                }
                MsgUpdate::Receipt {
                    msg_id,
                    recipient,
                    status,
                } => {
                    ret_err!(container::handle_receipt(
                        &mut self.container,
                        msg_id,
                        status,
                        recipient,
                        &mut self.model
                    ));
                }
                MsgUpdate::StoreDone(mid) => {
                    ret_none!(container::handle_store_done(
                        &mut self.container,
                        mid,
                        &mut self.model
                    ));
                }

                MsgUpdate::ExpiredMessages(mids) => self.handle_expiration(mids),

                MsgUpdate::Container(container) => {
                    if container.is_empty() {
                        continue;
                    }

                    self.model
                        .begin_insert_rows(0, container.len().saturating_sub(1));
                    self.container = container;
                    self.model.end_insert_rows();
                    self.emit.is_empty_changed();
                    self.emit_last_changed();
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
        self.search
            .pattern
            .as_ref()
            .map(SearchPattern::raw)
            .unwrap_or("")
    }

    fn set_search_pattern(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_search();
            return;
        }

        if ret_err!(self.search.set_pattern(pattern, &mut self.emit)).changed()
            && self.search.active
        {
            self.search.set_matches(
                container::apply_search(
                    &mut self.container,
                    &self.search,
                    &mut self.model,
                    &mut self.emit,
                )
                .unwrap_or_default(),
                &mut self.emit,
            );
        }
    }

    /// Indicates whether regex search is activated
    fn search_regex(&self) -> bool {
        self.search.is_regex()
    }

    /// Sets search mode
    fn set_search_regex(
        &mut self,
        use_regex: bool,
    ) {
        if ret_err!(self.search.set_regex(use_regex, &mut self.emit)).changed() {
            self.search.set_matches(
                container::apply_search(
                    &mut self.container,
                    &self.search,
                    &mut self.model,
                    &mut self.emit,
                )
                .unwrap_or_default(),
                &mut self.emit,
            );
        }
    }

    /// Indicates whether search is active
    fn search_active(&self) -> bool {
        self.search.active
    }

    /// Turns search on or off
    fn set_search_active(
        &mut self,
        active: bool,
    ) {
        if !active {
            self.search.active = false;
            self.clear_search();
        } else if !self.search.active {
            self.search.active = true;
            self.emit.search_active_changed();
            self.search.set_matches(
                apply_search(
                    &mut self.container,
                    &self.search,
                    &mut self.model,
                    &mut self.emit,
                )
                .unwrap_or_default(),
                &mut self.emit,
            );
        }
    }

    /// Clears search
    fn clear_search(&mut self) {
        clear_search(&mut self.container, &mut self.model);
        ret_err!(self.search.clear_search(&mut self.emit));
    }

    fn search_num_matches(&self) -> u64 {
        self.search.num_matches() as u64
    }

    fn next_search_match(&mut self) -> i64 {
        self.next_match_helper().map(|ix| ix as i64).unwrap_or(-1)
    }

    fn prev_search_match(&mut self) -> i64 {
        self.prev_match_helper().map(|ix| ix as i64).unwrap_or(-1)
    }

    fn search_index(&self) -> u64 {
        self.search.index.map(|ix| ix + 1).unwrap_or(0) as u64
    }

    fn set_search_hint(
        &mut self,
        scroll_position: f32,
        scroll_height: f32,
    ) {
        if scroll_position.is_nan() || scroll_height.is_nan() {
            return;
        }

        let percentage = scroll_position + scroll_height / 2.0;
        self.search.start_hint(percentage, &self.container);
    }

    fn builder(&self) -> &MessageBuilder {
        &self.builder
    }

    fn builder_mut(&mut self) -> &mut MessageBuilder {
        &mut self.builder
    }

    fn builder_op_msg_id(&self) -> Option<ffi::MsgIdRef> {
        self.builder.op_id_slice()
    }

    fn set_builder_op_msg_id(
        &mut self,
        id: Option<ffi::MsgIdRef>,
    ) {
        use builder::OpChanged;

        match ret_err!(self.builder.set_op_id(id, &self.container)) {
            OpChanged::Changed => {
                self.emit.builder_op_msg_id_changed();
            }
            OpChanged::NotChanged => {}
        }
    }

    fn reply_type(
        &self,
        index: usize,
    ) -> Option<u8> {
        Some(self.container.op_reply_type(index)? as u8)
    }

    fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        self.container.op_msg_id(index).map(MsgId::as_slice)
    }

    fn op_author(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        self.container.op_author(index).map(UserId::as_str)
    }

    fn op_body(
        &self,
        index: usize,
    ) -> Option<&str> {
        self.container
            .op_body(index)?
            .as_ref()
            .map(MessageBody::as_str)
    }

    fn op_has_attachments(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.container.op_has_attachments(index)
    }

    fn op_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_time(index)?.0)
    }
}
