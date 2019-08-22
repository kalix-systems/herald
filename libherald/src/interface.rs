/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use crate::implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> COption<T> {
    #![allow(dead_code)]
    fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct ConfigQObject {}

pub struct ConfigEmitter {
    qobject: Arc<AtomicPtr<ConfigQObject>>,
    id_changed: fn(*mut ConfigQObject),
    name_changed: fn(*mut ConfigQObject),
    profile_picture_changed: fn(*mut ConfigQObject),
}

unsafe impl Send for ConfigEmitter {}

impl ConfigEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ConfigEmitter {
        ConfigEmitter {
            qobject: self.qobject.clone(),
            id_changed: self.id_changed,
            name_changed: self.name_changed,
            profile_picture_changed: self.profile_picture_changed,
        }
    }
    fn clear(&self) {
        let n: *const ConfigQObject = null();
        self.qobject.store(n as *mut ConfigQObject, Ordering::SeqCst);
    }
    pub fn id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.id_changed)(ptr);
        }
    }
    pub fn name_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.name_changed)(ptr);
        }
    }
    pub fn profile_picture_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.profile_picture_changed)(ptr);
        }
    }
}

pub trait ConfigTrait {
    fn new(emit: ConfigEmitter) -> Self;
    fn emit(&mut self) -> &mut ConfigEmitter;
    fn id(&self) -> &str;
    fn set_id(&mut self, value: String);
    fn name(&self) -> Option<&str>;
    fn set_name(&mut self, value: Option<String>);
    fn profile_picture(&self) -> Option<&str>;
    fn set_profile_picture(&mut self, value: Option<String>);
    fn exists(&self) -> bool;
}

#[no_mangle]
pub extern "C" fn config_new(
    config: *mut ConfigQObject,
    config_id_changed: fn(*mut ConfigQObject),
    config_name_changed: fn(*mut ConfigQObject),
    config_profile_picture_changed: fn(*mut ConfigQObject),
) -> *mut Config {
    let config_emit = ConfigEmitter {
        qobject: Arc::new(AtomicPtr::new(config)),
        id_changed: config_id_changed,
        name_changed: config_name_changed,
        profile_picture_changed: config_profile_picture_changed,
    };
    let d_config = Config::new(config_emit);
    Box::into_raw(Box::new(d_config))
}

