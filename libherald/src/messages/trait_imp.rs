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

    fn last_author(&self) -> Option<ffi::UserId> {
        self.last_author_()
    }

    fn last_status(&self) -> Option<u32> {
        self.last_status_()
    }

    fn last_body(&self) -> Option<String> {
        self.last_body_()
    }

    fn last_time(&self) -> Option<i64> {
        self.last_time_()
    }

    fn last_aux_code(&self) -> Option<u8> {
        self.last_aux_code_()
    }

    fn last_has_attachments(&self) -> Option<bool> {
        self.last_has_attachments_()
    }

    fn index_by_id(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> i64 {
        self.index_by_id_(msg_id)
    }

    fn author(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.author_(index)
    }

    fn author_color(
        &self,
        index: usize,
    ) -> Option<u32> {
        self.author_color_(index)
    }

    fn author_name(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.author_name_(index)
    }

    fn author_profile_picture(
        &self,
        index: usize,
    ) -> String {
        self.author_profile_picture_(index).unwrap_or_default()
    }

    fn body(
        &self,
        index: usize,
    ) -> String {
        self.body_(index).unwrap_or_default()
    }

    fn full_body(
        &self,
        index: usize,
    ) -> String {
        self.full_body_(index).unwrap_or_default()
    }

    fn receipt_status(
        &self,
        index: usize,
    ) -> Option<u32> {
        self.receipt_status_(index)
    }

    fn aux_data(
        &self,
        index: usize,
    ) -> String {
        self.aux_data_(index)
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

    fn doc_attachments(
        &self,
        index: usize,
    ) -> String {
        self.doc_attachments_(index).unwrap_or_default()
    }

    fn media_attachments(
        &self,
        index: usize,
    ) -> String {
        self.media_attachments_(index).unwrap_or_default()
    }

    fn full_media_attachments(
        &self,
        index: usize,
    ) -> String {
        self.full_media_attachments_(index).unwrap_or_default()
    }

    fn delete_message(
        &mut self,
        index: u64,
    ) -> bool {
        self.delete_message_(index)
    }

    fn delete_message_by_id(
        &mut self,
        id: ffi::MsgIdRef,
    ) -> bool {
        self.delete_message_by_id_(id)
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

    fn op_body(
        &self,
        index: usize,
    ) -> String {
        self.op_body_(index).unwrap_or_default()
    }

    fn op_aux_data(
        &self,
        index: usize,
    ) -> String {
        self.op_aux_data_(index).unwrap_or_default()
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

    fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgId> {
        self.op_msg_id_(index)
    }

    fn op_author(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.op_author_(index)
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

    fn op_media_attachments(
        &self,
        index: usize,
    ) -> String {
        self.op_media_attachments_(index).unwrap_or_default()
    }

    fn op_doc_attachments(
        &self,
        index: usize,
    ) -> String {
        self.op_doc_attachments_(index).unwrap_or_default()
    }

    fn op_color(
        &self,
        index: usize,
    ) -> Option<u32> {
        self.op_color_(index)
    }

    fn op_name(
        &self,
        index: usize,
    ) -> Option<String> {
        self.op_name_(index)
    }

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        self.msg_id_(index)
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

    fn set_elision_line_count(
        &mut self,
        line_count: u8,
    ) {
        self.set_elision_line_count_(line_count)
    }

    fn set_elision_char_count(
        &mut self,
        char_count: u16,
    ) {
        self.set_elision_char_count_(char_count)
    }

    fn set_elision_chars_per_line(
        &mut self,
        chars_per_line: u8,
    ) {
        self.set_elision_chars_per_line_(chars_per_line)
    }

    fn save_all_attachments(
        &self,
        index: u64,
        dest: String,
    ) -> bool {
        self.save_all_attachments_(index as usize, dest)
    }

    fn user_receipts(
        &self,
        index: usize,
    ) -> String {
        self.user_receipts_(index).unwrap_or_default()
    }

    fn mark_read_by_id(
        &mut self,
        id: ffi::MsgIdRef,
    ) {
        self.mark_read_(id)
    }

    fn add_reaction(
        &mut self,
        index: u64,
        content: String,
    ) {
        self.add_reaction_(index, content)
    }

    fn remove_reaction(
        &mut self,
        index: u64,
        content: String,
    ) {
        self.remove_reaction_(index, content)
    }

    fn reactions(
        &self,
        index: usize,
    ) -> String {
        self.reactions_(index).unwrap_or_default()
    }
}
