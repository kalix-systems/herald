use crate::{db::Database, errors::HErr, image_utils, types::*};
use chrono::{DateTime, TimeZone, Utc};
use herald_common::*;
use rusqlite::params;
use std::convert::TryInto;

/// Gets a contact's name by their `id`.
pub fn name(id: UserId) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_name.sql"))?;

    Ok(stmt.query_row(params![id], |row| row.get(0))?)
}

/// Change name of contact by their `id`
pub fn set_name(id: UserId, name: Option<&str>) -> Result<(), HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/update_name.sql"))?;

    stmt.execute(params![name, id])?;
    Ok(())
}

/// Gets a contact's profile picture by their `id`.
pub fn profile_picture(id: UserId) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_profile_picture.sql"))?;

    Ok(stmt.query_row(params![id], |row| row.get(0))?)
}

/// Returns all members of a conversation.
pub fn conversation_members(conversation_id: &ConversationId) -> Result<Vec<Contact>, HErr> {
    conversation_members_since(conversation_id, chrono::MIN_DATE.and_hms(0, 0, 0))
}

/// Returns all members of a conversation.
pub fn conversation_members_since(
    conversation_id: &ConversationId,
    since: DateTime<Utc>,
) -> Result<Vec<Contact>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_by_conversation.sql"))?;

    let rows = stmt.query_map(
        params![conversation_id, since.timestamp()],
        Contact::from_db,
    )?;

    let mut contacts: Vec<Contact> = Vec::new();
    for contact in rows {
        contacts.push(contact?);
    }

    Ok(contacts)
}

/// Updates a contact's profile picture.
pub fn set_profile_picture(
    id: UserId,
    profile_picture: Option<String>,
    old_path: Option<&str>,
) -> Result<Option<String>, HErr> {
    let profile_picture = match profile_picture {
        Some(path) => {
            let path_string =
                image_utils::save_profile_picture(id.as_str(), path, old_path.map(|p| p.into()))?
                    .into_os_string()
                    .into_string()?;
            Some(path_string)
        }
        None => None,
    };

    let db = Database::get()?;
    db.execute(
        include_str!("sql/update_profile_picture.sql"),
        params![profile_picture, id],
    )?;
    Ok(profile_picture)
}

/// Sets a contact's color
pub fn set_color(id: UserId, color: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(include_str!("sql/update_color.sql"), params![color, id])?;
    Ok(())
}

/// Indicates whether contact exists
pub fn contact_exists(id: UserId) -> Result<bool, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/contact_exists.sql"))?;
    Ok(stmt.exists(&[id])?)
}

/// Sets contact status
pub fn set_status(
    id: UserId,
    pairwise_conv: ConversationId,
    status: ContactStatus,
) -> Result<(), HErr> {
    use ContactStatus::*;
    let mut db = Database::get()?;
    match status {
        Deleted => {
            let tx = db.transaction()?;
            tx.execute(include_str!("sql/delete_contact_meta.sql"), params![id])?;
            crate::message_status::delete_by_conversation_tx(&tx, pairwise_conv)?;
            tx.execute(
                include_str!("../message/sql/delete_pairwise_conversation.sql"),
                params![id],
            )?;
            tx.commit()?;
        }
        _ => {
            db.execute(include_str!("sql/set_status.sql"), params![status, id])?;
        }
    }
    Ok(())
}

/// Gets contact status
pub fn status(id: UserId) -> Result<ContactStatus, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_status.sql"))?;

    Ok(stmt.query_row(&[id], |row| row.get(0))?)
}

/// Returns all contacts
pub fn all() -> Result<Vec<Contact>, HErr> {
    all_since(chrono::MIN_DATE.and_hms(0, 0, 0))
}

/// Returns all contacts
pub fn all_since(since: DateTime<Utc>) -> Result<Vec<Contact>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_all.sql"))?;

    let rows = stmt.query_map(params![since.timestamp()], Contact::from_db)?;

    let mut names: Vec<Contact> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

/// Returns a single contact by `user_id`
pub fn by_user_id(user_id: UserId) -> Result<Contact, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_by_id.sql"))?;

    Ok(stmt.query_row(params![user_id], Contact::from_db)?)
}

/// Returns all contacts with the specified `status`
pub fn get_by_status(status: ContactStatus) -> Result<Vec<Contact>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("sql/get_by_status.sql"))?;

    let rows = stmt.query_map(
        params![status, chrono::MIN_DATE.and_hms(0, 0, 0).timestamp()],
        Contact::from_db,
    )?;

    let mut names: Vec<Contact> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
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
    /// The contact is local (ie it is you)
    Local = 0,
    /// The contact is remote
    Remote = 1,
}

