use super::*;

impl Messages {
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
}
