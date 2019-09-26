use crate::{
    abort_err,
    config::{Config, ConfigBuilder},
    errors::HErr::{self, *},
    members, message, message_status,
    types::*,
    utils,
};
use chrono::prelude::*;
use dashmap::DashMap;
// use futures::compat::*;
use herald_common::*;
use lazy_static::*;
use sodiumoxide::{
    crypto::{box_, generichash as hash, sealedbox, sign},
    randombytes::randombytes_into,
};
use std::{
    collections::HashMap,
    env,
    net::{SocketAddr, SocketAddrV4},
    ops::DerefMut,
    sync::Arc,
};
use surf::{http, url};
use tokio::{
    net::TcpStream,
    prelude::*,
    sync::{
        mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender},
        oneshot,
    },
};

const DEFAULT_PORT: u16 = 8000;
const DEFAULT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

lazy_static! {
    static ref SERVER_ADDR: SocketAddr = match env::var("SERVER_ADDR") {
        Ok(addr) => addr.parse().unwrap_or_else(|e| {
            eprintln!("Provided address {} is invalid: {}", addr, e);
            std::process::abort();
        }),
        Err(_) => SocketAddr::V4(SocketAddrV4::new(
            DEFAULT_SERVER_IP_ADDR.into(),
            DEFAULT_PORT
        )),
    };
}

fn server_url(ext: &str) -> String {
    format!("http://{}/{}", *SERVER_ADDR, ext)
}

pub type QID = [u8; 32];

#[derive(Debug)]
/// `Notification`s contain info about what updates were made to the db.
pub enum Notification {
    /// A new message has been received.
    NewMsg(ConversationId),
    /// An ack has been received.
    Ack(MsgId),
    /// A new contact has been added
    NewContact,
    /// A new conversation has been added
    NewConversation,
    AddContactReponse(bool),
    AddConversationReponse(bool),
}

macro_rules! mk_request {
    ($method: tt, $path: tt) => {
        pub async fn $path(req: &$path::Req) -> Result<$path::Res, HErr> {
            let res_bytes = surf::$method(server_url(stringify!($path)))
                .body_bytes(serde_cbor::to_vec(req)?)
                .recv_bytes()
                .await?;
            let res = serde_cbor::from_slice(&res_bytes)?;
            Ok(res)
        }
    };
}

mk_request!(get, keys_of);
mk_request!(get, key_info);
mk_request!(get, keys_exist);
mk_request!(get, users_exist);
mk_request!(post, register);
mk_request!(post, new_key);
mk_request!(post, dep_key);
