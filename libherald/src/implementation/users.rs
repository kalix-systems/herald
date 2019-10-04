use crate::{ffi, interface::*, ret_err, ret_none};
use herald_common::UserId;
use heraldcore::{
    abort_err, chrono,
    contact::{self, ContactBuilder, ContactStatus},
    types::*,
    utils::SearchPattern,
};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Clone)]
/// Thin wrapper around [`heraldcore::contact::Contact`],
/// with an additional field to facilitate filtering
/// in the UI.
pub struct User {
    id: UserId,
    matched: bool,
}

/// A wrapper around a vector of [`User`]s, with additional
/// fields to facilitate interaction with Qt.
pub struct Users {
    emit: UsersEmitter,
    model: UsersList,
    filter: SearchPattern,
    filter_regex: bool,
    list: Vec<User>,
    map: HashMap<UserId, contact::Contact>,
    conversation_id: Option<ConversationId>,
    updated: chrono::DateTime<chrono::Utc>,
}

impl UsersTrait for Users {
    fn new(emit: UsersEmitter, model: UsersList) -> Users {
        let (list, map) = match contact::all() {
            Ok(v) => v
                .into_iter()
                .map(|c| {
                    (
                        User {
                            id: c.id,
                            matched: true,
                        },
                        (c.id, c),
                    )
                })
                .unzip(),
            Err(e) => {
                eprintln!("{}", e);
                (Vec::new(), HashMap::new())
            }
        };

        // this should *really* never fail
        let filter = abort_err!(SearchPattern::new_normal("".into()));

        Users {
            emit,
            model,
            list,
            map,
            filter,
            filter_regex: false,
            conversation_id: None,
            updated: chrono::Utc::now(),
        }
    }

