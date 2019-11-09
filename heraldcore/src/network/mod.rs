use crate::{
    chainkeys,
    config::Config,
    conversation::settings,
    errors::HErr::{self, *},
    pending,
    types::*,
};
use chainmail::block::*;
use herald_common::*;
use lazy_static::*;
use std::sync::atomic::Ordering;
use websocket::{message::OwnedMessage as WMessage, sync::client as wsclient};

mod login_imp;
pub use login_imp::login;

mod statics;
use statics::*;

mod types;
pub use types::*;

mod rpc;
// use rpc::*;

pub(crate) fn server_url(ext: &str) -> String {
    format!("http://{}/{}", *SERVER_ADDR, ext)
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
    let req = register::Req(uid, sig);
    let res = helper::register(&req)?;
    // TODO: retry if this fails?
    if let register::Res::Success = &res {
        crate::config::ConfigBuilder::new(uid, kp).add()?;
    }

    Ok(res)
}

pub(crate) fn send_normal_message(cid: ConversationId, msg: cmessages::Msg) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessageBody::Msg(msg))
}

pub(crate) fn send_conversation_settings_update(
    cid: ConversationId,
    update: settings::SettingsUpdate,
) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessageBody::Settings(update))
}

fn send_cmessage(cid: ConversationId, content: &ConversationMessageBody) -> Result<(), HErr> {
    if CAUGHT_UP.load(Ordering::Acquire) {
        let (cm, hash, key) = ConversationMessage::seal(cid, &content)?;

        let to = crate::members::members(&cid)?;
        let exc = *crate::config::Config::static_keypair()?.public_key();
        let msg = Bytes::from(serde_cbor::to_vec(&cm)?);
        let req = push_users::Req { to, exc, msg };

        let mut db = chainkeys::CK_CONN.lock();
        let mut tx = db.transaction()?;
        let unlocked = chainkeys::store_key(&mut tx, cid, hash, &key)?;
        debug_assert!(unlocked.is_empty());
        // TODO: replace used with probably_used here
        // in general we probably want a slightly smarter system for dealing with scenarios where
        // we thought a message wasn't sent but it was
        chainkeys::mark_used(&mut tx, cid, cm.body().parent_hashes().iter())?;

        match helper::push_users(&req) {
            Ok(push_users::Res::Success) => {
                tx.commit()?;
                Ok(())
            }
            Ok(push_users::Res::Missing(missing)) => Err(HeraldError(format!(
                "tried to send messages to nonexistent users {:?}",
                missing
            ))),
            Err(e) => {
                chainkeys::mark_used(&mut tx, cid, [hash].iter())?;
                tx.commit()?;

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
pub fn send_contact_req(uid: UserId, cid: ConversationId) -> Result<(), HErr> {
    let kp = Config::static_keypair()?;

    let gen = Genesis::new(kp.secret_key());

    cid.store_genesis(&gen)?;

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

    let cid = conv_builder.add_with_tx(&tx)?;

    crate::members::db::add_members_with_tx(&tx, cid, members)?;
    tx.commit()?;

    let kp = crate::config::Config::static_keypair()?;
    let gen = Genesis::new(kp.secret_key());
    cid.store_genesis(&gen)?;

    let body = ConversationMessageBody::AddedToConvo(Box::new(cmessages::AddedToConvo {
        members: Vec::from(members),
        gen,
        cid,
        title: title.map(String::from),
    }));

    for pw_cid in pairwise {
        send_cmessage(pw_cid, &body)?;
    }

    Ok(cid)
}

fn form_ack(mid: MsgId) -> Result<ConversationMessageBody, HErr> {
    Ok(ConversationMessageBody::Ack(cmessages::Ack {
        of: mid,
        stat: MessageReceiptStatus::Received,
    }))
}
