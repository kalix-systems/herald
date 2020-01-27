use super::*;

impl Conversations {
    pub(crate) fn handle_update(
        &mut self,
        update: ConvUpdate,
    ) -> Option<()> {
        use ConvUpdate::*;
        match update {
            Init(contents) => {
                self.model.begin_reset_model();
                self.list = contents;
                self.loaded = true;
                self.model.end_reset_model();
            }
            BuilderFinished(meta) => {
                let cid = meta.conversation_id;
                self.insert_new_conversation(meta);
                self.builder_cid.replace(cid);
                self.emit.builder_conversation_id_changed();
            }
            NewConversation(meta) => self.insert_new_conversation(meta),
            NewActivity(cid) => {
                let pos = self
                    .list
                    .iter()
                    .position(|Conversation { id, .. }| id == &cid)?;

                // NOTE: If this check isn't here,
                // the program will segfault.
                if pos == 0 {
                    return Some(());
                }

                self.model.begin_move_rows(pos, pos, 0);
                let conv = self.list.remove(pos);
                self.list.push_front(conv);
                self.model.end_move_rows();
            }
        };

        Some(())
    }

    fn insert_new_conversation(
        &mut self,
        meta: ConversationMeta,
    ) {
        let matched = match self.filter.as_ref() {
            Some(filter) => meta.matches(filter),
            None => true,
        };

        let (mut conv, data) = split_meta(meta);
        conv.matched = matched;

        self.model.begin_insert_rows(0, 0);
        insert_data(conv.id, data);
        self.list.push_front(conv);
        self.model.end_insert_rows();
    }
}
