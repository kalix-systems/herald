use crate::{
    ffi,
    interface::*,
    ret_err, ret_none,
    shared::{user_global::*, USER_DATA},
};
use herald_common::UserId;
use heraldcore::{
    abort_err,
    contact::{self, ContactBuilder, ContactStatus},
    utils::SearchPattern,
};
use std::convert::{TryFrom, TryInto};

type Emitter = UsersEmitter;
type List = UsersList;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
/// Thin wrapper around `heraldcore::contact::Contact`,
/// with an additional field to facilitate filtering
/// in the UI.
pub struct User {
    id: UserId,
    matched: bool,
}

/// A wrapper around a vector of `User`s, with additional
/// fields to facilitate interaction with Qt.
pub struct Users {
    emit: Emitter,
    model: List,
    filter: SearchPattern,
    filter_regex: bool,
    list: Vec<User>,
}

fn color(uid: &UserId) -> Option<u32> {
    Some(USER_DATA.get(&uid)?.color)
}

fn name(uid: &UserId) -> Option<String> {
    let inner = USER_DATA.get(uid)?;

    Some(inner.name.clone())
}

fn profile_picture(uid: &UserId) -> Option<String> {
    let inner = USER_DATA.get(uid)?;

    inner.profile_picture.clone()
}

impl UsersTrait for Users {
    fn new(mut emit: Emitter, model: List) -> Users {
        let list = match contact::all() {
            Ok(v) => v
                .into_iter()
                .map(|u| {
                    let id = u.id;
                    USER_DATA.insert(id, u);
                    User { id, matched: true }
                })
                .collect(),
            Err(e) => {
                eprintln!("{}", e);
                Vec::new()
            }
        };

        let global_emit = emit.clone();

        USER_EMITTER.lock().replace(global_emit);
        // this should *really* never fail
        let filter = abort_err!(SearchPattern::new_normal("".into()));

        Users {
            emit,
            model,
            list,
            filter,
            filter_regex: false,
            // try_poll: USER_TRY_POLL.clone(),
        }
    }

    /// Adds a contact by their `id`
    fn add(&mut self, id: ffi::UserId) -> ffi::ConversationId {
        let id = ret_err!(id.as_str().try_into(), ffi::NULL_CONV_ID.to_vec());
        let contact = ret_err!(ContactBuilder::new(id).add(), ffi::NULL_CONV_ID.to_vec());

        let pairwise_conversation = contact.pairwise_conversation;

        let user = User {
            matched: contact.matches(&self.filter),
            id: contact.id,
        };

        let pos = match self.list.binary_search(&user) {
            Ok(_) => return pairwise_conversation.to_vec(),
            Err(pos) => pos,
        };

        self.model.begin_insert_rows(pos, pos);
        self.list.insert(pos, user);
        USER_DATA.insert(contact.id, contact);
        self.model.end_insert_rows();

        pairwise_conversation.to_vec()
    }

    /// Returns user id.
    fn user_id(&self, row_index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), "").id.as_str()
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(&self, row_index: usize) -> ffi::ConversationId {
        let uid = &ret_none!(self.list.get(row_index), ffi::NULL_CONV_ID.to_vec()).id;
        let inner = ret_none!(USER_DATA.get(uid), ffi::NULL_CONV_ID.to_vec());
        inner.pairwise_conversation.to_vec()
    }

    /// Returns users name
    fn name(&self, row_index: usize) -> String {
        let uid = &ret_none!(self.list.get(row_index), "".to_owned()).id;

        ret_none!(name(uid), uid.to_string())
    }

    /// Returns name if it is set, otherwise returns empty string
    fn name_by_id(&self, id: ffi::UserId) -> String {
        let uid = &ret_err!(id.as_str().try_into(), "".to_owned());
        name(uid).unwrap_or_else(|| "".to_owned())
    }

    /// Updates a user's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: String) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(USER_DATA.get_mut(&uid), false);
        ret_err!(contact::set_name(uid, name.as_str()), false);

        inner.name = name;
        true
    }

    /// Returns profile picture
    fn profile_picture(&self, row_index: usize) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        profile_picture(&uid)
    }

    /// Returns path to profile if it is set, otherwise returns the empty string.
    fn profile_picture_by_id(&self, id: ffi::UserId) -> String {
        let uid = &ret_err!(id.as_str().try_into(), "".to_owned());
        profile_picture(uid).unwrap_or_else(|| "".to_owned())
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
        color(&uid).unwrap_or(0)
    }

    /// Returns name if it is set, otherwise returns the user's id.
    fn color_by_id(&self, id: ffi::UserId) -> u32 {
        let uid = &ret_err!(id.as_str().try_into(), 0);
        ret_none!(color(&uid), 0)
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

    fn poll_update(&mut self) -> bool {
        for update in USER_CHANNEL.rx.try_recv() {
            match update {
                UsersUpdates::NewUser(uid) => {
                    let new_contact = ret_err!(contact::by_user_id(uid), false);

                    let new_user = User {
                        matched: new_contact.matches(&self.filter),
                        id: uid,
                    };

                    let pos = match self.list.binary_search(&new_user) {
                        Ok(_) => return true, // this should never happen
                        Err(pos) => pos,
                    };

                    self.model.begin_insert_rows(pos, pos);
                    self.list.push(new_user);
                    USER_DATA.insert(uid, new_contact);
                    self.model.end_insert_rows();
                }
                UsersUpdates::ReqResp(uid, accepted) => {
                    if accepted {
                        println!("PLACEHOLDER: {} accepted your contact request", uid);
                    } else {
                        println!("PLACEHOLDER: {} did not accept your contact request", uid);
                    }
                }
            }
        }

        true
    }
}

impl Users {
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
