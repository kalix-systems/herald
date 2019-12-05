use crate::{
    conversations::shared as conv,
    ffi,
    interface::{
        MessageSearchEmitter as Emitter, MessageSearchList as List, MessageSearchTrait as Interface,
    },
    ret_err,
};
use crossbeam_channel::{unbounded, Receiver};
use heraldcore::message::{Search, SearchResult};
use search_pattern::SearchPattern;
use std::ops::Not;

mod imp;

enum SearchThreadUpdate {
    Res(Vec<SearchResult>),
    Done,
}

/// Global message search handle
pub struct MessageSearch {
    pattern: Option<SearchPattern>,
    emit: Emitter,
    model: List,
    results: Vec<SearchResult>,
    num_results: usize,
    rx: Option<Receiver<SearchThreadUpdate>>,
}

impl Interface for MessageSearch {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Self {
        Self {
            pattern: None,
            results: Vec::new(),
            num_results: 0,
            model,
            emit,
            rx: None,
        }
    }

    fn regex_search(&self) -> Option<bool> {
        Some(self.pattern.as_ref()?.is_regex())
    }

    fn set_regex_search(
        &mut self,
        is_regex: Option<bool>,
    ) {
        if let (Some(new), Some(mut pattern)) = (is_regex, self.pattern.take()) {
            match (new, pattern.is_regex()) {
                (true, false) => {
                    self.clear_search();

                    ret_err!(pattern.regex_mode());
                    self.pattern = Some(pattern.clone());
                    self.emit.regex_search_changed();

                    ret_err!(self.start_search(pattern));
                }
                (false, true) => {
                    ret_err!(pattern.normal_mode());
                    self.emit.regex_search_changed();
                    ret_err!(self.start_search(pattern));
                }
                _ => {}
            }
        }
    }

    fn search_pattern(&self) -> Option<&str> {
        Some(self.pattern.as_ref()?.raw())
    }

    fn set_search_pattern(
        &mut self,
        pattern: Option<String>,
    ) {
        match (pattern, self.pattern.take()) {
            (Some(new), Some(mut old)) => {
                self.num_results = 0;
                if new.is_empty() {
                    self.clear_search();
                    return;
                }

                ret_err!(old.set_pattern(new));
                self.pattern = Some(old.clone());
                self.emit.search_pattern_changed();

                ret_err!(self.start_search(old));
            }
            (Some(new), None) => {
                self.num_results = 0;
                if new.is_empty() {
                    self.clear_search();
                    return;
                }

                let pattern = ret_err!(SearchPattern::new_normal(new));
                self.pattern = Some(pattern.clone());
                self.emit.search_pattern_changed();

                ret_err!(self.start_search(pattern));
            }
            (None, _) => self.clear_search(),
        }
    }

    fn clear_search(&mut self) {
        self.model.begin_reset_model();
        self.pattern = None;
        self.rx = None;
        self.results = Vec::new();
        self.num_results = 0;
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
            for new in rx.try_iter() {
                match new {
                    SearchThreadUpdate::Res(new_results) => {
                        for new_res in new_results.into_iter() {
                            match self.results.get_mut(self.num_results) {
                                Some(old_res) => {
                                    *old_res = new_res;
                                    self.model.data_changed(self.num_results, self.num_results);
                                }
                                None => {
                                    self.model
                                        .begin_insert_rows(self.num_results, self.num_results);
                                    self.results.push(new_res);
                                    self.model.end_insert_rows();
                                }
                            }

                            self.num_results += 1;
                        }
                    }
                    SearchThreadUpdate::Done => {
                        let new_len = self.num_results;
                        let diff = self.results.len().saturating_sub(new_len);

                        if diff != 0 {
                            self.model
                                .begin_remove_rows(new_len, self.results.len().saturating_sub(1));
                            self.results.truncate(new_len);
                            self.model.end_remove_rows();
                        }
                    }
                }
            }
        }
    }

    fn author(
        &self,
        index: usize,
    ) -> Option<ffi::UserIdRef> {
        Some(self.results.get(index).as_ref()?.author.as_str())
    }

    fn before_first_match(
        &self,
        index: usize,
    ) -> &str {
        match self.results.get(index).as_ref() {
            Some(res) => res.body.before_first.as_str(),
            None => "",
        }
    }

    fn first_match(
        &self,
        index: usize,
    ) -> &str {
        match self.results.get(index).as_ref() {
            Some(res) => res.body.first_match.as_str(),
            None => "",
        }
    }

    fn after_first_match(
        &self,
        index: usize,
    ) -> &str {
        match self.results.get(index).as_ref() {
            Some(res) => res.body.after_first.as_str(),
            None => "",
        }
    }

    fn conversation(
        &self,
        index: usize,
    ) -> Option<ffi::ConversationIdRef> {
        Some(self.results.get(index).as_ref()?.conversation.as_slice())
    }

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.results.get(index).as_ref()?.message_id.as_slice())
    }

    fn time(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.results.get(index).as_ref()?.time.into())
    }

    fn conversation_pairwise(
        &self,
        index: usize,
    ) -> Option<bool> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::pairwise(&cid)
    }

    fn conversation_title(
        &self,
        index: usize,
    ) -> Option<String> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::title(&cid)
    }

    fn conversation_color(
        &self,
        index: usize,
    ) -> Option<u32> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::color(&cid)
    }

    fn conversation_picture(
        &self,
        index: usize,
    ) -> Option<String> {
        let cid = self.results.get(index).as_ref()?.conversation;
        conv::picture(&cid)
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
