use super::*;
use sharded_slab::*;
use std::{
    cmp::{max, min},
    net::SocketAddr,
    ops::Deref,
    sync::Arc,
};
use tungstenite::{protocol::*, Message};

mod frames;
use frames::*;

#[cfg(feature = "ws")]
pub mod server;
#[cfg(feature = "ws")]
pub use server::prelude::*;

#[cfg(feature = "ws")]
pub mod async_client;
#[cfg(feature = "ws")]
pub use async_client::Client;

trait WsProtocol: Protocol {
    fn max_item_size() -> usize {
        max(
            max(Self::MAX_REQ_SIZE, Self::MAX_ACK_SIZE),
            max(Self::MAX_RESP_SIZE, Self::MAX_PUSH_SIZE),
        )
    }
}

impl<P: Protocol> WsProtocol for P {}

#[derive(Debug, Eq, PartialEq)]
pub struct ConnectionClosed(pub Option<CloseFrame<'static>>);

impl std::fmt::Display for ConnectionClosed {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(fmt, "Connection closed with frame: {:?}", self.0)
    }
}

impl std::error::Error for ConnectionClosed {}
