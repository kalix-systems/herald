use crate::*;
use coretypes::messages::{Item, PlainItem};
use herald_common::{Time, UserId};
use heraldcore::message::Elider;
use message_cache as cache;
use revec::Revec;
use search::*;
use std::collections::HashSet;
use types::*;

const FLURRY_FUZZ: i64 = 5 * 60_000;

pub mod handlers;
pub use cache::{access, get, update};

#[derive(Default, Debug)]
/// Container type for messages
pub struct Container {
    pub list: Revec<MessageMeta>,
    op_body_elider: Elider,
}

impl Container {
    pub fn new(list: Vec<MessageMeta>) -> Self {
        Self {
            list: list.into(),
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

    pub fn update_by_index<T, F: FnOnce(&mut MsgData) -> T>(
        &self,
        index: usize,
        f: F,
    ) -> Option<T> {
        let mid = self.msg_id(index)?;

        update(&mid, f)
    }

    pub fn media_attachments_data_json(
        &self,
        index: usize,
        limit: Option<usize>,
    ) -> Option<String> {
        let mid = self.list.get(index)?.msg_id;
        self.get_media_attachments_data_json(&mid, limit)
    }

    pub fn get_media_attachments_data_json(
        &self,
        mid: &MsgId,
        limit: Option<usize>,
    ) -> Option<String> {
        access(mid, |m| m.attachments().map(Clone::clone))
            .flatten()
            .and_then(|attachments| crate::media_attachments_json(&attachments, limit))
    }

    pub fn doc_attachments_data_json(
        &self,
        index: usize,
        limit: Option<usize>,
    ) -> Option<String> {
        let mid = self.list.get(index)?.msg_id;
        self.get_doc_attachments_data_json(&mid, limit)
    }

    pub fn get_doc_attachments_data_json(
        &self,
        mid: &MsgId,
        limit: Option<usize>,
    ) -> Option<String> {
        access(mid, |m| m.attachments().map(Clone::clone))
            .flatten()
            .and_then(|attachments| crate::doc_attachments_json(&attachments, limit))
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

    pub fn aux_data_json(
        &self,
        ix: usize,
    ) -> Option<String> {
        let mid = self.msg_id(ix)?;
        self.aux_data_json_by_id(&mid)
    }

    pub fn aux_data_json_by_id(
        &self,
        msg_id: &MsgId,
    ) -> Option<String> {
        let update = cache::access(msg_id, |data| match &data.content {
            Item::Aux(update) => Some(update.clone()),
            _ => None,
        })
        .flatten()?;

        json::JsonValue::from(update).dump().into()
    }

    pub fn aux_data_code_by_id(
        &self,
        msg_id: &MsgId,
    ) -> Option<u8> {
        let update = cache::access(msg_id, |data| match &data.content {
            Item::Aux(update) => Some(update.clone()),

            _ => None,
        })
        .flatten()?;

        update.code().into()
    }

    /// Removes the item from the container. *Does not modify disk storage*.
    pub fn remove(
        &mut self,
        ix: usize,
    ) -> Option<MsgData> {
        let msg = self.list.remove(ix)?;
        cache::remove(&msg.msg_id)
    }

    pub fn insert_helper<
        E: MessageEmit,
        M: MessageModel,
        P: FnOnce(heraldcore::types::ConversationId),
    >(
        &mut self,
        msg: Message,
        emit: &mut E,
        model: &mut M,
        search: &mut SearchState,
        cid: heraldcore::types::ConversationId,
        push: P,
    ) {
        let (message, data) = msg.split();

        let msg_id = message.msg_id;

        let ix = self.insert_ord(message, data);
        model.begin_insert_rows(ix, ix);
        model.end_insert_rows();

        search.try_insert_match(msg_id, ix, self, emit, model);

        if ix == 0 {
            emit.last_changed();
        } else {
            model.entry_changed(ix - 1);
        }

        if self.len() == 1 {
            emit.is_empty_changed();
        }

        if ix + 1 < self.len() {
            model.entry_changed(ix + 1);
        }

        push(cid);
    }

    pub fn remove_helper<E: MessageEmit, M: MessageModel, B: FnMut()>(
        &mut self,
        msg_id: MsgId,
        ix: usize,
        emit: &mut E,
        model: &mut M,
        search: &mut SearchState,
        mut builder: B,
    ) {
        {
            search.try_remove_match(&msg_id, self, emit, model);
        }

        builder();

        let old_len = self.len();

        model.begin_remove_rows(ix, ix);
        let data = self.remove(ix);
        model.end_remove_rows();

        if let Some(MsgData { replies, .. }) = data {
            self.set_dangling(replies, model);
        }

        if ix > 0 {
            model.entry_changed(ix - 1);
        }

        if ix + 1 < self.len() {
            model.entry_changed(ix + 1);
        }

        if old_len == 1 {
            emit.is_empty_changed();
        }

        if ix == 0 {
            emit.last_changed();
        }
    }

    pub fn binary_search(
        &self,
        msg: &MessageMeta,
    ) -> Result<usize, usize> {
        self.list.binary_search(msg)
    }

    #[must_use]
    pub fn insert_ord(
        &mut self,
        msg: MessageMeta,
        data: MsgData,
    ) -> usize {
        let mid = msg.msg_id;

        if let ReplyId::Known(op) = &data.op() {
            cache::update(op, |m| {
                m.replies.insert(mid);
            });
        }

        let ix = self.list.insert_ord(msg);
        cache::insert(mid, data);

        ix
    }

    pub fn msg_id(
        &self,
        index: usize,
    ) -> Option<&MsgId> {
        Some(&self.list.get(index).as_ref()?.msg_id)
    }

    pub fn op_reply_type(
        &self,
        index: usize,
    ) -> Option<ReplyType> {
        Some(reply_type(&cache::access(self.msg_id(index)?, |m| {
            *m.op()
        })?))
    }

    pub fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<MsgId> {
        match cache::access(self.msg_id(index)?, |m| *m.op())? {
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

        access(&mid, |m| m.text().map(ToString::to_string))
            .flatten()
            .map(|b| self.op_body_elider.elided_body(b))
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

        self.get_doc_attachments_data_json(&mid, Some(1))
    }

    pub fn op_media_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;
        self.get_media_attachments_data_json(&mid, Some(4))
    }

    pub fn op_aux_data_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;
        self.aux_data_json_by_id(&mid)
    }

    pub fn clear_search<M: MessageModel>(
        &mut self,
        model: &mut M,
    ) -> Option<()> {
        for (ix, msg) in self.list.iter_mut().enumerate() {
            if msg.match_status.is_match() {
                msg.match_status = MatchStatus::NotMatched;
                model.entry_changed(ix);
            }
        }

        Some(())
    }

    // FIXME make this incremental, long conversations with a large number of matches freeze the UI
    pub fn apply_search<M: MessageModel, E: MessageEmit>(
        &mut self,
        search: &SearchState,
        emit: &mut E,
        model: &mut M,
    ) -> Option<Vec<Match>> {
        let pattern = search.pattern.as_ref()?;

        if !search.active || pattern.raw().is_empty() {
            return None;
        }

        let mut matches: Vec<Match> = Vec::new();

        for (ix, msg) in self.list.iter_mut().enumerate() {
            let old_match_status = msg.match_status;

            let matched = access(&msg.msg_id, |m| m.matches(pattern))?;

            msg.match_status = if matched {
                MatchStatus::Matched
            } else {
                MatchStatus::NotMatched
            };

            if (old_match_status != msg.match_status)
                || (old_match_status.is_match() && msg.match_status.is_match())
            {
                model.entry_changed(ix);
            }

            if !matched {
                continue;
            };

            matches.push(Match(*msg))
        }

        emit.search_num_matches_changed();

        Some(matches)
    }

    /// Sets the reply type of a message to "dangling"
    pub fn set_dangling<M: MessageModel>(
        &self,
        ids: HashSet<MsgId>,
        model: &mut M,
    ) -> Option<()> {
        for id in ids.into_iter() {
            let changed = update(&id, |data| match data.content {
                Item::Plain(PlainItem { ref mut op, .. }) => {
                    if *op != ReplyId::Dangling {
                        *op = ReplyId::Dangling;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            });

            if changed.unwrap_or(false) {
                if let Some(ix) = self.index_by_id(id) {
                    model.entry_changed(ix);
                }
            }
        }

        Some(())
    }

    pub fn same_flurry(
        &self,
        a_ix: usize,
        b_ix: usize,
    ) -> Option<bool> {
        let flurry_info =
            |data: &MsgData| (data.author, data.time.insertion, data.content.is_plain());

        let (a_author, a_ts, a_is_plain) = self.access_by_index(a_ix, flurry_info)?;
        let (b_author, b_ts, b_is_plain) = self.access_by_index(b_ix, flurry_info)?;

        if !a_is_plain || !b_is_plain {
            return Some(false);
        }

        ((a_author == b_author) && a_ts.within(FLURRY_FUZZ, b_ts)).into()
    }
}

#[cfg(test)]
mod tests;
