use crate::interface::*;
use heraldcore::{
    contact::{self, ContactStatus, Contacts as Core},
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
}

impl ContactsTrait for Contacts {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Contacts {
        let list = match Core::all() {
            Ok(v) => v
                .into_iter()
                .map(|c| ContactsItem {
                    inner: c,
                    matched: true,
                })
                .collect(),
            Err(_) => Vec::new(),
        };

        let filter = match SearchPattern::new_normal("".into()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}, process aborting", e);
                std::process::abort();
            }
        };

        Contacts {
            emit,
            model,
            list,
            filter,
            filter_regex: false,
        }
    }

    /// Adds a contact by their `id`
    ///
    /// Returns `false` on failure.
    fn add(&mut self, id: String) -> bool {
        if id.len() > 256 {
            return false;
        }

        let contact =
            match Core::add_contact(id.as_str(), None, None, None, ContactStatus::Active, None) {
                Ok(contact) => contact,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return false;
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
        true
    }

    /// Returns contact id.
    fn contact_id(&self, row_index: usize) -> &str {
        self.list[row_index].inner.id.as_str()
    }

    /// Returns contacts name
    fn name(&self, row_index: usize) -> Option<&str> {
        self.list[row_index].inner.name.as_ref().map(|n| n.as_str())
    }

    /// Updates a contact's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: Option<String>) -> bool {
        if self.list[row_index]
            .inner
            .set_name((&name).as_ref().map(|n| n.as_str()))
            .is_err()
        {
            return false;
        }

        true
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
        match self.list[row_index]
            .inner
            .set_profile_picture(crate::utils::strip_qrc(picture))
        {
            Ok(_) => true,
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
        match self.list[row_index].inner.set_color(color) {
            Ok(_) => true,
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

        self.list[row_index].inner.set_status(status);

        true
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
