use super::*;
use lazy_static::*;
use parking_lot::RwLock;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::atomic::AtomicBool,
};

pub(super) const DEFAULT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

lazy_static! {
    pub(super) static ref SERVER_ADDR: Ipv4Addr = match &crate::utils::CONF.server_addr {
        Some(addr) => addr.parse().unwrap_or_else(|e| {
            eprintln!("Provided address {} is invalid: {}", addr, e);
            std::process::abort();
        }),
        None => DEFAULT_SERVER_IP_ADDR.into(),
    };
    pub(super) static ref SERVER_TCP_ADDR: SocketAddr = (*SERVER_ADDR.deref(), TCP_PORT).into();
    pub(super) static ref SERVER_RPC_ADDR: SocketAddr = (*SERVER_ADDR.deref(), RPC_PORT).into();
}

pub(super) static CAUGHT_UP: AtomicBool = AtomicBool::new(false);

lazy_static! {
    pub(super) static ref RPC_CLIENT: RwLock<Option<HeraldServiceClient>> = RwLock::new(None);
}

/// Attempts to load the cached client, and creates a new one if one doesn't already exist.
pub(super) async fn get_client() -> Result<HeraldServiceClient, HErr> {
    let o_client: Option<HeraldServiceClient> = RPC_CLIENT.read().as_ref().cloned();
    match o_client {
        Some(h) => Ok(h),
        None => {
            let mut slot = RPC_CLIENT.write();
            let transport = transport::connect(&SERVER_RPC_ADDR).await?;
            let client =
                HeraldServiceClient::new(tarpc::client::Config::default(), transport).spawn()?;
            *slot = Some(client.clone());
            Ok(client)
        }
    }
}
