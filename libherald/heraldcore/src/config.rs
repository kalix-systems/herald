use crate::{
    db::{DBTable, Database},
    errors::*,
    types::*,
};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

/// Default name for the "Note to Self" conversation
pub static NTS_CONVERSATION_NAME: &str = "Note to Self";
/// User configuration
#[derive(Clone, Default)]
pub struct Config {
    /// ID of the local user
    pub id: UserId,
    /// Colorscheme
    pub colorscheme: u32,
    /// Name of the local user
    pub name: Option<String>,
    /// Profile picture of the local user
    pub profile_picture: Option<String>,
    /// Color of the local user
    pub color: u32,
    /// The *Note to Self* conversation id.
    pub nts_conversation: ConversationId,
}

/// Builder for `Config`
pub struct ConfigBuilder {
    /// ID of the local user
    id: UserId,
    /// Colorscheme
    colorscheme: Option<u32>,
    /// Name of the local user
    name: Option<String>,
    /// Profile picture of the local user
    profile_picture: Option<String>,
    /// Color of the local user
    color: Option<u32>,
    nts_conversation: Option<ConversationId>,
}

impl ConfigBuilder {
    /// Creates new `ConfigBuilder`
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            name: None,
            color: None,
            colorscheme: None,
            profile_picture: None,
            nts_conversation: None,
        }
    }

    /// Sets colorscheme, defaults to 0 if not set.
    pub fn colorscheme(mut self, colorscheme: u32) -> Self {
        self.colorscheme = Some(colorscheme);
        self
    }

    /// Sets color, computed from hash of the UserId if not set.
    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set profile picture.
    pub fn profile_picture(mut self, profile_picture: String) -> Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    /// Sets conversation id for "Note to Self" conversation, a new conversation is created
    /// if this is not set.
    pub fn nts_conversation(mut self, conv_id: ConversationId) -> Self {
        self.nts_conversation = Some(conv_id);
        self
    }

    /// Adds configuration.
    pub fn add(self) -> Result<Config, HErr> {
        let Self {
            color,
            colorscheme,
            id,
            name,
            nts_conversation,
            profile_picture,
        } = self;

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(id.as_str()));
        let colorscheme = colorscheme.unwrap_or(0);

        let mut contact_builder = crate::contact::ContactBuilder::new(id.clone()).local();

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

        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(
            include_str!("sql/config/add_config.sql"),
            params![id, colorscheme],
        )?;

        let contact = contact_builder.add_with_tx(&tx)?;
        tx.commit()?;

        let config = Config {
            id: contact.id,
            name: contact.name,
            profile_picture: contact.profile_picture,
            color,
            colorscheme,
            nts_conversation: contact.pairwise_conversation,
        };

        Ok(config)
    }
}

impl DBTable for Config {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/config/create_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/config/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/config/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/config/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/config/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

impl Config {
    /// Gets the user's configuration
    pub fn get() -> Result<Config, HErr> {
        let db = Database::get()?;

        let (id, name, profile_picture, color, colorscheme, nts_conversation) = db.query_row(
            include_str!("sql/config/get_config.sql"),
            NO_PARAMS,
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )?;

        Ok(Config {
            id,
            name,
            profile_picture,
            color,
            colorscheme,
            nts_conversation,
        })
    }

    /// Gets user id
    pub fn id(&self) -> UserIdRef {
        self.id.as_str()
    }

    /// Gets user id directly from database.
    pub fn static_id() -> Result<UserId, HErr> {
        let db = Database::get()?;
        Ok(
            db.query_row(include_str!("sql/config/get_id.sql"), NO_PARAMS, |row| {
                row.get(0)
            })?,
        )
    }

    /// Updates user's display name
    pub fn set_name(&mut self, name: Option<String>) -> Result<(), HErr> {
        crate::contact::set_name(&self.id, name.as_ref().map(|s| s.as_str()))?;

        self.name = name;
        Ok(())
    }

    /// Updates user's profile picture
    pub fn set_profile_picture(&mut self, profile_picture: Option<String>) -> Result<(), HErr> {
        let path = crate::contact::set_profile_picture(
            &self.id,
            profile_picture,
            self.profile_picture.as_ref().map(|s| s.as_str()),
        )?;

        self.profile_picture = path;

        Ok(())
    }

