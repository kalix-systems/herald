use crate::{
    config::Config,
    errors::HErr::{self, *},
    pending,
    types::*,
};
use chrono::prelude::*;
use herald_common::*;
use lazy_static::*;
use std::{
    collections::HashMap,
    env,
    net::{SocketAddr, SocketAddrV4},
    sync::atomic::{AtomicBool, Ordering},
};
use websocket::sync::client as wsclient;

const DEFAULT_PORT: u16 = 8080;
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

/// Query ID.
pub type QID = [u8; 32];

#[derive(Copy, Clone, Debug)]
/// `Notification`s contain info about what updates were made to the database.
pub enum Notification {
    /// A new message has been received.
    NewMsg(MsgId, ConversationId),
    /// A message has been received.
    MsgReceipt {
        /// The message that was received
        mid: MsgId,
        /// The conversation the message was part of
        cid: ConversationId,
        /// The new status of the message
        stat: MessageReceiptStatus,
        /// The recipient of the message
        by: UserId,
    },
    /// A new contact has been added
    NewContact(UserId, ConversationId),
    /// A new conversation has been added
    NewConversation(ConversationId),
    /// Response to contact request.
    AddContactResponse(ConversationId, UserId, bool),
    /// Response to request to join conversation.
    AddConversationResponse(ConversationId, UserId, bool),
}

mod helper {
    use super::server_url;
    use crate::errors::*;
    use herald_common::*;

    macro_rules! mk_request {
        ($method: tt, $path: tt) => {
            pub fn $path(req: &$path::Req) -> Result<$path::Res, HErr> {
                let res_reader = ureq::$method(&server_url(stringify!($path)))
                    .send_bytes(&serde_cbor::to_vec(req)?)
                    .into_reader();
                let res = serde_cbor::from_reader(res_reader)?;
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
        #[allow(missing_docs)]
        pub fn $name(of: $of) -> Result<$to, HErr> {
            Ok(helper::$name(&$name::Req(of))?.0)
        }
    };
}

get_of_helper!(keys_of, Vec<UserId>, HashMap<UserId, UserMeta>);
get_of_helper!(key_info, Vec<sig::PublicKey>, HashMap<sig::PublicKey, sig::PKMeta>);
get_of_helper!(keys_exist, Vec<sig::PublicKey>, Vec<bool>);
get_of_helper!(users_exist, Vec<UserId>, Vec<bool>);

/// Deprecates key on server.
pub fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = dep_key::Req(kp.sign(to_dep));
    Ok(helper::dep_key(&req)?.0)
}

/// Adds new key to the server's key registry.
pub fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = new_key::Req(kp.sign(to_new));
    Ok(helper::new_key(&req)?.0)
}

/// Registers new user on the server.
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

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
///
/// Takes a callback as an argument that is called whenever a message is received.
pub fn login<F: FnMut(Notification) + Send + 'static>(mut f: F) -> Result<(), HErr> {
    use login::*;

    CAUGHT_UP.store(false, Ordering::Release);

    let uid = Config::static_id()?;
    let kp = Config::static_keypair()?;
    let gid = GlobalId {
        uid,
        did: *kp.public_key(),
    };

    let wsurl = format!("ws://{}/login", *SERVER_ADDR);
    let mut ws = wsclient::ClientBuilder::new(&wsurl)
        .expect("failed to parse server url")
        .connect_insecure()?;

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

    catchup(&mut ws, &mut f)?;

    std::thread::spawn(move || {
        recv_messages(&mut ws, &mut f)
            .unwrap_or_else(|e| eprintln!("login connection closed with message: {}", e));
        CAUGHT_UP.store(false, Ordering::Release);
    });

    Ok(())
}

