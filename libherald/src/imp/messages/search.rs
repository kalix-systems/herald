use super::*;

#[derive(PartialEq)]
pub(super) enum SearchChanged {
    Changed,
    NotChanged,
}

#[derive(Clone, Copy)]
pub(super) struct Match {
    pub(super) ix: usize,
}

pub(super) struct SearchMachine {
    pub(super) pattern: SearchPattern,
    pub(super) active: bool,
    pub(super) cursor: Option<usize>,
    pub(super) matches: Vec<Match>,
}

impl SearchMachine {
    pub(super) fn new() -> Self {
        Self {
            pattern: abort_err!(SearchPattern::new_normal("".into())),
            active: false,
            cursor: None,
            matches: Vec::new(),
        }
    }

    pub(super) fn is_regex(&self) -> bool {
        match self.pattern {
            SearchPattern::Normal { .. } => false,
            SearchPattern::Regex { .. } => true,
        }
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
        self.cursor = None;

        self.matches = Vec::new();

        emit.search_active_changed();
        emit.search_num_matches_changed();
    }

    fn init_cursor(&self) -> Option<usize> {
        Some(self.matches.last()?.ix)
    }

    pub(super) fn next(&mut self) -> Option<&Match> {
        let init_cursor = self.init_cursor()?;
        self.cursor = Some(self.cursor.unwrap_or(init_cursor));

        self.cursor = if self.cursor.unwrap_or(init_cursor) == self.matches.len().saturating_sub(1)
        {
            Some(0)
        } else {
            Some(self.cursor?.saturating_add(1))
        };

        self.matches.get(self.cursor?)
    }

    pub(super) fn prev(&mut self) -> Option<&Match> {
        let init_cursor = self.init_cursor()?;
        self.cursor = Some(self.cursor.unwrap_or(init_cursor));

        self.cursor = if self.cursor.unwrap_or(init_cursor) == 0 {
            Some(self.matches.len().saturating_sub(1))
        } else {
            Some(self.cursor?.saturating_sub(1))
        };

        self.matches.get(self.cursor?)
    }
}
