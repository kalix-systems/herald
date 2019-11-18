use super::*;
use std::collections::VecDeque;
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

#[derive(Clone, Copy)]
pub(super) struct Match {
    pub(super) mid: MsgId,
}

pub(super) struct SearchMachine {
    pub(super) pattern: SearchPattern,
    pub(super) active: bool,
    pub(super) matches: VecDeque<Match>,
}

impl SearchMachine {
    pub(super) fn new() -> Self {
        Self {
            pattern: abort_err!(SearchPattern::new_normal("".into())),
            active: false,
            matches: VecDeque::new(),
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

        self.pattern = if self.is_regex() {
            SearchPattern::new_regex(pattern)?
        } else {
            SearchPattern::new_normal(pattern)?
        };

        emit.search_pattern_changed();

        Ok(SearchChanged::Changed)
    }

    pub(super) fn set_regex(&mut self, use_regex: bool) -> Result<SearchChanged, HErr> {
        match (use_regex, self.is_regex()) {
            (true, false) => {
                self.pattern.regex_mode()?;
            }
            (false, true) => {
                self.pattern.normal_mode()?;
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

    pub(super) fn clear_search(&mut self, emit: &mut Emitter) {
        self.active = false;
        self.pattern = abort_err!(SearchPattern::new_normal("".into()));
        self.matches = VecDeque::new();

        emit.search_active_changed();
        emit.search_num_matches_changed();
    }

    pub(super) fn peek_next(&mut self, container: &Container) -> Option<Match> {
        if self.active.not() {
            return None;
        }

        let peek = loop {
            let match_val = *self.matches.front()?;
            if container.contains(&match_val.mid) {
                break match_val;
            } else {
                self.matches.pop_front()?;
            }
        };

        Some(peek)
    }

    pub(super) fn next(&mut self, container: &Container) -> Option<Match> {
        if self.active.not() {
            return None;
        }

        let next = loop {
            let match_val = self.matches.pop_front()?;
            if container.contains(&match_val.mid) {
                break match_val;
            }
        };

        self.matches.push_back(next);
        Some(next)
    }

    pub(super) fn next_would_loop(&mut self, container: &Container) -> bool {
        self.next_would_loop_helper(container).unwrap_or(true)
    }

    fn next_would_loop_helper(&mut self, container: &Container) -> Option<bool> {
        if self.active.not() {
            return None;
        }

        let (next, cursor) = (self.peek_next(container)?, self.peek_prev(container)?);

        Some(
            container.get_data(&next.mid)?.time.insertion
                < container.get_data(&cursor.mid)?.time.insertion,
        )
    }

    pub(super) fn peek_prev(&mut self, container: &Container) -> Option<Match> {
        if self.active.not() {
            return None;
        }

        let peek = loop {
            let match_val = *self.matches.back()?;
            if container.contains(&match_val.mid) {
                break match_val;
            } else {
                self.matches.pop_back()?;
            }
        };

        Some(peek)
    }

    pub(super) fn prev(&mut self, container: &Container) -> Option<Match> {
        if self.active.not() {
            return None;
        }

        let prev = loop {
            let match_val = self.matches.pop_back()?;
            if container.contains(&match_val.mid) {
                break match_val;
            }
        };

        self.matches.push_front(prev);

        Some(prev)
    }

    pub(super) fn prev_would_loop(&mut self, container: &Container) -> bool {
        self.prev_would_loop_helper(container).unwrap_or(true)
    }

    fn prev_would_loop_helper(&mut self, container: &Container) -> Option<bool> {
        if self.active.not() {
            return None;
        }

        let (prev, cursor) = (self.peek_prev(container)?, self.peek_next(container)?);

        Some(
            container.get_data(&prev.mid)?.time.insertion
                < container.get_data(&cursor.mid)?.time.insertion,
        )
    }
}