fn catchup<S: websocket::stream::Stream, F: FnMut(Notification)>(
    ws: &mut wsclient::Client<S>,
    f: &mut F,
) -> Result<(), HErr> {
    use catchup::*;

    while let Catchup::Messages(p) = sock_get_msg(ws)? {
        let len = p.len() as u64;
        for push in p.iter() {
            handle_push(push)?.execute(f)?;
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

fn recv_messages<S: websocket::stream::Stream, F: FnMut(Notification)>(
    ws: &mut wsclient::Client<S>,
    f: &mut F,
) -> Result<(), HErr> {
    loop {
        let next = sock_get_msg(ws)?;
        let ev = handle_push(&next)?;
        ev.execute(f)?;
    }
}

fn sock_get_msg<S: websocket::stream::Stream, T: for<'a> Deserialize<'a>>(
    ws: &mut wsclient::Client<S>,
) -> Result<T, HErr> {
    let res = ws.recv_message()?;
    let parsed = serde_cbor::from_slice(websocket::message::Message::from(res).payload.as_ref())?;
    Ok(parsed)
}

fn sock_send_msg<S: websocket::stream::Stream, T: Serialize>(
    ws: &mut wsclient::Client<S>,
    t: &T,
) -> Result<(), HErr> {
    use ::websocket::message::OwnedMessage;
    let m = OwnedMessage::Binary(serde_cbor::to_vec(t)?);
    ws.send_message(&m)?;
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

/// An event. These are produced in response a message being received from the server.
pub struct Event {
    notifications: Vec<Notification>,
    replies: Vec<(ConversationId, ConversationMessageBody)>,
}

impl Event {
    /// Merges two events.
    pub fn merge(&mut self, mut other: Self) {
        self.notifications.append(&mut other.notifications);
        self.replies.append(&mut other.replies);
    }

    /// Sends replies to inbound messages and calls `f`, passing each notification in as an
    /// argument.
    pub fn execute<F: FnMut(Notification)>(&self, f: &mut F) -> Result<(), HErr> {
        for note in self.notifications.iter() {
            f(*note);
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
        NewKey(nk) => crate::contact_keys::add_keys(cm.from().uid, &[nk.0])?,
        DepKey(dk) => crate::contact_keys::deprecate_keys(&[dk.0])?,
        AddedToConvo(ac) => {
            let mut db = crate::db::Database::get()?;
            let tx = db.transaction()?;
            let cid = ac.cid;
            let title = ac.title.as_ref().map(String::as_str);
            crate::conversation::add_conversation_with_tx(&tx, Some(&cid), title, false)?;
            crate::members::add_members_with_tx(&tx, cid, &ac.members)?;
            tx.commit()?;
            ev.notifications.push(Notification::NewConversation(cid));
        }
        ContactReqAck(cr) => ev.notifications.push(Notification::AddContactResponse(
            cm.cid(),
            cm.from().uid,
            cr.0,
        )),
        NewMembers(nm) => {
            let mut db = crate::db::Database::get()?;
            let tx = db.transaction()?;
            crate::members::add_members_with_tx(&tx, cm.cid(), &nm.0)?;
            tx.commit()?;
        }
        Msg(msg) => {
            // fix for message loopback back until this can be handled server side
            if cm.from().did == *Config::static_keypair()?.public_key() {
                return Ok(ev);
            }

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
                    ev.notifications.push(Notification::NewMsg(mid, cm.cid()));
                    ev.replies.push((cm.cid(), form_ack(mid)?));
                }
                cmessages::Message::Blob(body) => unimplemented!(),
            }
        }
        Ack(ack) => {
            // TODO: This will cause a foreign key constraint error if the receipt is
            // received after a message has been removed locally.
            // We should check the sqlite3 extended error code (787) here.
            crate::message_receipts::add_receipt(ack.of, cm.from().uid, ack.stat)?;
            ev.notifications.push(Notification::MsgReceipt {
                mid: ack.of,
                cid: cm.cid(),
                stat: ack.stat,
                by: cm.from().uid,
            });
        }
    }

    Ok(ev)
}

#[allow(unused_variables)]
fn handle_dmessage(ts: DateTime<Utc>, msg: DeviceMessage) -> Result<Event, HErr> {
    let mut ev = Event::default();

    match msg {
        DeviceMessage::ContactReq(cr) => {
            crate::contact::ContactBuilder::new(cr.uid)
                .pairwise_conversation(cr.cid)
                .add()?;
            ev.notifications
                .push(Notification::NewContact(cr.uid, cr.cid));
            ev.replies.push((
                cr.cid,
                ConversationMessageBody::ContactReqAck(cmessages::ContactReqAck(true)),
            ))
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
        pending::add_to_pending(cid, content)
    }
}

fn send_dmessage(dids: &[sig::PublicKey], msg: &DeviceMessage) -> Result<(), HErr> {
    let msg = Bytes::from(serde_cbor::to_vec(msg)?);

    let req = push_devices::Req {
        to: dids.to_vec(),
        msg,
    };

    // TODO retry logic? for now, things go to the void
    match helper::push_devices(&req)? {
        push_devices::Res::Success => Ok(()),
        push_devices::Res::Missing(missing) => Err(HeraldError(format!(
            "tried to send messages to nonexistent key_infos {:?}",
            missing
        ))),
    }
}

fn send_umessage(uid: UserId, msg: &DeviceMessage) -> Result<(), HErr> {
    let meta = keys_of(vec![uid])?
        .remove(&uid)
        .ok_or_else(|| HErr::HeraldError(format!("No keys associated with {}", uid)))?;

    let keys: Vec<sig::PublicKey> = meta.keys.into_iter().map(|(k, _)| k).collect();

    send_dmessage(&keys, msg)
}

/// Sends a contact request to `uid` with a proposed conversation id `cid`.
pub fn send_contact_req(uid: UserId, cid: ConversationId) -> Result<(), HErr> {
    let req = dmessages::ContactReq {
        uid: Config::static_id()?,
        cid,
    };
    send_umessage(uid, &DeviceMessage::ContactReq(req))
}

/// Starts a conversation with `members`. Note: all members must be in the user's contacts already.
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

/// Sends a text message `body` with id `mid` to the conversation associated with `cid`.
pub fn send_text(
    cid: ConversationId,
    body: String,
    mid: MsgId,
    op: Option<MsgId>,
) -> Result<(), HErr> {
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
