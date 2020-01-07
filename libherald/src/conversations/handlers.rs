use super::*;

impl Conversations {
    pub(crate) fn handle_update(
        &mut self,
        update: ConvUpdate,
    ) {
        use ConvUpdate::*;

        match update {
            Global(update) => self.handle_global_update(update),
            Item(ConvItemUpdate { cid, variant }) => {
                self.handle_item_update(cid, variant);
            }
        }
    }

    fn handle_item_update(
        &mut self,
        cid: ConversationId,
        variant: ConvItemUpdateVariant,
    ) -> Option<()> {
        use ConvItemUpdateVariant::*;

        let pos = self
            .list
            .iter()
            .position(|Conversation { id, .. }| id == &cid)?;

        match variant {
            PictureChanged(path) => {
                self.set_picture_inner(pos, path);
                self.model.data_changed(pos, pos);
            }
            TitleChanged(title) => {
                self.set_title_inner(pos, title);
                self.model.data_changed(pos, pos);
            }
            ExpirationChanged(period) => {
                self.set_expiration_inner(pos, period)?;
                self.model.data_changed(pos, pos);
            }
            NewActivity => {
                self.set_status_inner(pos, heraldcore::conversation::Status::Active);
                self.model.data_changed(pos, pos);

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
            LastChanged => {
                // update last message
                self.model.data_changed(pos, pos);
            }
        }

        Some(())
    }

    fn handle_global_update(
        &mut self,
        update: GlobalConvUpdate,
    ) {
        use GlobalConvUpdate::*;
        match update {
            Init(contents) => {
                self.model.begin_reset_model();
                self.list = contents;
                self.loaded = true;
                self.model.end_reset_model();
            }
            BuilderFinished(meta) => self.insert_new_conversation(meta),
            NewConversation(meta) => self.insert_new_conversation(meta),
        }
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
