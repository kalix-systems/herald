use crate::types::*;
use herald_common::{Time, UserId};
use im::vector::Vector;
use std::collections::HashMap;

#[derive(Default)]
/// A container type for messages backed by an RRB-tree vector
/// and a hash map.
pub struct Container {
    pub list: Vector<Message>,
    pub map: HashMap<MsgId, MsgData>,
}

impl Container {
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn contains(
        &self,
        msg_id: &MsgId,
    ) -> bool {
        self.map.contains_key(msg_id)
    }

    pub fn get(
        &self,
        ix: usize,
    ) -> Option<&Message> {
        self.list.get(ix)
    }

    pub fn get_data_mut(
        &mut self,
        msg_id: &MsgId,
    ) -> Option<&mut MsgData> {
        self.map.get_mut(msg_id)
    }

    pub fn get_data(
        &self,
        msg_id: &MsgId,
    ) -> Option<&MsgData> {
        self.map.get(msg_id)
    }

    pub fn last_msg(&self) -> Option<&MsgData> {
        let mid = self.list.last()?.msg_id;
        self.map.get(&mid)
    }

    pub fn msg_data(
        &self,
        index: usize,
    ) -> Option<&MsgData> {
        let msg = self.list.get(index);
        self.map.get(&msg?.msg_id)
    }

    pub fn msg_data_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut MsgData> {
        let msg = self.list.get(index);
        self.map.get_mut(&msg?.msg_id)
    }

    pub fn media_attachments_data_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.list.get(index)?.msg_id;
        self.get_media_attachments_data_json(&mid)
    }

    pub fn get_media_attachments_data_json(
        &self,
        mid: &MsgId,
    ) -> Option<String> {
        let attachments = &self.get_data(mid)?.attachments;

        if attachments.is_empty() {
            return None;
        }

        let media = attachments.media_attachments().ok()?;

        if media.is_empty() {
            return None;
        }

        Some(json::JsonValue::from(media).dump())
    }

    pub fn doc_attachments_data_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.list.get(index)?.msg_id;
        self.get_doc_attachments_data_json(&mid)
    }

    pub fn get_doc_attachments_data_json(
        &self,
        mid: &MsgId,
    ) -> Option<String> {
        let attachments = &self.get_data(mid)?.attachments;

        if attachments.is_empty() {
            return None;
        }

        let docs = attachments.doc_attachments().ok()?;

        if docs.is_empty() {
            return None;
        }

        Some(json::JsonValue::from(docs).dump())
    }

    pub fn last(&self) -> Option<&Message> {
        self.list.last()
    }

    pub fn index_of(
        &self,
        msg: &Message,
    ) -> Option<usize> {
        self.list.binary_search(&msg).ok()
    }

    pub fn index_by_id(
        &self,
        msg_id: MsgId,
    ) -> Option<usize> {
        let m = from_msg_id(msg_id, &self)?;

        self.list.binary_search(&m).ok()
    }

    /// Removes the item from the container. *Does not modify disk storage*.
    pub fn remove(
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

    pub fn binary_search(
        &self,
        msg: &Message,
    ) -> Result<usize, usize> {
        self.list.binary_search(msg)
    }

    #[must_use]
    pub fn insert(
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

    fn op(
        &self,
        index: usize,
    ) -> Option<&MsgData> {
        match self.msg_data(index)?.op {
            ReplyId::Known(mid) => self.get_data(&mid),
            _ => None,
        }
    }

    pub fn op_reply_type(
        &self,
        index: usize,
    ) -> Option<ReplyType> {
        Some(reply_type(&self.msg_data(index)?.op))
    }

    pub fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<&MsgId> {
        match &self.msg_data(index).as_ref()?.op {
            ReplyId::Known(mid) => Some(mid),
            _ => None,
        }
    }

    pub fn op_author(
        &self,
        index: usize,
    ) -> Option<&UserId> {
        Some(&self.op(index)?.author)
    }

    pub fn op_body(
        &self,
        index: usize,
    ) -> Option<&Option<MessageBody>> {
        Some(&self.op(index)?.body)
    }

    pub fn op_insertion_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        Some(self.op(index)?.time.insertion)
    }

    pub fn op_expiration_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        Some(self.op(index)?.time.expiration?)
    }

    pub fn op_doc_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.list.get(index)?.msg_id;
        self.get_doc_attachments_data_json(&mid)
    }

    pub fn op_media_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.list.get(index)?.msg_id;
        self.get_media_attachments_data_json(&mid)
    }
}
