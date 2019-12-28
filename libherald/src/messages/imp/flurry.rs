use super::*;

impl Messages {
    pub(crate) fn is_tail_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if index == 0 {
            return Some(true);
        }

        // other cases
        self.container
            .same_flurry(index, index - 1)
            .map(std::ops::Not::not)
    }

    pub(crate) fn is_head_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if index + 1 == self.container.len() {
            return Some(true);
        }

        // other cases
        self.container
            .same_flurry(index, index + 1)
            .map(std::ops::Not::not)
    }

    pub(crate) fn clear_conversation_history_(&mut self) -> bool {
        let id = none!(self.conversation_id, false);

        spawn!(conversation::delete_conversation(&id), false);

        self.clear_search();
        self.model
            .begin_remove_rows(0, self.container.len().saturating_sub(1));
        self.container = Default::default();
        self.model.end_remove_rows();

        self.emit_last_changed();
        self.emit.is_empty_changed();
        true
    }
}
