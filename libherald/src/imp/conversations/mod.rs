use crate::{ffi, interface::*, ret_err, ret_none};
use heraldcore::{
    abort_err,
    conversation::{self, ConversationMeta},
    errors::HErr,
    types::{ConversationId, ExpirationPeriod},
    utils::SearchPattern,
};
use im_rc::vector::Vector;

pub(crate) mod shared;
use shared::*;

/// Thin wrapper around `ConversationMeta`,
/// with an additional field to facilitate filtering
/// in the UI.
#[derive(Clone)]
pub struct Conversation {
    inner: ConversationMeta,
    matched: bool,
}

/// A wrapper around a vector of `Conversation`, with additional
/// fields to facilitate interaction with Qt.
pub struct Conversations {
    emit: ConversationsEmitter,
    model: ConversationsList,
    filter: SearchPattern,
    filter_regex: bool,
    list: Vector<Conversation>,
}

impl Conversations {
    fn raw_fetch_and_insert(&mut self, cid: ConversationId) -> Result<(), HErr> {
        let meta = conversation::meta(&cid)?;
        let conv = Conversation {
            matched: meta.matches(&self.filter),
            inner: meta,
        };
        self.model.begin_insert_rows(0, 0);
        self.list.push_front(conv);
        self.model.end_insert_rows();
        Ok(())
    }
}

impl ConversationsTrait for Conversations {
    fn new(mut emit: ConversationsEmitter, model: ConversationsList) -> Self {
        let list = abort_err!(conversation::all_meta())
            .into_iter()
            .map(|inner| Conversation {
                inner,
                matched: true,
            })
            .collect();

        let filter = abort_err!(SearchPattern::new_normal("".into()));

        let global_emit = emit.clone();

        CONV_EMITTER.lock().replace(global_emit);

        Self {
            emit,
            filter,
            filter_regex: false,
            model,
            list,
        }
    }

    fn emit(&mut self) -> &mut ConversationsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn color(&self, index: usize) -> u32 {
        ret_none!(self.list.get(index), 0).inner.color
    }

    fn set_color(&mut self, index: usize, color: u32) -> bool {
        let meta = &mut ret_none!(self.list.get_mut(index), false).inner;
        ret_err!(conversation::set_color(&meta.conversation_id, color), false);

        meta.color = color;
        true
    }

    fn conversation_id(&self, index: usize) -> ffi::ConversationIdRef {
        ret_none!(self.list.get(index), &[])
            .inner
            .conversation_id
            .as_slice()
    }

    fn expiration_period(&self, index: usize) -> u8 {
        ret_none!(self.list.get(index), ExpirationPeriod::default() as u8)
            .inner
            .expiration_period as u8
    }

    fn set_expiration_period(&mut self, index: usize, period: u8) -> bool {
        let meta = &mut ret_none!(self.list.get_mut(index), false).inner;
        let period = period.into();
        ret_err!(
            conversation::set_expiration_period(&meta.conversation_id, &period),
            false
        );

        meta.expiration_period = period;

        true
    }

    fn muted(&self, index: usize) -> bool {
        ret_none!(self.list.get(index), true).inner.muted
    }

    fn set_muted(&mut self, index: usize, muted: bool) -> bool {
        let meta = &mut ret_none!(self.list.get_mut(index), false).inner;
        ret_err!(conversation::set_muted(&meta.conversation_id, muted), false);

        meta.muted = muted;
        true
    }

    fn picture(&self, index: usize) -> Option<&str> {
        // Note: this should not be using the `?` operator
        ret_none!(self.list.get(index), None)
            .inner
            .picture
            .as_ref()
            .map(|p| p.as_str())
    }

    fn set_picture(&mut self, index: usize, picture: Option<String>) -> bool {
        let meta = &mut ret_none!(self.list.get_mut(index), false).inner;
        ret_err!(
            conversation::set_picture(
                &meta.conversation_id,
                picture.as_ref().map(|p| p.as_str()),
                meta.picture.as_ref().map(|p| p.as_str())
            ),
            false
        );

        meta.picture = picture;
        true
    }

