use crate::{conversation::ExpirationPeriod, db::Database, errors::*, types::*};
pub use coretypes::config::Config;
use herald_common::*;
use rusqlite::NO_PARAMS;
use std::net::SocketAddr;

/// Default name for the "Note to Self" conversation
pub const NTS_CONVERSATION_NAME: &str = "Note to Self";

pub(crate) mod db;

/// Builder for `Config`
pub struct ConfigBuilder {
    /// ID of the local user
    id: UserId,
    keypair: sig::KeyPair,
    /// Colorscheme
    colorscheme: Option<u32>,
    /// Name of the local user
    name: Option<String>,
    /// Color of the local user
    color: Option<u32>,
    nts_conversation: Option<ConversationId>,
    home_server: Option<SocketAddr>,
    preferred_expiration: Option<ExpirationPeriod>,
}

impl ConfigBuilder {
    /// Creates new `ConfigBuilder`
    pub fn new(
        id: UserId,
        keypair: sig::KeyPair,
    ) -> Self {
        Self {
            id,
            keypair,
            name: None,
            color: None,
            colorscheme: None,
            nts_conversation: None,
            home_server: None,
            preferred_expiration: None,
        }
    }

    /// Sets colorscheme, defaults to 0 if not set.
    pub fn colorscheme(
        mut self,
        colorscheme: u32,
    ) -> Self {
        self.colorscheme = Some(colorscheme);
        self
    }

    /// Sets color, computed from hash of the UserId if not set.
    pub fn color(
        mut self,
        color: u32,
    ) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets name.
    pub fn name(
        mut self,
        name: String,
    ) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets conversation id for "Note to Self" conversation, a new conversation is created
    /// if this is not set.
    pub fn nts_conversation(
        mut self,
        conv_id: ConversationId,
    ) -> Self {
        self.nts_conversation = Some(conv_id);
        self
    }

    /// Sets the home server for this user
    pub fn home_server(
        mut self,
        home_server: SocketAddr,
    ) -> Self {
        self.home_server.replace(home_server);
        self
    }

    /// Sets the preferred expiration period
    pub fn preferred_expiration(
        mut self,
        expiration: ExpirationPeriod,
    ) -> Self {
        self.preferred_expiration.replace(expiration);
        self
    }

    /// Adds configuration.
    pub fn add(self) -> Result<Config, HErr> {
        let mut db = Database::get()?;
        let conf = self.add_db(&mut db)?;

        let ratchet = channel_ratchet::RatchetState::new();
        chainkeys::store_state(conf.nts_conversation, &ratchet)?;
        Ok(conf)
    }
}

/// Gets the user's configuration
pub fn get() -> Result<Config, HErr> {
    let db = Database::get()?;
    db::get(&db)
}

/// Gets user id
pub fn id() -> Result<UserId, HErr> {
    let db = Database::get()?;
    db::id(&db)
}

/// Gets the current user's keypair
pub fn keypair() -> Result<sig::KeyPair, HErr> {
    let db = Database::get()?;
    db::keypair(&db)
}

/// Gets the current user's GlobalId
pub fn gid() -> Result<GlobalId, HErr> {
    let db = Database::get()?;
    db::gid(&db)
}

/// Gets the current user's preferred expiration period
pub fn preferred_expiration() -> Result<ExpirationPeriod, HErr> {
    let db = Database::get()?;
    Ok(db::preferred_expiration(&db)?)
}

/// Gets the server address where the current user is registered
pub fn home_server() -> Result<std::net::SocketAddr, HErr> {
    let db = Database::get()?;
    db::home_server(&db)
}

/// Updates user's display name
pub fn set_name(name: String) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_name(&db, name)
}

/// Updates user's profile picture
pub fn set_profile_picture(profile_picture: Option<String>) -> Result<Option<String>, HErr> {
    let db = Database::get()?;
    db::set_profile_picture(&db, profile_picture)
}

/// Update user's preferred expiration period
pub fn set_preferred_expiration(period: ExpirationPeriod) -> Result<(), HErr> {
    let db = Database::get()?;
    Ok(db::set_preferred_expiration(&db, period)?)
}

/// Update user's color
pub fn set_color(color: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_color(&db, color)
}

/// Update user's colorscheme
pub fn set_colorscheme(colorscheme: u32) -> Result<(), HErr> {
    let db = Database::get()?;
    db::set_colorscheme(&db, colorscheme)
}

#[cfg(test)]
pub(crate) fn test_config() -> crate::config::Config {
    let mut db = Database::get().expect("failed to get database");
    db::test_config(&mut db)
}

#[cfg(test)]
mod tests;
