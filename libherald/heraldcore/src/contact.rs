use crate::{
    db::{DBTable, Database},
    errors::HErr,
    image_utils,
};
use herald_common::{ConversationId, UserId, UserIdRef};
use rusqlite::{params, NO_PARAMS};
use std::convert::TryInto;

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

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/contact/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

impl Contacts {
    /// Inserts contact into contacts table.
    /// TODO AHH MUTEX
    pub fn add_contact(
        id: UserIdRef,
        name: Option<&str>,
        profile_picture: Option<&str>,
        color: Option<u32>,
        status: ContactStatus,
        pairwise_conversation: Option<ConversationId>,
    ) -> Result<(), HErr> {
        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id));

        let pairwise_conversation = match pairwise_conversation {
            Some(conv_id) => {
                crate::conversation::Conversations::add_conversation(Some(&conv_id), name)?
            }
            None => crate::conversation::Conversations::add_conversation(None, name)?,
        };

        let mut db = Database::get()?;

        let tx = db.transaction()?;
        tx.execute(
            include_str!("sql/contact/add.sql"),
            params![
                id,
                name,
                profile_picture,
                color,
                status as u8,
                pairwise_conversation.as_slice()
            ],
        )?;
        tx.execute(
            include_str!("sql/members/add_member.sql"),
            params![pairwise_conversation.as_slice(), id],
        )?;
        tx.commit()?;

        Ok(())
    }

    /// Gets a contact's name by their `id`.
    pub fn name(id: UserIdRef) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

        Ok(stmt.query_row(&[id], |row| row.get(0))?)
    }

    /// Change name of contact by their `id`
    pub fn set_name(id: UserIdRef, name: Option<&str>) -> Result<(), HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

        stmt.execute(params![name, id])?;
        Ok(())
    }

    /// Gets a contact's profile picture by their `id`.
    pub fn profile_picture(id: UserIdRef) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_profile_picture.sql"))?;

        Ok(stmt.query_row(&[id], |row| row.get(0))?)
    }

    /// Updates a contact's profile picture.
    pub fn set_profile_picture(
        id: UserIdRef,
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
            params![profile_picture, id],
        )?;
        Ok(profile_picture)
    }

    /// Sets a contact's color
    pub fn set_color(id: &str, color: u32) -> Result<(), HErr> {
        let db = Database::get()?;

        db.execute(
            include_str!("sql/contact/update_color.sql"),
            params![id, color],
        )?;
        Ok(())
    }

    /// Indicates whether contact exists
    pub fn contact_exists(id: &str) -> Result<bool, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/contact/contact_exists.sql"))?;
        Ok(stmt.exists(&[id])?)
    }

    /// Sets contact status
    pub fn set_status(id: &str, status: ContactStatus) -> Result<(), HErr> {
        // TODO
        Ok(())
    }

    /// Returns all contacts, including archived contacts
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

    /// Returns all active contacts excluding archived contacts
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
#[repr(u8)]
/// Status of the contact
pub enum ContactStatus {
    /// The contact is active
    Active = 0,
    /// The contact is archived
    Archived = 1,
    /// The contact is deleted
    Deleted = 2,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
/// Type of the contact
pub enum ContactType {
    /// The contact is local
    Local = 0,
    /// The contact is remote
    Remote = 1,
}

impl rusqlite::types::FromSql for ContactStatus {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl std::convert::TryFrom<u8> for ContactStatus {
    type Error = HErr;

