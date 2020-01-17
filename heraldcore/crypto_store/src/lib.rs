use ratchet_chat::StoreLike;
use rusqlite::named_params as np;

pub use coremacros::w;
pub use errors::Error;
pub mod prelude {
    pub use crate::errors::Error;
    pub use crate::Conn;
    pub use ratchet_chat::protocol::{ConversationStore, PendingStore, RatchetStore, SigStore};
}

pub struct Conn<'conn>(rusqlite::Transaction<'conn>);

impl<'conn> From<rusqlite::Transaction<'conn>> for Conn<'conn> {
    fn from(tx: rusqlite::Transaction<'conn>) -> Self {
        Self(tx)
    }
}

pub use connection::raw_conn;

mod connection {
    use coremacros::exit_err;
    use once_cell::sync::OnceCell;
    use parking_lot::Mutex;
    use platform_dirs::db_dir;

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

macro_rules! ok_none {
    ($maybe: expr) => {
        match $maybe {
            Some(val) => val,
            None => return Ok(None),
        }
    };
}

macro_rules! sql {
    ($category:literal, $file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/sql/",
            $category,
            "/",
            $file,
            ".sql"
        ))
    };
}

macro_rules! st {
    ($slf: ident, $category:literal, $file:literal) => {
        coremacros::w!($slf.prepare_cached(sql!($category, $file)))
    };
}

mod conversation;
mod errors;
mod pending;
mod ratchet;
mod sigstore;
