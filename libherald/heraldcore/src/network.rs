use crate::{
    abort_err,
    config::{Config, ConfigBuilder},
    errors::HErr::{self, *},
    members, message, message_status,
    types::*,
    utils,
};
use chrono::prelude::*;
use crossbeam::channel;
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
};
use surf::{http, url};
use tungstenite::client;

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

mod helper {
    use super::server_url;
    use crate::errors::*;
    use herald_common::*;
    use surf::*;

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
}

macro_rules! of_helper {
    ($name: tt, $of: ty, $to: ty) => {
        pub async fn $name(of: $of) -> Result<$to, HErr> {
            Ok(helper::$name(&$name::Req(of)).await?.0)
        }
    };
}

of_helper!(keys_of, Vec<UserId>, HashMap<UserId, UserMeta>);
of_helper!(key_info, Vec<sig::PublicKey>, HashMap<sig::PublicKey, sig::PKMeta>);
of_helper!(keys_exist, Vec<sig::PublicKey>, Vec<bool>);
of_helper!(users_exist, Vec<UserId>, Vec<bool>);

pub async fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = dep_key::Req(kp.sign(to_dep));
    Ok(helper::dep_key(&req).await?.0)
}

pub async fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = new_key::Req(kp.sign(to_new));
    Ok(helper::new_key(&req).await?.0)
}

pub async fn register(uid: UserId) -> Result<register::Res, HErr> {
    let kp = sig::KeyPair::gen_new();
    let sig = kp.sign(*kp.public_key());
    Ok(helper::register(&register::Req(uid, sig)).await?)
}

pub fn login() -> Result<channel::Receiver<Push>, HErr> {
    use login::*;
    use tungstenite::*;

    let uid = Config::static_id()?;
    let kp = Config::static_keypair()?;
    let gid = GlobalId {
        uid,
        did: *kp.public_key(),
    };

    let (mut sender, receiver) = channel::unbounded();
    let wsurl = url::Url::parse(&format!("ws://{}/login", *SERVER_ADDR))
        .expect("failed to parse login url");
    let (mut ws, _) = client::connect(wsurl)?;

    let m = Message::binary(serde_cbor::to_vec(&SignAs(gid))?);
    ws.write_message(m)?;

    if let SignAsResponse::Sign(u) = serde_cbor::from_slice(&ws.read_message()?.into_data())? {
        let token = LoginToken(kp.raw_sign_detached(u.as_ref()));
        let m = Message::binary(serde_cbor::to_vec(&token)?);
        ws.write_message(m)?;

        match serde_cbor::from_slice(&ws.read_message()?.into_data())? {
            LoginTokenResponse::Success => {}
            LoginTokenResponse::BadSig => return Err(LoginError),
        }
    } else {
        return Err(LoginError);
    }

    // catchup(ws)?;

    // std::thread::spawn(move || {

    // });
    Ok(receiver)
}

// fn catchup<S: Read+Write>(ws: tungstenite::WebSocket<S>)
