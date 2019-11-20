use crate::{ffi, interface::*, ret_err, ret_none, spawn};
use herald_common::UserId;
use heraldcore::{
    network,
    user::{self, UserBuilder, UserStatus},
};
use search_pattern::SearchPattern;
use std::convert::{TryFrom, TryInto};

pub(crate) mod shared;
use shared::*;

type Emitter = UsersEmitter;
type List = UsersList;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
/// Thin wrapper around a `UserId`,
/// with an additional field to facilitate filtering
/// in the UI.
pub struct User {
    pub(crate) id: UserId,
    pub(crate) matched: bool,
}

/// A wrapper around a vector of `User`s, with additional
/// fields to facilitate interaction with Qt.
pub struct Users {
    emit: Emitter,
    model: List,
    filter: Option<SearchPattern>,
    filter_regex: bool,
    list: Vec<User>,
}

impl UsersTrait for Users {
    fn new(mut emit: Emitter, model: List) -> Users {
        let list = match user::all() {
            Ok(v) => v
                .into_iter()
                .map(|u| {
                    let id = u.id;
                    shared::USER_DATA.insert(id, u);
                    User { id, matched: true }
                })
                .collect(),
            Err(_) => Vec::new(),
        };

        let global_emit = emit.clone();

        shared::USER_EMITTER.lock().replace(global_emit);

        // this should *really* never fail
        let filter = SearchPattern::new_normal("".into()).ok();

        Users {
            emit,
            model,
            list,
            filter,
            filter_regex: false,
        }
    }

    /// Adds a user by their `id`
    fn add(&mut self, id: ffi::UserId) -> ffi::ConversationId {
        let id = ret_err!(id.as_str().try_into(), ffi::NULL_CONV_ID.to_vec());
        let (data, _) = ret_err!(UserBuilder::new(id).add(), ffi::NULL_CONV_ID.to_vec());

        let pairwise_conversation = data.pairwise_conversation;

        let user = User {
            matched: self
                .filter
                .as_ref()
                .map(|filter| data.matches(filter))
                .unwrap_or(true),
            id: data.id,
        };

        let pos = match self.list.binary_search(&user) {
            Ok(_) => return pairwise_conversation.to_vec(),
            Err(pos) => pos,
        };

        self.model.begin_insert_rows(pos, pos);
        self.list.insert(pos, user);
        shared::USER_DATA.insert(data.id, data);
        self.model.end_insert_rows();

        spawn!(
            ret_err!(network::send_user_req(id, pairwise_conversation)),
            ffi::NULL_CONV_ID.to_vec()
        );

        pairwise_conversation.to_vec()
    }

    /// Returns user id.
    fn user_id(&self, row_index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), "").id.as_str()
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
        let mut inner = ret_none!(get_user_mut(&uid), false);

        {
            let name = name.clone();
            spawn!(user::set_name(uid, name.as_str()), false);
        }

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
        let mut inner = ret_none!(get_user_mut(&uid), false);

        let picture = picture.map(crate::utils::strip_qrc);

        // FIXME this is not exception safe
        let path = ret_err!(user::set_profile_picture(uid, picture), false);

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
        color(&uid).unwrap_or(0)
    }

    /// Sets color
    fn set_color(&mut self, row_index: usize, color: u32) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(get_user_mut(&uid), false);

        spawn!(user::set_color(uid, color), false);

        inner.color = color;
        true
    }

    fn status(&self, row_index: usize) -> u8 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(get_user(&uid), 0);
        inner.status as u8
    }

    fn set_status(&mut self, row_index: usize, status: u8) -> bool {
        let status = ret_err!(UserStatus::try_from(status), false);
        let uid = ret_none!(self.list.get(row_index), false).id;
        let mut inner = ret_none!(get_user_mut(&uid), false);

        spawn!(user::set_status(uid, status), false);

        inner.status = status;

        if status == UserStatus::Deleted {
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

    fn filter(&self) -> &str {
        self.filter.as_ref().map(SearchPattern::raw).unwrap_or("")
    }

    fn set_filter(&mut self, pattern: String) {
        if pattern.is_empty() {
            self.clear_filter();
            return;
        }

        self.filter = match self.filter.take() {
            Some(mut pat) => {
                ret_err!(pat.set_pattern(pattern));
                Some(pat)
            }
            None => SearchPattern::new_normal(pattern).ok(),
        };

        self.emit.filter_changed();

        self.inner_filter();
    }

    /// Indicates whether regex search is activated
    fn filter_regex(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    fn set_filter_regex(&mut self, use_regex: bool) {
        self.filter = match self.filter.take() {
            Some(mut filter) => {
                if use_regex {
                    ret_err!(filter.regex_mode());
                    Some(filter)
                } else {
                    ret_err!(filter.normal_mode());
                    Some(filter)
                }
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

    fn can_fetch_more(&self) -> bool {
        !USER_BUS.rx.is_empty()
    }

    fn fetch_more(&mut self) {
        for update in USER_BUS.rx.try_iter() {
            match update {
                UsersUpdates::NewUser(data) => {
                    let new_user = User {
                        matched: self
                            .filter
                            .as_ref()
                            .map(|filter| data.matches(filter))
                            .unwrap_or(true),
                        id: data.id,
                    };

                    let pos = match self.list.binary_search(&new_user) {
                        Ok(_) => continue, // this should never happen
                        Err(pos) => pos,
                    };

                    self.model.begin_insert_rows(pos, pos);
                    self.list.push(new_user);
                    USER_DATA.insert(data.id, data);
                    self.model.end_insert_rows();
                }
                UsersUpdates::ReqResp(uid, accepted) => {
                    if accepted {
                        println!("PLACEHOLDER: {} accepted your user request", uid);
                    } else {
                        println!("PLACEHOLDER: {} did not accept your user request", uid);
                    }
                }
            }
        }
    }

    fn clear_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            if !user.matched {
                user.matched = true;
                self.model.data_changed(ix, ix);
            }
        }

        if self.filter_regex {
            self.filter = SearchPattern::new_regex("".to_owned()).ok();
        } else {
            self.filter = SearchPattern::new_normal("".to_owned()).ok();
        }

        self.emit.filter_changed();
    }
}

impl Users {
    fn inner_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            let inner = ret_none!(get_user(&user.id));
            let old_matched = user.matched;
            user.matched = self
                .filter
                .as_ref()
                .map(|filter| inner.matches(&filter))
                .unwrap_or(true);
            if user.matched != old_matched {
                self.model.data_changed(ix, ix);
            }
        }
    }
}
