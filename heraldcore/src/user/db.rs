use super::*;
use crate::conversation::Conversation;
use rusqlite::named_params;

// TODO: this should be a struct
type PathStr<'a> = &'a str;

/// Gets a user's name by their `id`.
pub(crate) fn name(conn: &rusqlite::Connection, id: UserId) -> Result<Option<String>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_name.sql"))?;

    Ok(stmt.query_row(params![id], |row| row.get(0))?)
}

/// Change name of user by their `id`
pub(crate) fn set_name(conn: &rusqlite::Connection, id: UserId, name: &str) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/update_name.sql"))?;

    stmt.execute(params![name, id])?;
    Ok(())
}

/// Gets a user's profile picture by their `id`.
pub fn profile_picture(conn: &rusqlite::Connection, id: UserId) -> Result<Option<String>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_profile_picture.sql"))?;

    Ok(stmt.query_row(params![id], |row| row.get(0))?)
}

/// Returns all members of a conversation.
pub fn conversation_members(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<User>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_by_conversation.sql"))?;

    let rows = stmt.query_map(params![conversation_id], User::from_db)?;

    let mut users: Vec<User> = Vec::new();
    for user in rows {
        users.push(user?);
    }

    Ok(users)
}

/// Updates a user's profile picture.
pub fn set_profile_picture(
    conn: &rusqlite::Connection,
    id: UserId,
    profile_picture: Option<String>,
    old_path: Option<PathStr>,
) -> Result<Option<String>, HErr> {
    let profile_picture = match profile_picture {
        Some(path) => {
            let path_string = image_utils::update_picture(path, old_path.map(|p| p.into()))?
                .into_os_string()
                .into_string()?;
            Some(path_string)
        }
        None => {
            if let Some(old) = old_path {
                std::fs::remove_file(old).ok();
            }
            None
        }
    };

    let mut stmt = conn.prepare(include_str!("sql/set_conversation_picture.sql"))?;
    stmt.execute_named(named_params! {"@picture": profile_picture, "@user_id": id})?;

    conn.execute(
        include_str!("sql/update_profile_picture.sql"),
        params![profile_picture, id],
    )?;
    Ok(profile_picture)
}

/// Sets a user's color
pub fn set_color(conn: &rusqlite::Connection, id: UserId, color: u32) -> Result<(), HErr> {
    conn.execute(include_str!("sql/update_color.sql"), params![color, id])?;
    Ok(())
}

/// Indicates whether user exists
pub fn user_exists(conn: &rusqlite::Connection, id: UserId) -> Result<bool, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/user_exists.sql"))?;
    Ok(stmt.exists(&[id])?)
}

/// Sets user status
pub fn set_status(
    conn: &mut rusqlite::Connection,
    id: UserId,
    status: UserStatus,
) -> Result<(), HErr> {
    use UserStatus::*;
    match status {
        Deleted => {
            let tx = conn.transaction()?;
            tx.execute(include_str!("sql/delete_user_meta.sql"), params![id])?;
            tx.execute_named(
                include_str!("../message/sql/delete_pairwise_conversation.sql"),
                named_params! {"@user_id": id},
            )?;
            tx.commit()?;
        }
        _ => {
            conn.execute(include_str!("sql/set_status.sql"), params![status, id])?;
        }
    }
    Ok(())
}

/// Gets user status
pub fn status(conn: &rusqlite::Connection, id: UserId) -> Result<UserStatus, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_status.sql"))?;

    Ok(stmt.query_row(&[id], |row| row.get(0))?)
}

/// Returns all users
pub fn all(conn: &rusqlite::Connection) -> Result<Vec<User>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_all.sql"))?;

    let rows = stmt.query_map(NO_PARAMS, User::from_db)?;

    let mut names: Vec<User> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

/// Returns a single user by `user_id`
pub fn by_user_id(conn: &rusqlite::Connection, user_id: UserId) -> Result<User, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_by_id.sql"))?;

    Ok(stmt.query_row(params![user_id], User::from_db)?)
}

/// Returns all users with the specified `status`
pub fn get_by_status(conn: &rusqlite::Connection, status: UserStatus) -> Result<Vec<User>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_by_status.sql"))?;

    let rows = stmt.query_map(params![status], User::from_db)?;

    let mut names: Vec<User> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

impl UserBuilder {
    pub(crate) fn add_db(
        self,
        conn: &mut rusqlite::Connection,
    ) -> Result<(User, Conversation), HErr> {
        let tx = conn.transaction()?;
        let (user, conv) = Self::add_with_tx(self, &tx)?;
        tx.commit()?;
        Ok((user, conv))
    }

    pub(crate) fn add_with_tx(
        self,
        tx: &rusqlite::Transaction,
    ) -> Result<(User, Conversation), HErr> {
        use crate::conversation::ConversationBuilder;

        let Self {
            id,
            color,
            name,
            user_type,
            status,
            pairwise_conversation,
        } = self;

        let mut conv_builder = ConversationBuilder::new();
        conv_builder.pairwise(true);

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id.as_str()));
        conv_builder.color(color);

        let name = name.unwrap_or_else(|| id.to_string());

        let user_type = user_type.unwrap_or(UserType::Remote);

        let title = if UserType::Local == user_type {
            crate::config::NTS_CONVERSATION_NAME
        } else {
            name.as_str()
        };

        conv_builder.title(title.to_owned());
        if let Some(cid) = pairwise_conversation {
            conv_builder.conversation_id(cid);
        }

        let conv = if user_type == UserType::Local {
            conv_builder.add_nts(&tx, self.id)?
        } else {
            conv_builder.add_tx(&tx)?
        };

        let pairwise_conversation = conv.meta.conversation_id;

        let user = User {
            id,
            name,
            profile_picture: None,
            color,
            status: status.unwrap_or(UserStatus::Active),
            pairwise_conversation,
            user_type,
        };

        tx.execute(
            include_str!("sql/add.sql"),
            params![
                user.id,
                user.name,
                user.profile_picture,
                user.color,
                user.status,
                user.pairwise_conversation,
                user.user_type
            ],
        )?;
        tx.execute(
            include_str!("../members/sql/add_member.sql"),
            params![user.pairwise_conversation, user.id],
        )?;

        Ok((user, conv))
    }
}

#[cfg(test)]
pub(crate) fn test_user(conn: &mut rusqlite::Connection, user_id: &str) -> User {
    let receiver = user_id
        .try_into()
        .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!()));
    UserBuilder::new(receiver)
        .add_db(conn)
        .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!()))
        .0
}
