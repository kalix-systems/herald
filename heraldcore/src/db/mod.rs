use super::*;
use crate::errors::*;
use coremacros::w;
use once_cell::sync::OnceCell;
use platform_dirs::db_dir;
use rusqlite::{Connection, NO_PARAMS};
use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

mod pool;
use pool::*;

static DB_POOL: OnceCell<Pool> = OnceCell::new();

fn db_pool() -> &'static Pool {
    DB_POOL.get_or_init(Pool::new)
}

fn db_path() -> PathBuf {
    db_dir().join("store.sqlite3")
}

/// Thin wrapper around sqlite3 database connection.
pub(crate) struct Database(Connection);

impl Database {
    pub(crate) fn get() -> Result<Wrapper, HErr> {
        db_pool().get()
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
    let db = Database::get()?;

    w!(db.execute_batch(include_str!("../sql/create_all.sql")));

    Ok(())
}

/// Resets all tables in database.
pub fn reset_all() -> Result<(), HErr> {
    let mut db = w!(Database::get());
    let tx = w!(db.transaction());

    // drop
    w!(tx.execute_batch(include_str!("../sql/drop_all.sql")));

    // create
    w!(tx.execute_batch(include_str!("../sql/create_all.sql")));
    w!(tx.commit());
    Ok(())
}

impl Database {
    /// Connect to database at path `P`.
    /// Creates a database if one does not exist.
    fn new<P: AsRef<Path>>(path: P) -> Result<Database, HErr> {
        let conn = Connection::open(path)?;
        Self::setup(conn)
    }

    fn setup(conn: Connection) -> Result<Self, HErr> {
        fn busy_handler(_: i32) -> bool {
            true
        }

        w!(conn.busy_handler(Some(busy_handler)));

        // set foreign key constraint
        w!(conn.execute("PRAGMA foreign_keys = ON", NO_PARAMS));

        // enable WAL
        w!(conn.query_row("PRAGMA journal_mode = WAL", NO_PARAMS, |_| Ok(())));

        let db = Database(conn);

        Ok(db)
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Result<Self, HErr> {
        let conn = w!(Connection::open_in_memory());
        w!(conn.execute_batch(include_str!("../sql/create_all.sql")));
        Self::setup(conn)
    }

    #[cfg(test)]
    pub(crate) fn in_memory_with_config() -> Result<Self, HErr> {
        let conn = w!(Connection::open_in_memory());
        w!(conn.execute_batch(include_str!("../sql/create_all.sql")));
        let mut conn = Self::setup(conn)?;
        crate::config::db::test_config(&mut conn);
        Ok(conn)
    }

    /// Resets all tables in database
    #[cfg(test)]
    pub(crate) fn reset_all() -> Result<(), HErr> {
        let mut db = w!(Self::get());
        let tx = w!(db.transaction());

        // drop
        w!(tx.execute_batch(include_str!("../sql/drop_all.sql")));

        // create
        w!(tx.execute_batch(include_str!("../sql/create_all.sql")));
        w!(tx.commit());
        Ok(())
    }
}
