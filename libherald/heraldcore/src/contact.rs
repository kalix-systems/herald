use crate::{
    db::{DBTable, Database},
    errors::HErr,
    image_utils,
};
use rusqlite::{
    types::{Null, ToSql},
    NO_PARAMS,
};

#[derive(Default)]
/// Wrapper around contacts table.
/// TODO This will be stateful when we have caching logic.
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
    /// Inserts contact into contacts table.
    pub fn add(
        id: &str,
        name: Option<&str>,
        profile_picture: Option<&str>,
        color: Option<u32>,
    ) -> Result<(), HErr> {
        let name = match name {
            Some(name) => name.to_sql()?,
            None => id.to_sql()?,
        };

        let profile_picture = match profile_picture {
            Some(profile_picture) => profile_picture.to_sql()?,
            None => Null.to_sql()?,
        };

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id));

        let db = Database::get()?;
        db.execute(
            include_str!("sql/contact/add.sql"),
            &[id.to_sql()?, name, profile_picture, color.to_sql()?],
        )?;
        Ok(())
    }

    /// Gets a contact's name by their `id`.
    pub fn name(id: &str) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

        Ok(stmt.query_row(&[id], |row| row.get(0))?)
    }

    /// Change name of contact by their `id`
    pub fn set_name(id: &str, name: Option<&str>) -> Result<(), HErr> {
        let name = match name {
            Some(name) => name.to_sql()?,
            None => Null.to_sql()?,
        };

        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

        stmt.execute(&[name, id.to_sql()?])?;
        Ok(())
    }

    /// Gets a contact's profile picture by their `id`.
    pub fn profile_picture(id: String) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_profile_picture.sql"))?;

        Ok(stmt.query_row(&[id.as_str()], |row| row.get(0))?)
    }

    /// Updates a contact's profile picture.
    pub fn set_profile_picture(
        id: &str,
        profile_picture: Option<String>,
        old_path: Option<&str>,
    ) -> Result<Option<String>, HErr> {
        let profile_picture = match profile_picture {
            Some(path) => {
                let path_string =
                    image_utils::save_profile_picture(id, path, old_path.map(|p| p.into()))?
                        .into_os_string()
                        .into_string()?;
                Some(path_string)
            }
            None => None,
        };

        let db = Database::get()?;

        db.execute(
            include_str!("sql/contact/update_profile_picture.sql"),
            &[profile_picture.to_sql()?, id.to_sql()?],
        )?;
        Ok(profile_picture)
    }

    /// Sets a contact's color
    pub fn set_color(id: &str, color: u32) -> Result<(), HErr> {
        let db = Database::get()?;

        db.execute(
            include_str!("sql/contact/update_color.sql"),
            &[id.to_sql()?, color.to_sql()?],
        )?;
        Ok(())
    }

    /// Indicates whether contact exists
    pub fn contact_exists(id: &str) -> Result<bool, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/contact/contact_exists.sql"))?;
        Ok(stmt.exists(&[id])?)
    }

    /// Deletes a contact by their `id`.
    pub fn delete(id: &str) -> Result<(), HErr> {
        let mut db = Database::get()?;

        let tx = db.transaction()?;
        tx.execute(include_str!("sql/message/delete_conversation.sql"), &[id])?;
        tx.execute(include_str!("sql/contact/delete_contact.sql"), &[id])?;
        tx.commit()?;
        Ok(())
    }

    /// Archives a contact if it is not already archived.
    pub fn archive(id: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/archive_contact.sql"), &[id])?;
        Ok(())
    }

    /// Activates contact if it is not already activated.
    pub fn activate(id: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/activate_contact.sql"), &[id])?;
        Ok(())
    }

    /// Indicates whether a contact is archived.
    pub fn is_archived(id: &str) -> Result<bool, HErr> {
        let db = Database::get()?;

        let val: i64 = db.query_row(include_str!("sql/contact/is_archived.sql"), &[id], |row| {
            Ok(row.get(0)?)
        })?;

        Ok(val == 1)
    }

    /// Returns all contact, including archived contacts
    pub fn all() -> Result<Vec<Contact>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_all.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, Contact::from_db)?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }

    /// Returns all active contacts excluding archived Contacts::
    pub fn active() -> Result<Vec<Contact>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_active.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, Contact::from_db)?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
/// Whether or not the the contact is archived
pub enum ArchiveStatus {
    /// The contact is active
    Active,
    /// The contact is archived
    Archived,
}

impl From<bool> for ArchiveStatus {
    fn from(archived: bool) -> Self {
        if archived {
            ArchiveStatus::Archived
        } else {
            ArchiveStatus::Active
        }
    }
}

