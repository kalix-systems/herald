pub use crate::messages::{builder::MessageBuilder, Messages};
use crate::{
    ffi,
    interface::{MessagesEmitter as Emitter, MessagesList as List, MessagesTrait as Interface},
};

impl Interface for Messages {
    fn new(
        emit: Emitter,
        model: List,
        builder: MessageBuilder,
    ) -> Self {
        Self::new_(emit, model, builder)
    }

    fn is_empty(&self) -> bool {
        self.is_empty_()
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        self.last_author_()
    }

    fn last_status(&self) -> Option<u32> {
        self.last_status_()
    }

    fn last_body(&self) -> Option<&str> {
        self.last_body_()
    }

    fn last_time(&self) -> Option<i64> {
        self.last_time_()
    }

    fn index_by_id(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> u64 {
        self.index_by_id_(msg_id)
    }

    fn data_saved(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.data_saved_(index)
    }

    fn author(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        self.author_(index)
    }

    fn body(
        &self,
        index: usize,
    ) -> Option<&str> {
        self.body_(index)
    }

    fn receipt_status(
        &self,
        index: usize,
    ) -> Option<u32> {
        self.receipt_status_(index)
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

    fn delete_message(
        &mut self,
        index: u64,
    ) -> bool {
        self.delete_message_(index)
    }

    fn clear_conversation_history(&mut self) -> bool {
        self.clear_conversation_history_()
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

    fn set_search_regex(
        &mut self,
        use_regex: bool,
    ) {
        self.set_search_regex_(use_regex)
    }

    fn set_search_active(
        &mut self,
        active: bool,
    ) {
        self.set_search_active_(active)
    }

    fn clear_search(&mut self) {
        self.clear_search_()
    }

    fn set_search_hint(
        &mut self,
        scroll_position: f32,
        scroll_height: f32,
    ) {
        self.set_search_hint_(scroll_position, scroll_height)
    }

    fn set_builder_op_msg_id(
        &mut self,
        id: Option<ffi::MsgIdRef>,
    ) {
        self.set_builder_op_msg_id_(id)
    }

    fn op_body(
        &self,
        index: usize,
    ) -> Option<&str> {
        self.op_body_(index)
    }

    fn insertion_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.insertion_time_(index)
    }

    fn expiration_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.expiration_time_(index)
    }

    fn server_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.server_time_(index)
    }

    fn reply_type(
        &self,
        index: usize,
    ) -> Option<u8> {
        self.reply_type_(index)
    }

    fn builder(&self) -> &MessageBuilder {
        self.builder_()
    }

    fn builder_mut(&mut self) -> &mut MessageBuilder {
        self.builder_mut_()
    }

    fn builder_op_msg_id(&self) -> Option<ffi::MsgIdRef> {
        self.builder_op_msg_id_()
    }

    fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        self.op_msg_id_(index)
    }

    fn op_author(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        self.op_author_(index)
    }

    fn op_has_attachments(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.op_has_attachments_(index)
    }

    fn op_insertion_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.op_insertion_time_(index)
    }

    fn op_expiration_time(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.op_expiration_time_(index)
    }

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        self.msg_id_(index)
    }

    fn has_attachments(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.has_attachments_(index)
    }

    fn emit(&mut self) -> &mut Emitter {
        self.emit_()
    }

    fn row_count(&self) -> usize {
        self.row_count_()
    }

    fn search_regex(&self) -> bool {
        self.search_regex_()
    }

    fn search_active(&self) -> bool {
        self.search_active_()
    }

    fn search_num_matches(&self) -> u64 {
        self.search_num_matches_()
    }

    fn next_search_match(&mut self) -> i64 {
        self.next_search_match_()
    }

    fn prev_search_match(&mut self) -> i64 {
        self.prev_search_match_()
    }

    fn search_index(&self) -> u64 {
        self.search_index_()
    }

    fn match_status(
        &self,
        index: usize,
    ) -> Option<u8> {
        self.match_status_(index)
    }
}