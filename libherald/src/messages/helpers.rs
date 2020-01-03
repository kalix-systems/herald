use super::*;
use crate::{content_push, spawn};
pub use messages_helper::{container::*, types::*};

impl Messages {
    pub(super) fn emit_last_changed(&mut self) {
        self.emit.last_author_changed();
        self.emit.last_body_changed();
        self.emit.last_time_changed();
        self.emit.last_status_changed();
        self.emit.last_aux_code_changed();
        self.emit.last_has_attachments_changed();
    }

    pub(super) fn handle_expiration(
        &mut self,
        mids: Vec<MsgId>,
    ) {
        for mid in mids {
            if let Some(ix) = self.container.index_by_id(mid) {
                let model = &mut self.model;
                let emit = &mut self.emit;
                let search = &mut self.search;
                let container = &mut self.container;
                let builder = &mut self.builder;
                container.remove_helper(mid, ix, emit, model, search, || {
                    builder.try_clear_reply(&mid)
                });
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
            let list: Vec<MessageMeta> = err!(heraldcore::message::conversation_message_meta(&id));

            err!(content_push(
                id,
                MsgUpdate::Container(Box::new(Container::new(list)))
            ));
        });
    }
}
