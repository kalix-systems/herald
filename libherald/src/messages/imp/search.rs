use super::*;

impl Messages {
    pub(crate) fn match_status_(
        &self,
        index: usize,
    ) -> Option<u8> {
        Some(self.container.list.get(index)?.match_status as u8)
    }

    pub(crate) fn search_pattern_(&self) -> &str {
        self.search
            .pattern
            .as_ref()
            .map(SearchPattern::raw)
            .unwrap_or("")
    }

    pub(crate) fn set_search_hint_(
        &mut self,
        scroll_position: f32,
        scroll_height: f32,
    ) {
        if scroll_position.is_nan() || scroll_height.is_nan() {
            return;
        }

        let percentage = (1.0 - scroll_position) + scroll_height / 2.0;
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

            let model = &mut self.model;

            let matches = self
                .container
                .apply_search(&self.search, emit, model)
                .unwrap_or_default();

            self.search.set_matches(matches, emit);
        }
    }

    /// Clears search
    pub(crate) fn clear_search_(&mut self) {
        let model = &mut self.model;
        let emit = &mut self.emit;
        self.container.clear_search(model);

        err!(self.search.clear_search(emit));
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
            .set_pattern(pattern, emit)
            .map(messages_helper::search::SearchChanged::changed));

        if changed && self.search.active {
            let model = &mut self.model;
            let matches = self
                .container
                .apply_search(&self.search, emit, model)
                .unwrap_or_default();

            self.search.set_matches(matches, emit);
        }
    }

    /// Sets search mode
    pub(crate) fn set_search_regex_(
        &mut self,
        use_regex: bool,
    ) {
        let emit = &mut self.emit;

        let changed = err!(self.search.set_regex(use_regex, emit)).changed();

        if changed {
            let model = &mut self.model;

            let matches = self
                .container
                .apply_search(&self.search, emit, model)
                .unwrap_or_default();

            self.search.set_matches(matches, emit);
        }
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
        self.search
            .next_match_helper(&mut self.container, &mut self.emit, &mut self.model)
            .map(|ix| ix as i64)
            .unwrap_or(-1)
    }

    pub(crate) fn prev_search_match_(&mut self) -> i64 {
        self.search
            .prev_match_helper(&mut self.container, &mut self.emit, &mut self.model)
            .map(|ix| ix as i64)
            .unwrap_or(-1)
    }

    pub(crate) fn search_index_(&self) -> u64 {
        self.search.index.map(|ix| ix + 1).unwrap_or(0) as u64
    }
}
