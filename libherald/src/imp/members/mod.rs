use crate::{ffi, imp::users::shared::get_user, interface::*, ret_err, ret_none};
use herald_common::UserId;
use heraldcore::{types::*, user, utils::SearchPattern};
use std::{
    convert::{TryFrom, TryInto},
    ops::Drop,
};

type Emitter = MembersEmitter;
type List = MembersList;

pub(crate) mod shared;

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
        if let (Some(id), None) = (conversation_id, self.conversation_id) {
            let conversation_id = ret_err!(ConversationId::try_from(id));

            shared::EMITTERS.insert(conversation_id, self.emit().clone());
            let list = ret_err!(user::conversation_members(&conversation_id));

            self.model
                .begin_insert_rows(0, list.len().saturating_sub(1));
            self.list = list
                .into_iter()
                .map(|u| {
                    let id = u.id;
                    User { id, matched: true }
                })
                .collect();
            self.model.end_insert_rows();

            self.emit.conversation_id_changed();
        }
    }

    /// Returns user id.
    fn user_id(&self, row_index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), ffi::NULL_USER_ID)
            .id
            .as_str()
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(&self, row_index: usize) -> ffi::ConversationId {
        let uid = &ret_none!(self.list.get(row_index), ffi::NULL_CONV_ID.to_vec()).id;
        let inner = ret_none!(get_user(uid), ffi::NULL_CONV_ID.to_vec());
        inner.pairwise_conversation.to_vec()
    }

    /// Returns users name
    fn name(&self, row_index: usize) -> String {
        let uid = &ret_none!(self.list.get(row_index), "".to_owned()).id;
        let inner = ret_none!(get_user(uid), uid.to_string());

        inner.name.clone()
    }

    /// Returns profile picture
    fn profile_picture(&self, row_index: usize) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        let inner = get_user(uid)?;
        inner.profile_picture.clone()
    }

    /// Returns user's color
    fn color(&self, row_index: usize) -> u32 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(get_user(&uid), 0);
        inner.color
    }

    fn status(&self, row_index: usize) -> u8 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(get_user(&uid), 0);
        inner.status as u8
    }

    fn matched(&self, row_index: usize) -> bool {
        ret_none!(self.list.get(row_index), true).matched
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

        let user = ret_none!(get_user(&user_id), false);
        self.model
            .begin_insert_rows(self.list.len(), self.list.len());
        self.list.push(User {
            matched: user.matches(&self.filter),
            id: user_id,
        });
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

    fn can_fetch_more(&self) -> bool {
        let cid = &ret_none!(self.conversation_id, false);
        let rx = match shared::RXS.get(&cid) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return false,
        };

        !rx.is_empty()
    }

    fn fetch_more(&mut self) {
        let cid = &ret_none!(self.conversation_id);
        let rx = ret_none!(shared::RXS.get(cid));

        use shared::MemberUpdate::*;

        for update in rx.try_iter() {
            match update {
                ReqResp(uid, accepted) => {
                    if accepted {
                        let matched = match get_user(&uid) {
                            Some(meta) => meta.matches(&self.filter),
                            None => continue,
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
}

impl Members {
    fn clear_filter(&mut self) {
        for user in self.list.iter_mut() {
            user.matched = true;
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
        for user in self.list.iter_mut() {
            let inner = ret_none!(get_user(&user.id));
            user.matched = inner.matches(&self.filter);
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));
    }
}

impl Drop for Members {
    fn drop(&mut self) {
        use shared::*;
        if let Some(cid) = self.conversation_id {
            EMITTERS.remove(&cid);
            TXS.remove(&cid);
            RXS.remove(&cid);
        }
    }
}
