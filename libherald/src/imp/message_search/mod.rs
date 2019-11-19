use crate::{
    ffi,
    imp::conversations::shared as conv,
    interface::{
        MessageSearchEmitter as Emitter, MessageSearchList as List, MessageSearchTrait as Interface,
    },
    ret_err,
};
use crossbeam_channel::{unbounded, Receiver};
use heraldcore::{
    message::{Search, SearchResult},
    utils::SearchPattern,
};
use std::ops::Not;

mod imp;

/// Global message search handle
pub struct MessageSearch {
    pattern: Option<SearchPattern>,
    emit: Emitter,
    model: List,
    results: Vec<SearchResult>,
    rx: Option<Receiver<Vec<SearchResult>>>,
}

impl Interface for MessageSearch {
    fn new(emit: Emitter, model: List) -> Self {
        Self {
            pattern: None,
            results: vec![],
            model,
            emit,
            rx: None,
        }
    }

    fn regex_search(&self) -> Option<bool> {
        Some(self.pattern.as_ref()?.is_regex())
    }

    fn set_regex_search(&mut self, is_regex: Option<bool>) {
        if let (Some(new), Some(mut pattern)) = (is_regex, self.pattern.take()) {
            match (new, pattern.is_regex()) {
                (true, false) => {
                    self.clear_search();

                    ret_err!(pattern.regex_mode());
                    self.pattern = Some(pattern.clone());
                    self.emit.regex_search_changed();

                    ret_err!(imp::start_search(pattern.clone(), self.emit()));
                }
                (false, true) => {
                    ret_err!(pattern.normal_mode());
                    self.emit.regex_search_changed();
                    ret_err!(imp::start_search(pattern.clone(), self.emit()));
                }
                _ => {}
            }
        }
    }

    fn search_pattern(&self) -> Option<&str> {
        Some(self.pattern.as_ref()?.raw())
    }

    fn set_search_pattern(&mut self, pattern: Option<String>) {
        match (pattern, self.pattern.take()) {
            (Some(new), Some(mut old)) => {
                self.clear_search();
                if new.is_empty() {
                    return;
                }

                ret_err!(old.set_pattern(new));
                self.pattern = Some(old.clone());
                self.emit.search_pattern_changed();

                ret_err!(imp::start_search(old, self.emit()));
            }
            (Some(new), None) => {
                self.clear_search();
                if new.is_empty() {
                    return;
                }

                let pattern = ret_err!(SearchPattern::new_normal(new));
                self.pattern = Some(pattern.clone());
                self.emit.search_pattern_changed();

                ret_err!(imp::start_search(pattern, self.emit()));
            }
            (None, _) => self.clear_search(),
        }
    }

    fn clear_search(&mut self) {
        self.model.begin_reset_model();
        self.pattern = None;
        self.rx = None;
        self.results = Vec::new();
        self.model.end_reset_model();
    }

    fn row_count(&self) -> usize {
        self.results.len()
    }

    fn can_fetch_more(&self) -> bool {
        match self.rx.as_ref() {
            Some(rx) => rx.is_empty().not(),
            None => false,
        }
    }

    fn fetch_more(&mut self) {
        if let Some(rx) = self.rx.as_ref() {
            for mut results in rx.try_iter() {
                if results.is_empty() {
                    continue;
                }

                let last_ix = self.results.len().saturating_sub(1);

                self.model
                    .begin_insert_rows(last_ix, last_ix + results.len());
                self.results.append(&mut results);
                self.model.end_insert_rows();
            }
        }
    }

    fn author(&self, index: usize) -> Option<ffi::UserIdRef> {
        Some(self.results.get(index).as_ref()?.author.as_str())
    }

    fn body(&self, index: usize) -> Option<&str> {
        Some(self.results.get(index).as_ref()?.body.as_str())
    }

    fn conversation(&self, index: usize) -> Option<ffi::ConversationIdRef> {
        Some(self.results.get(index).as_ref()?.conversation.as_slice())
    }

    fn has_attachments(&self, index: usize) -> Option<bool> {
        Some(self.results.get(index).as_ref()?.has_attachments)
    }

    fn msg_id(&self, index: usize) -> Option<ffi::MsgIdRef> {
        Some(self.results.get(index).as_ref()?.message_id.as_slice())
    }

    fn time(&self, index: usize) -> Option<i64> {
        Some(self.results.get(index).as_ref()?.time.0)
    }

    fn conversation_pairwise(&self, index: usize) -> Option<bool> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::pairwise(&cid)
    }

    fn conversation_title(&self, index: usize) -> Option<String> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::title(&cid)
    }

    fn conversation_color(&self, index: usize) -> Option<u32> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::color(&cid)
    }

    fn conversation_picture(&self, index: usize) -> Option<String> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::picture(&cid)
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
