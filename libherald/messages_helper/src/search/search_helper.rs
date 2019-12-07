use super::*;
use search_pattern::Captures;

impl SearchState {
    pub(crate) fn set_pattern_inner(
        &mut self,
        pattern: String,
    ) -> Result<SearchChanged, SearchPatternError> {
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
    ) -> Result<SearchChanged, SearchPatternError> {
        match (use_regex, self.is_regex(), self.pattern.as_mut()) {
            (true, false, Some(pattern)) => {
                pattern.regex_mode()?;
                Ok(SearchChanged::Changed)
            }
            (false, true, Some(pattern)) => {
                pattern.normal_mode()?;
                Ok(SearchChanged::Changed)
            }
            _ => Ok(SearchChanged::NotChanged),
        }
    }

    pub(super) fn clear_search_inner(&mut self) -> Result<(), SearchPatternError> {
        if let Some(pattern) = self.pattern.as_mut() {
            pattern.set_pattern("".into())?;
        }

        self.matches = Vec::new();
        self.index = None;
        self.start_index = None;

        Ok(())
    }
}

pub(crate) fn highlight_message(
    search: &SearchPattern,
    body: &MessageBody,
) -> String {
    let start_tag = "<span style = \"background-color: #F0C80C\">";
    let end_tag = "</span>";

    let replace_pattern = search.replace_all(body.as_str(), |caps: &Captures| {
        format!(
            "{}{}{}",
            start_tag,
            caps.get(0).map(|m| m.as_str()).unwrap_or(""),
            end_tag
        )
    });

    replace_pattern.to_string()
}
