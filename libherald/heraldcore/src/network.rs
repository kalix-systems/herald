use crate::{
    config::Config,
    errors::HErr::{self, *},
    types::*,
};
use chrono::prelude::*;
use crossbeam_channel::*;
use herald_common::*;
use lazy_static::*;
use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    net::{SocketAddr, SocketAddrV4},
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

pub(crate) fn server_url(ext: &str) -> String {
    format!("http://{}/{}", *SERVER_ADDR, ext)
}

pub type QID = [u8; 32];

#[derive(Copy, Clone, Debug)]
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

    macro_rules! mk_request {
        ($method: tt, $path: tt) => {
            pub fn $path(req: &$path::Req) -> Result<$path::Res, HErr> {
                let mut res_bytes = Vec::new();
                reqwest::Client::new()
                    .$method(&server_url(stringify!($path)))
                    .body(serde_cbor::to_vec(req)?)
                    .send()?
                    .copy_to(&mut res_bytes)?;
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
    // TODO: use these
    mk_request!(post, push_users);
    mk_request!(post, push_devices);
}

macro_rules! get_of_helper {
    ($name: tt, $of: ty, $to: ty) => {
        pub fn $name(of: $of) -> Result<$to, HErr> {
            Ok(helper::$name(&$name::Req(of))?.0)
        }
    };
}

get_of_helper!(keys_of, Vec<UserId>, HashMap<UserId, UserMeta>);
get_of_helper!(key_info, Vec<sig::PublicKey>, HashMap<sig::PublicKey, sig::PKMeta>);
get_of_helper!(keys_exist, Vec<sig::PublicKey>, Vec<bool>);
get_of_helper!(users_exist, Vec<UserId>, Vec<bool>);

// TODO: replace this w/nicer api
// this is just there to silence the warning
pub use helper::{push_devices, push_users};

pub fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = dep_key::Req(kp.sign(to_dep));
    Ok(helper::dep_key(&req)?.0)
}

pub fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = new_key::Req(kp.sign(to_new));
    Ok(helper::new_key(&req)?.0)
}

#[allow(unused_variables)]
pub fn register(uid: UserId) -> Result<register::Res, HErr> {
    let kp = sig::KeyPair::gen_new();
    let sig = kp.sign(*kp.public_key());
    let res = helper::register(&register::Req(uid, sig))?;
    unimplemented!()
    // Ok(res)
}

pub fn login() -> Result<Receiver<Notification>, HErr> {
    use login::*;
    use tungstenite::Message;

    let uid = Config::static_id()?;
    let kp = Config::static_keypair()?;
    let gid = GlobalId {
        uid,
        did: *kp.public_key(),
    };

    let (mut sender, receiver) = unbounded();
    let wsurl = url::Url::parse(&format!("ws://{}/login", *SERVER_ADDR))
        .expect("failed to parse login url");
    let (mut ws, _) = tungstenite::connect(wsurl)?;

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

    let ev = catchup(&mut ws)?;
    ev.execute(&mut sender)?;

    std::thread::spawn(move || recv_messages(ws, sender));

    Ok(receiver)
}

fn catchup<S: Read + Write>(ws: &mut tungstenite::WebSocket<S>) -> Result<Event, HErr> {
    use catchup::*;
    use tungstenite::*;

    let mut ev = Event::default();

    while let Catchup::Messages(p) = serde_cbor::from_slice(&ws.read_message()?.into_data())? {
        for push in p.iter() {
            ev.merge(match push.tag {
                PushTag::User => {
                    let umsg = serde_cbor::from_slice(&push.msg)?;
                    handle_cmessage(push.timestamp, umsg)?
                }
                PushTag::Device => {
                    let dmsg = serde_cbor::from_slice(&push.msg)?;
                    handle_dmessage(push.timestamp, dmsg)?
                }
            });
        }

        ws.write_message(Message::binary(serde_cbor::to_vec(&CatchupAck(
            p.len() as u64
        ))?))?;
    }

    Ok(ev)
}

#[allow(unused_variables)]
fn recv_messages<S: Read + Write>(ws: tungstenite::WebSocket<S>, sender: Sender<Notification>) {
    unimplemented!()
}

pub struct Event {
    notifications: Vec<Notification>,
    replies: Vec<(ConversationId, ConversationMessageBody)>,
}

impl Event {
    pub fn merge(&mut self, mut other: Self) {
        self.notifications.append(&mut other.notifications);
        self.replies.append(&mut other.replies);
    }

    pub fn execute(&self, sender: &mut Sender<Notification>) -> Result<(), HErr> {
        for note in self.notifications.iter() {
            // TODO: handle or drop this error
            sender.send(*note).expect("failed to send notification");
        }

        for (cid, content) in self.replies.iter() {
            let cm = ConversationMessage::seal(*cid, content)?;
            send_cmessage(cm)?;
        }

        Ok(())
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            notifications: Vec::new(),
            replies: Vec::new(),
        }
    }
}

#[allow(unused_variables)]
fn handle_cmessage(ts: DateTime<Utc>, cm: ConversationMessage) -> Result<Event, HErr> {
    // TODO: use this, remove allow
    let body = cm.open()?;
    unimplemented!()
}

#[allow(unused_variables)]
fn handle_dmessage(ts: DateTime<Utc>, msg: DeviceMessage) -> Result<Event, HErr> {
    unimplemented!()
}

// TODO: form push, send to server
#[allow(unused_variables)]
fn send_cmessage(cm: ConversationMessage) -> Result<(), HErr> {
    unimplemented!()
}
