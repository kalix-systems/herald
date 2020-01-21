use ratchet_chat::StoreLike;
use rusqlite::named_params as np;

pub use connection::Conn;
pub use coremacros::w;
pub use errors::Error;
pub mod connection;
pub mod prelude {
    pub use crate::connection::{as_conn, raw_conn, Conn};
    pub use crate::errors::Error;
    pub use ratchet_chat::protocol::*;
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
