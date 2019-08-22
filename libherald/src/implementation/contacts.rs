use crate::interface::*;
use heraldcore::{
    contact::{self, Contacts as Core},
    db::DBTable,
};
use im_rc::vector::Vector as ImVector;

#[derive(Clone)]
struct ContactsItem {
    inner: contact::Contact,
}

pub struct Contacts {
    emit: ContactsEmitter,
    model: ContactsList,
    list: ImVector<ContactsItem>,
}

impl ContactsTrait for Contacts {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Contacts {
        // create table if it does not already exist
        if let Err(e) = Core::create_table() {
            eprintln!("{}", e);
        }

        let list = match Core::all() {
            Ok(v) => v.into_iter().map(|c| ContactsItem { inner: c }).collect(),
            Err(_) => ImVector::new(),
        };
        Contacts { emit, model, list }
    }

    fn remove_all(&mut self) {
        self.model.begin_reset_model();

        if let Err(e) = Core::drop_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = Core::create_table() {
            eprintln!("{}", e);
        }

        self.list = ImVector::new();

        self.model.end_reset_model();
    }

    /// Adds a contact by their `id`
    ///
    /// Returns `false` on failure.
    fn add(&mut self, id: String) -> bool {
        assert!(id.len() < 256);

        if let Err(e) = Core::add(id.as_str(), None, None, None) {
            eprintln!("Error: {}", e);
            return false;
        }

        self.model.begin_insert_rows(0, 0);
        self.list.push_front(ContactsItem {
            inner: contact::Contact::new(id, None, None, None, contact::ArchiveStatus::Active),
        });
        self.model.end_insert_rows();
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
            .set_profile_picture(crate::strip_qrc(picture))
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
    /// User is active => false,
    fn archive_status(&self, row_index: usize) -> bool {
        self.list[row_index].inner.archive_status.into()
    }

    /// Removes a contact, returns a boolean to indicate success.
    fn remove(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;

        let id = self.list[row_index].inner.id.as_str();
        match Core::delete(id) {
            Ok(_) => {
                self.model.begin_remove_rows(row_index, row_index);
                self.list.remove(row_index);
                self.model.end_remove_rows();
                true
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                false
            }
        }
    }

    fn emit(&mut self) -> &mut ContactsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
