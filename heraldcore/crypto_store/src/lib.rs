//use coremacros::exit_err;
//use once_cell::sync::OnceCell;
use ratchet_chat::StoreLike;
use rusqlite::named_params as np;

pub struct Conn<'conn>(rusqlite::Transaction<'conn>);

impl<'conn> std::ops::Deref for Conn<'conn> {
    type Target = rusqlite::Transaction<'conn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

impl StoreLike for Conn<'_> {
    type Error = errors::Error;
}

mod conversation;
mod errors;
mod pending;
mod ratchet;
mod sigstore;

pub use errors::Error;

pub mod prelude {
    pub use crate::errors::Error;
    pub use crate::Conn;
    pub use ratchet_chat::protocol::{ConversationStore, PendingStore, RatchetStore, SigStore};
}
