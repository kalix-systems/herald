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
use std::sync::atomic::Ordering;

mod login_imp;
pub use login_imp::login;

mod statics;
use statics::*;

mod types;
pub use types::*;

mod rpc;
// use rpc::*;

mod helper {
    use crate::errors::*;
    use herald_common::*;

    macro_rules! mk_request {
        ($path: tt) => {
            pub async fn $path(req: $path::Req) -> Result<$path::Res, HErr> {
                let mut client = super::get_client().await?;
                let res = client.$path(tarpc::context::current(), req).await??;
                Ok(res)
            }
        };
    }

    mk_request!(keys_of);
    mk_request!(key_info);
    mk_request!(keys_exist);
    mk_request!(users_exist);
    mk_request!(register);
    mk_request!(new_key);
    mk_request!(dep_key);
    mk_request!(push_users);
    mk_request!(push_devices);
}

macro_rules! get_of_helper {
    ($name: tt, $of: ty, $to: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(of: $of) -> Result<$to, HErr> {
            Ok(helper::$name($name::Req(of)).await?.0)
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
pub async fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = dep_key::Req(kp.sign(to_dep));
    Ok(helper::dep_key(req).await?.0)
}

/// Adds new key to the server's key registry.
pub async fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = Config::static_keypair()?;
    let req = new_key::Req(kp.sign(to_new));
    Ok(helper::new_key(req).await?.0)
}

/// Registers new user on the server.
pub async fn register(uid: UserId) -> Result<register::Res, HErr> {
    let kp = sig::KeyPair::gen_new();
    let sig = kp.sign(*kp.public_key());
    let req = register::Req(uid, sig);
    let res = helper::register(req).await?;
    // TODO: retry if this fails?
    if let register::Res::Success = &res {
        crate::config::ConfigBuilder::new(uid, kp).add()?;
    }

    Ok(res)
}

pub(crate) async fn send_normal_message(
    cid: ConversationId,
    msg: cmessages::Msg,
) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessageBody::Msg(msg)).await
}

pub(crate) async fn send_conversation_settings_update(
    cid: ConversationId,
    update: settings::SettingsUpdate,
) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessageBody::Settings(update)).await
}

fn bare_push_users() {
    tokio::spawn(async {
        let res = helper::push_users(unimplemented!()).await;
    });
}

async fn send_cmessage(cid: ConversationId, content: &ConversationMessageBody) -> Result<(), HErr> {
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
        drop(tx);
        drop(db);

        // let res = helper::push_users(req).await;
        unimplemented!()
    // match helper::push_users(req).await {
    //     Ok(r) => {
    //         unimplemented!()
    //         // match r {
    //         // push_users::Res::Success => {
    //         //     // tx.commit()?;
    //         //     Ok(())
    //         // }
    //         // push_users::Res::Missing(missing) => Err(HeraldError(format!(
    //         //     "tried to send messages to nonexistent users {:?}",
    //         //     missing
    //         // ))),
    //     }
    //     Err(e) => {
    //         unimplemented!()
    //         // chainkeys::mark_used(&mut tx, cid, [hash].iter())?;
    //         // tx.commit()?;

    //         // TODO: maybe try more than once?
    //         // maybe have some mechanism to send a signal that more things have gone wrong?
    //         // eprintln!(
    //         //     "failed to send message, error was {}\n\
    //         //      assuming failed session and adding to pending now",
    //         //     e
    //         // );

    //         // CAUGHT_UP.store(false, Ordering::Release);

    //         // pending::add_to_pending(cid, content)
    //     }
    // }
    } else {
        // pending::add_to_pending(cid, content)
        Ok(())
    }
}

async fn send_dmessage(to: sig::PublicKey, dm: &DeviceMessageBody) -> Result<(), HErr> {
    let msg = Bytes::from(serde_cbor::to_vec(&DeviceMessage::seal(&to, dm)?)?);

    let req = push_devices::Req { to: vec![to], msg };

    // TODO retry logic? for now, things go to the void
    match helper::push_devices(req).await? {
        push_devices::Res::Success => Ok(()),
        push_devices::Res::Missing(missing) => Err(HeraldError(format!(
            "tried to send messages to nonexistent keys {:?}",
            missing
        ))),
    }
}

async fn send_umessage(uid: UserId, msg: &DeviceMessageBody) -> Result<(), HErr> {
    let meta = match keys_of(vec![uid]).await?.pop() {
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
        send_dmessage(key, msg).await?;
    }

    Ok(())
}

/// Sends a contact request to `uid` with a proposed conversation id `cid`.
pub async fn send_contact_req(uid: UserId, cid: ConversationId) -> Result<(), HErr> {
    let kp = Config::static_keypair()?;

    let gen = Genesis::new(kp.secret_key());

    cid.store_genesis(&gen)?;

    let req = dmessages::ContactReq { gen, cid };

    send_umessage(uid, &DeviceMessageBody::ContactReq(req)).await
}

/// Starts a conversation with `members`. Note: all members must be in the user's contacts already.
pub async fn start_conversation(
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
        send_cmessage(pw_cid, &body).await?;
    }

    Ok(cid)
}

fn form_ack(mid: MsgId) -> Result<ConversationMessageBody, HErr> {
    Ok(ConversationMessageBody::Ack(cmessages::Ack {
        of: mid,
        stat: MessageReceiptStatus::Received,
    }))
}