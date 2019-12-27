use super::*;

impl Messages {
    pub(crate) fn add_reaction_(
        &mut self,
        index: u64,
        content: String,
    ) {
        let index = index as usize;
        let local_id = none!(self.local_id);
        let cid = none!(self.conversation_id);

        let changed = none!(self.container.update_by_index(index, |data| {
            if data.reactions.is_none() {
                data.reactions.replace(Default::default());
            }

            if let Some(ref mut r) = data.reactions {
                return r.add((&content).clone(), local_id);
            }

            false
        }));

        if !changed {
            return;
        }

        self.model.data_changed(index, index);
        let mid = none!(self.container.msg_id(index).copied());

        spawn!({
            err!(heraldcore::message::add_reaction(&mid, &local_id, &content));
            err!(heraldcore::network::send_reaction(cid, mid, content));
        });
    }

    pub(crate) fn remove_reaction_(
        &mut self,
        index: u64,
        content: String,
    ) {
        let index = index as usize;
        let local_id = none!(self.local_id);
        let cid = none!(self.conversation_id);

        let changed = none!(self.container.update_by_index(index, |data| {
            if data.reactions.is_none() {
                data.reactions.replace(Default::default());
            }

            if let Some(ref mut r) = data.reactions {
                return r.remove((&content).clone(), local_id);
            }

            false
        }));

        if !changed {
            return;
        }
        self.model.data_changed(index, index);

        let mid = none!(self.container.msg_id(index).copied());

        spawn!({
            err!(heraldcore::message::remove_reaction(
                &mid, &local_id, &content
            ));
            err!(heraldcore::network::send_reaction(cid, mid, content));
        });
    }

    pub(crate) fn reactions_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container
            .access_by_index(index, |data| data.reactions.clone())
            .flatten()
            .map(json::JsonValue::from)
            .as_ref()
            .map(json::JsonValue::dump)
    }
}