    /// Update user's color
    pub fn set_color(&mut self, color: u32) -> Result<(), HErr> {
        crate::contact::set_color(&self.id, color)?;
        self.color = color;

        Ok(())
    }

    /// Update user's colorscheme
    pub fn set_colorscheme(&mut self, colorscheme: u32) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/config/update_colorscheme.sql"),
            &[colorscheme],
        )?;

        self.colorscheme = colorscheme;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::womp;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_drop_exists() {
        // drop twice, it shouldn't panic on multiple drops
        Config::drop_table().expect(womp!());
        Config::drop_table().expect(womp!());

        Config::create_table().expect(womp!());
        assert!(Config::exists().expect(womp!()));
        Config::create_table().expect(womp!());
        assert!(Config::exists().expect(womp!()));
        Config::drop_table().expect(womp!());
        assert!(!Config::exists().expect(womp!()));
        Config::reset().expect(womp!());
    }

    #[test]
    #[serial]
    fn add_get_set_config() {
        Database::reset_all().expect(womp!());

        let id = "HelloWorld";

        ConfigBuilder::new(id.into()).add().expect(womp!());

        let config = Config::get().expect(womp!());
        assert_eq!(config.id(), id);
        assert_eq!(config.colorscheme, 0);
        assert_eq!(config.color, crate::utils::id_to_color(id));
        assert_eq!(config.color, crate::utils::id_to_color(id));
        assert!(config.name.is_none());
        assert!(config.profile_picture.is_none());

        Database::reset_all().expect(womp!());

        let name = "stuff";
        let profile_picture = "stuff";
        let nts_id = [0u8; 32].into();
        let config = ConfigBuilder::new(id.into())
            .name(name.into())
            .colorscheme(1)
            .color(2)
            .profile_picture(profile_picture.into())
            .nts_conversation(nts_id)
            .add()
            .expect(womp!());

        let meta = crate::conversation::meta(&config.nts_conversation).expect(womp!());

        assert_eq!(meta.title.expect(womp!()), NTS_CONVERSATION_NAME);

        let db_config = Config::get().expect(womp!());

        assert_eq!(config.nts_conversation, db_config.nts_conversation);
        assert_eq!(db_config.id(), "HelloWorld");
        assert_eq!(db_config.name.as_ref().expect(womp!()), name);
        assert_eq!(db_config.nts_conversation, nts_id);
        assert_eq!(db_config.colorscheme, 1);
        assert_eq!(db_config.color, 2);
        assert_eq!(
            db_config.profile_picture.as_ref().expect(womp!()),
            profile_picture
        );

        let mut db_config = Config::get().expect(womp!());
        db_config.set_name(None).expect(womp!());
        assert_eq!(db_config.name, None);

        db_config.set_name(Some("hello".into())).expect(womp!());

        let mut db_config = Config::get().expect(womp!());
        assert_eq!(db_config.name, Some("hello".into()));

        db_config.set_colorscheme(0).expect(womp!());
        db_config.set_color(0).expect(womp!());
        assert_eq!(db_config.color, 0);
        assert_eq!(db_config.colorscheme, 0);
    }

    #[test]
    #[serial]
    fn two_configs() {
        Database::reset_all().expect(womp!());
        ConfigBuilder::new("1".into()).add().expect(womp!());
        assert!(ConfigBuilder::new("2".into()).add().is_err());
    }

    #[test]
    #[serial]
    fn invalid_id() {
        Database::reset_all().expect(womp!());
        assert!(ConfigBuilder::new(format!("{:?}", vec![0; 256]))
            .add()
            .is_err());
    }

    #[test]
    #[serial]
    fn get_id() {
        Database::reset_all().expect(womp!());

        let id = "HelloWorld";
        let config = ConfigBuilder::new(id.into()).add().expect(womp!());

        let static_id = Config::static_id().expect(womp!());
        assert_eq!(config.id, id);
        assert_eq!(config.id, static_id);
    }
}
