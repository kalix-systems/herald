use crate::{errors::*, utils::SearchPattern};
use lazy_static::*;
use rusqlite::{Connection, NO_PARAMS};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
    sync::{Mutex, MutexGuard},
};

lazy_static! {
    pub(crate) static ref DB: Mutex<Database> = Mutex::new(Database::default());
}

/// Canonical database path.
static DB_PATH: &str = "store.sqlite3";

/// Thin wrapper around sqlite3 database connection.
pub(crate) struct Database(Connection);

impl Database {
    pub(crate) fn get() -> Result<MutexGuard<'static, Database>, HErr> {
        Ok(DB.lock()?)
    }
}

impl Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Database {
    /// Connect to database at path `P`.
    fn new<P: AsRef<Path>>(path: P) -> Result<Database, HErr> {
        match Connection::open(path) {
            Ok(conn) => {
                // `NormalPattern`
                conn.create_scalar_function("normal_pattern", 2, true, |ctx| {
                    let pattern = ctx.get::<String>(0)?;
                    let value = ctx.get::<String>(1)?;

                    let re = match SearchPattern::new_normal(pattern) {
                        Ok(SearchPattern::Normal(re)) => re,
                        _ => return Ok(false),
                    };

                    Ok(re.is_match(value.as_str()))
                })?;

                // `RegexPattern`
                conn.create_scalar_function("regex_pattern", 2, true, |ctx| {
                    let pattern = ctx.get::<String>(0)?;
                    let value = ctx.get::<String>(1)?;

                    let re = match SearchPattern::new_regex(pattern) {
                        Ok(SearchPattern::Regex(re)) => re,
                        _ => return Ok(false),
                    };

                    Ok(re.is_match(value.as_str()))
                })?;

                // set foreign key constraint
                conn.execute("PRAGMA foreign_keys = ON", NO_PARAMS)?;

                let db = Database(conn);
                Ok(db)
            }
            Err(e) => Err(e.into()),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn reset_all() -> Result<(), HErr> {
        let mut db = Self::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/message/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/config/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/members/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/conversation/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/message/create_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/contact/create_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/config/create_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/members/create_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/conversation/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

impl Default for Database {
    /// Establish connection with canonical database.
    fn default() -> Self {
        match Self::new(DB_PATH) {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Failed to open database, aborting: {}", e);
                std::process::abort();
            }
        }
    }
}

/// Types that are wrappers around database tables.
pub trait DBTable: Default {
    /// Drops table if it exists.
    fn drop_table() -> Result<(), HErr>;

    /// Creates table if it does not exist.
    fn create_table() -> Result<(), HErr>;

    /// Indicates whether the table exists in the database.
    fn exists() -> Result<bool, HErr>;

    /// Resets the table.
    fn reset() -> Result<(), HErr>;
}
