use super::*;
use crate::push;
use crate::{content_push, spawn};
use heraldcore::{errors::HErr, message::Message as Msg, NE};
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

    pub(super) fn entry_changed(
        &mut self,
        ix: usize,
    ) {
        if ix < self.container.len() {
            self.model.data_changed(ix, ix);
        }
    }

    pub(super) fn remove_helper(
        &mut self,
        msg_id: MsgId,
        ix: usize,
    ) {
        {
            let emit = &mut self.emit;
            let model = &mut self.model;

            self.search
                .try_remove_match(&msg_id, &mut self.container, emit, model);
        }

        self.builder.try_clear_reply(&msg_id);

        let old_len = self.container.len();

        self.model.begin_remove_rows(ix, ix);
        let data = self.container.remove(ix);
        self.model.end_remove_rows();

        if let Some(MsgData { replies, .. }) = data {
            let model = &mut self.model;
            self.container.set_dangling(replies, model);
        }

        if ix > 0 {
            self.entry_changed(ix - 1);
        }

        if ix + 1 < self.container.len() {
            self.entry_changed(ix + 1);
        }

        if old_len == 1 {
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
        let (message, data) = msg.split();

        let cid = self.conversation_id.ok_or(NE!())?;

        let msg_id = message.msg_id;

        let ix = self.container.insert_ord(message, data);
        self.model.begin_insert_rows(ix, ix);
        self.model.end_insert_rows();

        {
            let emit = &mut self.emit;
            let model = &mut self.model;

            self.search
                .try_insert_match(msg_id, ix, &mut self.container, emit, model);
        }

        if ix == 0 {
            self.emit_last_changed();
        } else {
            self.entry_changed(ix - 1);
        }

        if self.container.len() == 1 {
            self.emit.is_empty_changed();
        }

        if ix + 1 < self.container.len() {
            self.entry_changed(ix + 1);
        }

        use crate::conversations::shared::*;

        push(ConvItemUpdate {
            cid,
            variant: ConvItemUpdateVariant::NewActivity,
        });

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
            let list: Vec<MessageMeta> = err!(heraldcore::message::conversation_message_meta(&id));

            err!(content_push(
                id,
                MsgUpdate::Container(Box::new(Container::new(list)))
            ));
        });
    }
}
