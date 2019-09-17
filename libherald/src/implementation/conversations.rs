use crate::{interface::*, ret_err, types::*};
use heraldcore::{
    abort_err,
    conversation::{ConversationMeta, Conversations as Core},
    utils::SearchPattern,
};

struct Conversation {
    inner: ConversationMeta,
    matched: bool,
}

pub struct Conversations {
    emit: ConversationsEmitter,
    model: ConversationsList,
    filter: SearchPattern,
    filter_regex: bool,
    list: Vec<Conversation>,
    handle: Core,
}

impl ConversationsTrait for Conversations {
    fn new(emit: ConversationsEmitter, model: ConversationsList) -> Self {
        let handle = abort_err!(Core::new());
        let list = abort_err!(handle.all_meta())
            .into_iter()
            .map(|inner| Conversation {
                inner,
                matched: true,
            })
            .collect();

        let filter = abort_err!(SearchPattern::new_normal("".into()));

        Self {
            emit,
            filter,
            filter_regex: false,
            model,
            handle,
            list,
        }
    }

    fn emit(&mut self) -> &mut ConversationsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn color(&self, index: usize) -> u32 {
        self.list[index].inner.color
    }

    fn set_color(&mut self, index: usize, color: u32) -> bool {
        let meta = &mut self.list[index].inner;
        ret_err!(self.handle.set_color(&meta.conversation_id, color), false);

        meta.color = color;
        true
    }

    fn conversation_id(&self, index: usize) -> FfiConversationIdRef {
        self.list[index].inner.conversation_id.as_slice()
    }

    fn muted(&self, index: usize) -> bool {
        self.list[index].inner.muted
    }

    fn set_muted(&mut self, index: usize, muted: bool) -> bool {
        let meta = &mut self.list[index].inner;
        ret_err!(self.handle.set_muted(&meta.conversation_id, muted), false);

        meta.muted = muted;
        true
    }

    fn picture(&self, index: usize) -> Option<&str> {
        self.list[index].inner.picture.as_ref().map(|p| p.as_str())
    }

    fn set_picture(&mut self, index: usize, picture: Option<String>) -> bool {
        let meta = &mut self.list[index].inner;
        ret_err!(
            self.handle.set_picture(
                &meta.conversation_id,
                picture.as_ref().map(|p| p.as_str()),
                meta.picture.as_ref().map(|p| p.as_str())
            ),
            false
        );

        meta.picture = picture;
        true
    }

    fn title(&self, index: usize) -> Option<&str> {
        self.list[index].inner.title.as_ref().map(|t| t.as_str())
    }

    fn set_title(&mut self, index: usize, title: Option<String>) -> bool {
        let meta = &mut self.list[index].inner;
        ret_err!(
            self.handle
                .set_title(&meta.conversation_id, title.as_ref().map(|t| t.as_str())),
            false
        );

        meta.title = title;
        true
    }

    fn pairwise(&self, index: usize) -> bool {
        self.list[index].inner.pairwise
    }

    fn add_conversation(&mut self) -> Vec<u8> {
        let conv_id = ret_err!(self.handle.add_conversation(None, None), vec![]);
        let inner = ret_err!(self.handle.meta(&conv_id), vec![]);

        let meta = Conversation {
            matched: inner.matches(&self.filter),
            inner,
        };

        self.model.begin_insert_rows(0, 0);
        self.list.insert(0, meta);
        self.model.end_insert_rows();
        conv_id.to_vec()
    }

    fn remove_conversation(&mut self, index: u64) -> bool {
        let index = index as usize;
        let meta = &mut self.list[index].inner;

        // cannot remove pairwise conversation!
        if meta.pairwise {
            return false;
        }

        ret_err!(
            self.handle.delete_conversation(&meta.conversation_id),
            false
        );

        self.model.begin_remove_rows(index, index);
        self.list.remove(index);
        self.model.end_remove_rows();

        true
    }

    fn matched(&self, row_index: usize) -> bool {
        self.list[row_index].matched
    }

    fn set_matched(&mut self, row_index: usize, value: bool) -> bool {
        self.list[row_index].matched = value;
        true
    }

    fn filter(&self) -> &str {
        self.filter.raw()
    }

    fn set_filter(&mut self, pattern: String) {
        if pattern.is_empty() {
            self.clear_filter();
            return;
        }

        let pattern = if self.filter_regex() {
            ret_err!(SearchPattern::new_regex(pattern))
        } else {
            ret_err!(SearchPattern::new_normal(pattern))
        };

        self.filter = pattern;
        self.emit.filter_changed();

        self.inner_filter();
    }

    /// Indicates whether regex search is activated
    fn filter_regex(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    fn set_filter_regex(&mut self, use_regex: bool) {
        if use_regex {
            ret_err!(self.filter.regex_mode());
        } else {
            ret_err!(self.filter.normal_mode());
        }
        self.filter_regex = use_regex;
        self.emit.filter_regex_changed();
        self.inner_filter();
    }

    /// Toggles filter mode
    ///
    /// Returns new value.
    fn toggle_filter_regex(&mut self) -> bool {
        let toggled = !self.filter_regex;
        self.set_filter_regex(toggled);
        toggled
    }
}

impl Conversations {
    fn clear_filter(&mut self) {
        for conv in self.list.iter_mut() {
            conv.matched = true;
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));

        if self.filter_regex {
            self.filter = ret_err!(SearchPattern::new_regex("".to_owned()));
        } else {
            self.filter = ret_err!(SearchPattern::new_normal("".to_owned()));
        }

        self.emit.filter_changed();
    }

    fn inner_filter(&mut self) {
        for conv in self.list.iter_mut() {
            conv.matched = conv.inner.matches(&self.filter);
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));
    }
}
