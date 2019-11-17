use super::*;
use conversation::settings::SettingsUpdate;

impl Conversations {
    pub(super) fn handle_settings_update(
        &mut self,
        cid: ConversationId,
        update: SettingsUpdate,
    ) -> Option<()> {
        let pos = self
            .list
            .iter()
            .position(|c| c.inner.conversation_id == cid)?;

        match update {
            SettingsUpdate::Expiration(period) => {
                self.list.get_mut(pos)?.inner.expiration_period = period;
            }
            SettingsUpdate::Color(color) => {
                self.list.get_mut(pos)?.inner.color = color;
            }
            SettingsUpdate::Title(title) => {
                self.list.get_mut(pos)?.inner.title = title;
            }
        }

        self.model.data_changed(pos, pos);

        Some(())
    }

    pub(super) fn handle_init(&mut self, contents: Vector<Conversation>) {
        self.model.begin_reset_model();
        self.list = contents;
        self.model.end_reset_model();
    }

    pub(super) fn handle_new_activity(&mut self, cid: ConversationId) {
        let pos = match self
            .list
            .iter()
            .position(|c| c.inner.conversation_id == cid)
        {
            Some(pos) => pos,
            None => return,
        };

        // NOTE: This is very important. If this check isn't here,
        // the program will crash.
        if pos == 0 {
            return;
        }

        self.model.begin_move_rows(pos, pos, 0);
        let conv = self.list.remove(pos);
        self.list.push_front(conv);
        self.model.end_move_rows();
    }

    pub(super) fn handle_builder_finished(&mut self, inner: ConversationMeta) {
        self.insert_new_conversation(inner)
    }

    pub(super) fn handle_new_conversation(&mut self, inner: ConversationMeta) {
        self.insert_new_conversation(inner)
    }

    fn insert_new_conversation(&mut self, inner: ConversationMeta) {
        let matched = match self.filter.as_ref() {
            Some(filter) => inner.matches(filter),
            None => true,
        };

        let conv = Conversation { matched, inner };

        self.model.begin_insert_rows(0, 0);
        self.list.push_front(conv);
        self.model.end_insert_rows();
    }
}
