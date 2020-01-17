use super::*;
use coremacros::exit_err;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use platform_dirs::db_dir;

pub struct Conn<'conn>(rusqlite::Transaction<'conn>);

impl<'conn> From<rusqlite::Transaction<'conn>> for Conn<'conn> {
    fn from(tx: rusqlite::Transaction<'conn>) -> Self {
        Self(tx)
    }
}

impl<'conn> std::ops::Deref for Conn<'conn> {
    type Target = rusqlite::Transaction<'conn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StoreLike for Conn<'_> {
    type Error = errors::Error;
}

static CONN: OnceCell<Mutex<rusqlite::Connection>> = OnceCell::new();

pub fn raw_conn() -> &'static Mutex<rusqlite::Connection> {
    CONN.get_or_init(|| {
        kcl::init();

        let path = db_dir().join("ck.sqlite3");
        let mut conn = exit_err!(rusqlite::Connection::open(path));
        let tx = exit_err!(conn.transaction());

        exit_err!(tx.execute_batch(include_str!("../schema/up.sql")));
        exit_err!(tx.commit());
        Mutex::new(conn)
    })
}

#[cfg(test)]
pub(crate) fn in_memory() -> rusqlite::Connection {
    use coremacros::womp;

    let mut conn = rusqlite::Connection::open_in_memory().expect(womp!());
    let tx = conn.transaction().expect(womp!());

    tx.execute_batch(include_str!("../schema/up.sql"))
        .expect(womp!());
    tx.commit().expect(womp!());

    conn
}
