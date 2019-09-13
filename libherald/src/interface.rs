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



pub enum QByteArray {}


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
    color_changed: fn(*mut ConfigQObject),
    colorscheme_changed: fn(*mut ConfigQObject),
    config_id_changed: fn(*mut ConfigQObject),
    init_changed: fn(*mut ConfigQObject),
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
            color_changed: self.color_changed,
            colorscheme_changed: self.colorscheme_changed,
            config_id_changed: self.config_id_changed,
            init_changed: self.init_changed,
            name_changed: self.name_changed,
            profile_picture_changed: self.profile_picture_changed,
        }
    }
    fn clear(&self) {
        let n: *const ConfigQObject = null();
        self.qobject.store(n as *mut ConfigQObject, Ordering::SeqCst);
    }
    pub fn color_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.color_changed)(ptr);
        }
    }
    pub fn colorscheme_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.colorscheme_changed)(ptr);
        }
    }
    pub fn config_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.config_id_changed)(ptr);
        }
    }
    pub fn init_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.init_changed)(ptr);
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
    fn color(&self) -> u32;
    fn set_color(&mut self, value: u32);
    fn colorscheme(&self) -> u32;
    fn set_colorscheme(&mut self, value: u32);
    fn config_id(&self) -> &str;
    fn set_config_id(&mut self, value: String);
    fn init(&self) -> bool;
    fn name(&self) -> Option<&str>;
    fn set_name(&mut self, value: Option<String>);
    fn profile_picture(&self) -> Option<&str>;
    fn set_profile_picture(&mut self, value: Option<String>);
    fn exists(&self) -> bool;
}

#[no_mangle]
pub extern "C" fn config_new(
    config: *mut ConfigQObject,
    config_color_changed: fn(*mut ConfigQObject),
    config_colorscheme_changed: fn(*mut ConfigQObject),
    config_config_id_changed: fn(*mut ConfigQObject),
    config_init_changed: fn(*mut ConfigQObject),
    config_name_changed: fn(*mut ConfigQObject),
    config_profile_picture_changed: fn(*mut ConfigQObject),
) -> *mut Config {
    let config_emit = ConfigEmitter {
        qobject: Arc::new(AtomicPtr::new(config)),
        color_changed: config_color_changed,
        colorscheme_changed: config_colorscheme_changed,
        config_id_changed: config_config_id_changed,
        init_changed: config_init_changed,
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
pub unsafe extern "C" fn config_color_get(ptr: *const Config) -> u32 {
    (&*ptr).color()
}

#[no_mangle]
pub unsafe extern "C" fn config_color_set(ptr: *mut Config, v: u32) {
    (&mut *ptr).set_color(v);
}

#[no_mangle]
pub unsafe extern "C" fn config_colorscheme_get(ptr: *const Config) -> u32 {
    (&*ptr).colorscheme()
}

#[no_mangle]
pub unsafe extern "C" fn config_colorscheme_set(ptr: *mut Config, v: u32) {
    (&mut *ptr).set_colorscheme(v);
}

#[no_mangle]
pub unsafe extern "C" fn config_config_id_get(
    ptr: *const Config,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.config_id();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_config_id_set(ptr: *mut Config, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_config_id(s);
}

#[no_mangle]
pub unsafe extern "C" fn config_init_get(ptr: *const Config) -> bool {
    (&*ptr).init()
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
    filter_changed: fn(*mut ContactsQObject),
    filter_regex_changed: fn(*mut ContactsQObject),
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
            filter_changed: self.filter_changed,
            filter_regex_changed: self.filter_regex_changed,
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const ContactsQObject = null();
        self.qobject.store(n as *mut ContactsQObject, Ordering::SeqCst);
    }
    pub fn filter_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.filter_changed)(ptr);
        }
    }
    pub fn filter_regex_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.filter_regex_changed)(ptr);
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
    fn filter(&self) -> &str;
    fn set_filter(&mut self, value: String);
    fn filter_regex(&self) -> bool;
    fn set_filter_regex(&mut self, value: bool);
    fn add(&mut self, id: String) -> bool;
    fn toggle_filter_regex(&mut self) -> bool;
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, _: u8, _: SortOrder) {}
    fn color(&self, index: usize) -> u32;
    fn set_color(&mut self, index: usize, _: u32) -> bool;
    fn contact_id(&self, index: usize) -> &str;
    fn matched(&self, index: usize) -> bool;
    fn set_matched(&mut self, index: usize, _: bool) -> bool;
    fn name(&self, index: usize) -> Option<&str>;
    fn set_name(&mut self, index: usize, _: Option<String>) -> bool;
    fn profile_picture(&self, index: usize) -> Option<&str>;
    fn set_profile_picture(&mut self, index: usize, _: Option<String>) -> bool;
    fn status(&self, index: usize) -> u8;
    fn set_status(&mut self, index: usize, _: u8) -> bool;
}

