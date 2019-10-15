use crate::{
    config::Config,
    errors::HErr::{self, *},
    pending,
    types::*,
};
use chainmail::block::*;
use chrono::prelude::*;
use herald_common::*;
use lazy_static::*;
use std::{
    env,
    net::{SocketAddr, SocketAddrV4},
    sync::atomic::{AtomicBool, Ordering},
};
use websocket::{message::OwnedMessage as WMessage, sync::client as wsclient};

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

get_of_helper!(keys_of, Vec<UserId>, Vec<(UserId, UserMeta)>);
get_of_helper!(
    key_info,
    Vec<sig::PublicKey>,
    Vec<(sig::PublicKey, sig::PKMeta)>
);
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
    crate::config::ConfigBuilder::new(uid, kp).add()?;
    Ok(res)
}

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
///
/// Takes a callback as an argument that is called whenever a message is received.
pub fn login<F: FnMut(Notification) + Send + 'static>(mut f: F) -> Result<(), HErr> {
    use login::*;

    if sodiumoxide::init().is_err() {
        eprintln!("failed to init libsodium - what are you doing");
        std::process::abort()
    }

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

    match sock_get_msg(&mut ws)? {
        SignAsResponse::Sign(u) => {
            let token = LoginToken(kp.raw_sign_detached(u.as_ref()));
            sock_send_msg(&mut ws, &token)?;

            match sock_get_msg(&mut ws)? {
                LoginTokenResponse::Success => {}
                e => return Err(SignInFailed(e)),
            }
        }
        e => return Err(GIDSpecFailed(e)),
    }

    let ev = catchup(&mut ws)?;

    CAUGHT_UP.store(true, Ordering::Release);

    // clear pending
    for (tag, cid, content) in pending::get_pending()? {
        send_cmessage(cid, &content)?;
        pending::remove_pending(tag)?;
    }

    // send read receipts, etc
    ev.execute(&mut f)?;

    std::thread::spawn(move || {
        move || -> Result<(), HErr> {
            loop {
                catchup(&mut ws)?.execute(&mut f)?;
            }
        }()
        .unwrap_or_else(|e| eprintln!("login connection closed with message: {}", e));
        CAUGHT_UP.store(false, Ordering::Release);
    });

    Ok(())
}

fn catchup<S: websocket::stream::Stream>(ws: &mut wsclient::Client<S>) -> Result<Event, HErr> {
    use catchup::*;

    let mut ev = Event::default();

    while let Catchup::Messages(p) = sock_get_msg(ws)? {
        let len = p.len() as u64;
        for push in p.iter() {
            ev.merge(handle_push(push)?);
        }
        sock_send_msg(ws, &CatchupAck(len))?;
    }

    Ok(ev)
}

fn sock_get_msg<S: websocket::stream::Stream, T: for<'a> Deserialize<'a>>(
    ws: &mut wsclient::Client<S>,
) -> Result<T, HErr> {
    let len;

    loop {
        let maybe_len = sock_get_block(ws)?;
        sock_send_msg(ws, &maybe_len)?;
        match sock_get_block(ws)? {
            PacketResponse::Success => {
                len = maybe_len;
                break;
            }
            PacketResponse::Retry => {}
        }
    }

    loop {
        let mut packets = Vec::with_capacity(len as usize);
        for _ in 0..len {
            packets.push(sock_get_block(ws)?);
        }
        match Packet::collect(&packets) {
            Some(v) => {
                // TODO: consider doing this later?
                // or maybe having a callback that has to succeeed here?
                // after the server receives this, it *will* delete the message,
                // so I'm inclined to be damn sure we're done with it
                sock_send_msg(ws, &PacketResponse::Success)?;
                return Ok(serde_cbor::from_slice(&v)?);
            }
            None => {
                sock_send_msg(ws, &PacketResponse::Retry)?;
            }
        }
    }
}

fn sock_get_block<S: websocket::stream::Stream, T: for<'a> Deserialize<'a>>(
    ws: &mut wsclient::Client<S>,
) -> Result<T, HErr> {
    loop {
        match ws.recv_message()? {
            WMessage::Binary(v) => return Ok(serde_cbor::from_slice(&v)?),
            _ => {}
        }
    }
}

