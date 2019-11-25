use crate::{cont_none, ffi, interface::*, ret_err, ret_none, shared::SingletonBus, spawn};
use heraldcore::{
    conversation::{self, ConversationMeta, ExpirationPeriod},
    types::ConversationId,
};
use im::vector::Vector;
use search_pattern::SearchPattern;
use std::convert::TryFrom;
use std::ops::Not;

pub(crate) mod shared;
use shared::*;
mod handlers;
mod imp;
pub(crate) mod types;
use types::*;

/// A wrapper around a vector of `Conversation`, with additional
/// fields to facilitate interaction with Qt.
pub struct Conversations {
    emit: ConversationsEmitter,
    model: ConversationsList,
    filter: Option<SearchPattern>,
    filter_regex: bool,
    list: Vector<Conversation>,
    loaded: bool,
}

impl ConversationsTrait for Conversations {
    fn new(
        mut emit: ConversationsEmitter,
        model: ConversationsList,
    ) -> Self {
        let filter = SearchPattern::new_normal("".into()).ok();

        let global_emit = emit.clone();

        CONV_EMITTER.lock().replace(global_emit);

        Self {
            emit,
            filter,
            filter_regex: false,
            model,
            list: Vector::new(),
            loaded: false,
        }
    }

    fn emit(&mut self) -> &mut ConversationsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn color(
        &self,
        index: usize,
    ) -> u32 {
        ret_none!(self.color_(index), 0)
    }

    fn set_color(
        &mut self,
        index: usize,
        color: u32,
    ) -> bool {
        let cid = ret_none!(self.id(index), false);

        spawn!(
            {
                use conversation::settings::*;
                let update = SettingsUpdate::Color(color);

                ret_err!(apply(&update, &cid));
                ret_err!(send_update(update, &cid));
            },
            false
        );

        ret_none!(self.set_color_(index, color), false);
        true
    }

    fn conversation_id(
        &self,
        index: usize,
    ) -> ffi::ConversationId {
        ret_none!(self.list.get(index), vec![]).id.to_vec()
    }

    fn expiration_period(
        &self,
        index: usize,
    ) -> u8 {
        ret_none!(self.expiration_(index), ExpirationPeriod::default() as u8) as u8
    }

    fn set_expiration_period(
        &mut self,
        index: usize,
        period: u8,
    ) -> bool {
        let period = period.into();

        let cid = ret_none!(self.id(index), false);

        spawn!(
            {
                use conversation::settings::*;
                let update = SettingsUpdate::Expiration(period);

                ret_err!(apply(&update, &cid));
                ret_err!(send_update(update, &cid));
            },
            false
        );

        ret_none!(self.set_expiration_(index, period), false);

        true
    }

    fn muted(
        &self,
        index: usize,
    ) -> bool {
        ret_none!(self.muted_(index), true)
    }

    fn set_muted(
        &mut self,
        index: usize,
        muted: bool,
    ) -> bool {
        let cid = ret_none!(self.id(index), false);

        spawn!(ret_err!(conversation::set_muted(&cid, muted)), false);

        ret_none!(self.set_muted_(index, muted), false);

        true
    }

    fn picture(
        &self,
        index: usize,
    ) -> Option<String> {
        self.picture_(index)?
    }

    fn set_picture(
        &mut self,
        index: usize,
        picture: Option<String>,
    ) -> bool {
        if self.pairwise_(index).unwrap_or(false) {
            return false;
        }

        let cid = ret_none!(self.id(index), false);

        // FIXME exception safety
        let path = ret_err!(
            conversation::set_picture(&cid, picture.as_ref().map(|p| p.as_str())),
            false
        );

        self.set_picture_(index, path);
        true
    }

    fn title(
        &self,
        index: usize,
    ) -> Option<String> {
        self.title_(index)?
    }

    fn set_title(
        &mut self,
        index: usize,
        title: Option<String>,
    ) -> bool {
        let cid = ret_none!(self.id(index), false);
        {
            let title = title.clone();
            spawn!(
                {
                    use conversation::settings::*;
                    let update = SettingsUpdate::Title(title);

                    ret_err!(apply(&update, &cid));
                    ret_err!(send_update(update, &cid));
                },
                false
            );
        }

        self.set_title_(index, title);
        true
    }

    fn pairwise(
        &self,
        index: usize,
    ) -> bool {
        ret_none!(self.pairwise_(index), false)
    }

    fn remove_conversation(
        &mut self,
        index: u64,
    ) -> bool {
        let index = index as usize;
        let cid = ret_none!(self.id(index), false);

        // cannot remove pairwise conversation!
        if self.pairwise_(index).unwrap_or(false) {
            return false;
        }

        spawn!(ret_err!(conversation::delete_conversation(&cid)), false);

        self.model.begin_remove_rows(index, index);
        self.list.remove(index);
        self.model.end_remove_rows();

        true
    }

    fn matched(
        &self,
        row_index: usize,
    ) -> bool {
        ret_none!(self.list.get(row_index), true).matched
    }

    fn filter(&self) -> &str {
        self.filter.as_ref().map(SearchPattern::raw).unwrap_or("")
    }

    fn set_filter(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_filter();
            return;
        }

        let pattern = if self.filter_regex() {
            ret_err!(SearchPattern::new_regex(pattern))
        } else {
            ret_err!(SearchPattern::new_normal(pattern))
        };

        self.filter.replace(pattern);
        self.emit.filter_changed();

        self.inner_filter();
    }

    fn can_fetch_more(&self) -> bool {
        !CONV_BUS.rx.is_empty()
    }

    fn fetch_more(&mut self) {
        use ConvUpdate::*;

        for update in CONV_BUS.rx.try_iter() {
            match update {
                NewConversation(inner) => self.handle_new_conversation(inner),
                BuilderFinished(inner) => self.handle_builder_finished(inner),
                NewActivity(cid) => self.handle_new_activity(cid),
                Settings(cid, update) => cont_none!(self.handle_settings_update(cid, update)),
                Init(contents) => self.handle_init(contents),
            }
        }
    }

    /// Indicates whether regex search is activated
    fn filter_regex(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    fn set_filter_regex(
        &mut self,
        use_regex: bool,
    ) {
        if use_regex {
            ret_err!(self
                .filter
                .as_mut()
                .map(SearchPattern::regex_mode)
                .transpose());
        } else {
            ret_err!(self
                .filter
                .as_mut()
                .map(SearchPattern::normal_mode)
                .transpose());
        }

        self.filter_regex = use_regex;
        self.emit.filter_regex_changed();
        self.inner_filter();
    }

    /// Toggles filter mode
    ///
    /// Returns new value.
    fn toggle_filter_regex(&mut self) -> bool {
        let toggled = !self.filter_regex;
        self.set_filter_regex(toggled);
        toggled
    }

    fn clear_filter(&mut self) {
        for (ix, conv) in self.list.iter_mut().enumerate() {
            if conv.matched.not() {
                conv.matched = true;
                self.model.data_changed(ix, ix);
            }
        }

        if let Some(filter) = self.filter.as_mut() {
            ret_err!(filter.set_pattern("".to_owned()));
        }

        self.emit.filter_changed();
    }

    fn index_by_id(
        &self,
        cid: ffi::ConversationIdRef,
    ) -> u64 {
        let ret_val = std::u32::MAX as u64;
        let conversation_id = ret_err!(ConversationId::try_from(cid), ret_val);
        self.list
            .iter()
            .position(|Conversation { id, .. }| id == &conversation_id)
            .map(|n| n as u64)
            .unwrap_or(ret_val)
    }
}
