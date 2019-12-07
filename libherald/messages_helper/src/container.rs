use crate::*;
use herald_common::{Time, UserId};
use im::vector::Vector;
use search::*;
use std::collections::HashMap;
use types::*;

#[derive(Default)]
/// A container type for messages backed by an RRB-tree vector
/// and a hash map.
pub struct Container {
    pub list: Vector<Message>,
    map: HashMap<MsgId, MsgData>,
    last: Option<MsgData>,
}

impl Container {
    pub fn new(
        list: Vector<Message>,
        map: HashMap<MsgId, MsgData>,
    ) -> Self {
        let last = match list.last().as_ref() {
            Some(Message { ref msg_id, .. }) => map.get(msg_id).cloned(),
            None => None,
        };

        Self { map, last, list }
    }

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
        self.last.as_ref()
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
        let old_len = self.len();
        if ix >= old_len {
            return None;
        }

        let msg = self.list.remove(ix);
        let data = self.map.remove(&msg.msg_id)?;

        if ix + 1 == old_len {
            self.last = self
                .list
                .last()
                .and_then(|Message { ref msg_id, .. }| self.map.get(msg_id))
                .cloned();
        }

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
        let old_len = self.list.len();
        let mid = msg.msg_id;

        if let ReplyId::Known(op) = &data.op {
            self.get_data_mut(op)?.replies.insert(mid);
        }

        self.list.insert(ix, msg);
        self.map.insert(mid, data);

        if ix == old_len {
            self.last = self
                .list
                .last()
                .and_then(|Message { ref msg_id, .. }| self.map.get(msg_id))
                .cloned();
        }

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

    pub fn clear_search<F: FnMut(usize)>(
        &mut self,
        mut data_changed: F,
    ) -> Option<()> {
        for (ix, Message { msg_id, .. }) in self.list.iter().enumerate() {
            let data = self.map.get_mut(&msg_id)?;

            if data.match_status.is_match() {
                data.match_status = MatchStatus::NotMatched;
                data_changed(ix);
            }
        }

        Some(())
    }

    pub fn apply_search<D: FnMut(usize), N: FnMut()>(
        &mut self,
        search: &SearchState,
        mut data_changed: D,
        mut num_matches_changed: N,
    ) -> Option<Vec<Match>> {
        let pattern = search.pattern.as_ref()?;

        if !search.active || pattern.raw().is_empty() {
            return None;
        }

        let mut matches: Vec<Match> = Vec::new();

        for (ix, msg) in self.list.iter().enumerate() {
            let data = self.map.get_mut(&msg.msg_id)?;
            let matched = data.matches(pattern);

            data.match_status = if matched {
                MatchStatus::Matched
            } else {
                MatchStatus::NotMatched
            };

            data.search_buf = if data.match_status.is_match() {
                Some(highlight_message(
                    search.pattern.as_ref()?,
                    data.body.as_ref()?,
                ))
            } else {
                None
            };

            data_changed(ix);

            if !matched {
                continue;
            };

            matches.push(Match(*msg))
        }

        num_matches_changed();

        Some(matches)
    }
}
