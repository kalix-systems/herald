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
mod underscore;

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
        Self::new_(emit, model, builder)
    }

    fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        self.last_author_()
    }

    fn last_status(&self) -> Option<u32> {
        self.last_status_()
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
        self.index_by_id_(msg_id)
    }

    fn set_conversation_id(
        &mut self,
        conversation_id: Option<ffi::ConversationIdRef>,
    ) {
        self.set_conversation_id_(conversation_id)
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|c| c.as_slice())
    }

    fn data_saved(
        &self,
        index: usize,
    ) -> Option<bool> {
        Some(self.container.msg_data(index)?.save_status == SaveStatus::Saved)
    }

    fn author(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        Some(self.container.msg_data(index)?.author.as_str())
    }

    fn body(
        &self,
        index: usize,
    ) -> Option<&str> {
        Some(self.container.msg_data(index)?.body.as_ref()?.as_str())
    }

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.container.get(index)?.msg_id.as_slice())
    }

    fn has_attachments(
        &self,
        index: usize,
    ) -> Option<bool> {
        Some(self.container.msg_data(index)?.has_attachments)
    }

    fn receipt_status(
        &self,
        index: usize,
    ) -> Option<u32> {
        self.receipt_status_(index)
    }

    fn match_status(
        &self,
        index: usize,
    ) -> Option<u8> {
        Some(self.container.msg_data(index)?.match_status as u8)
    }

    fn is_head(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.is_head_(index)
    }

    fn is_tail(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.is_tail_(index)
    }

    fn insertion_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.get(index)?.insertion_time.0)
    }

    fn expiration_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.msg_data(index)?.time.expiration?.0)
    }

    fn server_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.msg_data(index)?.time.server?.0)
    }

    fn delete_message(
        &mut self,
        index: u64,
    ) -> bool {
        self.delete_message_(index)
    }

    /// Deletes all messages in the current conversation.
    fn clear_conversation_history(&mut self) -> bool {
        self.clear_conversation_history_()
    }

    fn can_fetch_more(&self) -> bool {
        self.can_fetch_more_()
    }

    /// Polls for updates
    fn fetch_more(&mut self) {
        self.fetch_more_()
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.container.len()
    }

    fn search_pattern(&self) -> &str {
        self.search_pattern_()
    }

    fn set_search_pattern(
        &mut self,
        pattern: String,
    ) {
        self.set_search_pattern_(pattern)
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
        self.set_search_regex_(use_regex)
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
        self.set_search_active_(active)
    }

    /// Clears search
    fn clear_search(&mut self) {
        self.clear_search_()
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
        self.set_search_hint_(scroll_position, scroll_height)
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
        self.set_builder_op_msg_id_(id)
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
        self.op_body_(index)
    }

    fn op_has_attachments(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.container.op_has_attachments(index)
    }

    fn op_insertion_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_insertion_time(index)?.0)
    }

    fn op_expiration_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_insertion_time(index)?.0)
    }
}
