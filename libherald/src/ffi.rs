use crate::{db::Database, models::contact};
use libc::{c_char, c_int};
use std::{ffi::CString, ptr};

#[no_mangle]
/// Opens connection to canonical sqlite3 database.
///
/// Returns a null pointer on failure.
pub unsafe extern "C" fn database_open() -> *mut Database {
    match Database::new() {
        Ok(db) => {
            println!("Connected successfully");
            Box::into_raw(Box::new(db))
        }
        Err(_) => {
            eprintln!("Error: Failed to connect");
            ptr::null_mut()
        }
    }
}

#[no_mangle]
/// Closes connections to canonical sqlite3 database.
pub unsafe extern "C" fn database_close(db: *mut Database) {
    if db.is_null() {
        eprintln!("Error: Tried to close non-open database")
    } else {
        drop(Box::from_raw(db));
    }
}

#[no_mangle]
/// Creates empty contacts table in database.
///
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the table could not
/// be created.
pub unsafe extern "C" fn contacts_create_table(db: *mut Database) -> c_int {
    if db.is_null() {
        eprintln!("Error: Couldn't create contacts table, null pointer");
        -1
    } else {
        match contact::create_table(&mut *db) {
            Ok(_) => {
                println!("Created contacts table");
                0
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                -2
            }
        }
    }
}

#[no_mangle]
/// Insert default entries into contacts table in database.
///
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the
/// insertion failed.
pub unsafe extern "C" fn contact_insert(db: *mut Database) -> c_int {
    if db.is_null() {
        eprintln!("Error: Couldn't create contacts table, null pointer");
        return -1;
    }
    match contact::insert(&mut *db) {
        Ok(_) => {
            println!("Contact insertion succeeded");
            0
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            -2
        }
    }
}

#[repr(C)]
/// A constant buffer, templated over the `Item` type.
pub struct ConstBuffer<Item> {
    data: *const Item,
    len: usize,
}

impl<Item> ConstBuffer<Item> {
    /// Creates a new [`ConstBuffer`] from a vector.
    ///
    /// ATTENTION: This method intentionally cause a "memory leak" so that rustc doesn't drop the
    /// data when it goes out of scope. Remeber to call [`const_buffer_free`]!.
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

#[no_mangle]
/// Returns number of items in a `ConstBuffer`
///
/// Returns -1 on failure.
pub unsafe extern "C" fn const_buffer_string_len(buf: *const ConstBuffer<*const c_char>) -> c_int {
    if buf.is_null() {
        eprintln!("Error: Tried free non-existent ConstBuffer");
        return -1;
    }
    (&*buf).len() as c_int
}

#[no_mangle]
/// Frees a ConstBuffer.
pub unsafe extern "C" fn const_buffer_string_free(buf: *const ConstBuffer<*const c_char>) {
    if buf.is_null() {
        eprintln!("Warning: tried to free non-existent ConstBuffer");
        return;
    }
    let buf = &*buf;
    let s = std::slice::from_raw_parts(buf.data, buf.len);
    let s = s.as_ptr();
    Box::new(s);
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

#[no_mangle]
/// Gets a buffer of contact strings.
///
/// Returns null pointer on failure.
pub unsafe extern "C" fn contacts_get(db: *mut Database) -> *const ConstBuffer<*const c_char> {
    if db.is_null() {
        eprintln!("Error: Couldn't get contacts, database pointer null");
        return ptr::null_mut();
    };

    let contacts = match contact::contacts(&mut *db) {
        Ok(contacts) => contacts,
        Err(_) => {
            eprintln!("Error: Couldn't fetch contacts");
            return ptr::null_mut();
        }
    };

    let data: Vec<*const c_char> = contacts.into_iter().map(string_to_ptr).collect();

    Box::into_raw(Box::new(ConstBuffer::new(data)))
}

#[cfg(test)]
mod tests {
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn connect() {
        unsafe {
            assert!(!super::database_open().is_null());
        }
    }

    #[test]
    #[serial]
    fn contact_create_table() {
        // start fresh
        crate::utils::delete_db();

        unsafe {
            let db = super::database_open();
            assert_eq!(0, super::contacts_create_table(db));
            super::database_close(db);
        }
    }

    #[test]
    #[serial]
    fn contact_insert() {
        // start fresh
        crate::utils::delete_db();

        unsafe {
            let db = super::database_open();
            super::contacts_create_table(db);
            super::contact_insert(db);

            super::database_close(db);
        }
    }

    #[test]
    #[serial]
    fn get_contacts() {
        // start fresh
        crate::utils::delete_db();

        unsafe {
            let db = super::database_open();
            super::contacts_create_table(db);
            super::contact_insert(db);

            let contacts = super::contacts_get(db);
            if contacts.is_null() {
                panic!("contacts_get returned null pointer")
            };
            assert_eq!(super::const_buffer_string_len(contacts), 3);
            super::const_buffer_string_free(contacts);
            assert_eq!(super::const_buffer_string_len(contacts), 3);
            super::database_close(db);
        }
    }
}
