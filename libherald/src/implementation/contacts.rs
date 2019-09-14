use crate::interface::*;
use heraldcore::{
    abort_err,
    contact::{self, ContactBuilder, ContactStatus, ContactsHandle},
    utils::SearchPattern,
};

#[derive(Clone)]
struct ContactsItem {
    inner: contact::Contact,
    matched: bool,
}

pub struct Contacts {
    emit: ContactsEmitter,
    model: ContactsList,
    filter: SearchPattern,
    filter_regex: bool,
    list: Vec<ContactsItem>,
    handle: ContactsHandle,
}

impl ContactsTrait for Contacts {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Contacts {
        let handle = abort_err!(ContactsHandle::new());

        let list = match handle.all() {
            Ok(v) => v
                .into_iter()
                .map(|c| ContactsItem {
                    inner: c,
                    matched: true,
                })
                .collect(),
            Err(_) => Vec::new(),
        };

        let filter = abort_err!(SearchPattern::new_normal("".into()));

        Contacts {
            emit,
            model,
            list,
            filter,
            filter_regex: false,
            handle,
        }
    }

    /// Adds a contact by their `id`
    ///
    /// Returns `false` on failure.
    fn add(&mut self, id: String) -> Vec<u8> {
        if id.len() > 255 {
            return vec![];
        }

        let contact = match ContactBuilder::new(id).add() {
            Ok(contact) => contact,
            Err(e) => {
                eprintln!("Error: {}", e);
                return vec![];
            }
        };

        self.model.begin_insert_rows(0, 0);
        self.list.insert(
            0,
            ContactsItem {
                inner: contact,
                matched: true,
            },
        );
        self.model.end_insert_rows();
        self.inner_filter();

        self.list[0].inner.pairwise_conversation.to_vec()
    }

    /// Returns contact id.
    fn contact_id(&self, row_index: usize) -> &str {
        self.list[row_index].inner.id.as_str()
    }

    /// Returns contact id.
    fn pairwise_conversation_id(&self, row_index: usize) -> &[u8] {
        self.list[row_index].inner.pairwise_conversation.as_slice()
    }

    /// Returns contacts name
    fn name(&self, row_index: usize) -> Option<&str> {
        self.list[row_index].inner.name.as_ref().map(|n| n.as_str())
    }

    /// Updates a contact's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: Option<String>) -> bool {
        match self.handle.set_name(
            self.contact_id(row_index),
            name.as_ref().map(|s| s.as_str()),
        ) {
            Ok(()) => {
                self.list[row_index].inner.name = name;
                true
            }
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        }
    }

    /// Returns profile picture given the contact's id.
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
        match self.handle.set_profile_picture(
            self.contact_id(row_index),
            crate::utils::strip_qrc(picture),
            self.profile_picture(row_index),
        ) {
            Ok(path) => {
                self.list[row_index].inner.profile_picture = path;
                true
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    /// Returns contact's color
    fn color(&self, row_index: usize) -> u32 {
        self.list[row_index].inner.color
    }

    /// Sets color
    fn set_color(&mut self, row_index: usize, color: u32) -> bool {
        match self.handle.set_color(self.contact_id(row_index), color) {
            Ok(()) => {
                self.list[row_index].inner.color = color;
                true
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    /// Indicates whether user is archived.
    ///
    /// User is archived => true,
    /// User is active => false
    fn status(&self, row_index: usize) -> u8 {
        self.list[row_index].inner.status as u8
    }

    /// Updates archive status.
    fn set_status(&mut self, row_index: usize, status: u8) -> bool {
        use std::convert::TryFrom;
        let status = match ContactStatus::try_from(status) {
            Ok(status) => status,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        match self.handle.set_status(self.contact_id(row_index), status) {
            Ok(()) => {
                self.list[row_index].inner.status = status;
                true
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
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
            match SearchPattern::new_regex(pattern) {
                Ok(pat) => pat,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
        } else {
            match SearchPattern::new_normal(pattern) {
                Ok(pat) => pat,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
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
            if let Err(e) = self.filter.regex_mode() {
                eprintln!("{}", e);
                return;
            }
        } else {
            if let Err(e) = self.filter.normal_mode() {
                eprintln!("{}", e);
                return;
            }
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

    fn emit(&mut self) -> &mut ContactsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}

impl Contacts {
    fn clear_filter(&mut self) {
        for contact in self.list.iter_mut() {
            contact.matched = true;
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));

        if self.filter_regex {
            self.filter = match SearchPattern::new_regex("".to_owned()) {
                Ok(filter) => filter,
                Err(e) => {
                    eprintln!("This should be impossible: {}", e);
                    return;
                }
            };
        } else {
            self.filter = match SearchPattern::new_normal("".to_owned()) {
                Ok(filter) => filter,
                Err(e) => {
                    eprintln!("This should be impossible: {}", e);
                    return;
                }
            };
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
