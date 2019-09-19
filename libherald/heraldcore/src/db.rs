use crate::{abort_err, errors::*, utils::SearchPattern};
use lazy_static::*;
use rusqlite::{Connection, NO_PARAMS};

use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

lazy_static! {
    /// this can be set by the user so
    /// that during testing, two instances of
    /// herald may have different databases.
    static ref DB_PATH: String = match std::env::var("HERALD_DB_PATH") {
        Ok(path) =>  if cfg!(debug_assertions) { path }
                     else { "store.sqlite3".to_owned() },
        Err(_) => "store.sqlite3".to_owned(),
    };
}
/// Thin wrapper around sqlite3 database connection.
pub(crate) struct Database(Connection);

impl Clone for Database {
    fn clone(&self) -> Database {
        abort_err!(Database::new(DB_PATH.as_str()))
    }
}

impl Database {
    pub(crate) fn get() -> Result<Database, HErr> {
        Database::new(DB_PATH.as_str())
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
    /// Creates a database if one does not exist.
    fn new<P: AsRef<Path>>(path: P) -> Result<Database, HErr> {
        match Connection::open(path) {
            Ok(conn) => {
                // `NormalPattern`
                conn.create_scalar_function("normal_pattern", 2, true, |ctx| {
                    let pattern = ctx.get::<String>(0)?;
                    let value = ctx.get::<String>(1)?;

                    let re = SearchPattern::new_normal(pattern)
                        .map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?;

                    Ok(re.is_match(value.as_str()))
                })?;

                // `RegexPattern`
                conn.create_scalar_function("regex_pattern", 2, true, |ctx| {
                    let pattern = ctx.get::<String>(0)?;
                    let value = ctx.get::<String>(1)?;

                    let re = SearchPattern::new_regex(pattern)
                        .map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?;

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

    /// Reset all table in database
    #[allow(dead_code)]
    pub(crate) fn reset_all() -> Result<(), HErr> {
        let mut db = Self::get()?;
        let tx = db.transaction()?;

        // drop
        tx.execute(include_str!("sql/message_status/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/message/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/members/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/config/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/conversation/drop_table.sql"), NO_PARAMS)?;

        // create
        tx.execute(
            include_str!("sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
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
        abort_err!(Self::new(DB_PATH.as_str()))
    }
}

/// Types that are wrappers around database tables.
pub trait DBTable: Default {
    /// Drops table if it exists.
    fn drop_table() -> Result<(), HErr>;

    /// Creates table if it does not exist.
    fn create_table() -> Result<(), HErr>;

    /// Indicates whether a table exists
    fn exists() -> Result<bool, HErr>;

    /// Resets the table.
    fn reset() -> Result<(), HErr>;
}
