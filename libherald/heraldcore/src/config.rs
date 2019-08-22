use crate::{
    db::{DBTable, Database},
    errors::*,
    image_utils,
};
use rusqlite::{ToSql, NO_PARAMS};

/// User configuration
#[derive(Clone, Default)]
pub struct Config {
    /// ID of the current user
    pub id: Option<String>,
    /// Display name for the current user
    pub name: Option<String>,
    /// Path to profile picture for the current user
    pub profile_picture: Option<String>,
    /// Colorscheme
    pub colorscheme: u32,
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
                    colorscheme: row.get(3)?,
                })
            },
        )?)
    }

    /// Creates user configuration.
    pub fn new(
        id: String,
        name: Option<&str>,
        profile_picture: Option<&str>,
        colorscheme: Option<u32>,
    ) -> Result<Config, HErr> {
        let config = Config {
            id: Some(id.to_owned()),
            name: name.map(|n| n.to_owned()),
            profile_picture: profile_picture.map(|p| p.to_owned()),
            colorscheme: colorscheme.unwrap_or(1),
        };

        let id = id.to_sql()?;
        let name = name.to_sql()?;
        let profile_picture = profile_picture.to_sql()?;

        let db = Database::get()?;
        db.execute(
            include_str!("sql/config/add_config.sql"),
            &[id, name, profile_picture],
        )?;

        Ok(config)
    }

    /// Gets user id
    pub fn id(&self) -> Result<&str, HErr> {
        match &self.id {
            Some(id) => Ok(id),
            None => Err(HErr::HeraldError("User id has not been set".into())),
        }
    }

    /// Gets user id directly from database.
    pub fn static_id() -> Result<String, HErr> {
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
        db.execute(include_str!("sql/config/update_name.sql"), &[&self.name])?;

        self.name = name;

        Ok(())
    }

    /// Updates user's profile picture
    pub fn set_profile_picture(&mut self, profile_picture: Option<String>) -> Result<(), HErr> {
        let id = self.id()?;

        self.profile_picture = match profile_picture {
            Some(path) => {
                let path = image_utils::save_profile_picture(id, path)?
                    .into_os_string()
                    .into_string()?;

                Some(path)
            }
            None => {
                image_utils::delete_profile_picture(id)?;
                None
            }
        };

        let path = self.profile_picture.as_ref().map(|s| s.as_str());

        let db = Database::get()?;
        db.execute(include_str!("sql/config/update_name.sql"), &[path])?;

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

    #[test]
    #[serial]
    fn create_drop_exists() {
        // drop twice, it shouldn't panic on multiple drops
        Config::drop_table().unwrap();
        Config::drop_table().unwrap();

        Config::create_table().unwrap();
        assert!(Config::exists().unwrap());
        Config::create_table().unwrap();
        assert!(Config::exists().unwrap());
        Config::drop_table().unwrap();
        assert!(!Config::exists().unwrap());
    }

    #[test]
    #[serial]
    fn add_and_get_config() {
        Config::drop_table().unwrap();
        Config::create_table().unwrap();

        let id = "HelloWorld";

        Config::new(id.into(), None, None, None).unwrap();
        assert_eq!(Config::get().unwrap().id().unwrap(), "HelloWorld");

        Config::drop_table().unwrap();
        Config::create_table().unwrap();

        let name = "stuff";
        let profile_picture = "stuff";
        Config::new(id.into(), Some(name), Some(profile_picture), None).unwrap();
        assert_eq!(Config::get().unwrap().id().unwrap(), "HelloWorld");
        assert_eq!(Config::get().unwrap().name.unwrap(), name);
        assert_eq!(Config::get().unwrap().colorscheme, 0);
        assert_eq!(
            Config::get().unwrap().profile_picture.unwrap(),
            profile_picture
        );
    }

    #[test]
    #[serial]
    fn get_id() {
        Config::drop_table().unwrap();
        Config::create_table().unwrap();

        let id = "HelloWorld";
        let config = Config::new(id.into(), None, None, None).unwrap();

        assert_eq!(config.id().unwrap(), id);
    }
}
