use super::*;
use crate::interface::MessagesTrait as Interface;
use heraldcore::{errors::HErr, message::Message as Msg, NE};
use std::ops::Drop;

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
            let Match(msg) = new?;
            return self.container.index_of(&msg);
        }

        if let Some(Match(old)) = old {
            if let Some(data) = self.container.get_data_mut(&old.msg_id) {
                data.match_status = MatchStatus::Matched;
                let ix = self.container.index_of(&old)?;
                self.entry_changed(ix);
            }
        }

        let Match(new) = new?;

        let data = self.container.get_data_mut(&new.msg_id)?;
        data.match_status = MatchStatus::Focused;

        let ix = self.container.index_of(&new)?;
        self.entry_changed(ix);
        Some(ix)
    }

    pub(super) fn remove_helper(
        &mut self,
        msg_id: MsgId,
        ix: usize,
    ) {
        self.search.try_remove_match(
            &msg_id,
            &mut self.container,
            &mut self.emit,
            &mut self.model,
        );
        self.builder.try_clear_reply(&msg_id);

        let len = self.container.len();

        let prev_state = if ix > 0 {
            (self.is_tail(ix - 1), self.is_head(ix - 1))
        } else {
            (None, None)
        };

        let succ_state = (self.is_tail(ix), self.is_head(ix));

        self.model.begin_remove_rows(ix, ix);
        let data = self.container.remove(ix);
        self.model.end_remove_rows();

        if let Some(MsgData { replies, .. }) = data {
            container::set_dangling(&mut self.container, replies, &mut self.model);
        }

        if ix > 0 {
            let prev_head = self.is_head(ix - 1);

            if prev_state != (prev_head, self.is_tail(ix - 1)) {
                self.entry_changed(ix - 1);
            }

            if ix + 1 < self.container.len() && succ_state != (prev_head, self.is_tail(ix + 1)) {
                self.entry_changed(ix + 1);
            }
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
        save_status: SaveStatus,
    ) -> Result<(), HErr> {
        let (message, data) = split_msg(msg, save_status);

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

        self.search.try_insert_match(
            msg_id,
            ix,
            &mut self.container,
            &mut self.emit,
            &mut self.model,
        );

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

        use crate::imp::conversations::{shared::*, Conversations};
        Conversations::push(ConvUpdate::NewActivity(cid))?;

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
        EMITTERS.insert(id, self.emit().clone());
        // remove left over channel from previous session
        RXS.remove(&id);
        TXS.remove(&id);

        self.conversation_id = Some(id);
        self.builder.set_conversation_id(id);

        container::fill(id);
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