#[no_mangle]
pub unsafe extern "C" fn config_free(ptr: *mut Config) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn config_id_get(
    ptr: *const Config,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.id();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_id_set(ptr: *mut Config, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_id(s);
}

#[no_mangle]
pub unsafe extern "C" fn config_name_get(
    ptr: *const Config,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.name();
    if let Some(v) = v {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn config_name_set(ptr: *mut Config, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_name(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn config_name_set_none(ptr: *mut Config) {
    let o = &mut *ptr;
    o.set_name(None);
}

#[no_mangle]
pub unsafe extern "C" fn config_profile_picture_get(
    ptr: *const Config,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.profile_picture();
    if let Some(v) = v {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn config_profile_picture_set(ptr: *mut Config, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_profile_picture(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn config_profile_picture_set_none(ptr: *mut Config) {
    let o = &mut *ptr;
    o.set_profile_picture(None);
}

#[no_mangle]
pub unsafe extern "C" fn config_exists(ptr: *const Config) -> bool {
    let o = &*ptr;
    let r = o.exists();
    r
}

pub struct ContactsQObject {}

pub struct ContactsEmitter {
    qobject: Arc<AtomicPtr<ContactsQObject>>,
    new_data_ready: fn(*mut ContactsQObject),
}

unsafe impl Send for ContactsEmitter {}

impl ContactsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ContactsEmitter {
        ContactsEmitter {
            qobject: self.qobject.clone(),
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const ContactsQObject = null();
        self.qobject.store(n as *mut ContactsQObject, Ordering::SeqCst);
    }
    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct ContactsList {
    qobject: *mut ContactsQObject,
    layout_about_to_be_changed: fn(*mut ContactsQObject),
    layout_changed: fn(*mut ContactsQObject),
    data_changed: fn(*mut ContactsQObject, usize, usize),
    begin_reset_model: fn(*mut ContactsQObject),
    end_reset_model: fn(*mut ContactsQObject),
    begin_insert_rows: fn(*mut ContactsQObject, usize, usize),
    end_insert_rows: fn(*mut ContactsQObject),
    begin_move_rows: fn(*mut ContactsQObject, usize, usize, usize),
    end_move_rows: fn(*mut ContactsQObject),
    begin_remove_rows: fn(*mut ContactsQObject, usize, usize),
    end_remove_rows: fn(*mut ContactsQObject),
}

impl ContactsList {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait ContactsTrait {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Self;
    fn emit(&mut self) -> &mut ContactsEmitter;
    fn add(&mut self, id: String) -> bool;
    fn remove(&mut self, row_index: u64) -> bool;
    fn remove_all(&mut self) -> ();
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, _: u8, _: SortOrder) {}
    fn contact_id(&self, index: usize) -> &str;
    fn name(&self, index: usize) -> Option<&str>;
    fn set_name(&mut self, index: usize, _: Option<String>) -> bool;
    fn profile_picture(&self, index: usize) -> Option<&str>;
    fn set_profile_picture(&mut self, index: usize, _: Option<String>) -> bool;
}

#[no_mangle]
pub extern "C" fn contacts_new(
    contacts: *mut ContactsQObject,
    contacts_new_data_ready: fn(*mut ContactsQObject),
    contacts_layout_about_to_be_changed: fn(*mut ContactsQObject),
    contacts_layout_changed: fn(*mut ContactsQObject),
    contacts_data_changed: fn(*mut ContactsQObject, usize, usize),
    contacts_begin_reset_model: fn(*mut ContactsQObject),
    contacts_end_reset_model: fn(*mut ContactsQObject),
    contacts_begin_insert_rows: fn(*mut ContactsQObject, usize, usize),
    contacts_end_insert_rows: fn(*mut ContactsQObject),
    contacts_begin_move_rows: fn(*mut ContactsQObject, usize, usize, usize),
    contacts_end_move_rows: fn(*mut ContactsQObject),
    contacts_begin_remove_rows: fn(*mut ContactsQObject, usize, usize),
    contacts_end_remove_rows: fn(*mut ContactsQObject),
) -> *mut Contacts {
    let contacts_emit = ContactsEmitter {
        qobject: Arc::new(AtomicPtr::new(contacts)),
        new_data_ready: contacts_new_data_ready,
    };
    let model = ContactsList {
        qobject: contacts,
        layout_about_to_be_changed: contacts_layout_about_to_be_changed,
        layout_changed: contacts_layout_changed,
        data_changed: contacts_data_changed,
        begin_reset_model: contacts_begin_reset_model,
        end_reset_model: contacts_end_reset_model,
        begin_insert_rows: contacts_begin_insert_rows,
        end_insert_rows: contacts_end_insert_rows,
        begin_move_rows: contacts_begin_move_rows,
        end_move_rows: contacts_end_move_rows,
        begin_remove_rows: contacts_begin_remove_rows,
        end_remove_rows: contacts_end_remove_rows,
    };
    let d_contacts = Contacts::new(contacts_emit, model);
    Box::into_raw(Box::new(d_contacts))
}

#[no_mangle]
pub unsafe extern "C" fn contacts_free(ptr: *mut Contacts) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn contacts_add(ptr: *mut Contacts, id_str: *const c_ushort, id_len: c_int) -> bool {
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    let o = &mut *ptr;
    let r = o.add(id);
    r
}

#[no_mangle]
pub unsafe extern "C" fn contacts_remove(ptr: *mut Contacts, row_index: u64) -> bool {
    let o = &mut *ptr;
    let r = o.remove(row_index);
    r
}

#[no_mangle]
pub unsafe extern "C" fn contacts_remove_all(ptr: *mut Contacts) -> () {
    let o = &mut *ptr;
    let r = o.remove_all();
    r
}

#[no_mangle]
pub unsafe extern "C" fn contacts_row_count(ptr: *const Contacts) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn contacts_insert_rows(ptr: *mut Contacts, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn contacts_remove_rows(ptr: *mut Contacts, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn contacts_can_fetch_more(ptr: *const Contacts) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn contacts_fetch_more(ptr: *mut Contacts) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn contacts_sort(
    ptr: *mut Contacts,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn contacts_data_contact_id(
    ptr: *const Contacts, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.contact_id(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn contacts_data_name(
    ptr: *const Contacts, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.name(to_usize(row));
    if let Some(data) = data {
        let s: *const c_char = data.as_ptr() as (*const c_char);
        set(d, s, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_name(
    ptr: *mut Contacts, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_name(to_usize(row), Some(v))
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_name_none(ptr: *mut Contacts, row: c_int) -> bool {
    (&mut *ptr).set_name(to_usize(row), None)
}

#[no_mangle]
pub unsafe extern "C" fn contacts_data_profile_picture(
    ptr: *const Contacts, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.profile_picture(to_usize(row));
    if let Some(data) = data {
        let s: *const c_char = data.as_ptr() as (*const c_char);
        set(d, s, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_profile_picture(
    ptr: *mut Contacts, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_profile_picture(to_usize(row), Some(v))
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_profile_picture_none(ptr: *mut Contacts, row: c_int) -> bool {
    (&mut *ptr).set_profile_picture(to_usize(row), None)
}

pub struct HeraldStateQObject {}

pub struct HeraldStateEmitter {
    qobject: Arc<AtomicPtr<HeraldStateQObject>>,
}

unsafe impl Send for HeraldStateEmitter {}

impl HeraldStateEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> HeraldStateEmitter {
        HeraldStateEmitter {
            qobject: self.qobject.clone(),
        }
    }
    fn clear(&self) {
        let n: *const HeraldStateQObject = null();
        self.qobject.store(n as *mut HeraldStateQObject, Ordering::SeqCst);
    }
}

pub trait HeraldStateTrait {
    fn new(emit: HeraldStateEmitter) -> Self;
    fn emit(&mut self) -> &mut HeraldStateEmitter;
    fn create_min_config(&mut self, id: String) -> ();
}

#[no_mangle]
pub extern "C" fn herald_state_new(
    herald_state: *mut HeraldStateQObject,
) -> *mut HeraldState {
    let herald_state_emit = HeraldStateEmitter {
        qobject: Arc::new(AtomicPtr::new(herald_state)),
    };
    let d_herald_state = HeraldState::new(herald_state_emit);
    Box::into_raw(Box::new(d_herald_state))
}

#[no_mangle]
pub unsafe extern "C" fn herald_state_free(ptr: *mut HeraldState) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn herald_state_create_min_config(ptr: *mut HeraldState, id_str: *const c_ushort, id_len: c_int) -> () {
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    let o = &mut *ptr;
    let r = o.create_min_config(id);
    r
}

pub struct MessagesQObject {}

pub struct MessagesEmitter {
    qobject: Arc<AtomicPtr<MessagesQObject>>,
    conversation_id_changed: fn(*mut MessagesQObject),
    new_data_ready: fn(*mut MessagesQObject),
}

unsafe impl Send for MessagesEmitter {}

impl MessagesEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> MessagesEmitter {
        MessagesEmitter {
            qobject: self.qobject.clone(),
            conversation_id_changed: self.conversation_id_changed,
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const MessagesQObject = null();
        self.qobject.store(n as *mut MessagesQObject, Ordering::SeqCst);
    }
    pub fn conversation_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.conversation_id_changed)(ptr);
        }
    }
    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct MessagesList {
    qobject: *mut MessagesQObject,
    layout_about_to_be_changed: fn(*mut MessagesQObject),
    layout_changed: fn(*mut MessagesQObject),
    data_changed: fn(*mut MessagesQObject, usize, usize),
    begin_reset_model: fn(*mut MessagesQObject),
    end_reset_model: fn(*mut MessagesQObject),
    begin_insert_rows: fn(*mut MessagesQObject, usize, usize),
    end_insert_rows: fn(*mut MessagesQObject),
    begin_move_rows: fn(*mut MessagesQObject, usize, usize, usize),
    end_move_rows: fn(*mut MessagesQObject),
    begin_remove_rows: fn(*mut MessagesQObject, usize, usize),
    end_remove_rows: fn(*mut MessagesQObject),
}

impl MessagesList {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait MessagesTrait {
    fn new(emit: MessagesEmitter, model: MessagesList) -> Self;
    fn emit(&mut self) -> &mut MessagesEmitter;
    fn conversation_id(&self) -> Option<&str>;
    fn set_conversation_id(&mut self, value: Option<String>);
    fn clear_conversation_view(&mut self) -> ();
    fn delete_conversation(&mut self) -> bool;
    fn delete_conversation_by_id(&mut self, conversation_id: String) -> bool;
    fn delete_message(&mut self, row_index: u64) -> bool;
    fn send_message(&mut self, body: String) -> bool;
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, _: u8, _: SortOrder) {}
    fn author(&self, index: usize) -> &str;
    fn body(&self, index: usize) -> &str;
    fn message_id(&self, index: usize) -> i64;
    fn recipient(&self, index: usize) -> &str;
}

#[no_mangle]
pub extern "C" fn messages_new(
    messages: *mut MessagesQObject,
    messages_conversation_id_changed: fn(*mut MessagesQObject),
    messages_new_data_ready: fn(*mut MessagesQObject),
    messages_layout_about_to_be_changed: fn(*mut MessagesQObject),
    messages_layout_changed: fn(*mut MessagesQObject),
    messages_data_changed: fn(*mut MessagesQObject, usize, usize),
    messages_begin_reset_model: fn(*mut MessagesQObject),
    messages_end_reset_model: fn(*mut MessagesQObject),
    messages_begin_insert_rows: fn(*mut MessagesQObject, usize, usize),
    messages_end_insert_rows: fn(*mut MessagesQObject),
    messages_begin_move_rows: fn(*mut MessagesQObject, usize, usize, usize),
    messages_end_move_rows: fn(*mut MessagesQObject),
    messages_begin_remove_rows: fn(*mut MessagesQObject, usize, usize),
    messages_end_remove_rows: fn(*mut MessagesQObject),
) -> *mut Messages {
    let messages_emit = MessagesEmitter {
        qobject: Arc::new(AtomicPtr::new(messages)),
        conversation_id_changed: messages_conversation_id_changed,
        new_data_ready: messages_new_data_ready,
    };
    let model = MessagesList {
        qobject: messages,
        layout_about_to_be_changed: messages_layout_about_to_be_changed,
        layout_changed: messages_layout_changed,
        data_changed: messages_data_changed,
        begin_reset_model: messages_begin_reset_model,
        end_reset_model: messages_end_reset_model,
        begin_insert_rows: messages_begin_insert_rows,
        end_insert_rows: messages_end_insert_rows,
        begin_move_rows: messages_begin_move_rows,
        end_move_rows: messages_end_move_rows,
        begin_remove_rows: messages_begin_remove_rows,
        end_remove_rows: messages_end_remove_rows,
    };
    let d_messages = Messages::new(messages_emit, model);
    Box::into_raw(Box::new(d_messages))
}

#[no_mangle]
pub unsafe extern "C" fn messages_free(ptr: *mut Messages) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn messages_conversation_id_get(
    ptr: *const Messages,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.conversation_id();
    if let Some(v) = v {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_conversation_id_set(ptr: *mut Messages, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_conversation_id(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn messages_conversation_id_set_none(ptr: *mut Messages) {
    let o = &mut *ptr;
    o.set_conversation_id(None);
}

#[no_mangle]
pub unsafe extern "C" fn messages_clear_conversation_view(ptr: *mut Messages) -> () {
    let o = &mut *ptr;
    let r = o.clear_conversation_view();
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_delete_conversation(ptr: *mut Messages) -> bool {
    let o = &mut *ptr;
    let r = o.delete_conversation();
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_delete_conversation_by_id(ptr: *mut Messages, conversation_id_str: *const c_ushort, conversation_id_len: c_int) -> bool {
    let mut conversation_id = String::new();
    set_string_from_utf16(&mut conversation_id, conversation_id_str, conversation_id_len);
    let o = &mut *ptr;
    let r = o.delete_conversation_by_id(conversation_id);
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_delete_message(ptr: *mut Messages, row_index: u64) -> bool {
    let o = &mut *ptr;
    let r = o.delete_message(row_index);
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_send_message(ptr: *mut Messages, body_str: *const c_ushort, body_len: c_int) -> bool {
    let mut body = String::new();
    set_string_from_utf16(&mut body, body_str, body_len);
    let o = &mut *ptr;
    let r = o.send_message(body);
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_row_count(ptr: *const Messages) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn messages_insert_rows(ptr: *mut Messages, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn messages_remove_rows(ptr: *mut Messages, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn messages_can_fetch_more(ptr: *const Messages) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn messages_fetch_more(ptr: *mut Messages) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn messages_sort(
    ptr: *mut Messages,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_author(
    ptr: *const Messages, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.author(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_body(
    ptr: *const Messages, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.body(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_message_id(ptr: *const Messages, row: c_int) -> i64 {
    let o = &*ptr;
    o.message_id(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_recipient(
    ptr: *const Messages, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.recipient(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}
