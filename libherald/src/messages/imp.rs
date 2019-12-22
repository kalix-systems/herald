use super::*;
use crate::interface::MessagesTrait as Interface;
use crate::push;
use heraldcore::{errors::HErr, message::Message as Msg, NE};
use messages_helper::search::Match;

impl Messages {
    pub(super) fn emit_last_changed(&mut self) {
        self.emit.last_author_changed();
        self.emit.last_body_changed();
        self.emit.last_time_changed();
        self.emit.last_status_changed();
    }

    pub(super) fn entry_changed(
        &mut self,
        ix: usize,
    ) {
        if ix < self.container.len() {
            self.model.data_changed(ix, ix);
        }
    }

    pub(super) fn prev_match_helper(&mut self) -> Option<usize> {
        let old = (self.search.current(), self.search.index);

        let new = (self.search.prev_match(), self.search.index);

        self.match_helper(old, new)
    }

    pub(super) fn next_match_helper(&mut self) -> Option<usize> {
        let old = (self.search.current(), self.search.index);

        let new = (self.search.next_match(), self.search.index);

        self.match_helper(old, new)
    }

    fn match_helper(
        &mut self,
        (old, old_index): (Option<Match>, Option<usize>),
        (new, new_index): (Option<Match>, Option<usize>),
    ) -> Option<usize> {
        if old_index != new_index {
            self.emit.search_index_changed();
        }

        if old == new {
            let Match(msg) = new?;
            return self.container.index_of(&msg);
        }

        if let Some(Match(old)) = old {
            let ix = self.container.index_of(&old)?;
            self.container.list.get_mut(ix)?.match_status = MatchStatus::Matched;
            self.entry_changed(ix);
        }

        let Match(new) = new?;

        let ix = self.container.index_of(&new)?;
        self.container.list.get_mut(ix)?.match_status = MatchStatus::Focused;

        self.entry_changed(ix);

        Some(ix)
    }

    pub(super) fn remove_helper(
        &mut self,
        msg_id: MsgId,
        ix: usize,
    ) {
        {
            let emit = &mut self.emit;
            let mut emit_num = emit.clone();
            let model = &mut self.model;

            self.search.try_remove_match(
                &msg_id,
                &mut self.container,
                || emit.search_index_changed(),
                || emit_num.search_num_matches_changed(),
                |ix| model.data_changed(ix, ix),
            );
        }

        self.builder.try_clear_reply(&msg_id);

        let len = self.container.len();

        let (was_tail, was_head) = (self.is_tail(ix), self.is_head(ix));

        self.model.begin_remove_rows(ix, ix);
        let data = self.container.remove(ix);
        self.model.end_remove_rows();

        if let Some(MsgData { replies, .. }) = data {
            let model = &mut self.model;
            self.container
                .set_dangling(replies, |ix| model.data_changed(ix, ix));
        }

        if was_tail.unwrap_or(false) && (ix > 0) {
            self.entry_changed(ix - 1);
        }

        if was_head.unwrap_or(false) && (ix < self.container.len()) {
            self.entry_changed(ix);
        }

        if len == 1 {
            self.emit.is_empty_changed();
        }

        if ix + 1 == len {
            self.emit_last_changed();
        }
    }

    pub(super) fn insert_helper(
        &mut self,
        msg: Msg,
    ) -> Result<(), HErr> {
        let (message, data) = split_msg(msg);

        let cid = self.conversation_id.ok_or(NE!())?;

        let msg_id = message.msg_id;
        let ix = if self
            .container
            .last()
            .map(|last| last.insertion_time)
            .unwrap_or(message.insertion_time)
            <= message.insertion_time
        {
            self.container.len()
        } else {
            match self.container.binary_search(&message) {
                Ok(_) => {
                    return Ok(());
                }
                Err(ix) => ix,
            }
        };

        let prev_state = if ix > 0 { self.is_tail(ix - 1) } else { None };

        let succ_state = self.is_tail(ix);

        self.container.insert(ix, message, data).ok_or(NE!())?;

        self.model.begin_insert_rows(ix, ix);
        self.model.end_insert_rows();

        {
            let emit = &mut self.emit;
            let model = &mut self.model;
            self.search.try_insert_match(
                msg_id,
                ix,
                &mut self.container,
                || emit.search_num_matches_changed(),
                |ix| model.data_changed(ix, ix),
            );
        }

        if ix + 1 == self.container.len() {
            self.emit_last_changed();
        }

        if self.container.len() == 1 {
            self.emit.is_empty_changed();
        }

        if ix > 0 && prev_state != self.is_tail(ix - 1) {
            self.entry_changed(ix - 1);
        }

        if ix + 1 < self.container.len() && succ_state != self.is_tail(ix + 1) {
            self.entry_changed(ix + 1);
        }

        use crate::conversations::shared::*;

        push(ConvUpdate::NewActivity(cid));

        Ok(())
    }

    pub(super) fn handle_expiration(
        &mut self,
        mids: Vec<MsgId>,
    ) {
        for mid in mids {
            if let Some(ix) = self.container.index_by_id(mid) {
                self.remove_helper(mid, ix);
            }
        }
    }

    pub(crate) fn set_conversation_id(
        &mut self,
        id: ConversationId,
    ) {
        self.conversation_id = Some(id);
        self.builder.set_conversation_id(id);

        container::fill(id);
    }
}
