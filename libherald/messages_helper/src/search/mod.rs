use crate::*;
use container::Container;
use heraldcore::types::MsgId;
use search_pattern::{SearchPattern, SearchPatternError};
use std::ops::Not;
use types::*;

mod search_helper;
pub use search_helper::highlight_message;

#[derive(PartialEq, Debug)]
pub enum SearchChanged {
    Changed,
    NotChanged,
}

impl SearchChanged {
    pub fn changed(self) -> bool {
        self == SearchChanged::Changed
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Match(pub MessageMeta);

pub struct SearchState {
    pub pattern: Option<SearchPattern>,
    pub active: bool,
    matches: Vec<Match>,
    start_index: Option<usize>,
    pub index: Option<usize>,
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            pattern: SearchPattern::new_normal("".into()).ok(),
            active: false,
            matches: Vec::new(),
            start_index: None,
            index: None,
        }
    }

    pub fn is_regex(&self) -> bool {
        match self.pattern {
            Some(SearchPattern::Regex { .. }) => true,
            _ => false,
        }
    }

    pub fn start_hint(
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

    pub fn set_pattern<F: FnMut()>(
        &mut self,
        pattern: String,
        mut pattern_changed: F,
    ) -> Result<SearchChanged, SearchPatternError> {
        match self.set_pattern_inner(pattern)? {
            SearchChanged::NotChanged => Ok(SearchChanged::NotChanged),
            SearchChanged::Changed => {
                pattern_changed();
                Ok(SearchChanged::Changed)
            }
        }
    }

    pub fn set_matches<N: FnMut(), I: FnMut()>(
        &mut self,
        matches: Vec<Match>,
        mut num_matches_changed: N,
        mut index_changed: I,
    ) {
        self.set_matches_inner(matches);
        num_matches_changed();
        index_changed();
    }

    pub fn msg_matches(
        &self,
        msg_id: &MsgId,
    ) -> Option<bool> {
        let pattern = self.pattern.as_ref()?;
        crate::container::access(msg_id, |m| m.matches(pattern))
    }

    pub fn set_regex<F: FnMut()>(
        &mut self,
        use_regex: bool,
        mut regex_changed: F,
    ) -> Result<SearchChanged, SearchPatternError> {
        match self.set_regex_inner(use_regex)? {
            SearchChanged::NotChanged => Ok(SearchChanged::NotChanged),
            SearchChanged::Changed => {
                regex_changed();
                Ok(SearchChanged::Changed)
            }
        }
    }

    pub fn num_matches(&self) -> usize {
        self.matches.len()
    }

    pub fn clear_search<F: FnMut()>(
        &mut self,
        mut emit_cleared: F,
    ) -> Result<(), SearchPatternError> {
        self.clear_search_inner()?;

        emit_cleared();

        Ok(())
    }

    pub fn initial_prev_index(&self) -> usize {
        self.start_index.unwrap_or(1)
    }

    pub fn initial_next_index(&self) -> usize {
        self.start_index
            .unwrap_or_else(|| self.num_matches().saturating_sub(1))
    }

    pub fn current(&self) -> Option<Match> {
        let ix = self.index?;
        self.matches.get(ix).copied()
    }

    pub fn prev_match(&mut self) -> Option<Match> {
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

    pub fn next_match(&mut self) -> Option<Match> {
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

    pub fn try_remove_match<I: FnMut(), N: FnMut(), D: FnMut(usize)>(
        &mut self,
        msg_id: &MsgId,
        container: &mut Container,
        mut index_changed: I,
        mut num_matches_changed: N,
        mut data_changed: D,
    ) -> Option<()> {
        if self.active.not() || self.msg_matches(msg_id)?.not() {
            return Some(());
        }

        let pos = self
            .matches
            .iter()
            .position(|Match(MessageMeta { msg_id: mid, .. })| mid == msg_id)?;

        self.matches.remove(pos);
        num_matches_changed();

        if self.matches.is_empty() {
            self.index = None;
            index_changed();
            return Some(());
        }

        if let Some(ix) = self.index {
            if (0..=ix).contains(&pos) {
                let new_ix = ix.saturating_sub(1);
                self.index.replace(new_ix);

                index_changed();

                let Match(msg) = self.matches.get(new_ix)?;

                let container_ix = container.index_by_id(msg.msg_id)?;
                container.list.get_mut(container_ix)?.match_status = MatchStatus::Focused;

                data_changed(container_ix);
            }
        }

        Some(())
    }

    pub fn try_insert_match<N: FnMut(), D: FnMut(usize)>(
        &mut self,
        msg_id: MsgId,
        model_ix: usize,
        container: &mut Container,
        mut num_matches_changed: N,
        mut data_changed: D,
    ) -> Option<()> {
        // early return if search is inactive or message doesn't match
        if self.active.not() || self.msg_matches(&msg_id)?.not() {
            return Some(());
        }

        let message = from_msg_id(msg_id)?;
        let focus_ix = self.index;

        let match_pos = if self
            .matches
            .last()
            .map(|Match(last)| last.insertion_time)
            .unwrap_or(message.insertion_time)
            <= message.insertion_time
        {
            self.matches.len()
        } else {
            match self.matches.binary_search(&Match(message)) {
                Ok(_) => {
                    // early return if already matched
                    return Some(());
                }
                Err(ix) => ix,
            }
        };

        // insertion into matches
        self.matches.insert(match_pos, Match(message));
        num_matches_changed();

        // update match status
        container.list.get_mut(model_ix)?.match_status = MatchStatus::Matched;
        data_changed(model_ix);

        if let Some(focus_ix) = focus_ix {
            // if the match was in the first part, adjust the focus
            if (0..=focus_ix).contains(&match_pos) {
                let new_focus_ix = (focus_ix + 1) % self.matches.len();
                self.index.replace(new_focus_ix);

                let Match(MessageMeta { msg_id, .. }) = self.matches.get(new_focus_ix)?;
                let model_focused_ix = container.index_by_id(*msg_id)?;

                data_changed(model_focused_ix);
            }
        }

        Some(())
    }
}

#[cfg(test)]
mod tests;
