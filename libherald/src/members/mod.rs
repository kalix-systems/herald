use crate::{ffi, interface::*, ret_err, ret_none, users::shared::get_user};
use herald_common::UserId;
use heraldcore::{types::*, user};
use search_pattern::SearchPattern;
use std::convert::{TryFrom, TryInto};

type Emitter = MembersEmitter;
type List = MembersList;

pub(crate) mod imp;

#[derive(Clone)]
/// Thin wrapper around `heraldcore::user::Contact`,
/// with an additional field to facilitate filtering
/// in the UI.
pub struct User {
    id: UserId,
    matched: bool,
}

/// A wrapper around a vector of `User`s, with additional
/// fields to facilitate interaction with Qt.
pub struct Members {
    emit: Emitter,
    model: List,
    filter: Option<SearchPattern>,
    filter_regex: bool,
    list: Vec<User>,
    // Note: this is not really optional, but it is difficult to
    // pass as an argument.
    conversation_id: Option<ConversationId>,
}

impl MembersTrait for Members {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Members {
        // this should *really* never fail
        let filter = SearchPattern::new_normal("".into()).ok();

        Members {
            emit,
            model,
            list: Vec::new(),
            filter,
            filter_regex: false,
            conversation_id: None,
        }
    }

    /// Returns user id.
    fn user_id(
        &self,
        row_index: usize,
    ) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), ffi::NULL_USER_ID)
            .id
            .as_str()
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(
        &self,
        row_index: usize,
    ) -> ffi::ConversationId {
        let uid = &ret_none!(self.list.get(row_index), ffi::NULL_CONV_ID.to_vec()).id;
        let inner = ret_none!(get_user(uid), ffi::NULL_CONV_ID.to_vec());
        inner.pairwise_conversation.to_vec()
    }

    /// Returns users name
    fn name(
        &self,
        row_index: usize,
    ) -> String {
        let uid = &ret_none!(self.list.get(row_index), "".to_owned()).id;
        let inner = ret_none!(get_user(uid), uid.to_string());

        inner.name.clone()
    }

    /// Returns profile picture
    fn profile_picture(
        &self,
        row_index: usize,
    ) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        let inner = get_user(uid)?;
        inner.profile_picture.clone()
    }

    /// Returns user's color
    fn color(
        &self,
        row_index: usize,
    ) -> u32 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(get_user(&uid), 0);
        inner.color
    }

    fn status(
        &self,
        row_index: usize,
    ) -> u8 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(get_user(&uid), 0);
        inner.status as u8
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

        self.filter = match self.filter.take() {
            Some(mut filter) => {
                ret_err!(filter.set_pattern(pattern));
                Some(filter)
            }
            None => SearchPattern::new_regex(pattern).ok(),
        };

        self.emit.filter_changed();

        self.inner_filter();
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
        self.filter = match self.filter.take() {
            Some(mut filter) => {
                if use_regex {
                    ret_err!(filter.regex_mode());
                } else {
                    ret_err!(filter.normal_mode());
                }
                Some(filter)
            }
            None => None,
        };

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

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn add_to_conversation(
        &mut self,
        user_id: ffi::UserId,
    ) -> bool {
        let user_id = ret_err!(user_id.as_str().try_into(), false);
        let conv_id = ret_none!(self.conversation_id, false);
        ret_err!(heraldcore::members::add_member(&conv_id, user_id), false);

        let user = ret_none!(get_user(&user_id), false);
        self.model
            .begin_insert_rows(self.list.len(), self.list.len());
        self.list.push(User {
            matched: self
                .filter
                .as_ref()
                .map(|filter| user.matches(filter))
                .unwrap_or(true),
            id: user_id,
        });
        self.model.end_insert_rows();
        true
    }

    fn remove_from_conversation_by_index(
        &mut self,
        index: u64,
    ) -> bool {
        let index = index as usize;
        let conv_id = ret_err!(
            ConversationId::try_from(ret_none!(self.conversation_id, false)),
            false
        );
        let uid = ret_none!(self.list.get(index), false).id;

        ret_err!(heraldcore::members::remove_member(&conv_id, uid), false);
        true
    }
}

impl Members {
    fn clear_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            if !user.matched {
                user.matched = true;
                self.model.data_changed(ix, ix);
            }
        }

        if let Some(filter) = self.filter.as_mut() {
            ret_err!(filter.set_pattern("".into()));
            self.emit.filter_changed();
        }
    }

    fn inner_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            let inner = ret_none!(get_user(&user.id));
            user.matched = self
                .filter
                .as_ref()
                .map(|filter| inner.matches(filter))
                .unwrap_or(true);
            self.model.data_changed(ix, ix);
        }
    }

    pub(crate) fn process_update(
        &mut self,
        update: MemberUpdate,
    ) {
        use MemberUpdate::*;

        match update {
            ReqResp(uid, accepted) => {
                if accepted {
                    let matched = match get_user(&uid) {
                        Some(meta) => self
                            .filter
                            .as_ref()
                            .map(|filter| meta.matches(filter))
                            .unwrap_or(true),
                        None => return,
                    };

                    let user = User { matched, id: uid };
                    self.list.push(user);
                } else {
                    println!("PLACEHOLDER: {} is too good for your group chat", uid);
                }
            }
        }
    }
}

/// Conversation member related updates
pub enum MemberUpdate {
    /// Response to a conversation add request
    ReqResp(UserId, bool),
}
