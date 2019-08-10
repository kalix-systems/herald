use crate::{db::Database, errors::HErr};
use rusqlite::NO_PARAMS;

#[allow(dead_code)]
pub(crate) mod contact {
    #[derive(Debug, PartialEq)]
    pub struct Contact {
        pub name: String,
        pub uid: i64,
    }

    impl Contact {
        pub fn new(name: String, uid: i64) -> Self {
            Contact { name, uid }
        }
    }
    use super::*;
    /// Creates empty contacts table.
    pub fn create_table(db: &mut Database) -> Result<(), HErr> {
        db.execute(
            "CREATE TABLE IF NOT EXISTS contacts (
                    'uid'  INTEGER PRIMARY KEY,
                    'name' TEXT
           )",
            NO_PARAMS,
        )?;

        Ok(())
    }

    /// Inserts contact into contacts table. On success, returns the contacts UID.
    pub fn add(db: &mut Database, name: &str) -> Result<i64, HErr> {
        let mut stmt = db.prepare("INSERT INTO contacts(uid, name) VALUES(NULL, ?)")?;
        stmt.execute(&[name])?;

        Ok(db.last_insert_rowid())
    }

    /// Returns all contacts.
    pub fn get_all(db: &mut Database) -> Result<Vec<Contact>, HErr> {
        let mut stmt = db.prepare("SELECT uid, name FROM contacts")?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            Ok(Contact {
                uid: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }

    /// Drops the contacts table.
    pub fn drop(db: &mut Database) -> Result<(), HErr> {
        db.execute("DROP TABLE IF EXISTS contacts", NO_PARAMS)?;
        Ok(())
    }
}

//struct Message {
//    author: String,
//    recipient: String,
//    timestamp: String,
//    message: String,
//}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn drop_contacts() {
        let mut db = Database::new().unwrap();
        contact::drop(&mut db).unwrap();
    }

    #[test]
    #[serial]
    fn create_contacts() {
        let mut db = Database::new().unwrap();
        contact::drop(&mut db).unwrap();

        contact::create_table(&mut db).unwrap();
    }

    #[test]
    #[serial]
    fn add_contact() {
        let mut db = Database::new().unwrap();
        contact::drop(&mut db).unwrap();

        contact::create_table(&mut db).unwrap();
        contact::add(&mut db, "Hello World").expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn get_contacts() {
        let mut db = Database::new().unwrap();
        contact::drop(&mut db).unwrap();

        contact::create_table(&mut db).unwrap();

        contact::add(&mut db, "Hello").unwrap();
        contact::add(&mut db, "World").unwrap();

        let contacts = contact::get_all(&mut db).unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0], contact::Contact::new("Hello".into(), 1));
        assert_eq!(contacts[1], contact::Contact::new("World".into(), 2));
    }
}
