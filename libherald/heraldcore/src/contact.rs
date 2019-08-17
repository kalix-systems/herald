use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
/// Wrapper around contacts table.
pub struct Contacts {
    db: Database,
}

impl DBTable for Contacts {
    fn create_table(&self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/create_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table(&self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists(&self) -> bool {
        let db = &self.db;
        if let Ok(mut stmt) = db.prepare(include_str!("sql/contact/table_exists.sql")) {
            stmt.exists(NO_PARAMS).unwrap_or(false)
        } else {
            false
        }
    }
}

impl Contacts {
    #![allow(dead_code)]
    pub(crate) fn new(db: Database) -> Self {
        Contacts { db }
    }

    /// Inserts contact into contacts table. On success, returns the contacts UID.
    pub fn add(&self, name: &str, profile_picture: Option<&[u8]>) -> Result<i64, HErr> {
        let db = &self.db;
        match profile_picture {
            None => {
                let mut stmt = db.prepare(include_str!("sql/contact/add.sql"))?;
                stmt.execute(&[name])?;
            }
            Some(picture) => {
                let mut stmt =
                    db.prepare(include_str!("sql/contact/add_with_profile_picture.sql"))?;
                stmt.execute(&[name.as_bytes(), picture])?;
            }
        }
        Ok(db.last_insert_rowid())
    }

    /// Indicates whether contact exists
    pub fn contact_exists(&self, id: i64) -> Result<bool, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/contact_exists.sql"))?;
        Ok(stmt.exists(&[id])?)
    }

    /// Change name of contact by their `id`
    pub fn update_name(&self, id: i64, name: &str) -> Result<(), HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

        stmt.execute(&[name, &id.to_string()])?;
        Ok(())
    }

    /// Gets a contact's name by their `id`.
    pub fn get_name(&self, id: i64) -> Result<String, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

        Ok(stmt.query_row(&[id], |row| row.get(0))?)
    }

    /// Gets a contact's profile picture by their `id`.
    pub fn get_profile_picture(&self, id: i64) -> Result<Vec<u8>, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/get_profile_picture.sql"))?;

        Ok(stmt.query_row(&[id], |row| row.get(0))?)
    }

    /// Deletes a contact by their `id`.
    pub fn delete(&self, id: i64) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/delete.sql"), &[id])?;
        Ok(())
    }

    /// Returns all contacts, including archived contacts.
    pub fn get_all(&self) -> Result<Vec<Contact>, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/get_all.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            Ok(Contact {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }

    /// Returns all active contacts, excluding archived contacts.
    pub fn get_active(&self) -> Result<Vec<Contact>, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/get_active.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            Ok(Contact {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }

    /// Archives a contact.
    pub fn archive(&self, id: i64) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/archive_contact.sql"), &[id])?;
        Ok(())
    }

    /// Indicates whether a contact is archived.
    pub fn is_archived(&self, id: i64) -> Result<bool, HErr> {
        let db = &self.db;

        let val: i64 = db.query_row(include_str!("sql/contact/is_archived.sql"), &[id], |row| {
            Ok(row.get(0)?)
        })?;

        Ok(val == 1)
    }
}

#[derive(Debug, PartialEq)]
/// A Herald contact.
pub struct Contact {
    /// Contact name
    pub name: String,
    /// Contact id
    pub id: i64,
}

impl Contact {
    /// Create new contact.
    pub fn new(name: String, id: i64) -> Self {
        Contact { name, id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_drop_exists() {
        let contacts = Contacts::default();
        // drop twice, it shouldn't panic on multiple drops
        contacts.drop_table().unwrap();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        assert!(contacts.exists());
        contacts.create_table().unwrap();
        assert!(contacts.exists());
        contacts.drop_table().unwrap();
        assert!(!contacts.exists());
    }

    #[test]
    #[serial]
    fn add_contact() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        contacts
            .add("Hello World", None)
            .expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn delete_contact() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        let id1 = contacts.add("Hello", None).expect("Failed to add contact");
        let id2 = contacts.add("World", None).expect("Failed to add contact");

        contacts.delete(id1).expect("Failed to delete contact");

        assert!(contacts.get_name(id1).is_err());
        assert!(contacts.get_name(id2).is_ok());
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        let id = contacts
            .add("Hello World", None)
            .expect("Failed to add contact");
        assert_eq!(
            contacts.get_name(id).expect("Failed to get name"),
            "Hello World"
        );
    }

    #[test]
    #[serial]
    fn get_contact_profile_picture() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        let id = contacts
            .add("Hello World", Some(&[0]))
            .expect("Failed to add contact");
        assert_eq!(
            contacts
                .get_profile_picture(id)
                .expect("Failed to get name"),
            &[0]
        );
    }

    #[test]
    #[serial]
    fn update_name() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();
        contacts.create_table().unwrap();

        let id = contacts.add("Hello", None).unwrap();
        contacts
            .update_name(id, "World")
            .expect("Failed to update name");
        assert_eq!(
            contacts.get_name(id).expect("Failed to get contact"),
            "World"
        );
    }

    #[test]
    #[serial]
    fn get_all_contacts() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();

        contacts.add("Hello", None).unwrap();
        contacts.add("World", None).unwrap();

        let contacts_vec = contacts.get_all().unwrap();
        assert_eq!(contacts_vec.len(), 2);
        assert_eq!(contacts_vec[0], Contact::new("Hello".into(), 1));
        assert_eq!(contacts_vec[1], Contact::new("World".into(), 2));
    }

    #[test]
    #[serial]
    fn archive_contact() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();

        let id = contacts.add("Hello World", None).unwrap();
        contacts.archive(id).unwrap();

        assert!(contacts
            .is_archived(id)
            .expect("Failed to determine if contact was archived"));
    }

    #[test]
    #[serial]
    fn get_active_contacts() {
        let contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();

        contacts.add("Hello", None).unwrap();

        let archived_id = contacts.add("World", None).unwrap();
        contacts.archive(archived_id).unwrap();

        let contacts_vec = contacts.get_active().unwrap();
        assert_eq!(contacts_vec.len(), 1);
        assert_eq!(contacts_vec[0], Contact::new("Hello".into(), 1));
    }
}
