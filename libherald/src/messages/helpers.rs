use super::*;
use crate::push;
use crate::{content_push, spawn};
use heraldcore::{errors::HErr, message::Message as Msg, NE};
use messages_helper::search::Match;
pub use messages_helper::{container::*, types::*};

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

        self.model.begin_remove_rows(ix, ix);
        let data = self.container.remove(ix);
        self.model.end_remove_rows();

        if let Some(MsgData { replies, .. }) = data {
            let model = &mut self.model;
            self.container
                .set_dangling(replies, |ix| model.data_changed(ix, ix));
        }

        if ix > 0 {
            self.entry_changed(ix - 1);
        }

        if ix + 1 < self.container.len() {
            self.entry_changed(ix + 1);
        }

        if len == 1 {
            self.emit.is_empty_changed();
        }

        if ix == 0 {
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

        let ix = self.container.insert_ord(message, data);
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

        if ix == 0 {
            self.emit_last_changed();
        }

        if self.container.len() == 1 {
            self.emit.is_empty_changed();
        }

        if ix > 0 {
            self.entry_changed(ix - 1);
        }

        if ix + 1 < self.container.len() {
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

        spawn!({
            let list: Vec<MessageMeta> = err!(conversation::conversation_message_meta(&id));

            let last = match list.last().as_ref() {
                Some(MessageMeta { ref msg_id, .. }) => {
                    let msg = err!(heraldcore::message::get_message(msg_id));
                    Some(heraldcore::message::split_msg(msg).1)
                }
                None => None,
            };

            err!(content_push(
                id,
                MsgUpdate::Container(Box::new(Container::new(list, last)))
            ));
        });
    }
}
