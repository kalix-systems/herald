use crate::interface::*;
use heraldcore::{contact, db::DBTable};
use im_rc::vector::Vector as ImVector;

#[derive(Default, Clone)]
struct ContactsItem {
    contact_id: i64,
    name: String,
}

impl From<contact::Contact> for ContactsItem {
    #[inline]
    fn from(val: contact::Contact) -> Self {
        let contact::Contact {
            id: contact_id,
            name,
        } = val;
        ContactsItem { contact_id, name }
    }
}

pub struct Contacts {
    emit: ContactsEmitter,
    model: ContactsList,
    core: contact::Contacts,
    list: ImVector<ContactsItem>,
}

impl ContactsTrait for Contacts {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Contacts {
        let core = contact::Contacts::default();

        // create table if it does not already exist
        core.create_table().ok();

        let list = match core.get_all() {
            Ok(v) => v.into_iter().map(|c| c.into()).collect(),
            Err(_) => ImVector::new(),
        };
        Contacts {
            emit,
            core,
            model,
            list,
        }
    }

    fn remove_all(&mut self) {
        self.model.begin_reset_model();

        self.core.drop_table().expect("Couldn't drop contacts");
        self.core.create_table().unwrap();

        self.list = ImVector::new();

        self.model.end_reset_model();
    }

    /// Adds a contact by their `name`, returns their assigned `id`.
    ///
    /// Returns -1 on failure.
    fn add(&mut self, name: String) -> i64 {
        if name.is_empty() {
            return -1;
        }
        let contact_id = match self.core.add(&name, None) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Error: {}", e);
                return -1;
            }
        };

        self.model.begin_insert_rows(0, 0);
        self.list.push_front(ContactsItem { contact_id, name });
        self.model.end_insert_rows();
        contact_id
    }

    /// Adds a contact by their `name`, returns their assigned `id`.
    ///
    /// Returns -1 on failure.
    fn add_with_profile_picture(&mut self, name: String, picture: &[u8]) -> i64 {
        let contact_id = match self.core.add(&name, Some(picture)) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Error: {}", e);
                return -1;
            }
        };

        self.model.begin_insert_rows(0, 0);
        self.list.push_front(ContactsItem { contact_id, name });
        self.model.end_insert_rows();

        contact_id
    }

    /// Returns profile picture given the contact's id.
    fn profile_picture(&self, id: i64) -> Vec<u8> {
        self.core.get_profile_picture(id).unwrap_or(vec![])
    }

    /// Removes a contact by their `id`, returns a boolean to indicate success.
    fn remove(&mut self, id: i64) -> bool {
        if self.core.delete(id).is_err() {
            return false;
        }
        let index = match self.list.iter().position(|c| c.contact_id == id) {
            Some(index) => index,
            None => return false,
        };

        self.model.begin_remove_rows(0, 0);
        self.list.remove(index);
        self.model.end_remove_rows();

        true
    }

    /// Updates a contact's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_id: usize, name: String) -> bool {
        let id = self.list[row_id].contact_id;

        if self.core.update_name(id, &name).is_err() {
            return false;
        }
        let index = match self.list.iter().position(|c| c.contact_id == id) {
            Some(index) => index,
            None => return false,
        };

        self.list[index].name = name;
        true
    }

    fn emit(&mut self) -> &mut ContactsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn contact_id(&self, index: usize) -> i64 {
        self.list[index].contact_id
    }

    fn name(&self, index: usize) -> &str {
        &self.list[index].name
    }
}
