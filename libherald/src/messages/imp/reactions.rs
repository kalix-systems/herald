use super::*;

impl Messages {
    pub(crate) fn add_reaction_(
        &mut self,
        index: u64,
        content: String,
    ) {
        let index = index as usize;
        let local_id = none!(self.local_id);
        none!(self.container.update_by_index(index, |data| {
            if data.reactions.is_none() {
                data.reactions = Default::default();
            }

            if let Some(ref mut r) = data.reactions {
                r.add(content, local_id);
            }
        }));

        self.model.data_changed(index, index);
    }

    pub(crate) fn remove_reaction_(
        &mut self,
        index: u64,
        content: String,
    ) {
        let index = index as usize;
        let local_id = none!(self.local_id);

        none!(self.container.update_by_index(index, |data| {
            if data.reactions.is_none() {
                data.reactions = Default::default();
            }

            if let Some(ref mut r) = data.reactions {
                r.remove(content, local_id);
            }
        }));

        self.model.data_changed(index, index);
    }

    pub(crate) fn reactions_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container
            .access_by_index(index, |data| data.reactions.clone())
            .map(json::JsonValue::from)
            .as_ref()
            .map(json::JsonValue::dump)
    }
}
