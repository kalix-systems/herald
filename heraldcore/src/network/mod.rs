use super::*;
use crate::updates::Notification;
use crate::{
    errors::HErr::{self, *},
    message::ReceiptStatus,
    pending,
    types::*,
    *,
};
use coremacros::w;
use herald_common::sig::sign_ser;
use herald_common::*;
use ratchet_chat::protocol::{self as proto, SigStore};
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicBool, Ordering},
};
use websocket::{message::OwnedMessage as WMessage, sync::client as wsclient};

mod statics;
pub(crate) use statics::default_server;
use statics::*;

mod login_imp;
pub use login_imp::login;

mod message_handlers;
use message_handlers::*;

mod message_senders;
// pub(crate) use message_senders::send_cmessage;
pub use message_senders::SendOutcome;
use message_senders::*;

mod event;
use event::*;

mod helper;

#[macro_export]
macro_rules! get_crypto_conn {
    ($store:ident) => {
        let raw = crypto_store::prelude::raw_conn();
        let mut lock = raw.lock();
        let mut $store = w!(crypto_store::prelude::as_conn(&mut lock));
    };
    ($lock:ident, $store:ident) => {
        let raw = crypto_store::prelude::raw_conn();
        let mut $lock = raw.lock();
        let mut $store = w!(crypto_store::prelude::as_conn(&mut $lock));
    };
}

/// Deprecates key on server.
pub fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    get_crypto_conn!(store);

    let kp = w!(config::keypair());
    let gid = w!(config::gid());
    let update = sign_ser(&kp, sig::SigUpdate::Deprecate(to_dep));

    let res = w!(helper::new_sig(&Box::new(update)));

    if res == PKIResponse::Success {
        store.extend_sigchain(gid.uid, update.clone());

        let as_msg = proto::Msg::SigUpdate(update);
        let users: Vec<UserId> = w!(store.get_all_users());

        w!(store.commit());

        w!(helper::push(&push::Req {
            from: gid,
            to: Recip::Many(Recips::Users(users)),
            msg: kson::to_vec(&as_msg).into(),
        }));
    }

    Ok(res)
}

/// Adds new key to the server's key registry.
pub fn new_key(to_new: Signed<UserId>) -> Result<PKIResponse, HErr> {
    get_crypto_conn!(store);

    let kp = w!(config::keypair());
    let gid = w!(config::gid());
    let update = sign_ser(&kp, sig::SigUpdate::Endorse(to_new));

    let res = w!(helper::new_sig(&Box::new(update)));

    if res == PKIResponse::Success {
        store.extend_sigchain(gid.uid, update.clone());

        let as_msg = proto::Msg::SigUpdate(update);
        let users: Vec<UserId> = w!(store.get_all_users());

        w!(store.commit());

        w!(helper::push(&push::Req {
            from: gid,
            to: Recip::Many(Recips::Users(users)),
            msg: kson::to_vec(&as_msg).into(),
        }));
    }

    Ok(res)
}

/// Registers new user on the server.
pub fn register(
    uid: UserId,
    home_server: Option<SocketAddr>,
) -> Result<protocol::auth::RegisterResponse, HErr> {
    use protocol::auth::RegisterResponse;

    kcl::init();

    let home_server = home_server.unwrap_or_else(|| *default_server());

    let kp = sig::KeyPair::gen_new();

    let sig = sign_ser(&kp, uid);

    get_crypto_conn!(store);

    let res = w!(helper::register(&sig, home_server));

    // TODO: retry if this fails?
    if res == RegisterResponse::Success {
        w!(store.start_sigchain(sig));
        w!(crate::config::ConfigBuilder::new(uid, kp)
            .home_server(home_server)
            .add());
    }

    Ok(res)
}

/// Sends a message read receipt
pub fn send_read_receipt(
    cid: ConversationId,
    msg_id: MsgId,
) -> Result<(), HErr> {
    w!(send_cmessage(
        cid,
        ConversationMessage::Message(NetContent::Receipt(cmessages::Receipt {
            of: msg_id,
            stat: ReceiptStatus::Read,
        })),
    ));
    Ok(())
}

