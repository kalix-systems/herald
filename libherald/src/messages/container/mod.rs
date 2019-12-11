use super::*;
use crate::{content_push, spawn};
pub use messages_helper::{container::*, types::*};

mod handlers;
pub(super) use handlers::*;

pub(super) fn fill(cid: ConversationId) {
    spawn!({
        let (list, map): (Vector<MessageMeta>, HashMap<MsgId, MsgData>) =
            ret_err!(conversation::conversation_messages(&cid))
                .into_iter()
                .map(|m| {
                    let mid = m.message_id;
                    let (message, data) = split_msg(m);

                    (message, (mid, data))
                })
                .unzip();

        ret_err!(content_push(
            cid,
            MsgUpdate::Container(Box::new(Container::new(list, map)))
        ));
    });
}
