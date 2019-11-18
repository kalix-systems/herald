use super::*;
use std::ops::Drop;

impl Messages {
    pub(super) fn emit_last_changed(&mut self) {
        self.emit.last_author_changed();
        self.emit.last_body_changed();
        self.emit.last_epoch_timestamp_ms_changed();
        self.emit.last_status_changed();
    }

    pub(super) fn prev_match_helper(&mut self) -> Option<usize> {
        let old = (self.search.current(), self.search.index);

        let new = (self.search.prev(), self.search.index);

        self.match_helper(old, new)
    }

    pub(super) fn next_match_helper(&mut self) -> Option<usize> {
        let old = (self.search.current(), self.search.index);

        let new = (self.search.next(), self.search.index);

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
            let Match(mid) = new?;
            return self.container.index_of(mid);
        }

        if let Some(Match(old)) = old {
            if let Some(data) = self.container.get_data_mut(&old) {
                data.match_status = MatchStatus::Matched;
                let ix = self.container.index_of(old)?;
                self.model.data_changed(ix, ix);
            }
        }

        let Match(mid) = new?;

        let data = self.container.get_data_mut(&mid)?;
        data.match_status = MatchStatus::Focused;

        let ix = self.container.index_of(mid)?;
        self.model.data_changed(ix, ix);
        Some(ix)
    }

    pub(super) fn raw_remove(&mut self, msg_id: MsgId, ix: usize) {
        self.search.try_remove_match(
            &msg_id,
            &mut self.container,
            &mut self.emit,
            &mut self.model,
        );

        let len = self.container.len();

        let prev_state = if ix > 0 {
            (self.is_tail(ix - 1), self.is_head(ix - 1))
        } else {
            (None, None)
        };

        let succ_state = (self.is_tail(ix), self.is_head(ix));

        self.model.begin_remove_rows(ix, ix);
        self.container.mem_remove(ix);
        self.model.end_remove_rows();

        if ix > 0 {
            let prev_head = self.is_head(ix - 1);

            if prev_state != (prev_head, self.is_tail(ix - 1)) {
                self.model.data_changed(ix - 1, ix - 1);
            }

            if ix + 1 < self.container.len() && succ_state != (prev_head, self.is_tail(ix + 1)) {
                self.model.data_changed(ix + 1, ix + 1);
            }
        }

        if len == 1 {
            self.emit.is_empty_changed();
        }

        if ix + 1 == len {
            self.emit_last_changed();
        }
    }

    pub(super) fn raw_insert(&mut self, msg: Msg, save_status: SaveStatus) -> Result<(), HErr> {
        let (message, data) = Message::split_msg(msg, save_status);

        let cid = self.conversation_id.ok_or(NE!())?;

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
                    eprintln!(
                        "WARNING: tried to insert duplicate message at {file}:{line}:{col}",
                        file = file!(),
                        line = line!(),
                        col = column!()
                    );
                    return Ok(());
                }
                Err(ix) => ix,
            }
        };

        let prev_state = if ix > 0 { self.is_tail(ix - 1) } else { None };

        let succ_state = self.is_tail(ix);

        self.model.begin_insert_rows(ix, ix);
        self.container.insert(ix, message, data);
        self.model.end_insert_rows();

        if ix + 1 == self.container.len() {
            self.emit_last_changed();
        }

        if self.container.len() == 1 {
            self.emit.is_empty_changed();
        }

        if ix > 0 && prev_state != self.is_tail(ix - 1) {
            self.model.data_changed(ix - 1, ix - 1);
        }

        if ix + 1 < self.container.len() && succ_state != self.is_tail(ix + 1) {
            self.model.data_changed(ix + 1, ix + 1);
        }

        use crate::imp::conversations::{shared::*, Conversations};
        Conversations::push(ConvUpdate::NewActivity(cid))?;

        Ok(())
    }

    pub(super) fn handle_expiration(&mut self, mids: Vec<MsgId>) {
        for mid in mids {
            if let Some(ix) = self.container.index_of(mid) {
                self.raw_remove(mid, ix);
            }
        }
    }
}

impl Drop for Messages {
    fn drop(&mut self) {
        if let Some(cid) = self.conversation_id {
            EMITTERS.remove(&cid);
            TXS.remove(&cid);
            RXS.remove(&cid);
        }
    }
}
