use super::*;
use std::ops::Not;

mod search_helper;
pub(crate) use search_helper::highlight_message;

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

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub(super) struct Match(pub(super) Message);

pub(super) struct SearchState {
    pub(super) pattern: Option<SearchPattern>,
    pub(super) active: bool,
    matches: Vec<Match>,
    start_index: Option<usize>,
    pub(super) index: Option<usize>,
}

impl SearchState {
    pub(super) fn new() -> Self {
        Self {
            pattern: SearchPattern::new_normal("".into()).ok(),
            active: false,
            matches: Vec::new(),
            start_index: None,
            index: None,
        }
    }

    pub(super) fn is_regex(&self) -> bool {
        match self.pattern {
            Some(SearchPattern::Regex { .. }) => true,
            _ => false,
        }
    }

    pub(super) fn start_hint(
        &mut self,
        hint: f32,
        container: &Container,
    ) -> Option<()> {
        let approx_index = (container.len() as f64 * hint as f64).ceil() as usize;

        let closest_message = container.get(approx_index)?;

        let index = match self.matches.binary_search(&Match(*closest_message)) {
            Ok(ix) => ix,
            Err(ix) => ix.saturating_sub(1),
        };

        self.start_index.replace(index);

        Some(())
    }

    pub(super) fn set_pattern(
        &mut self,
        pattern: String,
        emit: &mut Emitter,
    ) -> Result<SearchChanged, HErr> {
        match self.set_pattern_inner(pattern)? {
            SearchChanged::NotChanged => Ok(SearchChanged::NotChanged),
            SearchChanged::Changed => {
                emit.search_pattern_changed();
                Ok(SearchChanged::Changed)
            }
        }
    }

    pub(super) fn set_matches(
        &mut self,
        matches: Vec<Match>,
        emit: &mut Emitter,
    ) {
        self.set_matches_inner(matches);
        emit.search_num_matches_changed();
        emit.search_index_changed();
    }

    pub(super) fn msg_matches(
        &self,
        msg_id: &MsgId,
        container: &Container,
    ) -> Option<bool> {
        let data = container.get_data(msg_id)?;
        Some(data.matches(self.pattern.as_ref()?))
    }

    pub(super) fn set_regex(
        &mut self,
        use_regex: bool,
        emit: &mut Emitter,
    ) -> Result<SearchChanged, HErr> {
        match self.set_regex_inner(use_regex)? {
            SearchChanged::NotChanged => Ok(SearchChanged::NotChanged),
            SearchChanged::Changed => {
                emit.search_regex_changed();
                Ok(SearchChanged::Changed)
            }
        }
    }

    pub(super) fn num_matches(&self) -> usize {
        self.matches.len()
    }

    pub(super) fn clear_search(
        &mut self,
        emit: &mut Emitter,
    ) -> Result<(), HErr> {
        self.clear_search_inner()?;

        emit.search_index_changed();
        emit.search_pattern_changed();
        emit.search_regex_changed();
        emit.search_num_matches_changed();

        Ok(())
    }

    pub(super) fn initial_next_index(&self) -> usize {
        self.start_index.unwrap_or(0)
    }

    pub(super) fn initial_prev_index(&self) -> usize {
        self.start_index
            .unwrap_or_else(|| self.num_matches().saturating_sub(1))
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

        let pos = self
            .matches
            .iter()
            .position(|Match(Message { msg_id: mid, .. })| mid == msg_id)?;

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

                let Match(msg) = self.matches.get(new_ix)?;
                let data = container.get_data_mut(&msg.msg_id)?;
                data.match_status = MatchStatus::Focused;

                let container_ix = container.index_of(msg)?;

                model.data_changed(container_ix, container_ix);
            }
        }

        Some(())
    }

    pub(super) fn try_insert_match(
        &mut self,
        msg_id: MsgId,
        model_ix: usize,
        container: &mut Container,
        emit: &mut Emitter,
        model: &mut List,
    ) -> Option<()> {
        if self.active.not() || self.msg_matches(&msg_id, container)?.not() {
            return Some(());
        }

        let message = from_msg_id(msg_id, &container)?;
        let ix = self.index;

        let pos = if self
            .matches
            .last()
            .map(|Match(last)| last.insertion_time)
            .unwrap_or(message.insertion_time)
            <= message.insertion_time
        {
            self.matches.len()
        } else {
            match container.binary_search(&message) {
                Ok(_) => {
                    return Some(());
                }
                Err(ix) => ix,
            }
        };

        self.matches.insert(pos, Match(message));
        let data = container.get_data_mut(&msg_id)?;
        data.match_status = MatchStatus::Matched;
        data.search_buf = Some(highlight_message(
            self.pattern.as_ref()?,
            data.body.as_ref()?,
        ));
        model.data_changed(pos, pos);
        emit.search_num_matches_changed();

        if let Some(ix) = ix {
            if (0..=ix).contains(&pos) {
                self.index.replace((ix + 1) % self.matches.len());
                model.data_changed(model_ix, model_ix);
            }
        }

        Some(())
    }
}
