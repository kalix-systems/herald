use crate::interface::{ConversationsEmitter, ConversationsList};
use crate::{ffi, ret_err, ret_none, spawn};
use heraldcore::{
    conversation::{self, ExpirationPeriod},
    types::ConversationId,
};
use im::Vector;
use search_pattern::SearchPattern;
use std::convert::TryFrom;
use std::ops::Not;

impl super::Conversations {
    pub(crate) fn new_(
        emit: ConversationsEmitter,
        model: ConversationsList,
    ) -> Self {
        let filter = SearchPattern::new_normal("".into()).ok();

        Self {
            emit,
            filter,
            filter_regex: false,
            model,
            list: Vector::new(),
            loaded: false,
        }
    }

    pub(crate) fn emit_(&mut self) -> &mut ConversationsEmitter {
        &mut self.emit
    }

    pub(crate) fn row_count_(&self) -> usize {
        self.list.len()
    }

    pub(crate) fn color_(
        &self,
        index: usize,
    ) -> u32 {
        ret_none!(self.color_inner(index), 0)
    }

    pub(crate) fn set_color_(
        &mut self,
        index: usize,
        color: u32,
    ) -> bool {
        let cid = ret_none!(self.id(index), false);

        spawn!(
            {
                use heraldcore::conversation::settings::*;
                let update = SettingsUpdate::Color(color);

                ret_err!(apply(&update, &cid));
                ret_err!(send_update(update, &cid));
            },
            false
        );

        ret_none!(self.set_color_inner(index, color), false);
        true
    }

    pub(crate) fn conversation_id_(
        &self,
        index: usize,
    ) -> ffi::ConversationId {
        ret_none!(self.list.get(index), vec![]).id.to_vec()
    }

    pub(crate) fn expiration_period_(
        &self,
        index: usize,
    ) -> u8 {
        ret_none!(
            self.expiration_inner(index),
            ExpirationPeriod::default() as u8
        ) as u8
    }

    pub(crate) fn set_expiration_period_(
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

        ret_none!(self.set_expiration_inner(index, period), false);

        true
    }

    pub(crate) fn muted_(
        &self,
        index: usize,
    ) -> bool {
        ret_none!(self.muted_inner(index), true)
    }

    pub(crate) fn set_muted_(
        &mut self,
        index: usize,
        muted: bool,
    ) -> bool {
        let cid = ret_none!(self.id(index), false);

        spawn!(
            ret_err!(heraldcore::conversation::set_muted(&cid, muted)),
            false
        );

        ret_none!(self.set_muted_inner(index, muted), false);

        true
    }

    pub(crate) fn picture_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.picture_inner(index)?
    }

    pub(crate) fn set_picture_(
        &mut self,
        index: usize,
        picture: Option<String>,
    ) -> bool {
        if self.pairwise_inner(index).unwrap_or(false) {
            return false;
        }

        let cid = ret_none!(self.id(index), false);

        // FIXME exception safety
        let path = ret_err!(
            conversation::set_picture(&cid, picture.as_ref().map(|p| p.as_str())),
            false
        );

        self.set_picture_inner(index, path);
        true
    }

    pub(crate) fn title_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.title_inner(index)?
    }

    pub(crate) fn set_title_(
        &mut self,
        index: usize,
        title: Option<String>,
    ) -> bool {
        let cid = ret_none!(self.id(index), false);
        {
            let title = title.clone();
            spawn!(
                {
                    use heraldcore::conversation::settings::*;
                    let update = SettingsUpdate::Title(title);

                    ret_err!(apply(&update, &cid));
                    ret_err!(send_update(update, &cid));
                },
                false
            );
        }

        self.set_title_inner(index, title);
        true
    }

    pub(crate) fn pairwise_(
        &self,
        index: usize,
    ) -> bool {
        ret_none!(self.pairwise_inner(index), false)
    }

    pub(crate) fn remove_conversation_(
        &mut self,
        index: u64,
    ) -> bool {
        let index = index as usize;
        let cid = ret_none!(self.id(index), false);

        // cannot remove pairwise conversation!
        if self.pairwise_inner(index).unwrap_or(false) {
            return false;
        }

        spawn!(ret_err!(conversation::delete_conversation(&cid)), false);

        self.model.begin_remove_rows(index, index);
        self.list.remove(index);
        self.model.end_remove_rows();

        true
    }

    pub(crate) fn matched_(
        &self,
        row_index: usize,
    ) -> bool {
        ret_none!(self.list.get(row_index), true).matched
    }

    pub(crate) fn filter_(&self) -> &str {
        self.filter.as_ref().map(SearchPattern::raw).unwrap_or("")
    }

    pub(crate) fn set_filter_(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_filter_();
            return;
        }

        let pattern = if self.filter_regex_() {
            ret_err!(SearchPattern::new_regex(pattern))
        } else {
            ret_err!(SearchPattern::new_normal(pattern))
        };

        self.filter.replace(pattern);
        self.emit.filter_changed();

        self.inner_filter();
    }

    /// Indicates whether regex search is activated
    pub(crate) fn filter_regex_(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    pub(crate) fn set_filter_regex_(
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
    pub(crate) fn toggle_filter_regex_(&mut self) -> bool {
        let toggled = !self.filter_regex;
        self.set_filter_regex_(toggled);
        toggled
    }

    pub(crate) fn clear_filter_(&mut self) {
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

    pub(crate) fn index_by_id_(
        &self,
        cid: ffi::ConversationIdRef,
    ) -> u64 {
        let ret_val = std::u32::MAX as u64;
        let conversation_id = ret_err!(ConversationId::try_from(cid), ret_val);
        self.list
            .iter()
            .position(|super::Conversation { id, .. }| id == &conversation_id)
            .map(|n| n as u64)
            .unwrap_or(ret_val)
    }
}
