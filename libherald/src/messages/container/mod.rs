use super::*;
use crate::{content_push, spawn};
pub use messages_helper::{container::*, types::*};

pub(super) fn fill(cid: ConversationId) {
    spawn!({
        let list: Vector<MessageMeta> = err!(conversation::conversation_message_meta(&cid))
            .into_iter()
            .collect();

        let last = match list.last().as_ref() {
            Some(MessageMeta { ref msg_id, .. }) => {
                let msg = err!(heraldcore::message::get_message(msg_id));
                Some(heraldcore::message::split_msg(msg).1)
            }
            None => None,
        };

        err!(content_push(
            cid,
            MsgUpdate::Container(Box::new(Container::new(list, last)))
        ));
    });
}
