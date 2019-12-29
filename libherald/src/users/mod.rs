use crate::{
    err, ffi,
    interface::{UsersEmitter as Emitter, UsersList as List, UsersTrait as Interface},
    none, spawn,
};
use herald_common::UserId;
use heraldcore::{
    network,
    user::{self, UserBuilder, UserStatus},
};
use search_pattern::SearchPattern;
use std::convert::{TryFrom, TryInto};

pub(crate) mod shared;
use shared::*;
mod imp;
mod trait_imp;

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
    loaded: bool,
}

impl Interface for Users {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Users {
        // this should *really* never fail
        let filter = SearchPattern::new_normal("".into()).ok();

        Users {
            emit,
            model,
            list: Vec::new(),
            filter,
            filter_regex: false,
            loaded: false,
        }
    }

    /// Adds a user by their `id`
    fn add(
        &mut self,
        id: ffi::UserId,
    ) -> ffi::ConversationId {
        let id = err!(id.as_str().try_into(), ffi::NULL_CONV_ID.to_vec());
        let (data, _) = err!(UserBuilder::new(id).add(), ffi::NULL_CONV_ID.to_vec());

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

        {
            shared::user_data().write().insert(data.id, data);
        }

        self.model.end_insert_rows();

        spawn!(
            err!(network::send_user_req(id, pairwise_conversation)),
            ffi::NULL_CONV_ID.to_vec()
        );

        pairwise_conversation.to_vec()
    }

    /// Returns user id.
    fn user_id(
        &self,
        row_index: usize,
    ) -> ffi::UserIdRef {
        none!(self.list.get(row_index), "").id.as_str()
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(
        &self,
        row_index: usize,
    ) -> ffi::ConversationId {
        let uid = &none!(self.list.get(row_index), ffi::NULL_CONV_ID.to_vec()).id;
        none!(shared::pairwise_cid(uid), ffi::NULL_CONV_ID.to_vec()).to_vec()
    }

    /// Returns users name
    fn name(
        &self,
        row_index: usize,
    ) -> String {
        let uid = &none!(self.list.get(row_index), "".to_owned()).id;

        none!(shared::name(uid), uid.to_string())
    }

    /// Returns name if it is set, otherwise returns empty string
    fn name_by_id(
        &self,
        id: ffi::UserId,
    ) -> String {
        let uid = &err!(id.as_str().try_into(), "".to_owned());
        name(uid).unwrap_or_else(|| "".to_owned())
    }

    /// Updates a user's name, returns a boolean to indicate success.
    fn set_name(
        &mut self,
        row_index: usize,
        name: String,
    ) -> bool {
        let uid = none!(self.list.get(row_index), false).id;
        {
            let name = name.clone();
            spawn!(user::set_name(uid, name.as_str()), false);
        }

        {
            let mut lock = user_data().write();
            let mut inner = none!(lock.get_mut(&uid), false);
            inner.name = name;
        }
        true
    }

    /// Returns profile picture
    fn profile_picture(
        &self,
        row_index: usize,
    ) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        profile_picture(&uid)
    }

    /// Returns path to profile if it is set, otherwise returns the empty string.
    fn profile_picture_by_id(
        &self,
        id: ffi::UserId,
    ) -> String {
        let uid = &err!(id.as_str().try_into(), "".to_owned());
        profile_picture(uid).unwrap_or_else(|| "".to_owned())
    }

    /// Sets profile picture.
    ///
    /// Returns bool indicating success.
    fn set_profile_picture(
        &mut self,
        index: u64,
        picture_json: String,
    ) {
        let index = index as usize;

        let uid = none!(self.list.get(index)).id;

        spawn!({
            let profile_picture =
                heraldcore::image_utils::ProfilePicture::from_json_string(picture_json);

            let path = err!(user::set_profile_picture(uid, profile_picture));

            {
                let mut lock = user_data().write();
                let mut inner = none!(lock.get_mut(&uid));
                inner.profile_picture = path;
            }

            crate::push(UserUpdate::DataChanged(uid));
        });
    }

    /// Returns user's color
    fn color(
        &self,
        row_index: usize,
    ) -> u32 {
        let uid = none!(self.list.get(row_index), 0).id;
        color(&uid).unwrap_or(0)
    }

    /// Returns name if it is set, otherwise returns the user's id.
    fn color_by_id(
        &self,
        id: ffi::UserId,
    ) -> u32 {
        let uid = &err!(id.as_str().try_into(), 0);

        color(&uid).unwrap_or(0)
    }

    /// Sets color
    fn set_color(
        &mut self,
        row_index: usize,
        color: u32,
    ) -> bool {
        let uid = none!(self.list.get(row_index), false).id;

        spawn!(user::set_color(uid, color), false);

        {
            let mut lock = user_data().write();
            let mut inner = none!(lock.get_mut(&uid), false);
            inner.color = color;
        }
        true
    }

    fn status(
        &self,
        row_index: usize,
    ) -> u8 {
        let uid = none!(self.list.get(row_index), 0).id;
        none!(status(&uid), 0) as u8
    }

    fn set_status(
        &mut self,
        row_index: usize,
        status: u8,
    ) -> bool {
        let status = err!(UserStatus::try_from(status), false);
        let uid = none!(self.list.get(row_index), false).id;

        spawn!(user::set_status(uid, status), false);

        {
            let mut lock = user_data().write();
            let mut inner = none!(lock.get_mut(&uid), false);
            inner.status = status;
        }

        if status == UserStatus::Deleted {
            self.model.begin_remove_rows(row_index, row_index);
            self.list.remove(row_index);
            user_data().write().remove(&uid);
            self.model.end_remove_rows();
        }

        true
    }

    fn matched(
        &self,
        row_index: usize,
    ) -> bool {
        none!(self.list.get(row_index), true).matched
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
            Some(mut pat) => {
                err!(pat.set_pattern(pattern));
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
    fn set_filter_regex(
        &mut self,
        use_regex: bool,
    ) {
        self.filter = match self.filter.take() {
            Some(mut filter) => {
                if use_regex {
                    err!(filter.regex_mode());
                    Some(filter)
                } else {
                    err!(filter.normal_mode());
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
