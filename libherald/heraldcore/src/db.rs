use crate::{abort_err, errors::*, utils::SearchPattern};
use lazy_pond::*;
use lazy_static::*;
use rusqlite::{Connection, NO_PARAMS};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

lazy_static! {
    static ref DB_POOL: LazyPond<Database> = LazyPond::new(Some(32));
}

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
    pub(crate) fn get<'a>() -> Result<Wrapper<'a, Database>, HErr> {
        DB_POOL.get().map_err(|_| HErr::LazyPondError)
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

/// Initializes storage
pub fn init() -> Result<(), HErr> {
    let mut db = Database::get()?;

    let tx = db.transaction()?;
    tx.execute_batch(include_str!("sql/create_all.sql"))?;
    tx.commit()?;

    Ok(())
}

impl Database {
    /// Connect to database at path `P`.
    /// Creates a database if one does not exist.
    fn new<P: AsRef<Path>>(path: P) -> Result<Database, HErr> {
        match Connection::open(path) {
            Ok(conn) => {
                conn.busy_timeout(std::time::Duration::from_secs(60))?;

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

                // enable WAL
                conn.query_row("PRAGMA journal_mode = WAL", NO_PARAMS, |_| Ok(()))?;

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
        tx.execute_batch(include_str!("sql/drop_all.sql"))?;

        // create
        tx.execute_batch(include_str!("sql/create_all.sql"))?;
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
