use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use herald_common::UserId;
use rusqlite::{
    types::{Null, ToSql},
    NO_PARAMS,
};

#[derive(Default)]
/// Wrapper around Contacts::table.
/// TODO This possibly should just be a module,
/// but we might want to store state in it?
pub struct Contacts {}

impl DBTable for Contacts {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/create_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }
}

impl Contacts {
    /// Inserts contact into Contacts::table.
    pub fn add(id: UserId, name: Option<&str>, profile_picture: Option<&[u8]>) -> Result<(), HErr> {
        let db = Database::get()?;

        let name = match name {
            Some(name) => name.to_sql()?,
            None => Null.to_sql()?,
        };

        let profile_picture = match profile_picture {
            Some(profile_picture) => profile_picture.to_sql()?,
            None => Null.to_sql()?,
        };

        db.execute(
            include_str!("sql/contact/add.sql"),
            &[id.as_str().to_sql()?, name, profile_picture],
        )?;

        Ok(())
    }

    /// Indicates whether contact exists
    pub fn contact_exists(id: UserId) -> Result<bool, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/contact/contact_exists.sql"))?;
        Ok(stmt.exists(&[id.as_str()])?)
    }

    /// Change name of contact by their `id`
    pub fn update_name(id: UserId, name: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

        stmt.execute(&[name, id.as_str()])?;
        Ok(())
    }

    /// Gets a contact's name by their `id`.
    pub fn get_name(id: UserId) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

        Ok(stmt.query_row(&[id.as_str()], |row| row.get(0))?)
    }

    /// Gets a contact's profile picture by their `id`.
    pub fn get_profile_picture(id: UserId) -> Result<Vec<u8>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_profile_picture.sql"))?;

        Ok(stmt.query_row(&[id.as_str()], |row| row.get(0))?)
    }

    /// Deletes a contact by their `id`.
    pub fn delete(id: UserId) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/delete.sql"), &[id.as_str()])?;
        Ok(())
    }

    /// Returns all contact, including archived contacts
    pub fn get_all() -> Result<Vec<Contact>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_all.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            let id: String = row.get(0)?;
            Ok(Contact {
                id: UserId::from(id.as_str()).unwrap(),
                name: row.get(1)?,
            })
        })?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }

    /// Returns all active Contacts:: excluding archived Contacts::
    pub fn get_active() -> Result<Vec<Contact>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_active.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            let id: String = row.get(0)?;
            Ok(Contact {
                id: UserId::from(id.as_str()).unwrap(),
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
    pub fn archive(id: UserId) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/contact/archive_contact.sql"),
            &[id.as_str()],
        )?;
        Ok(())
    }

    /// Indicates whether a contact is archived.
    pub fn is_archived(id: UserId) -> Result<bool, HErr> {
        let db = Database::get()?;

        let val: i64 = db.query_row(
            include_str!("sql/contact/is_archived.sql"),
            &[id.as_str()],
            |row| Ok(row.get(0)?),
        )?;

        Ok(val == 1)
    }
}

#[derive(Debug, PartialEq)]
/// A Herald contact.
pub struct Contact {
    /// Contact name
    pub name: Option<String>,
    /// Contact id
    pub id: UserId,
}

impl Contact {
    /// Create new contact.
    pub fn new(name: Option<String>, id: UserId) -> Self {
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
        // drop twice, it shouldn't panic on multiple drops
        Contacts::drop_table().unwrap();
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        assert!(Contacts::exists().unwrap());
        Contacts::create_table().unwrap();
        assert!(Contacts::exists().unwrap());
        Contacts::drop_table().unwrap();
        assert!(!Contacts::exists().unwrap());
    }

    #[test]
    #[serial]
    fn add_contact() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        Contacts::add(UserId::from("Hello World").unwrap(), Some("name"), None)
            .expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn delete_contact() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        let id1 = UserId::from("Hello").unwrap();
        let id2 = UserId::from("World").unwrap();

        Contacts::add(id1, None, None).expect("Failed to add contact");
        Contacts::add(id2, None, None).expect("Failed to add contact");

        Contacts::delete(id1).expect("Failed to delete contact");

        assert!(Contacts::get_name(id1).is_err());
        assert!(Contacts::get_name(id2).is_ok());
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        let id = UserId::from("Hello World").unwrap();

        Contacts::add(id, Some("name"), None).expect("Failed to add contact");
        assert_eq!(
            Contacts::get_name(id).expect("Failed to get name").unwrap(),
            "name"
        );
    }

    #[test]
    #[serial]
    fn get_contact_profile_picture() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        let id = UserId::from("Hello World").unwrap();
        Contacts::add(id, None, Some(&[0])).expect("Failed to add contact");
        assert_eq!(
            Contacts::get_profile_picture(id).expect("Failed to get profile picture"),
            &[0]
        );
    }

    #[test]
    #[serial]
    fn update_name() {
        Contacts::drop_table().unwrap();
        Contacts::create_table().unwrap();

        let id = UserId::from("userid").unwrap();

        Contacts::add(id, Some("Hello"), None).unwrap();
        Contacts::update_name(id, "World").expect("Failed to update name");

        assert_eq!(
            Contacts::get_name(id)
                .expect("Failed to get contact")
                .unwrap(),
            "World"
        );
    }

    #[test]
    #[serial]
    fn get_all_contacts() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id1 = UserId::from("Hello").unwrap();
        let id2 = UserId::from("World").unwrap();

        Contacts::add(id1, None, None).expect("Failed to add id1");
        Contacts::add(id2, None, None).expect("Failed to add id2");

        let contacts = Contacts::get_all().unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].id, id1);
        assert_eq!(contacts[1].id, id2);
    }

    #[test]
    #[serial]
    fn archive_contact() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id = UserId::from("Hello World").unwrap();
        Contacts::add(id, None, None).unwrap();
        Contacts::archive(id).unwrap();

        assert!(Contacts::is_archived(id).expect("Failed to determine if contact was archived"));
    }

    #[test]
    #[serial]
    fn get_active_contacts() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id1 = UserId::from("Hello").unwrap();
        let id2 = UserId::from("World").unwrap();

        Contacts::add(id1, None, None).unwrap();
        Contacts::add(id2, None, None).unwrap();

        Contacts::archive(id2).unwrap();

        let contacts = Contacts::get_active().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].id, id1);
    }
}
