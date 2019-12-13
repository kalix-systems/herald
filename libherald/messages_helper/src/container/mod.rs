use crate::*;
use herald_common::{Time, UserId};
use heraldcore::message::{
    attachments::{DocMeta, MediaMeta},
    Elider,
};
use im::vector::Vector;
use search::*;
use std::collections::HashSet;
use types::*;

mod cache;
pub mod handlers;
pub use cache::{access, update};

#[derive(Default)]
/// A container type for messages backed by an RRB-tree vector
/// and a hash map.
pub struct Container {
    pub list: Vector<MessageMeta>,
    last: Option<MsgData>,
    op_body_elider: Elider,
}

impl Container {
    pub fn new(
        list: Vector<MessageMeta>,
        last: Option<MsgData>,
    ) -> Self {
        Self {
            last,
            list,
            op_body_elider: Elider {
                line_count: 3,
                char_per_line: 60,
                char_count: 3 * 60,
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get(
        &self,
        ix: usize,
    ) -> Option<&MessageMeta> {
        self.list.get(ix)
    }

    pub fn get_data(
        &self,
        mid: &MsgId,
    ) -> Option<MsgData> {
        cache::get(mid)
    }

    pub fn last_msg(&self) -> Option<&MsgData> {
        self.last.as_ref()
    }

    pub fn msg_data(
        &self,
        index: usize,
    ) -> Option<MsgData> {
        let msg = self.list.get(index);
        cache::get(&msg?.msg_id)
    }

    pub fn access_by_index<T, F: FnOnce(&MsgData) -> T>(
        &self,
        index: usize,
        f: F,
    ) -> Option<T> {
        let mid = self.msg_id(index)?;

        access(&mid, f)
    }

    pub fn update_by_index<T, F: FnOnce(&mut MsgData)>(
        &self,
        index: usize,
        f: F,
    ) -> Option<()> {
        let mid = self.msg_id(index)?;

        update(&mid, f)
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
        access(mid, |m| crate::media_attachments_json(&m.attachments))?
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
        access(mid, |m| crate::doc_attachments_json(&m.attachments))?
    }

    pub fn last(&self) -> Option<&MessageMeta> {
        self.list.last()
    }

    pub fn index_of(
        &self,
        msg: &MessageMeta,
    ) -> Option<usize> {
        self.list.binary_search(&msg).ok()
    }

    pub fn index_by_id(
        &self,
        msg_id: MsgId,
    ) -> Option<usize> {
        let m = from_msg_id(msg_id)?;

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
        let data = cache::remove(&msg.msg_id);

        if ix + 1 == old_len {
            self.last = self
                .list
                .last()
                .and_then(|MessageMeta { ref msg_id, .. }| cache::get(msg_id));
        }

        data
    }

    pub fn binary_search(
        &self,
        msg: &MessageMeta,
    ) -> Result<usize, usize> {
        self.list.binary_search(msg)
    }

    #[must_use]
    pub fn insert(
        &mut self,
        ix: usize,
        msg: MessageMeta,
        data: MsgData,
    ) -> Option<()> {
        let old_len = self.list.len();
        let mid = msg.msg_id;

        if let ReplyId::Known(op) = &data.op {
            cache::update(op, |m| {
                m.replies.insert(mid);
            });
        }

        self.list.insert(ix, msg);
        cache::insert(mid, data);

        if ix == old_len {
            self.last = self
                .list
                .last()
                .and_then(|MessageMeta { ref msg_id, .. }| cache::get(msg_id));
        }

        Some(())
    }

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<&MsgId> {
        Some(&self.list.get(index).as_ref()?.msg_id)
    }

    pub fn op_reply_type(
        &self,
        index: usize,
    ) -> Option<ReplyType> {
        Some(reply_type(&cache::access(self.msg_id(index)?, |m| m.op)?))
    }

    pub fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<MsgId> {
        match cache::access(self.msg_id(index)?, |m| m.op)? {
            ReplyId::Known(mid) => Some(mid),
            _ => None,
        }
    }

    pub fn op_author(
        &self,
        index: usize,
    ) -> Option<UserId> {
        let mid = self.op_msg_id(index)?;
        access(&mid, |m| m.author)
    }

    pub fn op_body(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;

        access(&mid, |m| {
            m.body.as_ref().map(|b| self.op_body_elider.elided_body(b))
        })?
    }

    pub fn op_insertion_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        let mid = self.op_msg_id(index)?;
        access(&mid, |m| m.time.insertion)
    }

    pub fn op_expiration_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        let mid = self.op_msg_id(index)?;
        access(&mid, |m| m.time.expiration)?
    }

    pub fn op_doc_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;

        let (first, len): (DocMeta, usize) = access(&mid, |m| -> Option<_> {
            let docs = m.attachments.doc_attachments().ok()?;

            if docs.is_empty() {
                return None;
            }

            let len = docs.len();
            let first = docs.into_iter().next()?;
            Some((first, len))
        })??;

        Some(
            json::object! {
                "first" => first,
                "count" => len,
            }
            .dump(),
        )
    }

    pub fn op_media_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;

        let (first, len): (MediaMeta, usize) = access(&mid, |m| -> Option<_> {
            let medias = m.attachments.media_attachments().ok()?;

            if medias.is_empty() {
                return None;
            }

            let len = medias.len();
            let first = medias.into_iter().next()?;
            Some((first, len))
        })??;

        Some(
            json::object! {
                "first" => first,
                "count" => len,
            }
            .dump(),
        )
    }

    pub fn clear_search<F: FnMut(usize)>(
        &mut self,
        mut data_changed: F,
    ) -> Option<()> {
        for (ix, msg) in self.list.iter_mut().enumerate() {
            if msg.match_status.is_match() {
                msg.match_status = MatchStatus::NotMatched;
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

        for (ix, msg) in self.list.iter_mut().enumerate() {
            let matched = access(&msg.msg_id, |m| m.matches(pattern))?;

            msg.match_status = if matched {
                MatchStatus::Matched
            } else {
                MatchStatus::NotMatched
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

    /// Sets the reply type of a message to "dangling"
    pub fn set_dangling<F: FnMut(usize)>(
        &self,
        ids: HashSet<MsgId>,
        mut data_changed: F,
    ) -> Option<()> {
        for id in ids.into_iter() {
            let changed = update(&id, |data| {
                if data.op != ReplyId::Dangling {
                    data.op = ReplyId::Dangling;
                    true
                } else {
                    false
                }
            });

            if changed.unwrap_or(false) {
                if let Some(ix) = self.index_by_id(id) {
                    data_changed(ix);
                }
            }
        }

        Some(())
    }
}
