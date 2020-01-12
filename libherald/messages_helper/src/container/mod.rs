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
const SHORT_FUZZ: i64 = 10_000;

pub mod handlers;
pub mod helpers;
pub mod op;
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

    pub fn msg_id(
        &self,
        index: usize,
    ) -> Option<&MsgId> {
        Some(&self.list.get(index).as_ref()?.msg_id)
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

    pub fn index_by_id(
        &self,
        msg_id: MsgId,
    ) -> Option<usize> {
        let m = from_msg_id(msg_id)?;

        self.list.binary_search(&m).ok()
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

    // media attachments functions
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

    //doc attachments functions
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

    //aux data functions
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

    //search functions
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
        conversation_expiration_period: coretypes::conversation::ExpirationPeriod,
    ) -> Option<bool> {
        macro_rules! early {
            ($pred:expr) => {
                if $pred {
                    return Some(false);
                }
            };
        };

        let flurry_info = |data: &MsgData| {
            (
                data.author,
                data.time.insertion,
                data.time.expiration,
                data.content.is_plain(),
            )
        };

        let (a_author, a_ts, a_exp, a_is_plain) = self.access_by_index(a_ix, flurry_info)?;
        let (b_author, b_ts, b_exp, b_is_plain) = self.access_by_index(b_ix, flurry_info)?;

        early!(!(a_is_plain && b_is_plain) || (a_author != b_author));

        match (a_exp, b_exp) {
            (Some(a_exp), Some(b_exp)) => {
                early!(a_exp <= a_ts || b_exp <= b_ts);

                let diff = |exp: Time, ts: Time| *exp.as_i64() as f64 - *ts.as_i64() as f64;

                let a_diff = diff(a_exp, a_ts);
                let b_diff = diff(b_exp, b_ts);

                let ratio = (a_diff / b_diff).max(b_diff / a_diff);

                // eh, close enough
                early!(ratio > 1.5);
            }
            (None, None) => {}
            _ => return Some(false),
        };

        use coretypes::conversation::ExpirationPeriod::*;
        let fuzz = match conversation_expiration_period {
            ThirtySeconds | OneMinute => SHORT_FUZZ,
            _ => FLURRY_FUZZ,
        };

        a_ts.within(fuzz, b_ts).into()
    }
}

#[cfg(test)]
mod tests;