#[no_mangle]
pub extern "C" fn contacts_new(
    contacts: *mut ContactsQObject,
    contacts_filter_changed: fn(*mut ContactsQObject),
    contacts_filter_regex_changed: fn(*mut ContactsQObject),
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
        filter_changed: contacts_filter_changed,
        filter_regex_changed: contacts_filter_regex_changed,
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
pub unsafe extern "C" fn contacts_filter_get(
    ptr: *const Contacts,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.filter();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn contacts_filter_set(ptr: *mut Contacts, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_filter(s);
}

#[no_mangle]
pub unsafe extern "C" fn contacts_filter_regex_get(ptr: *const Contacts) -> bool {
    (&*ptr).filter_regex()
}

#[no_mangle]
pub unsafe extern "C" fn contacts_filter_regex_set(ptr: *mut Contacts, v: bool) {
    (&mut *ptr).set_filter_regex(v);
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
pub unsafe extern "C" fn contacts_toggle_filter_regex(ptr: *mut Contacts) -> bool {
    let o = &mut *ptr;
    let r = o.toggle_filter_regex();
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
pub unsafe extern "C" fn contacts_data_color(ptr: *const Contacts, row: c_int) -> u32 {
    let o = &*ptr;
    o.color(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_color(
    ptr: *mut Contacts, row: c_int,
    v: u32,
) -> bool {
    (&mut *ptr).set_color(to_usize(row), v)
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
pub unsafe extern "C" fn contacts_data_matched(ptr: *const Contacts, row: c_int) -> bool {
    let o = &*ptr;
    o.matched(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_matched(
    ptr: *mut Contacts, row: c_int,
    v: bool,
) -> bool {
    (&mut *ptr).set_matched(to_usize(row), v)
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

#[no_mangle]
pub unsafe extern "C" fn contacts_data_status(ptr: *const Contacts, row: c_int) -> u8 {
    let o = &*ptr;
    o.status(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn contacts_set_data_status(
    ptr: *mut Contacts, row: c_int,
    v: u8,
) -> bool {
    (&mut *ptr).set_status(to_usize(row), v)
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
    fn conversation_id(&self) -> Option<&[u8]>;
    fn set_conversation_id(&mut self, value: Option<&[u8]>);
    fn clear_conversation_view(&mut self) -> ();
    fn delete_conversation_by_id(&mut self, conversation_id: &[u8]) -> bool;
    fn delete_message(&mut self, row_index: u64) -> bool;
    fn delete_conversation(&mut self) -> bool;
    fn insert_message(&mut self, body: String) -> bool;
    fn reply(&mut self, body: String, op: &[u8]) -> bool;
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
    fn epoch_timestamp_ms(&self, index: usize) -> i64;
    fn message_id(&self, index: usize) -> &[u8];
    fn op(&self, index: usize) -> Option<&[u8]>;
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
    p: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.conversation_id();
    if let Some(v) = v {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_conversation_id_set(ptr: *mut Messages, v: *const c_char, len: c_int) {
    let o = &mut *ptr;
    let v = slice::from_raw_parts(v as *const u8, to_usize(len));
    o.set_conversation_id(Some(v.into()));
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
pub unsafe extern "C" fn messages_delete_conversation_by_id(ptr: *mut Messages, conversation_id_str: *const c_char, conversation_id_len: c_int) -> bool {
    let conversation_id = { slice::from_raw_parts(conversation_id_str as *const u8, to_usize(conversation_id_len)) };
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
pub unsafe extern "C" fn messages_delete_conversation(ptr: *mut Messages) -> bool {
    let o = &mut *ptr;
    let r = o.delete_conversation();
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_insert_message(ptr: *mut Messages, body_str: *const c_ushort, body_len: c_int) -> bool {
    let mut body = String::new();
    set_string_from_utf16(&mut body, body_str, body_len);
    let o = &mut *ptr;
    let r = o.insert_message(body);
    r
}

#[no_mangle]
pub unsafe extern "C" fn messages_reply(ptr: *mut Messages, body_str: *const c_ushort, body_len: c_int, op_str: *const c_char, op_len: c_int) -> bool {
    let mut body = String::new();
    set_string_from_utf16(&mut body, body_str, body_len);
    let op = { slice::from_raw_parts(op_str as *const u8, to_usize(op_len)) };
    let o = &mut *ptr;
    let r = o.reply(body, op);
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
pub unsafe extern "C" fn messages_data_epoch_timestamp_ms(ptr: *const Messages, row: c_int) -> i64 {
    let o = &*ptr;
    o.epoch_timestamp_ms(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_message_id(
    ptr: *const Messages, row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.message_id(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op(
    ptr: *const Messages, row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.op(to_usize(row));
    if let Some(data) = data {
        let s: *const c_char = data.as_ptr() as (*const c_char);
        set(d, s, to_c_int(data.len()));
    }
}

pub struct NetworkHandleQObject {}

pub struct NetworkHandleEmitter {
    qobject: Arc<AtomicPtr<NetworkHandleQObject>>,
    connection_pending_changed: fn(*mut NetworkHandleQObject),
    connection_up_changed: fn(*mut NetworkHandleQObject),
    new_message_changed: fn(*mut NetworkHandleQObject),
}

unsafe impl Send for NetworkHandleEmitter {}

impl NetworkHandleEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> NetworkHandleEmitter {
        NetworkHandleEmitter {
            qobject: self.qobject.clone(),
            connection_pending_changed: self.connection_pending_changed,
            connection_up_changed: self.connection_up_changed,
            new_message_changed: self.new_message_changed,
        }
    }
    fn clear(&self) {
        let n: *const NetworkHandleQObject = null();
        self.qobject.store(n as *mut NetworkHandleQObject, Ordering::SeqCst);
    }
    pub fn connection_pending_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.connection_pending_changed)(ptr);
        }
    }
    pub fn connection_up_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.connection_up_changed)(ptr);
        }
    }
    pub fn new_message_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_message_changed)(ptr);
        }
    }
}

pub trait NetworkHandleTrait {
    fn new(emit: NetworkHandleEmitter) -> Self;
    fn emit(&mut self) -> &mut NetworkHandleEmitter;
    fn connection_pending(&self) -> bool;
    fn connection_up(&self) -> bool;
    fn new_message(&self) -> bool;
    fn register_device(&mut self) -> bool;
    fn request_meta_data(&mut self, of: String) -> bool;
    fn send_message(&mut self, message_body: String, to: String) -> bool;
}

#[no_mangle]
pub extern "C" fn network_handle_new(
    network_handle: *mut NetworkHandleQObject,
    network_handle_connection_pending_changed: fn(*mut NetworkHandleQObject),
    network_handle_connection_up_changed: fn(*mut NetworkHandleQObject),
    network_handle_new_message_changed: fn(*mut NetworkHandleQObject),
) -> *mut NetworkHandle {
    let network_handle_emit = NetworkHandleEmitter {
        qobject: Arc::new(AtomicPtr::new(network_handle)),
        connection_pending_changed: network_handle_connection_pending_changed,
        connection_up_changed: network_handle_connection_up_changed,
        new_message_changed: network_handle_new_message_changed,
    };
    let d_network_handle = NetworkHandle::new(network_handle_emit);
    Box::into_raw(Box::new(d_network_handle))
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_free(ptr: *mut NetworkHandle) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_connection_pending_get(ptr: *const NetworkHandle) -> bool {
    (&*ptr).connection_pending()
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_connection_up_get(ptr: *const NetworkHandle) -> bool {
    (&*ptr).connection_up()
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_new_message_get(ptr: *const NetworkHandle) -> bool {
    (&*ptr).new_message()
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_register_device(ptr: *mut NetworkHandle) -> bool {
    let o = &mut *ptr;
    let r = o.register_device();
    r
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_request_meta_data(ptr: *mut NetworkHandle, of_str: *const c_ushort, of_len: c_int) -> bool {
    let mut of = String::new();
    set_string_from_utf16(&mut of, of_str, of_len);
    let o = &mut *ptr;
    let r = o.request_meta_data(of);
    r
}

#[no_mangle]
pub unsafe extern "C" fn network_handle_send_message(ptr: *mut NetworkHandle, message_body_str: *const c_ushort, message_body_len: c_int, to_str: *const c_ushort, to_len: c_int) -> bool {
    let mut message_body = String::new();
    set_string_from_utf16(&mut message_body, message_body_str, message_body_len);
    let mut to = String::new();
    set_string_from_utf16(&mut to, to_str, to_len);
    let o = &mut *ptr;
    let r = o.send_message(message_body, to);
    r
}
