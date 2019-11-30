use super::*;
use crate::messages::search::highlight_message;

use std::ops::Not;

pub(in crate::messages) fn apply_search(
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

        data.search_buf = if data.match_status.is_match() {
            Some(highlight_message(
                search.pattern.as_ref()?,
                data.body.as_ref()?,
            ))
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

pub(in crate::messages) fn clear_search(
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
