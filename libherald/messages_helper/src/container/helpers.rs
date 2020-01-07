use super::*;

impl Container {
    pub fn insert_helper<
        E: MessageEmit,
        M: MessageModel,
        P: FnOnce(heraldcore::types::ConversationId),
    >(
        &mut self,
        msg: Message,
        emit: &mut E,
        model: &mut M,
        search: &mut SearchState,
        cid: heraldcore::types::ConversationId,
        push: P,
    ) {
        let (message, data) = msg.split();

        let msg_id = message.msg_id;

        let ix = self.insert_ord(message, data);
        model.begin_insert_rows(ix, ix);
        model.end_insert_rows();

        search.try_insert_match(msg_id, ix, self, emit, model);

        if ix == 0 {
            emit.last_changed(cid, Some(msg_id));
        } else {
            model.entry_changed(ix - 1);
        }

        if ix + 1 < self.len() {
            model.entry_changed(ix + 1);
        }

        push(cid);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn remove_helper<E: MessageEmit, M: MessageModel, B: MessageBuilderHelper>(
        &mut self,
        msg_id: MsgId,
        ix: usize,
        emit: &mut E,
        model: &mut M,
        search: &mut SearchState,
        builder: &mut B,
        cid: ConversationId,
    ) {
        {
            search.try_remove_match(&msg_id, self, emit, model);
        }

        builder.try_clear_reply(&msg_id);

        let old_len = self.len();

        model.begin_remove_rows(ix, ix);
        let data = self.remove(ix);
        model.end_remove_rows();

        if let Some(MsgData { replies, .. }) = data {
            self.set_dangling(replies, model);
        }

        if ix > 0 {
            model.entry_changed(ix - 1);
        }

        if ix + 1 < self.len() {
            model.entry_changed(ix + 1);
        }

        if old_len == 1 || ix == 0 {
            emit.last_changed(cid, self.last().map(|m| m.msg_id));
        }
    }
}
