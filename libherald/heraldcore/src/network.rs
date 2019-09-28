use crate::{
    config::Config,
    errors::HErr::{self, *},
    pending,
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
    sync::atomic::{AtomicBool, Ordering},
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

static CAUGHT_UP: AtomicBool = AtomicBool::new(false);

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

pub fn register(uid: UserId) -> Result<register::Res, HErr> {
    let kp = sig::KeyPair::gen_new();
    let sig = kp.sign(*kp.public_key());
    let res = helper::register(&register::Req(uid, sig))?;
    // TODO: retry if this fails?
    crate::config::ConfigBuilder::new()
        .id(uid)
        .keypair(kp)
        .add()?;
    Ok(res)
}

pub fn login() -> Result<Receiver<Notification>, HErr> {
    use login::*;
    use tungstenite::Message;

    CAUGHT_UP.store(false, Ordering::Release);

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

    sock_send_msg(&mut ws, &SignAs(gid))?;

    if let SignAsResponse::Sign(u) = sock_get_msg(&mut ws)? {
        let token = LoginToken(kp.raw_sign_detached(u.as_ref()));
        sock_send_msg(&mut ws, &token)?;

        if LoginTokenResponse::BadSig == sock_get_msg(&mut ws)? {
            return Err(LoginError);
        }
    } else {
        return Err(LoginError);
    }

    catchup(&mut ws, &mut sender)?;

    std::thread::spawn(move || {
        recv_messages(&mut ws, &mut sender)
            .unwrap_or_else(|e| eprintln!("login connection closed with message: {}", e));
        CAUGHT_UP.store(false, Ordering::Release);
    });

    Ok(receiver)
}

fn catchup<S: Read + Write>(
    ws: &mut tungstenite::WebSocket<S>,
    sender: &mut Sender<Notification>,
) -> Result<(), HErr> {
    use catchup::*;
    use tungstenite::*;

    while let Catchup::Messages(p) = sock_get_msg(ws)? {
        let len = p.len() as u64;
        for push in p.iter() {
            handle_push(push)?.execute(sender)?;
        }
        sock_send_msg(ws, &CatchupAck(len))?;
    }

    CAUGHT_UP.store(true, Ordering::Release);

    for (tag, cid, content) in pending::get_pending()? {
        send_cmessage(cid, &content)?;
        pending::remove_pending(tag)?;
    }

    Ok(())
}

fn recv_messages<S: Read + Write>(
    ws: &mut tungstenite::WebSocket<S>,
    sender: &mut Sender<Notification>,
) -> Result<(), HErr> {
    loop {
        let next = sock_get_msg(ws)?;
        let ev = handle_push(&next)?;
        ev.execute(sender)?;
    }
}

fn sock_get_msg<S: Read + Write, T: for<'a> Deserialize<'a>>(
    ws: &mut tungstenite::WebSocket<S>,
) -> Result<T, HErr> {
    Ok(serde_cbor::from_slice(&ws.read_message()?.into_data())?)
}

fn sock_send_msg<S: Read + Write, T: Serialize>(
    ws: &mut tungstenite::WebSocket<S>,
    t: &T,
) -> Result<(), HErr> {
    let m = tungstenite::Message::binary(serde_cbor::to_vec(t)?);
    ws.write_message(m)?;
    Ok(())
}

fn handle_push(push: &Push) -> Result<Event, HErr> {
    match push.tag {
        PushTag::User => {
            let umsg = serde_cbor::from_slice(&push.msg)?;
            handle_cmessage(push.timestamp, umsg)
        }
        PushTag::Device => {
            let dmsg = serde_cbor::from_slice(&push.msg)?;
            handle_dmessage(push.timestamp, dmsg)
        }
    }
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
            // we drop this error because it's pretty ok if this fails - it means we're not
            // updating the UI, but that's not a catastrophic error.
            drop(sender.send(*note));
        }

        for (cid, content) in self.replies.iter() {
            send_cmessage(*cid, content)?;
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
    use ConversationMessageBody::*;
    let mut ev = Event::default();
    match cm.open()? {
        NewKey(nk) => crate::contact_keys::key_registered(nk.0)?,
        DepKey(dk) => crate::contact_keys::key_deprecated(dk.0)?,
        AddedToConvo(ac) => {
            let mut db = crate::db::Database::get()?;
            let tx = db.transaction()?;
            let cid = ac.cid;
            let title = ac.title.as_ref().map(String::as_str);
            crate::conversation::add_conversation_with_tx(&tx, Some(&cid), title, false)?;
            crate::members::add_members_with_tx(&tx, cid, &ac.members)?;
            ev.notifications.push(Notification::NewConversation);
        }
        ContactReqAck(cr) => {
            // TODO: handle this somehow
        }
        NewMembers(nm) => {
            for member in nm.0 {
                crate::members::add_member(&cm.cid(), member)?;
            }
        }
        Msg(msg) => {
            let cmessages::Msg { mid, content, op } = msg;
            match content {
                cmessages::Message::Text(body) => {
                    crate::message::add_message(
                        Some(mid),
                        cm.from().uid,
                        &cm.cid(),
                        &body,
                        Some(ts),
                        None,
                        &op,
                    )?;
                    ev.replies.push((cm.cid(), form_ack(mid)?));
                }
                cmessages::Message::Blob(body) => unimplemented!(),
            }
        }
        Ack(ack) => {
            crate::message::add_receipt(ack.of, cm.from().uid, ack.stat)?;
            ev.notifications.push(Notification::Ack(ack.of));
        }
    }

    Ok(ev)
}

#[allow(unused_variables)]
fn handle_dmessage(ts: DateTime<Utc>, msg: DeviceMessage) -> Result<Event, HErr> {
    let mut ev = Event::default();

    match msg {
        DeviceMessage::ContactReq(cr) => {
            let mut db = crate::db::Database::get()?;
            let tx = db.transaction()?;
            crate::conversation::add_conversation_with_tx(&tx, Some(&cr.cid), None, true)?;
            crate::members::add_members_with_tx(&tx, cr.cid, &[cr.uid])?;
            ev.notifications.push(Notification::NewConversation);
        }
    }

    Ok(ev)
}

fn send_cmessage(cid: ConversationId, content: &ConversationMessageBody) -> Result<(), HErr> {
    if CAUGHT_UP.load(Ordering::Acquire) {
        let cm = ConversationMessage::seal(cid, &content)?;
        let to = crate::members::members(&cid)?;
        let msg = Bytes::from(serde_cbor::to_vec(&cm)?);
        let req = push_users::Req { to, msg };
        match helper::push_users(&req) {
            Ok(push_users::Res::Success) => Ok(()),
            Ok(push_users::Res::Missing(missing)) => Err(HeraldError(format!(
                "tried to send messages to nonexistent users {:?}",
                missing
            ))),
            Err(e) => {
                // TODO: maybe try more than once?
                // maybe have some mechanism to send a signal that more things have gone wrong?
                eprintln!(
                    "failed to send message {:?}, error was {}\n\
                     assuming failed session and adding to pending now",
                    req, e
                );

                CAUGHT_UP.store(false, Ordering::Release);

                pending::add_to_pending(cid, content)
            }
        }
    } else {
        // TODO: load it to pending here
        pending::add_to_pending(cid, content)
    }
}

pub fn start_conversation(members: &[UserId], title: Option<&str>) -> Result<(), HErr> {
    use crate::conversation;
    let pairwise = conversation::get_pairwise_conversations(members)?;
    let cid = conversation::add_conversation(None, title)?;
    let body = ConversationMessageBody::AddedToConvo(cmessages::AddedToConvo {
        members: Vec::from(members),
        cid,
        title: title.map(String::from),
    });

    for cid in pairwise {
        send_cmessage(cid, &body)?;
    }

    Ok(())
}

pub fn send_text(cid: ConversationId, body: String, op: Option<MsgId>) -> Result<(), HErr> {
    use crate::message;
    let (mid, ts) = message::add_message(
        None,
        crate::config::Config::static_id()?,
        &cid,
        &body,
        None,
        None,
        &op,
    )?;
    let content = cmessages::Message::Text(body);
    let body = ConversationMessageBody::Msg(cmessages::Msg { mid, op, content });
    send_cmessage(cid, &body)
}

fn form_ack(mid: MsgId) -> Result<ConversationMessageBody, HErr> {
    Ok(ConversationMessageBody::Ack(cmessages::Ack {
        of: mid,
        stat: MessageReceiptStatus::Received,
    }))
}
