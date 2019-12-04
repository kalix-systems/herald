use super::*;
use rusqlite::named_params;
use std::net::SocketAddr;

pub(crate) fn set_name(
    conn: &rusqlite::Connection,
    name: String,
) -> Result<(), HErr> {
    let id = id(conn)?;
    Ok(crate::user::db::set_name(conn, id, name.as_str())?)
}

pub(crate) fn set_profile_picture(
    conn: &rusqlite::Connection,
    profile_picture: Option<String>,
) -> Result<Option<String>, HErr> {
    let id = id(conn)?;
    Ok(crate::user::db::set_profile_picture(
        conn,
        id,
        profile_picture,
    )?)
}

/// Update user's color
pub(crate) fn set_color(
    conn: &rusqlite::Connection,
    color: u32,
) -> Result<(), HErr> {
    let id = id(conn)?;
    crate::user::db::set_color(conn, id, color)?;

    Ok(())
}

pub(crate) fn set_colorscheme(
    conn: &rusqlite::Connection,
    colorscheme: u32,
) -> Result<(), HErr> {
    conn.execute(include_str!("sql/update_colorscheme.sql"), &[colorscheme])?;

    Ok(())
}

/// Gets the user's configuration
pub(crate) fn get(conn: &rusqlite::Connection) -> Result<Config, HErr> {
    let (id, name, profile_picture, color, colorscheme, nts_conversation, home_server) = conn
        .query_row(include_str!("sql/get_config.sql"), NO_PARAMS, |row| {
            Ok((
                row.get("id")?,
                row.get("name")?,
                row.get("profile_picture")?,
                row.get("color")?,
                row.get("colorscheme")?,
                row.get("pairwise_conversation")?,
                row.get::<_, String>("home_server")?,
            ))
        })?;

    Ok(Config {
        id,
        name,
        profile_picture,
        color,
        colorscheme,
        nts_conversation,
        home_server: home_server.parse()?,
    })
}

/// Gets user id
pub(crate) fn id(conn: &rusqlite::Connection) -> Result<UserId, HErr> {
    Ok(conn.query_row(include_str!("sql/get_id.sql"), NO_PARAMS, |row| row.get(0))?)
}

/// Gets the current user's keypair
pub(crate) fn keypair(conn: &rusqlite::Connection) -> Result<sig::KeyPair, HErr> {
    Ok(
        conn.query_row(include_str!("sql/get_keypair.sql"), NO_PARAMS, |row| {
            row.get(0)
        })?,
    )
}

/// Gets the current user's GlobalId
pub(crate) fn gid(conn: &rusqlite::Connection) -> Result<GlobalId, HErr> {
    let uid = id(conn)?;
    let did = *keypair(conn)?.public_key();
    Ok(GlobalId { uid, did })
}

/// Gets the server address where the current user is registered
pub(crate) fn home_server(conn: &rusqlite::Connection) -> Result<SocketAddr, HErr> {
    let server_addr_raw: String =
        conn.query_row(include_str!("sql/home_server.sql"), NO_PARAMS, |row| {
            row.get("home_server")
        })?;

    Ok(server_addr_raw.parse()?)
}

impl ConfigBuilder {
    /// Adds configuration.
    pub(crate) fn add_db(
        self,
        conn: &mut rusqlite::Connection,
    ) -> Result<Config, HErr> {
        let ConfigBuilder {
            id,
            keypair,
            color,
            colorscheme,
            name,
            nts_conversation,
            home_server,
        } = self;

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id.as_str()));
        let colorscheme = colorscheme.unwrap_or(0);

        let home_server = home_server.unwrap_or_else(|| *crate::network::default_server());

        let mut user_builder = crate::user::UserBuilder::new(id).local();

        if let Some(name) = name {
            user_builder = user_builder.name(name);
        }

        if let Some(pairwise_conversation) = nts_conversation {
            user_builder = user_builder.pairwise_conversation(pairwise_conversation);
        }

        user_builder = user_builder.color(color);

        let tx = conn.transaction()?;
        tx.execute_named(
            include_str!("sql/add_config.sql"),
            named_params!["@id": id, "@kp": keypair, "@colorscheme": colorscheme, "@home_server": home_server.to_string() ],
        )?;

        let (user, _conv) = user_builder.add_with_tx(&tx)?;
        tx.commit()?;

        let config = Config {
            id: user.id,
            name: user.name,
            profile_picture: user.profile_picture,
            color,
            colorscheme,
            nts_conversation: user.pairwise_conversation,
            home_server,
        };

        Ok(config)
    }
}

#[cfg(test)]
pub(crate) fn test_config(conn: &mut rusqlite::Connection) -> crate::config::Config {
    use std::convert::TryInto;
    let uid = "111NOCONFLICT111".try_into().expect("Bad user id");
    crate::config::ConfigBuilder::new(uid, sig::KeyPair::gen_new())
        .add_db(conn)
        .expect("Failed to create config")
}
