use super::*;
use crate::{shared::AddressedBus, spawn};
use std::{collections::HashSet, ops::Not};

mod handlers;
pub use handlers::*;

mod search;
pub use search::*;

mod op;
pub use op::*;

#[derive(Default)]
/// A container type for messages backed by an RRB-tree vector
/// and a hash map.
pub struct Container {
    list: Vector<Message>,
    map: HashMap<MsgId, MsgData>,
}

impl Container {
    pub(super) fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub(super) fn len(&self) -> usize {
        self.list.len()
    }

    #[allow(unused)]
    pub(super) fn contains(
        &self,
        msg_id: &MsgId,
    ) -> bool {
        self.map.contains_key(msg_id)
    }

    pub(super) fn get(
        &self,
        ix: usize,
    ) -> Option<&Message> {
        self.list.get(ix)
    }

    pub(super) fn get_data_mut(
        &mut self,
        msg_id: &MsgId,
    ) -> Option<&mut MsgData> {
        self.map.get_mut(msg_id)
    }

    pub(super) fn get_data(
        &self,
        msg_id: &MsgId,
    ) -> Option<&MsgData> {
        self.map.get(msg_id)
    }

    /// Sets the reply type of a message to "dangling"
    pub(super) fn set_dangling(
        &mut self,
        ids: HashSet<MsgId>,
        model: &mut List,
    ) -> Option<()> {
        for id in ids.into_iter() {
            if let Some(data) = self.get_data_mut(&id) {
                if data.op != ReplyId::Dangling {
                    data.op = ReplyId::Dangling;

                    let ix = self.index_by_id(id)?;
                    model.data_changed(ix, ix);
                }
            }
        }

        Some(())
    }

    pub(super) fn last_msg(&self) -> Option<&MsgData> {
        let mid = self.list.last()?.msg_id;
        self.map.get(&mid)
    }

    pub(super) fn msg_data(
        &self,
        index: usize,
    ) -> Option<&MsgData> {
        let msg = self.list.get(index);
        self.map.get(&msg?.msg_id)
    }

    #[allow(unused)]
    pub(super) fn msg_data_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut MsgData> {
        let msg = self.list.get(index);
        self.map.get_mut(&msg?.msg_id)
    }

    pub(super) fn last(&self) -> Option<&Message> {
        self.list.last()
    }

    pub(super) fn index_of(
        &self,
        msg: &Message,
    ) -> Option<usize> {
        self.list.binary_search(&msg).ok()
    }

    pub(super) fn index_by_id(
        &self,
        msg_id: MsgId,
    ) -> Option<usize> {
        let m = Message::from_msg_id(msg_id, &self)?;

        self.list.binary_search(&m).ok()
    }

    /// Removes the item from the container. *Does not modify disk storage*.
    pub(super) fn remove(
        &mut self,
        ix: usize,
    ) -> Option<MsgData> {
        if ix >= self.len() {
            return None;
        }

        let msg = self.list.remove(ix);
        let data = self.map.remove(&msg.msg_id)?;

        Some(data)
    }

    pub(super) fn binary_search(
        &self,
        msg: &Message,
    ) -> Result<usize, usize> {
        self.list.binary_search(msg)
    }

    #[must_use]
    pub(super) fn insert(
        &mut self,
        ix: usize,
        msg: Message,
        data: MsgData,
    ) -> Option<()> {
        let mid = msg.msg_id;

        if let ReplyId::Known(op) = &data.op {
            self.get_data_mut(op)?.replies.insert(mid);
        }

        self.list.insert(ix, msg);
        self.map.insert(mid, data);

        Some(())
    }
}

pub(super) fn fill(cid: ConversationId) {
    spawn!({
        let (list, map): (Vector<Message>, HashMap<MsgId, MsgData>) =
            ret_err!(conversation::conversation_messages(&cid))
                .into_iter()
                .map(|m| {
                    let mid = m.message_id;
                    let (message, data) = Message::split_msg(m, SaveStatus::Saved);

                    (message, (mid, data))
                })
                .unzip();

        ret_err!(Messages::push(
            cid,
            MsgUpdate::Container(Container { list, map })
        ));
    });
}
