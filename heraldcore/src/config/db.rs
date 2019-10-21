use super::*;
use std::ops::{Deref, DerefMut};

impl ConfigBuilder {
    /// Adds configuration.
    pub(crate) fn add_db<D>(self, mut conn: D) -> Result<Config, HErr>
    where
        D: Deref<Target = Database> + DerefMut,
    {
        let ConfigBuilder {
            id,
            keypair,
            color,
            colorscheme,
            name,
            nts_conversation,
            profile_picture,
        } = self;

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id.as_str()));
        let colorscheme = colorscheme.unwrap_or(0);

        let mut contact_builder = crate::contact::ContactBuilder::new(id).local();

        if let Some(name) = name {
            contact_builder = contact_builder.name(name);
        }

        if let Some(pairwise_conversation) = nts_conversation {
            contact_builder = contact_builder.pairwise_conversation(pairwise_conversation);
        }

        if let Some(picture) = profile_picture {
            contact_builder = contact_builder.profile_picture(picture);
        }

        contact_builder = contact_builder.color(color);

        let tx = conn.transaction()?;
        tx.execute(
            include_str!("sql/add_config.sql"),
            params![id, keypair, colorscheme],
        )?;

        let contact = contact_builder.add_with_tx(&tx)?;
        tx.commit()?;

        let config = Config {
            id: contact.id,
            name: contact.name,
            keypair,
            profile_picture: contact.profile_picture,
            color,
            colorscheme,
            nts_conversation: contact.pairwise_conversation,
        };

        Ok(config)
    }
}

/// Gets the user's configuration
pub(crate) fn get<D>(conn: D) -> Result<Config, HErr>
where
    D: Deref<Target = Database> + DerefMut,
{
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
pub(crate) fn static_id<D>(conn: &D) -> Result<UserId, HErr>
where
    D: Deref<Target = Database> + DerefMut,
{
    Ok(conn.query_row(include_str!("sql/get_id.sql"), NO_PARAMS, |row| row.get(0))?)
}

/// Gets the current user's kepair directly from the database.
pub(crate) fn static_keypair<D>(conn: &D) -> Result<sig::KeyPair, HErr>
where
    D: Deref<Target = Database> + DerefMut,
{
    Ok(
        conn.query_row(include_str!("sql/get_keypair.sql"), NO_PARAMS, |row| {
            row.get(0)
        })?,
    )
}

/// Gets the current user's GlobalId
pub(crate) fn static_gid<D>(conn: &D) -> Result<GlobalId, HErr>
where
    D: Deref<Target = Database> + DerefMut,
{
    let uid = static_id(conn)?;
    let did = *static_keypair(conn)?.public_key();
    Ok(GlobalId { uid, did })
}

#[cfg(test)]
pub(crate) fn test_config<D>(conn: D) -> crate::config::Config
where
    D: Deref<Target = Database> + DerefMut,
{
    use std::convert::TryInto;
    let uid = "userid".try_into().expect("Bad user id");
    crate::config::ConfigBuilder::new(uid, sig::KeyPair::gen_new())
        .add_db(conn)
        .expect("Failed to create config")
}
