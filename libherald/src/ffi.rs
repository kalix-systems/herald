use crate::{db::Database, models::Contact};
use libc::c_int;
use std::ptr;

#[no_mangle]
/// Opens connection to cannonical sqlite3 database.
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
/// Closes connections to cannonical sqlite3 database.
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
        eprintln!("Couldn't create contacts table, null pointer");
        -1
    } else {
        match Contact::create_table(&mut *db) {
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
/// Creates empty contacts table in database.
///
/// Returns 0 on success, -1 if the pointer to the database was null, and -2 if the
/// insertion failed.
pub unsafe extern "C" fn contact_insert(db: *mut Database) -> c_int {
    if db.is_null() {
        eprintln!("Couldn't create contacts table, null pointer");
        -1
    } else {
        match Contact::insert(&mut *db) {
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
}
