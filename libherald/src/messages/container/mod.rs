use super::*;
use crate::{content_push, spawn};
pub use messages_helper::{container::*, types::*};
use std::collections::HashSet;

mod handlers;
mod search;
pub(super) use handlers::*;
pub(super) use search::*;

/// Sets the reply type of a message to "dangling"
pub(super) fn set_dangling(
    container: &mut Container,
    ids: HashSet<MsgId>,
    model: &mut List,
) -> Option<()> {
    for id in ids.into_iter() {
        if let Some(data) = container.get_data_mut(&id) {
            if data.op != ReplyId::Dangling {
                data.op = ReplyId::Dangling;

                let ix = container.index_by_id(id)?;
                model.data_changed(ix, ix);
            }
        }
    }

    Some(())
}

pub(super) fn fill(cid: ConversationId) {
    spawn!({
        let (list, map): (Vector<Message>, HashMap<MsgId, MsgData>) =
            ret_err!(conversation::conversation_messages(&cid))
                .into_iter()
                .map(|m| {
                    let mid = m.message_id;
                    let (message, data) = split_msg(m, SaveStatus::Saved);

                    (message, (mid, data))
                })
                .unzip();

        ret_err!(content_push(
            cid,
            MsgUpdate::Container(Container { list, map })
        ));
    });
}
