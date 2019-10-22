use super::*;

/// Gets a contact's name by their `id`.
pub(crate) fn name(conn: &rusqlite::Connection, id: UserId) -> Result<Option<String>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_name.sql"))?;

    Ok(stmt.query_row(params![id], |row| row.get(0))?)
}

/// Change name of contact by their `id`
pub(crate) fn set_name(conn: &rusqlite::Connection, id: UserId, name: &str) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/update_name.sql"))?;

    stmt.execute(params![name, id])?;
    Ok(())
}

/// Gets a contact's profile picture by their `id`.
pub fn profile_picture(conn: &rusqlite::Connection, id: UserId) -> Result<Option<String>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_profile_picture.sql"))?;

    Ok(stmt.query_row(params![id], |row| row.get(0))?)
}

/// Returns all members of a conversation.
pub fn conversation_members(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<Contact>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_by_conversation.sql"))?;

    let rows = stmt.query_map(params![conversation_id], Contact::from_db)?;

    let mut contacts: Vec<Contact> = Vec::new();
    for contact in rows {
        contacts.push(contact?);
    }

    Ok(contacts)
}

/// Updates a contact's profile picture.
pub fn set_profile_picture(
    conn: &rusqlite::Connection,
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

    conn.execute(
        include_str!("sql/update_profile_picture.sql"),
        params![profile_picture, id],
    )?;
    Ok(profile_picture)
}

/// Sets a contact's color
pub fn set_color(conn: &rusqlite::Connection, id: UserId, color: u32) -> Result<(), HErr> {
    conn.execute(include_str!("sql/update_color.sql"), params![color, id])?;
    Ok(())
}

/// Indicates whether contact exists
pub fn contact_exists(conn: &rusqlite::Connection, id: UserId) -> Result<bool, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/contact_exists.sql"))?;
    Ok(stmt.exists(&[id])?)
}

/// Sets contact status
pub fn set_status(
    conn: &mut rusqlite::Connection,
    id: UserId,
    status: ContactStatus,
) -> Result<(), HErr> {
    use ContactStatus::*;
    match status {
        Deleted => {
            let tx = conn.transaction()?;
            tx.execute(include_str!("sql/delete_contact_meta.sql"), params![id])?;
            tx.execute(
                include_str!("../message/sql/delete_pairwise_conversation.sql"),
                params![id],
            )?;
            tx.commit()?;
        }
        _ => {
            conn.execute(include_str!("sql/set_status.sql"), params![status, id])?;
        }
    }
    Ok(())
}

/// Gets contact status
pub fn status(conn: &rusqlite::Connection, id: UserId) -> Result<ContactStatus, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_status.sql"))?;

    Ok(stmt.query_row(&[id], |row| row.get(0))?)
}

/// Returns all contacts
pub fn all(conn: &rusqlite::Connection) -> Result<Vec<Contact>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_all.sql"))?;

    let rows = stmt.query_map(NO_PARAMS, Contact::from_db)?;

    let mut names: Vec<Contact> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

/// Returns a single contact by `user_id`
pub fn by_user_id(conn: &rusqlite::Connection, user_id: UserId) -> Result<Contact, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_by_id.sql"))?;

    Ok(stmt.query_row(params![user_id], Contact::from_db)?)
}

/// Returns all contacts with the specified `status`
pub fn get_by_status(
    conn: &rusqlite::Connection,
    status: ContactStatus,
) -> Result<Vec<Contact>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_by_status.sql"))?;

    let rows = stmt.query_map(params![status], Contact::from_db)?;

    let mut names: Vec<Contact> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

impl ContactBuilder {
    pub(crate) fn add_db(self, conn: &mut rusqlite::Connection) -> Result<Contact, HErr> {
        let tx = conn.transaction()?;
        let contact = Self::add_with_tx(self, &tx);
        tx.commit()?;
        contact
    }

    pub(crate) fn add_with_tx(self, tx: &rusqlite::Transaction) -> Result<Contact, HErr> {
        use crate::conversation::ConversationBuilder;

        let mut conv_builder = ConversationBuilder::new();
        conv_builder.pairwise(true);

        let color = self
            .color
            .unwrap_or_else(|| crate::utils::id_to_color(self.id.as_str()));

        conv_builder.color(color);

        let name = self.name.clone().unwrap_or_else(|| self.id.to_string());

        let contact_type = self.contact_type.unwrap_or(ContactType::Remote);

        let title = if let ContactType::Local = contact_type {
            crate::config::NTS_CONVERSATION_NAME
        } else {
            name.as_str()
        };

        conv_builder.title(title.to_owned());

        if let Some(cid) = self.pairwise_conversation {
            conv_builder.conversation_id(cid);
        }

        let pairwise_conversation = conv_builder.add_with_tx(&tx)?;

        let contact = Contact {
            id: self.id,
            name,
            profile_picture: self.profile_picture,
            color,
            status: self.status.unwrap_or(ContactStatus::Active),
            pairwise_conversation,
            contact_type,
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