    fn title(&self, index: usize) -> Option<&str> {
        self.list
            .get(index)?
            .inner
            .title
            .as_ref()
            .map(|t| t.as_str())
    }

    fn set_title(&mut self, index: usize, title: Option<String>) -> bool {
        let meta = &mut ret_none!(self.list.get_mut(index), false).inner;
        ret_err!(
            conversation::set_title(&meta.conversation_id, title.as_ref().map(|t| t.as_str())),
            false
        );

        meta.title = title;
        true
    }

    fn pairwise(&self, index: usize) -> bool {
        ret_none!(self.list.get(index), false).inner.pairwise
    }

    fn remove_conversation(&mut self, index: u64) -> bool {
        let index = index as usize;
        let meta = &mut ret_none!(self.list.get_mut(index), false).inner;

        // cannot remove pairwise conversation!
        if meta.pairwise {
            return false;
        }

        ret_err!(
            conversation::delete_conversation(&meta.conversation_id),
            false
        );

        self.model.begin_remove_rows(index, index);
        self.list.remove(index);
        self.model.end_remove_rows();

        true
    }

    fn matched(&self, row_index: usize) -> bool {
        ret_none!(self.list.get(row_index), true).matched
    }

    fn set_matched(&mut self, row_index: usize, value: bool) -> bool {
        ret_none!(self.list.get_mut(row_index), false).matched = value;
        true
    }

    fn filter(&self) -> &str {
        self.filter.raw()
    }

    fn set_filter(&mut self, pattern: String) {
        if pattern.is_empty() {
            self.clear_filter();
            return;
        }

        let pattern = if self.filter_regex() {
            ret_err!(SearchPattern::new_regex(pattern))
        } else {
            ret_err!(SearchPattern::new_normal(pattern))
        };

        self.filter = pattern;
        self.emit.filter_changed();

        self.inner_filter();
    }

    fn can_fetch_more(&self) -> bool {
        !CONV_BUS.rx.is_empty()
    }

    fn fetch_more(&mut self) {
        use ConvUpdates::*;
        // TODO these ret_err's should probably just push to the error queue
        for update in CONV_BUS.rx.try_iter() {
            match update {
                NewConversation(cid) => ret_err!(self.raw_fetch_and_insert(cid)),
                BuilderFinished(cid) => ret_err!(self.raw_fetch_and_insert(cid)),
                NewActivity(cid) => {
                    let pos = ret_none!(self
                        .list
                        .iter()
                        .position(|c| c.inner.conversation_id == cid));

                    // NOTE: This is very important. If this check isn't here,
                    // the program will crash.
                    if pos == 0 {
                        return;
                    }

                    self.model.begin_move_rows(pos, pos, 0);
                    let conv = self.list.remove(pos);
                    self.list.push_front(conv);
                    self.model.end_move_rows();
                }
                Settings(cid, settings) => {
                    let pos = ret_none!(self
                        .list
                        .iter()
                        .position(|c| c.inner.conversation_id == cid));

                    use conversation::settings::SettingsUpdate;
                    match settings {
                        SettingsUpdate::Expiration(period) => {
                            self.list[pos].inner.expiration_period = period;
                            self.model.data_changed(pos, pos);
                        }
                    }
                }
            }
        }
    }

    /// Indicates whether regex search is activated
    fn filter_regex(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    fn set_filter_regex(&mut self, use_regex: bool) {
        if use_regex {
            ret_err!(self.filter.regex_mode());
        } else {
            ret_err!(self.filter.normal_mode());
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
}

impl Conversations {
    fn clear_filter(&mut self) {
        for conv in self.list.iter_mut() {
            conv.matched = true;
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));

        if self.filter_regex {
            self.filter = ret_err!(SearchPattern::new_regex("".to_owned()));
        } else {
            self.filter = ret_err!(SearchPattern::new_normal("".to_owned()));
        }

        self.emit.filter_changed();
    }

    fn inner_filter(&mut self) {
        for conv in self.list.iter_mut() {
            conv.matched = conv.inner.matches(&self.filter);
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));
    }
}
