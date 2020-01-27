use crate::interface::{ConversationsEmitter, ConversationsList};
use crate::{err, ffi, none, spawn};
use heraldcore::types::ConversationId;
use im::Vector;
use search_pattern::SearchPattern;
use std::convert::TryFrom;
use std::ops::Not;

impl super::Conversations {
    pub(crate) fn new_(
        emit: ConversationsEmitter,
        model: ConversationsList,
    ) -> Self {
        let filter = SearchPattern::new_normal("".into()).ok();

        Self {
            emit,
            filter,
            filter_regex: false,
            model,
            list: Vector::new(),
            loaded: false,
            builder_cid: None,
        }
    }

    pub(crate) fn emit_(&mut self) -> &mut ConversationsEmitter {
        &mut self.emit
    }

    pub(crate) fn row_count_(&self) -> usize {
        self.list.len()
    }

    pub(crate) fn conversation_id_(
        &self,
        index: usize,
    ) -> ffi::ConversationId {
        none!(self.list.get(index), vec![]).id.to_vec()
    }

    pub(crate) fn remove_conversation_(
        &mut self,
        index: u64,
    ) -> bool {
        let index = index as usize;
        let cid = none!(self.id(index), false);

        // cannot remove pairwise conversation!
        if self.pairwise_inner(index).unwrap_or(false) {
            return false;
        }

        spawn!(
            err!(heraldcore::message::delete_conversation_messages(&cid)),
            false
        );

        self.model.begin_remove_rows(index, index);
        self.list.remove(index);
        self.model.end_remove_rows();

        true
    }

    pub(crate) fn matched_(
        &self,
        row_index: usize,
    ) -> bool {
        none!(self.list.get(row_index), true).matched
    }

    pub(crate) fn filter_(&self) -> &str {
        self.filter.as_ref().map(SearchPattern::raw).unwrap_or("")
    }

    pub(crate) fn set_filter_(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_filter_();
            return;
        }

        let pattern = if self.filter_regex_() {
            err!(SearchPattern::new_regex(pattern))
        } else {
            err!(SearchPattern::new_normal(pattern))
        };

        self.filter.replace(pattern);
        self.emit.filter_changed();

        self.inner_filter();
    }

    /// Indicates whether regex search is activated
    pub(crate) fn filter_regex_(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    pub(crate) fn set_filter_regex_(
        &mut self,
        use_regex: bool,
    ) {
        if use_regex {
            err!(self
                .filter
                .as_mut()
                .map(SearchPattern::regex_mode)
                .transpose());
        } else {
            err!(self
                .filter
                .as_mut()
                .map(SearchPattern::normal_mode)
                .transpose());
        }

        self.filter_regex = use_regex;
        self.emit.filter_regex_changed();
        self.inner_filter();
    }

    /// Toggles filter mode
    ///
    /// Returns new value.
    pub(crate) fn toggle_filter_regex_(&mut self) -> bool {
        let toggled = !self.filter_regex;
        self.set_filter_regex_(toggled);
        toggled
    }

    pub(crate) fn clear_filter_(&mut self) {
        for (ix, conv) in self.list.iter_mut().enumerate() {
            if conv.matched.not() {
                conv.matched = true;
                self.model.data_changed(ix, ix);
            }
        }

        if let Some(filter) = self.filter.as_mut() {
            err!(filter.set_pattern("".to_owned()));
        }

        self.emit.filter_changed();
    }

    pub(crate) fn index_by_id_(
        &self,
        cid: ffi::ConversationIdRef,
    ) -> i64 {
        let conversation_id = err!(ConversationId::try_from(cid), -1);
        self.list
            .iter()
            .position(|super::Conversation { id, .. }| id == &conversation_id)
            .map(|n| n as i64)
            .unwrap_or(-1)
    }
}
