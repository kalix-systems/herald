use crate::{
    db::{DBTable, Database},
    errors::*,
};
use rusqlite::{ToSql, NO_PARAMS};

/// User configuration
#[derive(Clone, Default)]
pub struct Config {
    /// id of the current user
    pub id: String,
    /// Display name for the current user
    pub name: Option<String>,
    /// Profile picture for the current user
    pub profile_picture: Option<Vec<u8>>,
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
    /// Gets the users configuration
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
                })
            },
        )?)
    }

    /// Creates config.
    pub fn add(id: &str, name: Option<&str>, profile_picture: Option<&[u8]>) -> Result<(), HErr> {
        let id = id.to_sql()?;
        let name = name.to_sql()?;
        let profile_picture = profile_picture.to_sql()?;

        let db = Database::get()?;
        db.execute(
            include_str!("sql/config/add_config.sql"),
            &[id, name, profile_picture],
        )?;

        Ok(())
    }

    /// Gets id
    pub fn get_id() -> Result<String, HErr> {
        let db = Database::get()?;
        Ok(
            db.query_row(include_str!("sql/config/get_id.sql"), NO_PARAMS, |row| {
                row.get(0)
            })?,
        )
    }

    /// Updates name
    pub fn update_name(name: Option<&str>) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/config/update_name.sql"), &[name])?;

        Ok(())
    }

    /// Updates profile picture
    pub fn update_profile_picture(profile_picture: Option<&[u8]>) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/config/update_name.sql"),
            &[profile_picture],
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

        Config::add(id, None, None).unwrap();
        assert_eq!(Config::get().unwrap().id, "HelloWorld");

        Config::drop_table().unwrap();
        Config::create_table().unwrap();

        let name = "stuff";
        let profile_picture = b"stuff";
        Config::add(id, Some(name), Some(profile_picture)).unwrap();
        assert_eq!(Config::get().unwrap().id, "HelloWorld");
        assert_eq!(Config::get().unwrap().name.unwrap(), name);
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
        Config::add(id, None, None).unwrap();

        assert_eq!(Config::get_id().unwrap(), id);
    }
}
