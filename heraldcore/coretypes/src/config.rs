use crate::*;
use conversation::ExpirationPeriod;
use herald_common::*;
use herald_ids::ConversationId;
use std::net::SocketAddr;

/// User configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// ID of the local user
    pub id: UserId,
    /// Colorscheme
    pub colorscheme: u32,
    /// Name of the local user
    pub name: String,
    /// Profile picture of the local user
    pub profile_picture: Option<String>,
    /// Color of the local user
    pub color: u32,
    /// The *Note to Self* conversation id.
    pub nts_conversation: ConversationId,
    /// The server this account is registered on
    pub home_server: SocketAddr,
    /// The default preferred expiration period
    pub preferred_expiration: ExpirationPeriod,
}
