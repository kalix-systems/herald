use crate::{interface::*, ret_err, types::*};
use herald_common::{UserId, UserIdRef};
use heraldcore::{
    abort_err,
    contact::{self, ContactBuilder, ContactStatus, ContactsHandle},
    types::*,
    utils::SearchPattern,
};

#[derive(Clone)]
struct User {
    inner: contact::Contact,
    matched: bool,
}

pub struct Users {
    emit: UsersEmitter,
    model: UsersList,
    filter: SearchPattern,
    filter_regex: bool,
    list: Vec<User>,
    handle: ContactsHandle,
    conversation_id: Option<ConversationId>,
}

impl UsersTrait for Users {
    fn new(emit: UsersEmitter, model: UsersList) -> Users {
        let handle = abort_err!(ContactsHandle::new());

        let list = match handle.all() {
            Ok(v) => v
                .into_iter()
                .map(|c| User {
                    inner: c,
                    matched: true,
                })
                .collect(),
            Err(_) => Vec::new(),
        };

        let filter = abort_err!(SearchPattern::new_normal("".into()));

        Users {
            emit,
            model,
            list,
            filter,
            filter_regex: false,
            handle,
            conversation_id: None,
        }
    }

    /// Adds a contact by their `id`
    fn add(&mut self, id: UserId) -> FfiConversationId {
        if id.len() > 255 {
            return vec![];
        }

        let contact = ret_err!(ContactBuilder::new(id).add(), vec![]);

        self.model.begin_insert_rows(0, 0);
        self.list.insert(
            0,
            User {
                inner: contact,
                matched: true,
            },
        );
        self.model.end_insert_rows();
        self.inner_filter();

        self.list[0].inner.pairwise_conversation.to_vec()
    }

    fn conversation_id(&self) -> Option<FfiConversationIdRef> {
        self.conversation_id.as_ref().map(|id| id.as_slice())
    }

    /// Returns user id.
    fn user_id(&self, row_index: usize) -> UserIdRef {
        self.list[row_index].inner.id.as_str()
    }

    /// Returns conversation id.
    fn pairwise_conversation_id(&self, row_index: usize) -> FfiConversationIdRef {
        self.list[row_index].inner.pairwise_conversation.as_slice()
    }

    /// Returns users name
    fn name(&self, row_index: usize) -> Option<&str> {
        self.list[row_index].inner.name.as_ref().map(|n| n.as_str())
    }

    /// Updates a user's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: Option<String>) -> bool {
        ret_err!(
            self.handle
                .set_name(self.user_id(row_index), name.as_ref().map(|s| s.as_str())),
            false
        );

        self.list[row_index].inner.name = name;
        true
    }

    /// Returns profile picture
    fn profile_picture(&self, row_index: usize) -> Option<&str> {
        self.list[row_index]
            .inner
            .profile_picture
            .as_ref()
            .map(|s| s.as_str())
    }

    /// Sets profile picture.
    ///
    /// Returns bool indicating success.
    fn set_profile_picture(&mut self, row_index: usize, picture: Option<String>) -> bool {
        let path = ret_err!(
            self.handle.set_profile_picture(
                self.user_id(row_index),
                crate::utils::strip_qrc(picture),
                self.profile_picture(row_index),
            ),
            false
        );

        self.list[row_index].inner.profile_picture = path;
        true
    }

    /// Returns user's color
    fn color(&self, row_index: usize) -> u32 {
        self.list[row_index].inner.color
    }

    /// Sets color
    fn set_color(&mut self, row_index: usize, color: u32) -> bool {
        ret_err!(self.handle.set_color(self.user_id(row_index), color), false);
        self.list[row_index].inner.color = color;
        true
    }

    fn status(&self, row_index: usize) -> u8 {
        self.list[row_index].inner.status as u8
    }

    fn set_status(&mut self, row_index: usize, status: u8) -> bool {
        use std::convert::TryFrom;
        let status = ret_err!(ContactStatus::try_from(status), false);

        ret_err!(
            self.handle
                .set_status(self.list[row_index].inner.id.as_str(), status),
            false
        );

        self.list[row_index].inner.status = status;

        if status == ContactStatus::Deleted {
            self.model.begin_remove_rows(row_index, row_index);
            self.list.remove(row_index);
            self.model.end_remove_rows();
        }

        true
    }

    fn index_from_conversation_id(&self, conv_id: FfiConversationIdRef) -> i64 {
        use std::convert::TryFrom;

        let conv_id = ret_err!(ConversationId::try_from(conv_id), -1);

        self.list
            .iter()
            .enumerate()
            .find(|(_ix, contact)| contact.inner.pairwise_conversation == conv_id)
            .map(|(ix, _contact)| ix as i64)
            .unwrap_or(-1)
    }

    fn matched(&self, row_index: usize) -> bool {
        self.list[row_index].matched
    }

    fn set_matched(&mut self, row_index: usize, value: bool) -> bool {
        self.list[row_index].matched = value;
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

    fn add_member(&mut self, index: u64, conversation_id: FfiConversationIdRef) -> bool {
        unimplemented!()
    }

    fn remove_member(&mut self, index: u64, conversation_id: FfiConversationIdRef) -> bool {
        unimplemented!()
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
            match contact.inner.name.as_ref() {
                Some(name) => {
                    contact.matched = self.filter.is_match(name);
                }
                None => {}
            }
            contact.matched |= self.filter.is_match(contact.inner.id.as_str());
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));
    }
}