    fn try_from(n: u8) -> Result<Self, HErr> {
        use ContactStatus::*;
        match n {
            0 => Ok(Active),
            1 => Ok(Archived),
            2 => Ok(Deleted),
            unknown => Err(HErr::HeraldError(format!(
                "Unknown contact status {}",
                unknown
            ))),
        }
    }
}

impl std::convert::TryFrom<i64> for ContactStatus {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n.try_into(),
            Err(_) => Err(HErr::HeraldError(format!("Unknown contact status {}", n))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A Herald contact.
pub struct Contact {
    /// Contact id
    pub id: UserId,
    /// Contact name
    pub name: Option<String>,
    /// Path of profile picture
    pub profile_picture: Option<String>,
    /// User set color for user
    pub color: u32,
    /// Indicates whether user is archived
    pub status: ContactStatus,
    /// Pairwise conversation corresponding to contact
    pub pairwise_conversation: ConversationId,
}

impl Contact {
    /// Create new contact.
    pub fn new(
        id: String,
        name: Option<String>,
        profile_picture: Option<String>,
        color: Option<u32>,
        status: ContactStatus,
        pairwise_conversation: ConversationId,
    ) -> Self {
        let color = color.unwrap_or_else(|| crate::utils::id_to_color(&id));

        Contact {
            name,
            id,
            profile_picture,
            color,
            status,
            pairwise_conversation,
        }
    }

    /// Returns name
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Sets contact name
    pub fn set_name(&mut self, name: Option<&str>) -> Result<(), HErr> {
        Contacts::set_name(&self.id, name)?;
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

    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Contact {
            id: row.get(0)?,
            name: row.get(1)?,
            profile_picture: row.get(2)?,
            color: row.get(3)?,
            status: row.get(4)?,
            pairwise_conversation: row.get::<_, Vec<u8>>(5)?.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use serial_test_derive::serial;
    use womp::*;

    #[test]
    #[serial]
    fn create_drop_exists() {
        Database::reset_all().expect(womp!());
        // drop twice, it shouldn't panic on multiple drops
        Contacts::drop_table().expect(womp!());
        Contacts::drop_table().expect(womp!());

        Contacts::create_table().expect(womp!());
        assert!(Contacts::exists().expect(womp!()));
        Contacts::create_table().expect(womp!());
        assert!(Contacts::exists().expect(womp!()));
        Contacts::drop_table().expect(womp!());
        assert!(!Contacts::exists().expect(womp!()));
    }

    #[test]
    #[serial]
    fn add_contact() {
        Database::reset_all().expect(womp!());

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add_contact(id1, Some("name"), None, None, ContactStatus::Active, None)
            .expect("Failed to add contact");
        Contacts::add_contact(id2, None, None, Some(1), ContactStatus::Active, None)
            .expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        Database::reset_all().expect(womp!());

        let id = "Hello World";

        Contacts::add_contact(id, Some("name"), None, None, ContactStatus::Active, None)
            .expect("Failed to add contact");
        assert_eq!(
            Contacts::name(id)
                .expect("Failed to get name")
                .expect(womp!()),
            "name"
        );
    }

    #[test]
    #[serial]
    fn get_contact_profile_picture() {
        Database::reset_all().expect(womp!());

        let id = "Hello World";
        let profile_picture = "picture";
        Contacts::add_contact(
            id,
            None,
            Some(profile_picture),
            None,
            ContactStatus::Active,
            None,
        )
        .expect("Failed to add contact");
        assert_eq!(
            Contacts::profile_picture(id.into())
                .expect("Failed to get profile picture")
                .expect(womp!())
                .as_str(),
            profile_picture
        );
    }

    #[test]
    #[serial]
    fn update_name() {
        Database::reset_all().expect(womp!());

        let id = "userid";

        Contacts::add_contact(id, Some("Hello"), None, None, ContactStatus::Active, None)
            .expect(womp!());
        Contacts::set_name(id, Some("World")).expect("Failed to update name");

        assert_eq!(
            Contacts::name(id)
                .expect("Failed to get contact")
                .expect(womp!()),
            "World"
        );
    }

    #[test]
    #[serial]
    fn all_contacts() {
        Database::reset_all().expect(womp!());

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add_contact(id1, None, None, None, ContactStatus::Active, None)
            .expect("Failed to add id1");
        Contacts::add_contact(id2, None, None, None, ContactStatus::Active, None)
            .expect("Failed to add id2");

        let contacts = Contacts::all().expect(womp!());
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].id, id1);
        assert_eq!(contacts[1].id, id2);
    }

    //#[test]
    //#[serial]
    //fn archive_contact() {
    //    Database::reset_all().expect(womp!());

    //    let id = "Hello World";
    //    Contacts::add_contact(id, None, None, None, ContactStatus::Active).expect(womp!());
    //    Contacts::archive(id).expect(womp!());

    //    assert!(Contacts::is_archived(id).expect("Failed to determine if contact was archived"));
    //}

    //#[test]
    //#[serial]
    //fn get_active_contacts() {
    //    Database::reset_all().expect(womp!());

    //    let id1 = "Hello";
    //    let id2 = "World";

    //    Contacts::add_contact(id1, None, None, None, ContactStatus::Active).expect(womp!());
    //    Contacts::add_contact(id2, None, None, None, ContactStatus::Active).expect(womp!());

    //    // Contacts::archive(id2).expect(womp!());

    //    let contacts = Contacts::active().expect(womp!());
    //    assert_eq!(contacts.len(), 1);
    //    assert_eq!(contacts[0].id, id1);
    //}
}
