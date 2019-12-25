use super::*;
use crate::{
    err, ffi,
    interface::{MessagesEmitter as Emitter, MessagesList as List, MessagesTrait as Interface},
    none, spawn,
};
use heraldcore::{
    config, conversation,
    message::{self, MessageBody, MessageReceiptStatus, MsgData},
};
use messages_helper::search::SearchState;

use std::collections::HashMap;
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
            elider: Default::default(),
        }
    }

    pub(crate) fn receipt_status_(
        &self,
        index: usize,
    ) -> Option<u32> {
        let local_id = self.local_id?;

        Some(
            self.container
                .access_by_index(index, |data| {
                    data.receipts
                        .iter()
                        .filter(|(k, _)| k != &&local_id)
                        .map(|(_, r)| *r as u32)
                        .max()
                })?
                .unwrap_or(MessageReceiptStatus::Nil as u32),
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
    ) -> i64 {
        let msg_id = err!(msg_id.try_into(), -1);

        none!(self.container.index_by_id(msg_id), -1) as i64
    }

    pub(crate) fn is_tail_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if index == 0 {
            return Some(true);
        }

        // other cases
        self.container
            .same_flurry(index, index - 1)
            .map(std::ops::Not::not)
    }

    pub(crate) fn is_head_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if index + 1 == self.container.len() {
            return Some(true);
        }

        // other cases
        self.container
            .same_flurry(index, index + 1)
            .map(std::ops::Not::not)
    }

    pub(crate) fn clear_conversation_history_(&mut self) -> bool {
        let id = none!(self.conversation_id, false);

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

        let id = none!(self.container.get(ix), false).msg_id;

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

    pub(crate) fn doc_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.doc_attachments_data_json(index, None)
    }

    pub(crate) fn media_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.media_attachments_data_json(index, 4.into())
    }

    pub(crate) fn full_media_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.media_attachments_data_json(index, None)
    }

    pub(crate) fn op_body_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.op_body(index)
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

            let emit = &mut self.emit;
            let mut emit_index = emit.clone();

            let model = &mut self.model;

            let matches = self
                .container
                .apply_search(
                    &self.search,
                    |ix| model.data_changed(ix, ix),
                    || emit_index.search_num_matches_changed(),
                )
                .unwrap_or_default();

            self.search.set_matches(
                matches,
                || emit.search_num_matches_changed(),
                || emit_index.search_index_changed(),
            );
        }
    }

    /// Clears search
    pub(crate) fn clear_search_(&mut self) {
        let model = &mut self.model;
        let emit = &mut self.emit;
        self.container.clear_search(|ix| model.data_changed(ix, ix));

        err!(self.search.clear_search(|| {
            emit.search_index_changed();
            emit.search_pattern_changed();
            emit.search_regex_changed();
            emit.search_num_matches_changed();
        }));
    }

    pub(crate) fn set_search_pattern_(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_search();
            return;
        }

        let emit = &mut self.emit;

        let changed = err!(self
            .search
            .set_pattern(pattern, || emit.search_pattern_changed())
            .map(messages_helper::search::SearchChanged::changed));

        if changed && self.search.active {
            let model = &mut self.model;
            let matches = self
                .container
                .apply_search(
                    &self.search,
                    |ix| model.data_changed(ix, ix),
                    || emit.search_num_matches_changed(),
                )
                .unwrap_or_default();

            let mut emit_index = emit.clone();

            self.search.set_matches(
                matches,
                || emit_index.search_index_changed(),
                || emit.search_num_matches_changed(),
            );
        }
    }

    /// Sets search mode
    pub(crate) fn set_search_regex_(
        &mut self,
        use_regex: bool,
    ) {
        let emit = &mut self.emit;

        let changed = err!(self
            .search
            .set_regex(use_regex, || emit.search_regex_changed()))
        .changed();

        if changed {
            let model = &mut self.model;

            let matches = self
                .container
                .apply_search(
                    &self.search,
                    |ix| model.data_changed(ix, ix),
                    || emit.search_num_matches_changed(),
                )
                .unwrap_or_default();

            let mut emit_index = emit.clone();
            self.search.set_matches(
                matches,
                || emit.search_num_matches_changed(),
                || emit_index.search_index_changed(),
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

    pub(crate) fn author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.container
            .access_by_index(index, |data| data.author.to_string())
    }

    pub(crate) fn author_color_(
        &self,
        index: usize,
    ) -> Option<u32> {
        let uid = self.container.access_by_index(index, |data| data.author)?;
        crate::users::shared::color(&uid)
    }

    pub(crate) fn author_name_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        let uid = self.container.access_by_index(index, |data| data.author)?;
        crate::users::shared::name(&uid)
    }

    pub(crate) fn author_profile_picture_(
        &self,
        index: usize,
    ) -> Option<String> {
        let uid = self.container.access_by_index(index, |data| data.author)?;
        crate::users::shared::profile_picture(&uid)
    }

    pub(crate) fn body_(
        &self,
        index: usize,
    ) -> Option<String> {
        let elider = &self.elider;
        let pattern = &self.search.pattern;
        let match_status = self.container.get(index).as_ref()?.match_status;

        let body = self
            .container
            .access_by_index(index, |data| data.body.clone())?;

        if match_status.is_match() {
            messages_helper::search::highlight_message(pattern.as_ref()?, body.as_ref()?).into()
        } else {
            elider.elided_body(body?).into()
        }
    }

    pub(crate) fn full_body_(
        &self,
        index: usize,
    ) -> Option<String> {
        let pattern = &self.search.pattern;
        let match_status = self.container.get(index).as_ref()?.match_status;

        self.container.access_by_index(index, |data| {
            if match_status.is_match() {
                Some(messages_helper::search::highlight_message(
                    pattern.as_ref()?,
                    data.body.as_ref()?,
                ))
            } else {
                data.body.as_ref().map(MessageBody::to_string)
            }
        })?
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
        self.container.access_by_index(index, |data| {
            data.time.expiration.map(herald_common::Time::into)
        })?
    }

    pub(crate) fn server_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.container.access_by_index(index, |data| {
            data.time.server.map(herald_common::Time::into)
        })?
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

    pub(crate) fn op_msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgId> {
        self.container.op_msg_id(index).map(MsgId::to_vec)
    }

    pub(crate) fn op_author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.container
            .op_author(index)
            .as_ref()
            .map(UserId::as_str)
            .map(str::to_string)
    }

    pub(crate) fn op_color_(
        &self,
        index: usize,
    ) -> Option<u32> {
        let uid = self.container.op_author(index)?;
        crate::users::shared::color(&uid)
    }

    pub(crate) fn op_name_(
        &self,
        index: usize,
    ) -> Option<String> {
        let uid = self.container.op_author(index)?;
        crate::users::shared::name(&uid)
    }

    pub(crate) fn op_doc_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.op_doc_attachments_json(index)
    }

    pub(crate) fn op_media_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.op_media_attachments_json(index)
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
        Some(self.container.op_expiration_time(index)?.into())
    }

    pub(crate) fn msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.container.get(index)?.msg_id.as_slice())
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
        Some(self.container.list.get(index)?.match_status as u8)
    }

    pub(crate) fn set_elision_line_count_(
        &mut self,
        line_count: u8,
    ) {
        self.elider.set_line_count(line_count as usize);
    }

    pub(crate) fn set_elision_char_count_(
        &mut self,
        char_count: u16,
    ) {
        self.elider.set_char_count(char_count as usize);
    }

    pub(crate) fn set_elision_chars_per_line_(
        &mut self,
        chars_per_line: u8,
    ) {
        self.elider.set_char_per_line(chars_per_line as usize);
    }

    pub(crate) fn save_all_attachments_(
        &self,
        index: usize,
        dest: String,
    ) -> bool {
        let dest = none!(crate::utils::strip_qrc(dest), false);
        let data = none!(self.container.access_by_index(index, MsgData::clone), false);

        spawn!(err!(data.save_all_attachments(dest)), false);
        true
    }

    pub(crate) fn user_receipts_(
        &self,
        index: usize,
    ) -> Option<String> {
        let receipts = self
            .container
            .access_by_index(index, |data| data.receipts.clone())?
            .into_iter()
            .filter(|(u, _)| Some(u) != self.local_id.as_ref())
            .map(|(userid, receipt)| (userid.to_string(), json::JsonValue::from(receipt as u32)))
            .collect::<HashMap<String, json::JsonValue>>();

        json::JsonValue::from(receipts).dump().into()
    }

    pub(crate) fn mark_read_(
        &mut self,
        index: u64,
    ) {
        let index = index as usize;

        let local_id = none!(self.local_id);
        let msg_id = *none!(self.container.msg_id(index));
        let cid = none!(self.conversation_id);

        let updated = none!(self.container.update_by_index(index, |data| {
            let status = data.receipts.entry(local_id).or_default();

            match *status {
                MessageReceiptStatus::Read => false,
                _ => {
                    *status = MessageReceiptStatus::Read;
                    true
                }
            }
        }));

        if !updated {
            return;
        }

        self.model.data_changed(index, index);

        spawn!({
            err!(heraldcore::message::add_receipt(
                msg_id,
                local_id,
                MessageReceiptStatus::Read
            ));
            err!(heraldcore::network::send_read_receipt(cid, msg_id));
        });
    }
}
