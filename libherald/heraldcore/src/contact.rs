use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
pub struct Contacts {
    db: Database,
}

impl DBTable for Contacts {
    fn create_table(&mut self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/create.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table(&mut self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/drop.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists(&self) -> bool {
        let db = &self.db;

        let cnt = db
            .query_row(include_str!("sql/contact/exists.sql"), NO_PARAMS, |row| {
                row.get(0)
            })
            .unwrap_or(0);

        cnt > 0
    }
}

impl Contacts {
    pub fn new(db: Database) -> Self {
        Contacts { db }
    }

    /// Inserts contact into contacts table. On success, returns the contacts UID.
    pub fn add(&mut self, name: &str) -> Result<i64, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/add.sql"))?;
        stmt.execute(&[name])?;

        Ok(db.last_insert_rowid())
    }

    /// Change name of contact by their `uid`
    pub fn update_name(&mut self, uid: i64, name: &str) -> Result<(), HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

        stmt.execute(&[name, &uid.to_string()])?;
        Ok(())
    }

    /// Gets a contact's name by their `uid`.
    pub fn get_name(&self, uid: i64) -> Result<String, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

        Ok(stmt.query_row(&[uid], |row| row.get(0))?)
    }

    /// Deletes a contact by their `uid`.
    pub fn delete(&mut self, uid: i64) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/contact/delete.sql"), &[uid])?;
        Ok(())
    }

    /// Returns all contacts.
    pub fn get_all(&self) -> Result<Vec<Contact>, HErr> {
        let db = &self.db;
        let mut stmt = db.prepare(include_str!("sql/contact/get_all.sql"))?;

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
}

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

#[cfg(test)]
mod tests {

    use super::*;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn drop_contacts() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();
        contacts.drop_table().unwrap();
    }

    #[test]
    #[serial]
    fn create_contacts() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
    }

    #[test]
    #[serial]
    fn add_contact() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        contacts.add("Hello World").expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn delete_contact() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        let uid1 = contacts.add("Hello").expect("Failed to add contact");
        let uid2 = contacts.add("World").expect("Failed to add contact");

        contacts.delete(uid1).expect("Failed to delete contact");

        assert!(contacts.get_name(uid1).is_err());
        assert!(contacts.get_name(uid2).is_ok());
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();
        let uid = contacts.add("Hello World").expect("Failed to add contact");
        assert_eq!(
            contacts.get_name(uid).expect("Failed to get name"),
            "Hello World"
        );
    }
    #[test]
    #[serial]
    fn update_name() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();
        contacts.create_table().unwrap();

        let uid = contacts.add("Hello").unwrap();
        contacts
            .update_name(uid, "World")
            .expect("Failed to update name");
        assert_eq!(
            contacts.get_name(uid).expect("Failed to get contact"),
            "World"
        );
    }

    #[test]
    #[serial]
    fn get_contacts() {
        let mut contacts = Contacts::default();
        contacts.drop_table().unwrap();

        contacts.create_table().unwrap();

        contacts.add("Hello").unwrap();
        contacts.add("World").unwrap();

        let contacts_vec = contacts.get_all().unwrap();
        assert_eq!(contacts_vec.len(), 2);
        assert_eq!(contacts_vec[0], Contact::new("Hello".into(), 1));
        assert_eq!(contacts_vec[1], Contact::new("World".into(), 2));
    }
}
