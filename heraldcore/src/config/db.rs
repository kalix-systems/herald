use super::*;
use crate::conversation::ExpirationPeriod;
use coremacros::w;
use rusqlite::named_params;
use std::net::SocketAddr;

/// Get's the "Note to Self" conversation
pub(crate) fn nts_conversation(conn: &rusqlite::Connection) -> Result<ConversationId, HErr> {
    let id = id(conn)?;

    crate::conversation::db::pairwise_conversation(&conn, &id)
}

pub(crate) fn set_name(
    conn: &rusqlite::Connection,
    name: &str,
) -> Result<(), HErr> {
    let id = id(conn)?;

    crate::user::db::set_name(conn, id, name.into())?;

    Ok(())
}

pub(crate) fn set_profile_picture(
    conn: &rusqlite::Connection,
    profile_picture: Option<image_utils::ProfilePicture>,
) -> Result<Option<String>, HErr> {
    let id = id(conn)?;
    let path = crate::user::db::set_profile_picture(conn, id, profile_picture)?;

    Ok(path)
}

pub(crate) fn id_kp(conn: &rusqlite::Connection) -> Result<(UserId, sig::KeyPair), HErr> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/id_kp.sql")));

    let (id, kp) = w!(stmt.query_row(NO_PARAMS, |row| {
        Ok((
            w!(row.get::<_, UserId>("id")),
            w!(row.get::<_, Vec<u8>>("kp")),
        ))
    }));

    let kp = w!(kson::from_slice(&kp));

    Ok((id, kp))
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
    w!(conn.execute(include_str!("sql/update_colorscheme.sql"), &[colorscheme]));

    Ok(())
}

pub(crate) fn set_preferred_expiration(
    conn: &rusqlite::Connection,
    expiration: ExpirationPeriod,
) -> Result<(), rusqlite::Error> {
    w!(conn.execute_named(
        include_str!("sql/update_preferred_expiration.sql"),
        named_params!["@preferred_expiration": expiration]
    ));

    Ok(())
}

/// Gets the user's configuration
pub(crate) fn get(conn: &rusqlite::Connection) -> Result<Config, HErr> {
    let (
        id,
        name,
        profile_picture,
        color,
        colorscheme,
        nts_conversation,
        home_server,
        preferred_expiration,
    ) = w!(
        conn.query_row(include_str!("sql/get_config.sql"), NO_PARAMS, |row| {
            Ok((
                row.get("id")?,
                row.get("name")?,
                row.get("profile_picture")?,
                row.get("color")?,
                row.get("colorscheme")?,
                row.get("pairwise_conversation")?,
                row.get::<_, String>("home_server")?,
                row.get::<_, ExpirationPeriod>("preferred_expiration")?,
            ))
        })
    );

    Ok(Config {
        id,
        name,
        profile_picture,
        color,
        colorscheme,
        nts_conversation,
        home_server: home_server.parse()?,
        preferred_expiration,
    })
}

/// Gets user id
pub(crate) fn id(conn: &rusqlite::Connection) -> Result<UserId, HErr> {
    Ok(w!(conn.query_row(
        include_str!("sql/get_id.sql"),
        NO_PARAMS,
        |row| row.get(0)
    )))
}

/// Gets the preferred expiration period
pub(crate) fn preferred_expiration(
    conn: &rusqlite::Connection
) -> Result<ExpirationPeriod, rusqlite::Error> {
    Ok(w!(conn.query_row(
        include_str!("sql/preferred_expiration.sql"),
        NO_PARAMS,
        |row| row.get("preferred_expiration")
    )))
}

/// Gets the current user's keypair
pub(crate) fn keypair(conn: &rusqlite::Connection) -> Result<sig::KeyPair, HErr> {
    todo!()
    //Ok(w!(conn.query_row(
    //    include_str!("sql/get_keypair.sql"),
    //    NO_PARAMS,
    //    |row| { row.get(0) }
    //)))
}

/// Gets the current user's GlobalId
pub(crate) fn gid(conn: &rusqlite::Connection) -> Result<GlobalId, HErr> {
    todo!()
    //let uid = id(conn)?;
    //let did = *keypair(conn)?.public_key();
    //Ok(GlobalId { uid, did })
}

/// Gets the server address where the current user is registered
pub(crate) fn home_server(conn: &rusqlite::Connection) -> Result<SocketAddr, HErr> {
    let server_addr_raw: String = w!(conn.query_row(
        include_str!("sql/home_server.sql"),
        NO_PARAMS,
        |row| { row.get("home_server") }
    ));

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
            preferred_expiration,
        } = self;

        let preferred_expiration = preferred_expiration.unwrap_or_default();

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

        todo!();
        //let tx = w!(conn.transaction());
        //w!(tx.execute_named(
        //    include_str!("sql/add_config.sql"),
        //    named_params! {
        //        "@id": id,
        //        "@kp": keypair,
        //        "@colorscheme": colorscheme,
        //        "@home_server": home_server.to_string(),
        //        "@preferred_expiration": preferred_expiration
        //    },
        //));

        //let (user, _conv) = w!(user_builder.add_with_tx(&tx));

        //w!(tx.commit());

        //let config = Config {
        //    id: user.id,
        //    name: user.name,
        //    profile_picture: user.profile_picture,
        //    color,
        //    colorscheme,
        //    nts_conversation: user.pairwise_conversation,
        //    home_server,
        //    preferred_expiration,
        //};

        //Ok(config)
    }
}

#[cfg(test)]
pub(crate) fn test_config(conn: &mut rusqlite::Connection) -> crate::config::Config {
    use std::convert::TryInto;
    let uid = "111NOCONFLICT111".try_into().expect("Bad user id");
    crate::config::ConfigBuilder::new::<String>(uid, sig::KeyPair::gen_new(), ("".into(), 0))
        .add_db(conn)
        .expect("Failed to create config")
}
