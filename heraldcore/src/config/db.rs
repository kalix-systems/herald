use super::*;

impl Config {
    pub(crate) fn set_name_db(
        &mut self,
        conn: &rusqlite::Connection,
        name: String,
    ) -> Result<(), HErr> {
        crate::user::db::set_name(conn, self.id, name.as_str())?;

        self.name = name;
        Ok(())
    }

    pub(crate) fn set_profile_picture_db(
        &mut self,
        conn: &rusqlite::Connection,
        profile_picture: Option<String>,
    ) -> Result<(), HErr> {
        let path = crate::user::db::set_profile_picture(
            conn,
            self.id,
            profile_picture,
            self.profile_picture.as_ref().map(|s| s.as_str()),
        )?;

        self.profile_picture = path;

        Ok(())
    }

    /// Update user's color
    pub(crate) fn set_color_db(
        &mut self,
        conn: &rusqlite::Connection,
        color: u32,
    ) -> Result<(), HErr> {
        crate::user::db::set_color(conn, self.id, color)?;
        self.color = color;

        Ok(())
    }

    pub(crate) fn set_colorscheme_db(
        &mut self,
        conn: &rusqlite::Connection,
        colorscheme: u32,
    ) -> Result<(), HErr> {
        conn.execute(include_str!("sql/update_colorscheme.sql"), &[colorscheme])?;

        self.colorscheme = colorscheme;

        Ok(())
    }
}

impl ConfigBuilder {
    /// Adds configuration.
    pub(crate) fn add_db(self, conn: &mut rusqlite::Connection) -> Result<Config, HErr> {
        let ConfigBuilder {
            id,
            keypair,
            color,
            colorscheme,
            name,
            nts_conversation,
        } = self;

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id.as_str()));
        let colorscheme = colorscheme.unwrap_or(0);

        let mut user_builder = crate::user::UserBuilder::new(id).local();

        if let Some(name) = name {
            user_builder = user_builder.name(name);
        }

        if let Some(pairwise_conversation) = nts_conversation {
            user_builder = user_builder.pairwise_conversation(pairwise_conversation);
        }

        user_builder = user_builder.color(color);

        let tx = conn.transaction()?;
        tx.execute(
            include_str!("sql/add_config.sql"),
            params![id, keypair, colorscheme],
        )?;

        let (user, _conv) = user_builder.add_with_tx(&tx)?;
        tx.commit()?;

        let config = Config {
            id: user.id,
            name: user.name,
            keypair,
            profile_picture: user.profile_picture,
            color,
            colorscheme,
            nts_conversation: user.pairwise_conversation,
        };

        Ok(config)
    }
}

/// Gets the user's configuration
pub(crate) fn get(conn: &rusqlite::Connection) -> Result<Config, HErr> {
    let (id, name, profile_picture, color, colorscheme, nts_conversation, keypair) = conn
        .query_row(include_str!("sql/get_config.sql"), NO_PARAMS, |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?;

    Ok(Config {
        id,
        name,
        profile_picture,
        color,
        colorscheme,
        nts_conversation,
        keypair,
    })
}

/// Gets user id directly from database.
pub(crate) fn static_id(conn: &rusqlite::Connection) -> Result<UserId, HErr> {
    Ok(conn.query_row(include_str!("sql/get_id.sql"), NO_PARAMS, |row| row.get(0))?)
}

/// Gets the current user's kepair directly from the database.
pub(crate) fn static_keypair(conn: &rusqlite::Connection) -> Result<sig::KeyPair, HErr> {
    Ok(
        conn.query_row(include_str!("sql/get_keypair.sql"), NO_PARAMS, |row| {
            row.get(0)
        })?,
    )
}

/// Gets the current user's GlobalId
pub(crate) fn static_gid(conn: &rusqlite::Connection) -> Result<GlobalId, HErr> {
    let uid = static_id(conn)?;
    let did = *static_keypair(conn)?.public_key();
    Ok(GlobalId { uid, did })
}

#[cfg(test)]
pub(crate) fn test_config(conn: &mut rusqlite::Connection) -> crate::config::Config {
    use std::convert::TryInto;
    let uid = "userid".try_into().expect("Bad user id");
    crate::config::ConfigBuilder::new(uid, sig::KeyPair::gen_new())
        .add_db(conn)
        .expect("Failed to create config")
}