/// Sends a typing indicator
pub fn send_typing_indicator(cid: ConversationId) -> Result<(), HErr> {
    w!(send_cmessage(
        cid,
        ConversationMessage::Message(NetContent::Typing(Time::now())),
    ));

    Ok(())
}

/// Sends a user request to `uid` with a proposed conversation id `cid`.
pub fn send_user_req(
    uid: UserId,
    cid: ConversationId,
) -> Result<(), HErr> {
    let req = network_types::umessages::UserReq { cid };

    let chain = w!(w!(helper::get_sigchain(&uid)).ok_or(HeraldError("missing user".into())));
    let valid = chain.validate();
    if valid != SigValid::Yes {
        return Err(HeraldError("bad sigchain found on server".into()));
    }
    let sig::SigChain { initial, sig_chain } = chain;

    get_crypto_conn!(lock, store);
    w!(store.start_sigchain(initial));
    for link in sig_chain {
        w!(store.extend_sigchain(uid, link));
    }
    w!(store.commit());
    drop(lock);

    w!(send_umessage(uid, UserMessage::Req(req)));

    Ok(())
}

pub(crate) fn send_normal_message(
    cid: ConversationId,
    msg: cmessages::Msg,
) -> Result<SendOutcome, HErr> {
    let mid = msg.mid;
    let outcome = w!(send_cmessage(
        cid,
        ConversationMessage::Message(NetContent::Msg(msg))
    ));

    if let SendOutcome::Success = outcome {
        {
            let conn = w!(crate::db::Database::get());
            w!(crate::message::db::update_send_status(
                &conn,
                mid,
                coretypes::messages::SendStatus::Ack,
            ));
        }
    }

    Ok(outcome)
}

pub(crate) fn send_group_settings_message(
    mid: MsgId,
    cid: ConversationId,
    expiration: Option<Time>,
    update: cmessages::GroupSettingsUpdate,
) -> Result<(), HErr> {
    if let SendOutcome::Success = w!(send_normal_message(
        cid,
        cmessages::Msg {
            mid,
            expiration,
            content: cmessages::MsgContent::GroupSettings(update),
        },
    )) {
        crate::push(crate::message::OutboundAux::SendDone(cid, mid));
    }
    Ok(())
}

// TODO: send this to all users instead
pub(crate) fn send_profile_update(update: cmessages::ProfileChanged) -> Result<(), HErr> {
    let conn = w!(crate::db::Database::get());

    use cmessages::ProfileChanged as P;
    match update {
        color @ P::Color(_) => {
            let cid = w!(crate::config::db::nts_conversation(&conn));
            let msg = ConversationMessage::Message(NetContent::ProfileChanged(color));

            w!(send_cmessage(cid, msg.clone()));
        }
        other => {
            let cids = w!(crate::conversation::db::get_all_pairwise_conversations(
                &conn
            ));

            let msg = ConversationMessage::Message(NetContent::ProfileChanged(other));

            for cid in cids {
                w!(send_cmessage(cid, msg.clone()));
            }
        }
    }

    Ok(())
}

/// Sends a reaction
pub fn send_reaction(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    w!(send_cmessage(
        cid,
        ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
            msg_id,
            react_content,
            remove: false,
        })),
    ));
    Ok(())
}

/// Sends a reaction removal update
pub fn send_reaction_removal(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    w!(send_cmessage(
        cid,
        ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
            msg_id,
            react_content,
            remove: true,
        })),
    ));
    Ok(())
}

pub fn run_action(act: NetworkAction) -> Result<(), HErr> {
    match act {
        NetworkAction::UpdateProfile(pc) => {
            w!(send_profile_update(pc));
        }
        NetworkAction::StartConvo(convo) => {
            let convos = w!(crate::conversation::get_pairwise_conversations(
                &convo.members
            ));

            for cid in convos {
                w!(send_cmessage(
                    cid,
                    ConversationMessage::AddedToConvo {
                        info: convo.clone()
                    }
                ));
            }
        }
        NetworkAction::UpdateSettings {
            mid,
            cid,
            expiration,
            update,
        } => {
            w!(send_group_settings_message(mid, cid, expiration, update));
        }
    }

    Ok(())
}

pub(crate) fn server_url(ext: &str) -> String {
    format!("http://{}/{}", home_server(), ext)
}
