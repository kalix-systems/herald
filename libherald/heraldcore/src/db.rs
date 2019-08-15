use crate::errors::*;
use rusqlite::Connection;
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

/// Canonical database path.
static DB_PATH: &str = "store.sqlite3";

/// Thin wrapper around sqlite3 database connection.
pub(crate) struct Database(Connection);

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
    /// Establish connection with canonical database.
    #[allow(dead_code)]
    pub(crate) fn default() -> Result<Database, HErr> {
        Self::new(DB_PATH)
    }

    /// Connect to database at path `P`.
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Database, HErr> {
        match Connection::open(path) {
            Ok(conn) => Ok(Database(conn)),
            Err(e) => Err(e.into()),
        }
    }
}

impl Default for Database {
    /// Establish connection with canonical database.
    fn default() -> Self {
        Self::new(DB_PATH).expect("Failed to open database")
    }
}

pub trait DBTable: Default {
    /// Drops table if it exists.
    fn drop_table(&mut self) -> Result<(), HErr>;

    /// Creates table if it does not exist.
    fn create_table(&mut self) -> Result<(), HErr>;

    /// Indicates whether the table exists in the database.
    fn exists(&self) -> bool;
}
