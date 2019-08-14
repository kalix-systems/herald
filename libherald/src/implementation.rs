#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use crate::interface::*;
use heraldcore::contact;
use std::collections::VecDeque;

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
    list: VecDeque<ContactsItem>,
}

impl ContactsTrait for Contacts {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Contacts {
        let core = contact::Contacts::default();
        let list = match core.get_all() {
            Ok(v) => v.into_iter().map(|c| c.into()).collect(),
            Err(e) => VecDeque::new(),
        };
        Contacts {
            emit,
            core,
            model,
            list,
        }
    }

    fn add(&mut self, name: String) -> i64 {
        let contact_uid = self.core.add(&name).unwrap_or(0);
        self.list.push_front(ContactsItem { contact_uid, name });
        contact_uid
    }

    fn remove(&mut self, uid: i64) -> bool {
        self.core.delete(uid).is_ok()
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
