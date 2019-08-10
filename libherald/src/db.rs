use crate::errors::*;
use rusqlite::Connection;
use std::ops::{Deref, DerefMut};

/// Thin wrapper around sqlite3 database connection.
pub struct Database(Connection);

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
    pub fn new() -> Result<Database, HErr> {
        match Connection::open("store.sqlite3") {
            Ok(conn) => Ok(Database(conn)),
            Err(e) => Err(e.into()),
        }
    }
}
