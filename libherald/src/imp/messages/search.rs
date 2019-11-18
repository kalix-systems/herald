use super::*;
use std::ops::Not;

#[derive(PartialEq)]
pub(super) enum SearchChanged {
    Changed,
    NotChanged,
}

impl SearchChanged {
    pub(super) fn changed(&self) -> bool {
        self == &SearchChanged::Changed
    }
}

#[derive(Copy, Clone, PartialEq)]
pub(super) struct Match(pub(super) MsgId);

pub(super) struct SearchState {
    pub(super) pattern: SearchPattern,
    pub(super) active: bool,
    matches: Vec<Match>,
    pub(super) index: Option<usize>,
}

impl SearchState {
    pub(super) fn new() -> Self {
        Self {
            pattern: abort_err!(SearchPattern::new_normal("".into())),
            active: false,
            matches: Vec::new(),
            index: None,
        }
    }

    pub(super) fn is_regex(&self) -> bool {
        match self.pattern {
            SearchPattern::Normal { .. } => false,
            SearchPattern::Regex { .. } => true,
        }
    }

    pub(super) fn set_pattern(
        &mut self,
        pattern: String,
        emit: &mut Emitter,
    ) -> Result<SearchChanged, HErr> {
        if pattern == self.pattern.raw() {
            return Ok(SearchChanged::NotChanged);
        }

        self.pattern.set_pattern(pattern)?;
        emit.search_pattern_changed();

        Ok(SearchChanged::Changed)
    }

    pub(super) fn set_matches(&mut self, matches: Vec<Match>, emit: &mut Emitter) {
        self.matches = matches;
        self.index = None;

        emit.search_num_matches_changed();
        emit.search_index_changed();
    }

    pub(super) fn msg_matches(&self, msg_id: &MsgId, container: &Container) -> Option<bool> {
        let data = container.get_data(msg_id)?;
        Some(data.matches(&self.pattern))
    }

    pub(super) fn set_regex(
        &mut self,
        use_regex: bool,
        emit: &mut Emitter,
    ) -> Result<SearchChanged, HErr> {
        match (use_regex, self.is_regex()) {
            (true, false) => {
                self.pattern.regex_mode()?;
                emit.search_regex_changed();
            }
            (false, true) => {
                self.pattern.normal_mode()?;
                emit.search_regex_changed();
            }
            _ => {
                return Ok(SearchChanged::NotChanged);
            }
        }
        Ok(SearchChanged::Changed)
    }

    pub(super) fn num_matches(&self) -> usize {
        self.matches.len()
    }

    pub(super) fn clear_search(&mut self, emit: &mut Emitter) -> Result<(), HErr> {
        self.pattern.set_pattern("".into())?;
        self.matches = Vec::new();
        self.index = None;

        emit.search_index_changed();
        emit.search_pattern_changed();
        emit.search_regex_changed();
        emit.search_num_matches_changed();

        Ok(())
    }

    pub(super) fn initial_next_index(&self) -> usize {
        0
    }

    pub(super) fn initial_prev_index(&self) -> usize {
        self.num_matches().saturating_sub(1)
    }

    pub(super) fn current(&self) -> Option<Match> {
        let ix = self.index?;
        self.matches.get(ix).copied()
    }

    pub(super) fn next(&mut self) -> Option<Match> {
        if self.active.not() {
            return None;
        }

        match self.index {
            Some(index) => {
                let index = (index + 1) % self.matches.len();
                self.index.replace(index);

                self.matches.get(index).copied()
            }
            None => {
                let index = self.initial_next_index();
                self.index.replace(index);

                self.matches.get(index).copied()
            }
        }
    }

    pub(super) fn prev(&mut self) -> Option<Match> {
        if self.active.not() {
            return None;
        }

        match self.index {
            Some(index) => {
                let index = if index == 0 {
                    self.matches.len().saturating_sub(1)
                } else {
                    index - 1
                };
                self.index.replace(index);

                self.matches.get(index).copied()
            }
            None => {
                let index = self.initial_prev_index();
                self.index.replace(index);

                self.matches.get(index).copied()
            }
        }
    }

    pub(super) fn try_remove_match(
        &mut self,
        msg_id: &MsgId,
        container: &mut Container,
        emit: &mut Emitter,
        model: &mut List,
    ) -> Option<()> {
        if self.active.not() || self.msg_matches(msg_id, container)?.not() {
            return Some(());
        }

        let pos = self.matches.iter().position(|Match(mid)| mid == msg_id)?;
        self.matches.remove(pos);
        emit.search_num_matches_changed();

        if self.matches.is_empty() {
            self.index = None;
            emit.search_index_changed();
            return Some(());
        }

        if let Some(ix) = self.index {
            if (0..=ix).contains(&pos) {
                let new_ix = ix.saturating_sub(1);
                self.index.replace(new_ix);
                emit.search_index_changed();

                let Match(mid) = self.matches.get(new_ix)?;
                let data = container.get_data_mut(mid)?;
                data.match_status = MatchStatus::Focused;

                let container_ix = container.index_of(*mid)?;

                model.data_changed(container_ix, container_ix);
            }
        }

        Some(())
    }
}