impl From<ArchiveStatus> for bool {
    fn from(archived: ArchiveStatus) -> Self {
        match archived {
            ArchiveStatus::Archived => true,
            ArchiveStatus::Active => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A Herald contact.
pub struct Contact {
    /// Contact id
    pub id: String,
    /// Contact name
    pub name: Option<String>,
    /// Path of profile picture
    pub profile_picture: Option<String>,
    /// User set color for user
    pub color: u32,
    /// Indicates wheter user is archived
    pub archive_status: ArchiveStatus,
}

impl Contact {
    /// Create new contact.
    pub fn new(
        id: String,
        name: Option<String>,
        profile_picture: Option<String>,
        color: Option<u32>,
        archive_status: ArchiveStatus,
    ) -> Self {
        let color = color.unwrap_or_else(|| crate::utils::id_to_color(&id));
        Contact {
            name,
            id,
            profile_picture,
            color,
            archive_status,
        }
    }

    /// Returns name
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Sets contact name
    pub fn set_name(&mut self, name: Option<&str>) -> Result<(), HErr> {
        Contacts::set_name(self.id.as_str(), name)?;
        self.name = name.map(|s| s.to_owned());
        Ok(())
    }

    /// Returns path to profile picture
    pub fn profile_picture(&self) -> Option<&str> {
        self.profile_picture.as_ref().map(|s| s.as_ref())
    }

    /// Sets profile picture
    pub fn set_profile_picture(&mut self, profile_picture: Option<String>) -> Result<(), HErr> {
        let path = Contacts::set_profile_picture(
            self.id.as_str(),
            profile_picture,
            self.profile_picture.as_ref().map(|p| p.as_str()),
        )?;
        self.profile_picture = path;
        Ok(())
    }

    /// Returns contact's color
    pub fn color(&self) -> u32 {
        self.color
    }

    /// Sets color
    pub fn set_color(&mut self, color: u32) -> Result<(), HErr> {
        Contacts::set_color(self.id.as_str(), color)?;
        self.color = color;
        Ok(())
    }

    /// Archives the contact if it is active.
    pub fn archive(&mut self) -> Result<(), HErr> {
        Contacts::archive(self.id.as_str())?;
        self.archive_status = ArchiveStatus::Archived;
        Ok(())
    }

    /// Activates the contact if it is archived.
    pub fn activate(&mut self) -> Result<(), HErr> {
        Contacts::activate(self.id.as_str())?;
        self.archive_status = ArchiveStatus::Active;
        Ok(())
    }

    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let archive_status: bool = row.get(4)?;

        Ok(Contact {
            id: row.get(0)?,
            name: row.get(1)?,
            profile_picture: row.get(2)?,
            color: row.get(3)?,
            archive_status: archive_status.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::Conversations;
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
        Contacts::reset().unwrap();
        Conversations::reset().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, Some("name"), None, None).expect("Failed to add contact");
        Contacts::add(id2, None, None, Some(1)).expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn delete_contact() {
        Contacts::reset().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, None, None, None).expect("Failed to add contact");
        Contacts::add(id2, None, None, None).expect("Failed to add contact");
        crate::message::Messages::create_table().unwrap();

        Contacts::delete(id1).expect("Failed to delete contact");

        assert!(Contacts::name(id1).unwrap().is_none());
        assert!(Contacts::name(id2).is_ok());
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        Contacts::reset().unwrap();

        let id = "Hello World";

        Contacts::add(id, Some("name"), None, None).expect("Failed to add contact");
        assert_eq!(
            Contacts::name(id).expect("Failed to get name").unwrap(),
            "name"
        );
    }

    #[test]
    #[serial]
    fn get_contact_profile_picture() {
        Contacts::reset().unwrap();

        let id = "Hello World";
        let profile_picture = "picture";
        Contacts::add(id, None, Some(profile_picture), None).expect("Failed to add contact");
        assert_eq!(
            Contacts::profile_picture(id.into())
                .expect("Failed to get profile picture")
                .unwrap()
                .as_str(),
            profile_picture
        );
    }

    #[test]
    #[serial]
    fn update_name() {
        Contacts::reset().unwrap();

        let id = "userid";

        Contacts::add(id, Some("Hello"), None, None).unwrap();
        Contacts::set_name(id, Some("World")).expect("Failed to update name");

        assert_eq!(
            Contacts::name(id).expect("Failed to get contact").unwrap(),
            "World"
        );
    }

    #[test]
    #[serial]
    fn all_contacts() {
        Contacts::reset().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, None, None, None).expect("Failed to add id1");
        Contacts::add(id2, None, None, None).expect("Failed to add id2");

        let contacts = Contacts::all().unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].id, id1);
        assert_eq!(contacts[1].id, id2);
    }

    #[test]
    #[serial]
    fn archive_contact() {
        Contacts::reset().unwrap();

        let id = "Hello World";
        Contacts::add(id, None, None, None).unwrap();
        Contacts::archive(id).unwrap();

        assert!(Contacts::is_archived(id).expect("Failed to determine if contact was archived"));
    }

    #[test]
    #[serial]
    fn get_active_contacts() {
        Contacts::reset().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, None, None, None).unwrap();
        Contacts::add(id2, None, None, None).unwrap();

        Contacts::archive(id2).unwrap();

        let contacts = Contacts::active().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].id, id1);
    }
}