    /// Adds a contact by their `id`
    fn add(&mut self, id: ffi::UserId) -> ffi::ConversationId {
        let id = ret_err!(id.as_str().try_into(), ffi::NULL_CONV_ID.to_vec());
        let contact = ret_err!(ContactBuilder::new(id).add(), ffi::NULL_CONV_ID.to_vec());

        let pairwise_conversation = contact.pairwise_conversation;
        self.model
            .begin_insert_rows(self.list.len(), self.list.len());
        self.list.push(User {
            matched: contact.matches(&self.filter),
            id: contact.id,
        });
        self.map.insert(contact.id, contact);
        self.model.end_insert_rows();

        pairwise_conversation.to_vec()
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|id| id.as_slice())
    }

    fn set_conversation_id(&mut self, conversation_id: Option<ffi::ConversationIdRef>) {
        let new_list = match conversation_id {
            Some(conv_id) => {
                let conv_id = ret_err!(ConversationId::try_from(conv_id));

                if Some(conv_id) == self.conversation_id {
                    return;
                }

                ret_err!(contact::conversation_members(&conv_id))
            }
            None => {
                if self.conversation_id.is_none() {
                    return;
                }
                ret_err!(contact::all())
            }
        };

        self.model
            .begin_remove_rows(0, self.row_count().saturating_sub(1));
        self.list = vec![];
        self.model.end_remove_rows();

        self.model
            .begin_insert_rows(0, new_list.len().saturating_sub(1));
        let (list, map) = new_list
            .into_iter()
            .map(|c| {
                (
                    User {
                        id: c.id,
                        matched: true,
                    },
                    (c.id, c),
                )
            })
            .unzip();
        self.list = list;
        self.map = map;
        self.model.end_insert_rows();

        self.emit.conversation_id_changed();
    }

    /// Returns user id.
    fn user_id(&self, row_index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), "").id.as_str()
    }

    /// Returns name if it is set, otherwise returns the user's id.
    fn display_name(&self, row_index: usize) -> &str {
        let uid = &ret_none!(self.list.get(row_index), "").id;
        let inner = ret_none!(self.map.get(uid), "");
        match inner.name.as_ref() {
            Some(name) => name.as_str(),
            None => inner.id.as_str(),
        }
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(&self, row_index: usize) -> ffi::ConversationIdRef {
        let uid = &ret_none!(self.list.get(row_index), &ffi::NULL_CONV_ID).id;
        let inner = ret_none!(self.map.get(uid), &ffi::NULL_CONV_ID);
        inner.pairwise_conversation.as_slice()
    }

    /// Returns users name
    fn name(&self, row_index: usize) -> Option<&str> {
        let uid = &self.list.get(row_index)?.id;
        let inner = self.map.get(uid)?;

        inner.name.as_ref().map(|n| n.as_str())
    }

    /// Updates a user's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: Option<String>) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let inner = ret_none!(self.map.get_mut(&uid), false);
        ret_err!(
            contact::set_name(uid, name.as_ref().map(|s| s.as_str())),
            false
        );

        // already checked
        inner.name = name;
        true
    }

    /// Returns profile picture
    fn profile_picture(&self, row_index: usize) -> Option<&str> {
        let uid = &self.list.get(row_index)?.id;
        let inner = self.map.get(uid)?;
        inner.profile_picture.as_ref().map(|s| s.as_str())
    }

    /// Sets profile picture.
    ///
    /// Returns bool indicating success.
    fn set_profile_picture(&mut self, row_index: usize, picture: Option<String>) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let inner = ret_none!(self.map.get_mut(&uid), false);
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
        let inner = ret_none!(self.map.get(&uid), 0);
        inner.color
    }

    /// Sets color
    fn set_color(&mut self, row_index: usize, color: u32) -> bool {
        let uid = ret_none!(self.list.get(row_index), false).id;
        let inner = ret_none!(self.map.get_mut(&uid), false);

        ret_err!(contact::set_color(uid, color), false);

        inner.color = color;
        true
    }

    fn status(&self, row_index: usize) -> u8 {
        let uid = ret_none!(self.list.get(row_index), 0).id;
        let inner = ret_none!(self.map.get(&uid), 0);
        inner.status as u8
    }

    fn set_status(&mut self, row_index: usize, status: u8) -> bool {
        let status = ret_err!(ContactStatus::try_from(status), false);
        let uid = ret_none!(self.list.get(row_index), false).id;
        let inner = ret_none!(self.map.get_mut(&uid), false);

        ret_err!(
            contact::set_status(uid, inner.pairwise_conversation, status),
            false
        );

        inner.status = status;

        if status == ContactStatus::Deleted {
            self.model.begin_remove_rows(row_index, row_index);
            self.list.remove(row_index);
            self.map.remove(&uid);
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

    fn emit(&mut self) -> &mut UsersEmitter {
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
        self.map.insert(user_id, contact);
        self.model.end_insert_rows();
        true
    }

    fn remove_from_conversation(
        &mut self,
        index: u64,
        conversation_id: ffi::ConversationIdRef,
    ) -> bool {
        let index = index as usize;
        let conv_id = ret_err!(ConversationId::try_from(conversation_id), false);
        let uid = ret_none!(self.list.get(index), false).id;

        ret_err!(heraldcore::members::remove_member(&conv_id, uid), false);
        true
    }

    // TODO handle removals
    // TODO replace polling
    fn refresh(&mut self, notif: String) -> bool {
        let uid = ret_err!(notif.as_str().try_into(), false);
        let new_user = ret_err!(contact::by_user_id(uid), false);

        self.updated = chrono::Utc::now();

        let filter = &self.filter;

        self.model
            .begin_insert_rows(self.list.len(), (self.list.len() + 1).saturating_sub(1));
        self.list.push(User {
            matched: new_user.matches(&filter),
            id: uid,
        });
        self.map.insert(uid, new_user);
        self.model.end_insert_rows();

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
            let inner = ret_none!(self.map.get(&contact.id));
            contact.matched = inner.matches(&self.filter);
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));
    }
}
