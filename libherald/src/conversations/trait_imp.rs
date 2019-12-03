use crate::conversations::Conversations;
use crate::ffi;
use crate::interface::{ConversationsEmitter, ConversationsList, ConversationsTrait};

impl ConversationsTrait for Conversations {
    fn new(
        emit: ConversationsEmitter,
        model: ConversationsList,
    ) -> Self {
        Self::new_(emit, model)
    }

    fn emit(&mut self) -> &mut ConversationsEmitter {
        self.emit_()
    }

    fn row_count(&self) -> usize {
        self.row_count_()
    }

    fn color(
        &self,
        index: usize,
    ) -> u32 {
        self.color_(index)
    }

    fn set_color(
        &mut self,
        index: usize,
        color: u32,
    ) -> bool {
        self.set_color_(index, color)
    }

    fn conversation_id(
        &self,
        index: usize,
    ) -> ffi::ConversationId {
        self.conversation_id_(index)
    }

    fn expiration_period(
        &self,
        index: usize,
    ) -> u8 {
        self.expiration_period_(index)
    }

    fn set_expiration_period(
        &mut self,
        index: usize,
        period: u8,
    ) -> bool {
        self.set_expiration_period_(index, period)
    }

    fn muted(
        &self,
        index: usize,
    ) -> bool {
        self.muted_(index)
    }

    fn set_muted(
        &mut self,
        index: usize,
        muted: bool,
    ) -> bool {
        self.set_muted_(index, muted)
    }

    fn picture(
        &self,
        index: usize,
    ) -> Option<String> {
        self.picture_(index)
    }

    fn set_picture(
        &mut self,
        index: usize,
        picture: Option<String>,
    ) -> bool {
        self.set_picture_(index, picture)
    }

    fn title(
        &self,
        index: usize,
    ) -> Option<String> {
        self.title_(index)
    }

    fn set_title(
        &mut self,
        index: usize,
        title: Option<String>,
    ) -> bool {
        self.set_title_(index, title)
    }

    fn pairwise(
        &self,
        index: usize,
    ) -> bool {
        self.pairwise_(index)
    }

    fn remove_conversation(
        &mut self,
        index: u64,
    ) -> bool {
        self.remove_conversation_(index)
    }

    fn matched(
        &self,
        index: usize,
    ) -> bool {
        self.matched_(index)
    }

    fn filter(&self) -> &str {
        self.filter_()
    }

    fn set_filter(
        &mut self,
        pattern: String,
    ) {
        self.set_filter_(pattern)
    }

    /// Indicates whether regex search is activated
    fn filter_regex(&self) -> bool {
        self.filter_regex_()
    }

    /// Sets filter mode
    fn set_filter_regex(
        &mut self,
        use_regex: bool,
    ) {
        self.set_filter_regex_(use_regex)
    }

    /// Toggles filter mode
    ///
    /// Returns new value.
    fn toggle_filter_regex(&mut self) -> bool {
        self.toggle_filter_regex_()
    }

    fn clear_filter(&mut self) {
        self.clear_filter_()
    }

    fn index_by_id(
        &self,
        cid: ffi::ConversationIdRef,
    ) -> i64 {
        self.index_by_id_(cid)
    }
}
