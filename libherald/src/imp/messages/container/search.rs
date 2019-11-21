use super::*;

impl Container {
    pub(in crate::imp::messages) fn apply_search(
        &mut self,
        search: &SearchState,
        model: &mut List,
        emit: &mut Emitter,
    ) -> Option<Vec<Match>> {
        let pattern = search.pattern.as_ref()?;

        if search.active.not() || pattern.raw().is_empty() {
            return None;
        }

        let mut matches: Vec<Match> = Vec::new();

        for (ix, msg) in self.list.iter().enumerate() {
            let data = self.map.get_mut(&msg.msg_id)?;

            let old_status = data.match_status;
            let matched = data.matches(pattern);

            data.match_status = if matched {
                MatchStatus::Matched
            } else {
                MatchStatus::NotMatched
            };

            if old_status != data.match_status {
                model.data_changed(ix, ix);
            }

            if !matched {
                continue;
            };

            matches.push(Match(*msg))
        }

        emit.search_num_matches_changed();

        Some(matches)
    }

    pub(in crate::imp::messages) fn clear_search(
        &mut self,
        model: &mut List,
    ) -> Option<()> {
        for (ix, Message { msg_id, .. }) in self.list.iter().enumerate() {
            let data = self.map.get_mut(&msg_id)?;

            if data.match_status.is_match() {
                data.match_status = MatchStatus::NotMatched;
                model.data_changed(ix, ix);
            }
        }

        Some(())
    }
}
