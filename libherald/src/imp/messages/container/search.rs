use super::*;
use search_pattern::Captures;

use std::ops::Not;

pub(in crate::imp::messages) fn apply_search(
    container: &mut Container,
    search: &SearchState,
    model: &mut List,
    emit: &mut Emitter,
) -> Option<Vec<Match>> {
    let pattern = search.pattern.as_ref()?;

    if search.active.not() || pattern.raw().is_empty() {
        return None;
    }

    let mut matches: Vec<Match> = Vec::new();

    for (ix, msg) in container.list.iter().enumerate() {
        let data = container.map.get_mut(&msg.msg_id)?;
        let matched = data.matches(pattern);

        data.match_status = if matched {
            MatchStatus::Matched
        } else {
            MatchStatus::NotMatched
        };

        let start_tag = "<span style = \"background-color: #F0C80C\">";
        let end_tag = "</span>";

        let replace_pattern = search.pattern.as_ref()?.replace_all(
            data.body.as_ref()?.as_str(),
            |caps: &Captures| {
                format!(
                    "{}{}{}",
                    start_tag,
                    caps.get(0).map(|m| m.as_str()).unwrap_or(""),
                    end_tag
                )
            },
        );

        data.search_buf = if data.match_status.is_match() {
            Some(replace_pattern.to_string())
        } else {
            None
        };

        model.data_changed(ix, ix);

        if !matched {
            continue;
        };

        matches.push(Match(*msg))
    }

    emit.search_num_matches_changed();

    Some(matches)
}

pub(in crate::imp::messages) fn clear_search(
    container: &mut Container,
    model: &mut List,
) -> Option<()> {
    for (ix, Message { msg_id, .. }) in container.list.iter().enumerate() {
        let data = container.map.get_mut(&msg_id)?;

        if data.match_status.is_match() {
            data.match_status = MatchStatus::NotMatched;
            model.data_changed(ix, ix);
        }
    }

    Some(())
}