fn sock_send_msg<S: websocket::stream::Stream, T: Serialize>(
    ws: &mut wsclient::Client<S>,
    t: &T,
) -> Result<(), HErr> {
    let m = WMessage::Binary(serde_cbor::to_vec(t)?);
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
#[derive(Debug)]
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

fn handle_cmessage(ts: DateTime<Utc>, cm: ConversationMessage) -> Result<Event, HErr> {
    use ConversationMessageBody::*;
    let mut ev = Event::default();

    let cid = cm.cid();

    let msgs = cm.open()?;

    for (msg, from) in msgs {
        match msg {
            NewKey(nk) => crate::contact_keys::add_keys(from.uid, &[nk.0])?,
            DepKey(dk) => crate::contact_keys::deprecate_keys(&[dk.0])?,
            AddedToConvo(ac) => {
                let mut db = crate::db::Database::get()?;
                let tx = db.transaction()?;
                let mut cid = ac.cid;

                let title = ac.title;

                let mut conv_builder = crate::conversation::ConversationBuilder::new();
                conv_builder.conversation_id(cid);

                if let Some(title) = title {
                    conv_builder.title(title);
                }

                conv_builder.add_with_tx(&tx)?;
                crate::members::add_members_with_tx(&tx, cid, &ac.members)?;
                tx.commit()?;

                cid.store_genesis(ac.gen)?;

                ev.notifications.push(Notification::NewConversation(cid));
            }
            ContactReqAck(cr) => ev
                .notifications
                .push(Notification::AddContactResponse(cid, from.uid, cr.0)),
            NewMembers(nm) => {
                let mut db = crate::db::Database::get()?;
                let tx = db.transaction()?;
                crate::members::add_members_with_tx(&tx, cid, &nm.0)?;
                tx.commit()?;
            }
            Msg(msg) => {
                let cmessages::Msg { mid, content, op } = msg;

                match content {
                    cmessages::Message::Text(body) => {
                        crate::message::add_message(
                            Some(mid),
                            from.uid,
                            &cid,
                            &body,
                            Some(ts),
                            None,
                            &op,
                        )?;
                        ev.notifications.push(Notification::NewMsg(mid, cid));
                        ev.replies.push((cid, form_ack(mid)?));
                    }
                    cmessages::Message::Blob(_) => unimplemented!(),
                }
            }
            Ack(ack) => {
                crate::message::add_receipt(ack.of, from.uid, ack.stat)?;
                ev.notifications.push(Notification::MsgReceipt {
                    mid: ack.of,
                    cid: cid,
                });
            }
        }
    }

    Ok(ev)
}

fn handle_dmessage(_: DateTime<Utc>, msg: DeviceMessage) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let (from, msg) = msg.open()?;

    match msg {
        DeviceMessageBody::ContactReq(cr) => {
            let dmessages::ContactReq { gen, mut cid } = cr;
            if gen.verify_sig(&from.did) {
                crate::contact::ContactBuilder::new(from.uid)
                    .pairwise_conversation(cid)
                    .add()?;

                cid.store_genesis(gen)?;

                ev.notifications
                    .push(Notification::NewContact(from.uid, cid));

                ev.replies.push((
                    cid,
                    ConversationMessageBody::ContactReqAck(cmessages::ContactReqAck(true)),
                ))
            }
        }
    }

    Ok(ev)
}

fn send_cmessage(mut cid: ConversationId, content: &ConversationMessageBody) -> Result<(), HErr> {
    if CAUGHT_UP.load(Ordering::Acquire) {
        let cm = ConversationMessage::seal(cid, &content)?;
        let to = crate::members::members(&cid)?;
        let exc = *crate::config::Config::static_keypair()?.public_key();
        let msg = Bytes::from(serde_cbor::to_vec(&cm)?);
        let req = push_users::Req { to, exc, msg };
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

                // TODO: make chainmail API return hash on sealing so this won't be necessary
                let hash = cm
                    .body()
                    .compute_hash()
                    .expect("failed to compute block hash");

                cid.mark_used([hash].iter())?;

                pending::add_to_pending(cid, content)
            }
        }
    } else {
        pending::add_to_pending(cid, content)
    }
}

fn send_dmessage(to: sig::PublicKey, dm: &DeviceMessageBody) -> Result<(), HErr> {
    let msg = Bytes::from(serde_cbor::to_vec(&DeviceMessage::seal(&to, dm)?)?);

    let req = push_devices::Req { to: vec![to], msg };

    // TODO retry logic? for now, things go to the void
    match helper::push_devices(&req)? {
        push_devices::Res::Success => Ok(()),
        push_devices::Res::Missing(missing) => Err(HeraldError(format!(
            "tried to send messages to nonexistent keys {:?}",
            missing
        ))),
    }
}

fn send_umessage(uid: UserId, msg: &DeviceMessageBody) -> Result<(), HErr> {
    let meta = match keys_of(vec![uid])?.pop() {
        Some((u, m)) => {
            if u == uid {
                Ok(m)
            } else {
                Err(HErr::HeraldError(format!(
                    "Response returned keys not associated with uid {}\n\
                     failed at line {}",
                    uid,
                    line!()
                )))
            }
        }
        None => Err(HErr::HeraldError(format!(
            "No keys associated with {}\n\
             failed at line {}",
            uid,
            line!()
        ))),
    }?;

    let keys: Vec<sig::PublicKey> = meta.keys.into_iter().map(|(k, _)| k).collect();
    for key in keys {
        send_dmessage(key, msg)?;
    }

    Ok(())
}

/// Sends a contact request to `uid` with a proposed conversation id `cid`.
pub fn send_contact_req(uid: UserId, mut cid: ConversationId) -> Result<(), HErr> {
    let kp = Config::static_keypair()?;

    let gen = Genesis::new(kp.secret_key());

    cid.store_genesis(gen.clone())?;

    let req = dmessages::ContactReq { gen, cid };

    send_umessage(uid, &DeviceMessageBody::ContactReq(req))
}

/// Starts a conversation with `members`. Note: all members must be in the user's contacts already.
pub fn start_conversation(
    members: &[UserId],
    title: Option<String>,
) -> Result<ConversationId, HErr> {
    use crate::conversation;

    let pairwise = conversation::get_pairwise_conversations(members)?;

    let mut db = crate::db::Database::get()?;
    let tx = db.transaction()?;

    let mut conv_builder = conversation::ConversationBuilder::new();
    if let Some(title) = title.as_ref() {
        conv_builder.title(title.clone());
    }

    let mut cid = conv_builder.add_with_tx(&tx)?;

    crate::members::add_members_with_tx(&tx, cid, members)?;
    tx.commit()?;

    let kp = crate::config::Config::static_keypair()?;
    let gen = Genesis::new(kp.secret_key());
    cid.store_genesis(gen.clone())?;

    let body = ConversationMessageBody::AddedToConvo(cmessages::AddedToConvo {
        members: Vec::from(members),
        gen,
        cid,
        title: title.map(String::from),
    });

    for pw_cid in pairwise {
        send_cmessage(pw_cid, &body)?;
    }

    Ok(cid)
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