impl rusqlite::types::FromSql for ContactType {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl rusqlite::ToSql for ContactType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
        use rusqlite::types::*;
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl std::convert::TryFrom<u8> for ContactType {
    type Error = HErr;

    fn try_from(n: u8) -> Result<Self, HErr> {
        use ContactType::*;
        match n {
            0 => Ok(Local),
            1 => Ok(Remote),
            unknown => Err(HErr::HeraldError(format!(
                "Unknown contact status {}",
                unknown
            ))),
        }
    }
}

impl std::convert::TryFrom<i64> for ContactType {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n.try_into(),
            Err(_) => Err(HErr::HeraldError(format!("Unknown contact status {}", n))),
        }
    }
}

impl rusqlite::types::FromSql for ContactStatus {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl rusqlite::ToSql for ContactStatus {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
        use rusqlite::types::*;
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
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

/// Builder for `Contact`
pub struct ContactBuilder {
    /// Contact id
    id: UserId,
    /// Contact name
    name: Option<String>,
    /// Path of profile picture
    profile_picture: Option<String>,
    /// User set color for user
    color: Option<u32>,
    /// Indicates whether user is archived
    status: Option<ContactStatus>,
    /// Pairwise conversation corresponding to contact
    pairwise_conversation: Option<ConversationId>,
    /// Indicates that the contact is the local contact
    contact_type: Option<ContactType>,
}

impl ContactBuilder {
    /// Creates new `ContactBuilder`
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            profile_picture: None,
            name: None,
            color: None,
            status: None,
            pairwise_conversation: None,
            contact_type: None,
        }
    }

    /// Sets the name of the contact being built.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the profile picture of the contact being built.
    pub fn profile_picture(mut self, profile_picture: String) -> Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    /// Sets the color of the contact being built.
    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the status of the contact being built.
    pub fn status(mut self, status: ContactStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the pairwise conversation id of the contact being built.
    pub fn pairwise_conversation(mut self, pairwise_conversation: ConversationId) -> Self {
        self.pairwise_conversation = Some(pairwise_conversation);
        self
    }

    pub(crate) fn local(mut self) -> Self {
        self.contact_type = Some(ContactType::Local);
        self
    }

    /// Adds contact to database
    pub fn add(self) -> Result<Contact, HErr> {
        let mut db = Database::get()?;

        let tx = db.transaction()?;
        let contact = Self::add_with_tx(self, &tx);
        tx.commit()?;
        contact
    }

    pub(crate) fn add_with_tx(self, tx: &rusqlite::Transaction) -> Result<Contact, HErr> {
        let color = self
            .color
            .unwrap_or_else(|| crate::utils::id_to_color(self.id.as_str()));

        let name = self.name.as_ref().map(|s| s.as_str());

        let contact_type = self.contact_type.unwrap_or(ContactType::Remote);

        let title = if let ContactType::Local = contact_type {
            Some(crate::config::NTS_CONVERSATION_NAME)
        } else {
            match name {
                Some(name) => Some(name),
                None => Some(self.id.as_str()),
            }
        };

        let pairwise_conversation = match self.pairwise_conversation {
            Some(conv_id) => {
                crate::conversation::add_pairwise_conversation(tx, Some(&conv_id), title)?
            }
            None => crate::conversation::add_pairwise_conversation(tx, None, title)?,
        };

        let contact = Contact {
            id: self.id,
            name: self.name,
            profile_picture: self.profile_picture,
            color,
            status: self.status.unwrap_or(ContactStatus::Active),
            pairwise_conversation,
            contact_type,
            added: chrono::Utc::now(),
        };

        tx.execute(
            include_str!("sql/add.sql"),
            params![
                contact.id,
                contact.name,
                contact.profile_picture,
                contact.color,
                contact.status,
                contact.pairwise_conversation,
                contact.contact_type
            ],
        )?;
        tx.execute(
            include_str!("../members/sql/add_member.sql"),
            params![contact.pairwise_conversation, contact.id],
        )?;

        Ok(contact)
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
    /// Contact type, local or remote
    pub contact_type: ContactType,
    /// when was the contact added?
    pub added: DateTime<Utc>,
}

impl Contact {
    /// Returns name
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Returns path to profile picture
    pub fn profile_picture(&self) -> Option<&str> {
        self.profile_picture.as_ref().map(|s| s.as_ref())
    }

    /// Returns contact's color
    pub fn color(&self) -> u32 {
        self.color
    }

    /// Returns contact's status
    pub fn status(&self) -> ContactStatus {
        self.status
    }

    /// Matches contact's text fields against a [`SearchPattern`]
    pub fn matches(&self, pattern: &crate::utils::SearchPattern) -> bool {
        pattern.is_match(self.id.as_str())
            || match self.name.as_ref() {
                Some(name) => pattern.is_match(name),
                None => false,
            }
    }

    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Contact {
            id: row.get(0)?,
            name: row.get(1)?,
            profile_picture: row.get(2)?,
            color: row.get(3)?,
            status: row.get(4)?,
            pairwise_conversation: row.get(5)?,
            contact_type: row.get(6)?,
            added: Utc
                .timestamp_opt(row.get(7)?, 0)
                .single()
                .unwrap_or_else(Utc::now),
        })
    }
}

#[cfg(test)]
mod tests;
