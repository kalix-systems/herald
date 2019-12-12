use super::*;
use conversation::settings::SettingsUpdate;

impl Conversations {
    pub(crate) fn handle_update(
        &mut self,
        update: ConvUpdate,
    ) {
        use ConvUpdate::*;

        match update {
            NewConversation(inner) => self.handle_new_conversation(inner),
            BuilderFinished(inner) => self.handle_builder_finished(inner),
            NewActivity(cid) => self.handle_new_activity(cid),
            Settings(cid, update) => none!(self.handle_settings_update(cid, update)),
            Init(contents) => self.handle_init(contents),
        }
    }

    pub(super) fn handle_settings_update(
        &mut self,
        cid: ConversationId,
        update: SettingsUpdate,
    ) -> Option<()> {
        let pos = self
            .list
            .iter()
            .position(|Conversation { id, .. }| id == &cid)?;

        match update {
            SettingsUpdate::Expiration(period) => {
                self.set_expiration_inner(pos, period)?;
            }
            SettingsUpdate::Color(color) => {
                self.set_color_inner(pos, color)?;
            }
            SettingsUpdate::Title(title) => {
                self.set_title_inner(pos, title)?;
            }
        }

        self.model.data_changed(pos, pos);

        Some(())
    }

    pub(super) fn handle_init(
        &mut self,
        contents: Vector<Conversation>,
    ) {
        self.model.begin_reset_model();
        self.list = contents;
        self.loaded = true;
        self.model.end_reset_model();
    }

    pub(super) fn handle_new_activity(
        &mut self,
        cid: ConversationId,
    ) {
        let pos = match self
            .list
            .iter()
            .position(|Conversation { id, .. }| id == &cid)
        {
            Some(pos) => pos,
            None => return,
        };

        // FIXME: If this check isn't here,
        // the program will segfault.
        if pos == 0 {
            return;
        }

        self.model.begin_move_rows(pos, pos, 0);
        let conv = self.list.remove(pos);
        self.list.push_front(conv);
        self.model.end_move_rows();
    }

    pub(super) fn handle_builder_finished(
        &mut self,
        inner: ConversationMeta,
    ) {
        self.insert_new_conversation(inner)
    }

    pub(super) fn handle_new_conversation(
        &mut self,
        inner: ConversationMeta,
    ) {
        self.insert_new_conversation(inner)
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
