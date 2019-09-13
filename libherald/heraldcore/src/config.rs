use crate::{
    db::{DBTable, Database},
    errors::*,
    image_utils,
};
use herald_common::{UserId, UserIdRef};
use rusqlite::{params, NO_PARAMS};

// static LOCAL_CONVERSATION_NAME: &str = "Note to Self";

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
}

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
}

impl ConfigBuilder {
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            name: None,
            color: None,
            colorscheme: None,
            profile_picture: None,
        }
    }

    pub fn with_colorscheme(mut self, colorscheme: u32) -> Self {
        self.colorscheme = Some(colorscheme);
        self
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_profile_picture(mut self, profile_picture: String) -> Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    pub fn build(self) -> Result<Config, HErr> {
        let color = self
            .color
            .unwrap_or_else(|| crate::utils::id_to_color(self.id.as_str()));
        let colorscheme = self.colorscheme.unwrap_or(0);

        let config = Config {
            id: self.id,
            name: self.name,
            profile_picture: self.profile_picture,
            color,
            colorscheme,
        };

        {
            let db = Database::get()?;
            db.execute(
                include_str!("sql/config/add_config.sql"),
                params![config.id(), colorscheme],
            )?;
        }
        let mut builder = crate::contact::ContactBuilder::new(config.id.clone());

        if let Some(name) = &config.name {
            builder = builder.with_name(name.to_string());
        }

        if let Some(picture) = &config.profile_picture {
            builder = builder.with_profile_picture(picture.to_string());
        }

        builder.with_color(config.color).add()?;
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
        Ok(db.query_row(
            include_str!("sql/config/get_config.sql"),
            NO_PARAMS,
            |row| {
                Ok(Config {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    profile_picture: row.get(2)?,
                    color: row.get(3)?,
                    colorscheme: row.get(4)?,
                })
            },
        )?)
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
        let db = Database::get()?;
        db.execute(include_str!("sql/config/update_name.sql"), params![name])?;

        self.name = name;
        Ok(())
    }

    /// Updates user's profile picture
    pub fn set_profile_picture(&mut self, profile_picture: Option<String>) -> Result<(), HErr> {
        self.profile_picture = match profile_picture {
            Some(path) => Some(
                image_utils::save_profile_picture(
                    self.id.as_str(),
                    path,
                    self.profile_picture.clone(),
                )?
                .into_os_string()
                .into_string()?,
            ),
            None => {
                if let Some(old_pic) = &self.profile_picture {
                    std::fs::remove_file(old_pic)?;
                }
                None
            }
        };

        let path = self.profile_picture.as_ref().map(|s| s.as_str());

        let db = Database::get()?;
        db.execute(
            include_str!("sql/config/update_profile_picture.sql"),
            &[path],
        )?;

        Ok(())
    }

    /// Update user's color
    pub fn set_color(&mut self, color: u32) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/config/update_color.sql"), &[color])?;

        Ok(())
    }

    /// Update user's colorscheme
    pub fn set_colorscheme(&mut self, colorscheme: u32) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/config/update_colorscheme.sql"),
            &[colorscheme],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;
    use womp::*;

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
    }

    #[test]
    #[serial]
    fn add_and_get_config() {
        Database::reset_all().expect(womp!());

        let id = "HelloWorld";

        ConfigBuilder::new(id.into()).build().expect(womp!());
        assert_eq!(Config::get().expect(womp!()).id(), id);

        Database::reset_all().expect(womp!());

        let name = "stuff";
        let profile_picture = "stuff";
        ConfigBuilder::new(id.into())
            .with_name(name.into())
            .with_profile_picture(profile_picture.into())
            .build()
            .expect(womp!());
        assert_eq!(Config::get().expect(womp!()).id, "HelloWorld");
        assert_eq!(Config::get().expect(womp!()).name.expect(womp!()), name);
        assert_eq!(Config::get().expect(womp!()).colorscheme, 0);
        assert_eq!(
            Config::get()
                .expect(womp!())
                .profile_picture
                .expect(womp!()),
            profile_picture
        );
    }

    #[test]
    #[serial]
    fn get_id() {
        Database::reset_all().expect(womp!());

        let id = "HelloWorld";
        let config = ConfigBuilder::new(id.into()).build().expect(womp!());

        assert_eq!(config.id, id);
    }
}
