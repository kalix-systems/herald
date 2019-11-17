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

#[derive(Clone, Copy, PartialEq)]
pub(super) struct Cursor(pub(super) MsgId);

impl Cursor {
    fn msg_id(&self) -> &MsgId {
        &self.0
    }

    pub(super) fn into_inner(self) -> MsgId {
        self.0
    }
}

pub(super) struct Match(pub(super) MsgId);

pub(super) struct SearchState {
    pub(super) pattern: SearchPattern,
    pub(super) active: bool,
    pub(super) matches: VecDeque<Match>,
    pub(super) cur: Option<Cursor>,
    pub(super) index: Option<usize>,
}

impl SearchState {
    pub(super) fn new() -> Self {
        Self {
            pattern: abort_err!(SearchPattern::new_normal("".into())),
            active: false,
            matches: VecDeque::new(),
            cur: None,
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
        match self.cur {
            Some(_) => self.matches.len() + 1,
            None => self.matches.len(),
        }
    }

    pub(super) fn clear_search(&mut self, emit: &mut Emitter) -> Result<(), HErr> {
        self.active = false;
        self.pattern = SearchPattern::new_normal("".into())?;
        self.matches = VecDeque::new();
        self.cur = None;
        self.index = None;

        emit.search_index_changed();
        emit.search_pattern_changed();
        emit.search_regex_changed();
        emit.search_active_changed();
        emit.search_num_matches_changed();

        Ok(())
    }

    pub(super) fn initial_next_index(&self) -> usize {
        1
    }

    pub(super) fn initial_prev_index(&self) -> usize {
        self.num_matches()
    }

    pub(super) fn increment_index(&mut self) {
        let num_matches = self.num_matches();

        self.index = match self.index {
            Some(ix) => Some((ix + 1) % num_matches),
            None => Some(self.initial_next_index()),
        };
    }

    pub(super) fn decrement_index(&mut self) {
        let num_matches = self.num_matches();

        self.index = match self.index {
            Some(ix) => Some(if ix > 0 { (ix - 1) % num_matches } else { 0 }),
            None => Some(self.initial_prev_index()),
        };
    }

    pub(super) fn next(&mut self, container: &Container) -> Option<Cursor> {
        if self.active.not() {
            return None;
        }

        let next = loop {
            let cur = match (self.matches.pop_front(), self.cur) {
                (Some(Match(msg_id)), Some(cursor)) => {
                    self.matches.push_back(Match(*cursor.msg_id()));
                    Some(Cursor(msg_id))
                }
                (Some(Match(msg_id)), None) => Some(Cursor(msg_id)),
                (None, cur @ Some(Cursor(_))) => cur,
                (None, None) => None,
            };

            let cur = match cur {
                Some(cur) => cur,
                None => {
                    self.cur = None;
                    self.index = None;
                    return None;
                }
            };

            // check if item is still valid
            if container.contains(cur.msg_id()) {
                self.increment_index();
                self.cur = Some(cur);
                break cur;
            }
        };

        Some(next)
    }

    pub(super) fn prev(&mut self, container: &Container) -> Option<Cursor> {
        if self.active.not() {
            return None;
        }

        let prev = loop {
            let cur = match (self.matches.pop_back(), self.cur) {
                (Some(Match(msg_id)), Some(cursor)) => {
                    self.matches.push_front(Match(*cursor.msg_id()));
                    Some(Cursor(msg_id))
                }
                (Some(Match(msg_id)), None) => Some(Cursor(msg_id)),
                (None, cur @ Some(Cursor(_))) => cur,
                (None, None) => None,
            };

            let cur = match cur {
                Some(cur) => cur,
                None => {
                    self.cur = None;
                    self.index = None;
                    return None;
                }
            };

            // check if item is still valid
            if container.contains(cur.msg_id()) {
                self.decrement_index();
                self.cur = Some(cur);
                break cur;
            }
        };

        Some(prev)
    }
}
