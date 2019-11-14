use super::*;
use lazy_static::*;
use once_cell::sync::*;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::atomic::{AtomicBool, Ordering},
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

pub(super) static RPC_CLIENT: OnceCell<HeraldServiceClient> = OnceCell::new();
static RPC_IS_BEING_INIT: AtomicBool = AtomicBool::new(false);

/// Attempts to load the cached client, and creates a new one if one doesn't already exist.
pub(super) async fn get_client() -> Result<HeraldServiceClient, HErr> {
    loop {
        if let Some(r) = RPC_CLIENT.get() {
            return Ok(r.clone());
        } else if !RPC_IS_BEING_INIT.swap(true, Ordering::SeqCst) {
            let _guard =
                scopeguard::guard((), |()| RPC_IS_BEING_INIT.store(false, Ordering::SeqCst));

            let transport = transport::connect(&SERVER_RPC_ADDR).await?;
            let client =
                HeraldServiceClient::new(tarpc::client::Config::default(), transport).spawn()?;

            RPC_CLIENT
                .set(client.clone())
                .expect("failed to set client");

            return Ok(client);
        }
        tokio::timer::delay_for(std::time::Duration::from_millis(100)).await;
    }
}
