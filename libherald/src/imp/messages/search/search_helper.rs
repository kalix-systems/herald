use super::*;
impl SearchState {
    pub(crate) fn set_pattern_inner(
        &mut self,
        pattern: String,
    ) -> Result<SearchChanged, HErr> {
        match self.pattern.as_mut() {
            Some(old_pattern) => {
                if pattern == old_pattern.raw() {
                    return Ok(SearchChanged::NotChanged);
                }

                old_pattern.set_pattern(pattern)?;
                Ok(SearchChanged::Changed)
            }
            None => {
                self.pattern.replace(SearchPattern::new_normal(pattern)?);
                Ok(SearchChanged::Changed)
            }
        }
    }

    pub(crate) fn set_matches_inner(
        &mut self,
        matches: Vec<Match>,
    ) {
        self.matches = matches;
        self.index = None;
    }

    pub(crate) fn set_regex_inner(
        &mut self,
        use_regex: bool,
    ) -> Result<SearchChanged, HErr> {
        match (use_regex, self.is_regex(), self.pattern.as_mut()) {
            (true, false, Some(pattern)) => {
                pattern.regex_mode()?;
                Ok(SearchChanged::Changed)
            }
            (false, true, Some(pattern)) => {
                pattern.normal_mode()?;
                Ok(SearchChanged::Changed)
            }
            _ => {
                return Ok(SearchChanged::NotChanged);
            }
        }
    }

    pub(super) fn clear_search_inner(&mut self) -> Result<(), HErr> {
        if let Some(pattern) = self.pattern.as_mut() {
            pattern.set_pattern("".into())?;
        }

        self.matches = Vec::new();
        self.index = None;
        self.start_index = None;

        Ok(())
    }
}