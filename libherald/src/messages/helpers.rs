use super::*;
use crate::{content_push, spawn};
pub use messages_helper::{container::*, types::*};

impl Messages {
    pub(super) fn emit_last_changed(&mut self) {
        use crate::conversations::shared::{
            update_last_msg_id, ConvItemUpdate, ConvItemUpdateVariant,
        };
        let cid = none!(self.conversation_id);

        update_last_msg_id(&cid, self.container.list.front().map(|m| m.msg_id));
        crate::push(ConvItemUpdate {
            cid,
            variant: ConvItemUpdateVariant::LastChanged,
        });
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
