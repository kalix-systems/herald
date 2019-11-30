use super::*;
use crate::{
    ffi,
    interface::{MessagesEmitter as Emitter, MessagesList as List, MessagesTrait as Interface},
    ret_err, ret_none, spawn,
};
use heraldcore::{
    config, conversation,
    message::{self, MessageBody, MessageReceiptStatus},
};
use std::convert::TryInto;

impl Messages {
    pub(crate) fn new_(
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

    pub(crate) fn receipt_status_(
        &self,
        index: usize,
    ) -> Option<u32> {
        Some(
            self.container
                .msg_data(index)?
                .receipts
                .values()
                .map(|r| *r as u32)
                .max()
                .unwrap_or(MessageReceiptStatus::NoAck as u32),
        )
    }

    pub(crate) fn last_author_(&self) -> Option<ffi::UserIdRef> {
        let last = self.container.last_msg()?;

        if last.author == self.local_id? {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    pub(crate) fn last_status_(&self) -> Option<u32> {
        self.container
            .last_msg()?
            .receipts
            .values()
            .max()
            .map(|status| *status as u32)
    }

    pub(crate) fn index_by_id_(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> u64 {
        let ret_val = std::u32::MAX as u64;

        let msg_id = ret_err!(msg_id.try_into(), ret_val);

        ret_none!(self.container.index_by_id(msg_id), ret_val) as u64
    }

    pub(crate) fn is_tail_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if index == self.container.len().saturating_sub(1) {
            return Some(true);
        }

        // other cases
        let (msg, succ) = (
            self.container.msg_data(index)?,
            self.container.msg_data(index + 1)?,
        );

        Some(!msg.same_flurry(succ))
    }

    pub(crate) fn is_head_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if index == 0 {
            return Some(true);
        }

        // other cases
        let (msg, prev) = (
            self.container.msg_data(index)?,
            self.container.msg_data(index - 1)?,
        );

        Some(!msg.same_flurry(prev))
    }

    pub(crate) fn clear_conversation_history_(&mut self) -> bool {
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

    pub(crate) fn delete_message_(
        &mut self,
        index: u64,
    ) -> bool {
        let ix = index as usize;

        let id = ret_none!(self.container.get(ix), false).msg_id;

        self.remove_helper(id, ix);
        spawn!(message::delete_message(&id), false);

        true
    }

    pub(crate) fn search_pattern_(&self) -> &str {
        self.search
            .pattern
            .as_ref()
            .map(SearchPattern::raw)
            .unwrap_or("")
    }

    pub(crate) fn op_body_(
        &self,
        index: usize,
    ) -> Option<&str> {
        self.container
            .op_body(index)?
            .as_ref()
            .map(MessageBody::as_str)
    }

    pub(crate) fn set_builder_op_msg_id_(
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

    pub(crate) fn set_search_hint_(
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

    /// Turns search on or off
    pub(crate) fn set_search_active_(
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
    pub(crate) fn clear_search_(&mut self) {
        clear_search(&mut self.container, &mut self.model);
        ret_err!(self.search.clear_search(&mut self.emit));
    }

    pub(crate) fn set_search_pattern_(
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

    /// Sets search mode
    pub(crate) fn set_search_regex_(
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

    pub(crate) fn is_empty_(&self) -> bool {
        self.container.is_empty()
    }

    pub(crate) fn last_body_(&self) -> Option<&str> {
        Some(self.container.last_msg()?.body.as_ref()?.as_str())
    }

    pub(crate) fn last_time_(&self) -> Option<i64> {
        Some(self.container.last_msg()?.time.insertion.into())
    }

    pub(crate) fn data_saved_(
        &self,
        index: usize,
    ) -> Option<bool> {
        Some(self.container.msg_data(index)?.save_status == SaveStatus::Saved)
    }

    pub(crate) fn author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        Some(self.container.msg_data(index)?.author.as_str())
    }

    pub(crate) fn body_(
        &self,
        index: usize,
    ) -> Option<&str> {
        if self.container.msg_data(index)?.match_status.is_match() {
            Some(
                self.container
                    .msg_data(index)?
                    .search_buf
                    .as_ref()?
                    .as_str(),
            )
        } else {
            Some(self.container.msg_data(index)?.body.as_ref()?.as_str())
        }
    }

    pub(crate) fn insertion_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.get(index)?.insertion_time.into())
    }

    pub(crate) fn expiration_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.msg_data(index)?.time.expiration?.into())
    }

    pub(crate) fn server_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.msg_data(index)?.time.server?.into())
    }

    pub(crate) fn reply_type_(
        &self,
        index: usize,
    ) -> Option<u8> {
        Some(self.container.op_reply_type(index)? as u8)
    }

    pub(crate) fn builder_(&self) -> &MessageBuilder {
        &self.builder
    }

    pub(crate) fn builder_mut_(&mut self) -> &mut MessageBuilder {
        &mut self.builder
    }

    pub(crate) fn builder_op_msg_id_(&self) -> Option<ffi::MsgIdRef> {
        self.builder.op_id_slice()
    }

    pub(crate) fn op_msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        self.container.op_msg_id(index).map(MsgId::as_slice)
    }

    pub(crate) fn op_author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        self.container.op_author(index).map(UserId::as_str)
    }

    pub(crate) fn op_has_attachments_(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.container.op_has_attachments(index)
    }

    pub(crate) fn op_insertion_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_insertion_time(index)?.into())
    }

    pub(crate) fn op_expiration_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_insertion_time(index)?.into())
    }

    pub(crate) fn msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.container.get(index)?.msg_id.as_slice())
    }

    pub(crate) fn has_attachments_(
        &self,
        index: usize,
    ) -> Option<bool> {
        Some(self.container.msg_data(index)?.has_attachments)
    }

    pub(crate) fn emit_(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    pub(crate) fn row_count_(&self) -> usize {
        self.container.len()
    }

    /// Indicates whether regex search is activated
    pub(crate) fn search_regex_(&self) -> bool {
        self.search.is_regex()
    }

    /// Indicates whether search is active
    pub(crate) fn search_active_(&self) -> bool {
        self.search.active
    }

    pub(crate) fn search_num_matches_(&self) -> u64 {
        self.search.num_matches() as u64
    }

    pub(crate) fn next_search_match_(&mut self) -> i64 {
        self.next_match_helper().map(|ix| ix as i64).unwrap_or(-1)
    }

    pub(crate) fn prev_search_match_(&mut self) -> i64 {
        self.prev_match_helper().map(|ix| ix as i64).unwrap_or(-1)
    }

    pub(crate) fn search_index_(&self) -> u64 {
        self.search.index.map(|ix| ix + 1).unwrap_or(0) as u64
    }

    pub(crate) fn match_status_(
        &self,
        index: usize,
    ) -> Option<u8> {
        Some(self.container.msg_data(index)?.match_status as u8)
    }
}
