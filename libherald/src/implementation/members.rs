use crate::{
    ffi,
    interface::*,
    ret_err, ret_none,
    shared::{members::*, USER_DATA},
};
use herald_common::UserId;
use heraldcore::{
    abort_err,
    contact::{self, ContactStatus},
    types::*,
    utils::SearchPattern,
};
use std::convert::{TryFrom, TryInto};

type Emitter = MembersEmitter;
type List = MembersList;

#[derive(Clone)]
/// Thin wrapper around `heraldcore::contact::Contact`,
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
    filter: SearchPattern,
    filter_regex: bool,
    list: Vec<User>,
    // Note: this is not really optional, but it is difficult to
    // pass as an argument.
    conversation_id: Option<ConversationId>,
}

impl MembersTrait for Members {
    fn new(emit: Emitter, model: List) -> Members {
        // this should *really* never fail
        let filter = abort_err!(SearchPattern::new_normal("".into()));

        Members {
            emit,
            model,
            list: Vec::new(),
            filter,
            filter_regex: false,
            conversation_id: None,
        }
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|id| id.as_slice())
    }

    fn set_conversation_id(&mut self, conversation_id: Option<ffi::ConversationIdRef>) {
        if self.conversation_id.is_some() {
            eprintln!("Cannot modify conversation id");
            return;
        }

        let new_list = match conversation_id {
            Some(conv_id) => {
                let conv_id = ret_err!(ConversationId::try_from(conv_id));

                ret_err!(contact::conversation_members(&conv_id))
            }
            None => {
                return;
            }
        };

        self.model
            .begin_insert_rows(0, new_list.len().saturating_sub(1));
        let list = new_list
            .into_iter()
            .map(|u| {
                let id = u.id;
                User { id, matched: true }
            })
            .collect();
        self.list = list;
        self.model.end_insert_rows();

        self.emit.conversation_id_changed();
    }

    /// Returns user id.
    fn user_id(&self, row_index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), ffi::NULL_USER_ID)
            .id
            .as_str()
    }

    /// Returns name if it is set, otherwise returns the user's id.
    fn display_name(&self, row_index: usize) -> String {
        let uid = &ret_none!(self.list.get(row_index), "".to_owned()).id;
        let inner = ret_none!(USER_DATA.get(uid), "".to_owned());
        match inner.name.as_ref() {
            Some(name) => name.clone(),
            None => inner.id.to_string(),
        }
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(&self, row_index: usize) -> ffi::ConversationId {
        let uid = &ret_none!(self.list.get(row_index), ffi::NULL_CONV_ID.to_vec()).id;
        let inner = ret_none!(USER_DATA.get(uid), ffi::NULL_CONV_ID.to_vec());
        inner.pairwise_conversation.to_vec()
    }

    /// Returns users name
    fn name(&self, row_index: usize) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        let inner = USER_DATA.get(uid)?;

        inner.name.clone()
    }

    /// Updates a user's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: Option<String>) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(USER_DATA.get_mut(&uid), false);
        ret_err!(
            contact::set_name(uid, name.as_ref().map(|s| s.as_str())),
            false
        );

        // already checked
        inner.name = name;
        true
    }

    /// Returns profile picture
    fn profile_picture(&self, row_index: usize) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        let inner = USER_DATA.get(uid)?;
        inner.profile_picture.clone()
    }

    /// Sets profile picture.
    ///
    /// Returns bool indicating success.
    fn set_profile_picture(&mut self, row_index: usize, picture: Option<String>) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(USER_DATA.get_mut(&uid), false);
        let path = ret_err!(
            contact::set_profile_picture(
                uid,
                crate::utils::strip_qrc(picture),
                inner.profile_picture.as_ref().map(String::as_str),
            ),
            false
        );

        inner.profile_picture = path;
        true
    }

    /// Returns user's color
    fn color(&self, row_index: usize) -> u32 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(USER_DATA.get(&uid), 0);
        inner.color
    }

    /// Sets color
    fn set_color(&mut self, row_index: usize, color: u32) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(USER_DATA.get_mut(&uid), false);

        ret_err!(contact::set_color(uid, color), false);

        inner.color = color;
        true
    }

    fn status(&self, row_index: usize) -> u8 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(USER_DATA.get(&uid), 0);
        inner.status as u8
    }

    fn set_status(&mut self, row_index: usize, status: u8) -> bool {
        let status = ret_err!(ContactStatus::try_from(status), false);
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(USER_DATA.get_mut(&uid), false);

        ret_err!(contact::set_status(uid, status), false);

        inner.status = status;

        if status == ContactStatus::Deleted {
            self.model.begin_remove_rows(row_index, row_index);
            self.list.remove(row_index);
            USER_DATA.remove(&uid);
            self.model.end_remove_rows();
        }

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

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn add_to_conversation(&mut self, user_id: ffi::UserId) -> bool {
        let user_id = ret_err!(user_id.as_str().try_into(), false);
        let conv_id = ret_none!(self.conversation_id, false);
        ret_err!(heraldcore::members::add_member(&conv_id, user_id), false);

        let contact = ret_err!(heraldcore::contact::by_user_id(user_id), false);
        self.model
            .begin_insert_rows(self.list.len(), self.list.len());
        self.list.push(User {
            matched: contact.matches(&self.filter),
            id: user_id,
        });
        USER_DATA.insert(user_id, contact);
        self.model.end_insert_rows();
        true
    }

    fn remove_from_conversation_by_index(&mut self, index: u64) -> bool {
        let index = index as usize;
        let conv_id = ret_err!(
            ConversationId::try_from(ret_none!(self.conversation_id, false)),
            false
        );
        let uid = ret_none!(self.list.get(index), false).id;

        ret_err!(heraldcore::members::remove_member(&conv_id, uid), false);
        true
    }

    fn poll_update(&mut self) -> bool {
        let cid = &ret_none!(self.conversation_id, false);
        let rx = ret_none!(MEMBER_RXS.get(cid), false);

        use MemberUpdate::*;
        for update in rx.try_iter() {
            match update {
                ReqResp(uid, _accepted) => {
                    // TODO use accepted somehow
                    let matched = match USER_DATA.get(&uid) {
                        Some(meta) => meta.matches(&self.filter),
                        None => continue,
                    };

                    let user = User { matched, id: uid };
                    self.list.push(user);
                }
            }
        }
        true
    }
}

impl Members {
    fn clear_filter(&mut self) {
        for contact in self.list.iter_mut() {
            contact.matched = true;
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
        for contact in self.list.iter_mut() {
            let inner = ret_none!(USER_DATA.get(&contact.id));
            contact.matched = inner.matches(&self.filter);
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));
    }
}
