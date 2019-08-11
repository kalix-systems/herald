pub(crate) mod errors;

use crate::models::contact::{self, Contact};
use ffi_support::{call_with_result, implement_into_ffi_by_pointer, FfiStr, IntoFfi};
use libc::{c_char, c_int};
use std::{ffi::CString, process::abort, ptr};

/// Error struct. Typically included as the final argument of a function that can produce an error.
pub type ExternError = ffi_support::ExternError;
/// Type alias for a raw string.
pub type RawStr = *const c_char;

fn abort_on_null<T>(ptr: *mut T) -> () {
    if ptr.is_null() {
        abort()
    }
}

/// Box destructor macro that allows adding a docstring.
macro_rules! box_destructor {
    ($(#[$attr:meta])* => $T:ty, $destructor_name:ident) => {
        #[no_mangle]
        $(#[$attr])*
            pub unsafe extern "C" fn $destructor_name(v: *mut $T) {
                if !v.is_null() {
                    drop(Box::from_raw(v))
                } else {
                    eprintln!("Warning: tried to drop null pointer");
                }
            }
    };
}

macro_rules! constbuffer_destructor {
    ($(#[$attr:meta])* => $T:ty, $destructor_name:ident) => {
        #[no_mangle]
        $(#[$attr])*
        pub unsafe extern "C" fn $destructor_name(buf: *const ConstBuffer<*const $T>) {
            if buf.is_null() {
                eprintln!("Warning: tried to free non-existent ConstBuffer");
                return;
            }
            let buf = &*buf;
            let s = std::slice::from_raw_parts(buf.data, buf.len);
            let s = s.as_ptr();
            Box::new(s);
        }
    };
}

pub mod db {
    use super::*;
    use crate::db::Database;

    /// Database handle
    pub type HeraldDB = Database;

    implement_into_ffi_by_pointer!(HeraldDB);
    box_destructor! {
        /// Destructor for `HeraldDB`
        => HeraldDB,
        herald_db_close
    }

    #[no_mangle]
    /// Initializes connection to database. Aborts if `e` is null.
    pub unsafe extern "C" fn herald_db_init(e: *mut ExternError) -> *mut HeraldDB {
        abort_on_null(e);
        call_with_result(&mut *e, HeraldDB::new)
    }
}

/// Functions and data structures related to contacts.
pub mod contacts {
    use super::*;
    use crate::errors::HErr;
    use const_buffer::*;

    #[repr(C)]
    /// A contact, consisting of a (local) uid, and a UTF-8 representation of their name.
    pub struct HeraldContact {
        uid: i64,
        name: RawStr,
    }

    impl From<Contact> for HeraldContact {
        fn from(val: Contact) -> Self {
            let Contact { uid, name } = val;
            let name = string_to_ptr(name);
            HeraldContact { name, uid }
        }
    }

    pub type Contacts = ConstBuffer<HeraldContact>;

    constbuffer_destructor! {
    /// `Contacts` destructor
    => Contact,
    herald_contacts_destructor
    }

    impl From<Vec<Contact>> for Contacts {
        fn from(contacts: Vec<Contact>) -> Contacts {
            ConstBuffer::new(contacts.into_iter().map(|c| c.into()).collect())
        }
    }

    #[no_mangle]
    /// Creates the contacts table in the database if it does not already exist.
    ///
    /// Aborts if `error` is null.
    pub unsafe extern "C" fn herald_contacts_create_table(
        db: *mut super::db::HeraldDB,
        error: *mut ExternError,
    ) {
        abort_on_null(error);

        if db.is_null() {
            *error = HErr::NullPtr.into();
            return;
        }

        let db = &mut *db;

        match contact::create_table(db) {
            Ok(_) => {
                *error = ExternError::success();
            }
            Err(e) => *error = e.into(),
        }
    }

    #[no_mangle]
    /// Adds a contact, returning their UID. Returns 0 if the operation failed.
    ///
    /// Aborts if `error` is a null pointer.
    pub unsafe extern "C" fn herald_contacts_add(
        db: *mut super::db::HeraldDB,
        name: RawStr,
        error: *mut ExternError,
    ) -> i64 {
        abort_on_null(db);
        if db.is_null() {
            *error = HErr::NullPtr.into();
            return 0;
        }

        let db = &mut *db;

        let name = match FfiStr::from_raw(name).as_opt_str() {
            Some(s) => s,
            None => {
                *error = ExternError::from(crate::errors::HErr::InvalidString);
                return 0;
            }
        };

        match contact::add(db, name) {
            Ok(uid) => {
                *error = ExternError::success();
                uid
            }
            Err(e) => {
                *error = e.into();
                0
            }
        }
    }

    #[no_mangle]
    /// Drops the contacts table from the database.
    ///
    /// Aborts if `error` is a null pointer.
    pub unsafe extern "C" fn herald_contacts_drop(
        db: *mut super::db::HeraldDB,
        error: *mut ExternError,
    ) {
        abort_on_null(error);
        if db.is_null() {
            *error = HErr::NullPtr.into();
            return;
        }

        let db = &mut *db;

        match contact::drop(db) {
            Ok(_) => {
                *error = ExternError::success();
            }
            Err(e) => {
                *error = e.into();
            }
        }
    }
    #[no_mangle]
    /// Returns all contacts.
    ///
    /// Aborts if `error` is a null ponter.
    pub unsafe extern "C" fn herald_contacts_load(
        db: *mut super::db::HeraldDB,
        error: *mut ExternError,
    ) -> *const Contacts {
        abort_on_null(error);
        if db.is_null() {
            *error = HErr::NullPtr.into();
            return Contacts::ffi_default();
        }

        let db = &mut *db;

        match contact::get_all(db) {
            Ok(contacts) => {
                let contacts: Contacts = contacts.into();
                *error = ExternError::success();
                contacts.into_ffi_value()
            }
            Err(e) => {
                *error = e.into();
                Contacts::ffi_default()
            }
        }
    }
}

pub mod const_buffer {
    use super::*;

    #[repr(C)]
    /// A constant buffer, templated over the `Item` type.
    pub struct ConstBuffer<Item> {
        pub data: *const Item,
        pub len: usize,
    }

    impl<Item> ConstBuffer<Item> {
        /// Creates a new [`ConstBuffer`] from a vector.
        ///
        /// ATTENTION: This method intentionally cause a "memory leak" so that rustc doesn't drop the
        /// data when it goes out of scope. Remeber to call the destructor!.
        pub fn new(input: Vec<Item>) -> ConstBuffer<Item> {
            let mut buf = input.into_boxed_slice();
            let data = buf.as_mut_ptr();
            let len = buf.len();

            std::mem::forget(buf);
            ConstBuffer { data, len }
        }

        /// Returns number of items in ConstBuffer
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    unsafe impl<Item> IntoFfi for ConstBuffer<Item> {
        type Value = *const ConstBuffer<Item>;

        fn into_ffi_value(self) -> Self::Value {
            Box::into_raw(Box::new(self))
        }

        fn ffi_default() -> Self::Value {
            ptr::null()
        }
    }

    //#[no_mangle]
    ///// Returns number of items in a `ConstBuffer`
    /////
    ///// Returns -1 on failure.
    //pub unsafe extern "C" fn const_buffer_string_len(buf: *const ConstBuffer) -> c_int {
    //    if buf.is_null() {
    //        eprintln!("Error: Tried free non-existent ConstBuffer");
    //        return -1;
    //    }
    //    (&*buf).len() as c_int
    //}

    //#[no_mangle]
    ///// Frees a ConstBuffer.
    //pub unsafe extern "C" fn const_buffer_string_free(buf: *const ConstBuffer<*const c_char>) {
    //    if buf.is_null() {
    //        eprintln!("Warning: tried to free non-existent ConstBuffer");
    //        return;
    //    }
    //    let buf = &*buf;
    //    let s = std::slice::from_raw_parts(buf.data, buf.len);
    //    let s = s.as_ptr();
    //    Box::new(s);
    //}
}

/// Utility function, converts Rust [`String`] to C-friendly null terminated pointer.
fn string_to_ptr(s: String) -> *const c_char {
    let cs = match CString::new(s) {
        Ok(cs) => cs,
        Err(_) => {
            eprintln!("Error: Failed to convert Rust string");
            return ptr::null();
        }
    };

    let p = cs.as_ptr();
    std::mem::forget(cs);
    p
}

#[cfg(test)]
mod tests {
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn db_init() {
        unsafe {
            let e = super::ExternError::default();
            let e = Box::into_raw(Box::new(e));
            super::db::herald_db_init(e);

            assert_eq!((&*e).get_code().code(), 0);
        };
    }

    #[test]
    #[serial]
    fn db_close() {
        unsafe {
            let e = super::ExternError::default();
            let e = Box::into_raw(Box::new(e));
            let db = super::db::herald_db_init(e);

            super::db::herald_db_close(db);
        };
    }

    #[test]
    #[serial]
    fn contact_create_table() {
        unsafe {
            let e = super::ExternError::default();
            let e = Box::into_raw(Box::new(e));
            let db = super::db::herald_db_init(e);

            super::contacts::herald_contacts_create_table(db, e);
            assert_eq!((&*e).get_code().code(), 0);

            super::db::herald_db_close(db);
        };
    }

    #[test]
    #[serial]
    fn contact_insert() {
        unsafe {
            let e = super::ExternError::default();
            let e = Box::into_raw(Box::new(e));
            let db = super::db::herald_db_init(e);

            super::contacts::herald_contacts_drop(db, e);
            super::contacts::herald_contacts_create_table(db, e);

            let uid = super::contacts::herald_contacts_add(
                db,
                super::string_to_ptr("Hello World".into()),
                e,
            );
            assert_eq!(uid, 1);
            assert_eq!((&*e).get_code().code(), 0);

            super::db::herald_db_close(db);
        };
    }

    #[test]
    #[serial]
    fn get_contacts() {
        unsafe {
            let e = super::ExternError::default();
            let e = Box::into_raw(Box::new(e));
            let db = super::db::herald_db_init(e);

            super::contacts::herald_contacts_drop(db, e);
            super::contacts::herald_contacts_create_table(db, e);

            super::contacts::herald_contacts_add(db, super::string_to_ptr("Hello".into()), e);
            super::contacts::herald_contacts_add(db, super::string_to_ptr("World".into()), e);

            let contacts = super::contacts::herald_contacts_load(db, e);
            assert_eq!((*contacts).len, 2);

            super::db::herald_db_close(db);
        };
    }
}
