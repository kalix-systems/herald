#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use crate::interface::*;
use heraldcore::contact;
use im_rc::vector::Vector as ImVector;

#[derive(Default, Clone)]
struct ContactsItem {
    contact_uid: i64,
    name: String,
}

impl From<contact::Contact> for ContactsItem {
    fn from(val: contact::Contact) -> Self {
        let contact::Contact {
            uid: contact_uid,
            name,
        } = val;
        ContactsItem { contact_uid, name }
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
        let list = match core.get_all() {
            Ok(v) => v.into_iter().map(|c| c.into()).collect(),
            Err(e) => ImVector::new(),
        };
        Contacts {
            emit,
            core,
            model,
            list,
        }
    }

    /// Adds a contact by their `name`, returns their assigned `uid`.
    ///
    /// Returns -1 on failure.
    fn add(&mut self, name: String) -> i64 {
        let contact_uid = match self.core.add(&name) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Error: {}", e);
                return -1;
            }
        };
        self.list.push_front(ContactsItem { contact_uid, name });
        contact_uid
    }

    /// Removes a contact by their `uid`, returns a boolean to indicate success.
    fn remove(&mut self, uid: i64) -> bool {
        if self.core.delete(uid).is_err() {
            return false;
        }
        let index = match self.list.iter().position(|c| c.contact_uid == uid) {
            Some(index) => index,
            None => return false,
        };

        self.list.remove(index);
        true
    }

    /// Updates a contact's name, returns a boolean to indicate success.
    fn update(&mut self, uid: i64) -> bool {
        false
    }
    fn emit(&mut self) -> &mut ContactsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn contact_uid(&self, index: usize) -> i64 {
        self.list[index].contact_uid
    }

    fn name(&self, index: usize) -> &str {
        &self.list[index].name
    }
    fn set_name(&mut self, index: usize, v: String) -> bool {
        self.list[index].name = v;
        true
    }
}
