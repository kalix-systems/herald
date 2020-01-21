use crate::updates::Notification;
use crate::{errors::HErr, message::ReceiptStatus, pending, types::*, *};
use herald_common::*;
use herald_network as hn;
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicBool, Ordering},
};
use websocket::{message::OwnedMessage as WMessage, sync::client as wsclient};

mod requester;

mod statics;
use statics::*;

//mod login_imp;
//pub use login_imp::login;

//mod message_handlers;
//use message_handlers::*;

//mod message_senders;
//pub(crate) use message_senders::send_cmessage;
// pub use message_senders::SendOutcome;
//use message_senders::*;

/// Outcome of sending a `ConversationMessage`
pub enum SendOutcome {
    /// Message sent succesfully
    Success,
    /// Message was placed in pending
    Pending,
}
//mod event;
//use event::*;

//mod helper;

/// Deprecates key on server.
pub fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    todo!()
    //    let kp = config::keypair()?;
    //    let req = dep_key::Req(kp.sign(to_dep));
    //    Ok(helper::dep_key(&req)?.0)
}

/// Adds new key to the server's key registry.
pub fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    todo!()
    //let kp = config::keypair()?;
    //let req = new_key::Req(kp.sign(to_new));
    //Ok(helper::new_key(&req)?.0)
}

/// Registers new user on the server.
pub fn begin_registration((dns, port): (String, u16)) -> Result<hn::RegistrationHandle, HErr> {
    kcl::init();

    let kp = sig::KeyPair::gen_new();

    hn::register(
        kp,
        dns,
        port,
        handle_push,
        |_| todo!(),
        |e| push(Notification::ConnectionDown(e)),
    )
}

fn handle_push(
    Push {
        timestamp,
        msg,
        gid,
        ..
    }: Push
) {
    use crypto_store::prelude::{Msg as PLMsg, *};
    let payload: PLMsg = kson::from_bytes(msg).unwrap();
    let kp = crate::config::keypair().unwrap();
    let mut raw = raw_conn().lock();
    let mut tx: Conn = raw.transaction().unwrap().into();
    handle_incoming(&mut tx, &kp, gid, payload);
}

/// To be call this upon successful registration
pub fn finish_registration(handle: hn::RegistrationHandle) {
    requester::update(handle.into())
}

/// Runs login flow
pub fn login() -> Result<(), HErr> {
    let kp = crate::config::keypair()?;
    let uid = crate::config::id()?;
    let (dns, port) = crate::config::home_server()?;

    let handle = hn::login(uid, kp, dns, port, handle_push, |e| {
        push(Notification::ConnectionDown(e))
    })?;

    std::thread::Builder::new().spawn(move || {
        use crypto_store::prelude::*;
        let mut conn = raw_conn().lock();

        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(e) => {
                err(e);
                return;
            }
        };

        // TODO
        // - send pending somehow?
        requester::update(handle);
    })?;

    Ok(())
}

/// Sends a message read receipt
pub fn send_read_receipt(
    cid: ConversationId,
    msg_id: MsgId,
) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Receipt(cmessages::Receipt {
    //        of: msg_id,
    //        stat: ReceiptStatus::Read,
    //    })),
    //)?;
    //Ok(())
}

/// Sends a typing indicator
pub fn send_typing_indicator(cid: ConversationId) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Typing(Time::now())),
    //)?;

    //Ok(())
}

/// Sends a user request to `uid` with a proposed conversation id `cid`.
pub fn send_user_req(
    uid: UserId,
    cid: ConversationId,
) -> Result<(), HErr> {
    todo!()
    //let ratchet = RatchetState::new();
    //chainkeys::store_state(cid, &ratchet)?;

    //let req = dmessages::UserReq { ratchet, cid };

    //send_umessage(uid, &DeviceMessageBody::Req(req))
}

pub(crate) fn send_normal_message(
    cid: ConversationId,
    msg: cmessages::Msg,
) -> Result<SendOutcome, HErr> {
    todo!()
    //let mid = msg.mid;
    //let outcome = send_cmessage(cid, &ConversationMessage::Message(NetContent::Msg(msg)))?;

    //if let SendOutcome::Success = outcome {
    //    {
    //        let conn = crate::db::Database::get()?;
    //        crate::message::db::update_send_status(
    //            &conn,
    //            mid,
    //            coretypes::messages::SendStatus::Ack,
    //        )?;
    //    }
    //}

    //Ok(outcome)
}

pub(crate) fn send_group_settings_message(
    mid: MsgId,
    cid: ConversationId,
    expiration: Option<Time>,
    update: cmessages::GroupSettingsUpdate,
) -> Result<(), HErr> {
    if let SendOutcome::Success = send_normal_message(
        cid,
        cmessages::Msg {
            mid,
            expiration,
            content: cmessages::MsgContent::GroupSettings(update),
        },
    )? {
        crate::push(crate::message::OutboundAux::SendDone(cid, mid));
    }
    Ok(())
}

pub(crate) fn send_profile_update(update: cmessages::ProfileChanged) -> Result<(), HErr> {
    todo!()
    //let conn = crate::db::Database::get()?;

    //use cmessages::ProfileChanged as P;
    //match update {
    //    color @ P::Color(_) => {
    //        let cid = crate::config::db::nts_conversation(&conn)?;
    //        let msg = ConversationMessage::Message(NetContent::ProfileChanged(color));

    //        send_cmessage(cid, &msg)?;
    //    }
    //    other => {
    //        let cids = crate::conversation::db::get_all_pairwise_conversations(&conn)?;

    //        let msg = ConversationMessage::Message(NetContent::ProfileChanged(other));

    //        for cid in cids {
    //            send_cmessage(cid, &msg)?;
    //        }
    //    }
    //}

    //Ok(())
}

/// Sends a reaction
pub fn send_reaction(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
    //        msg_id,
    //        react_content,
    //        remove: false,
    //    })),
    //)?;
    //Ok(())
}

/// Sends a reaction removal update
pub fn send_reaction_removal(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
    //        msg_id,
    //        react_content,
    //        remove: true,
    //    })),
    //)?;
    //Ok(())
}

macro_rules! get_of_helper {
    ($name: tt, $of: ty, $to: ty) => {
        #[allow(missing_docs)]
        pub fn $name(of: $of) -> Result<$to, HErr> {
            Ok(helper::$name(&$name::Req(of))?.0)
        }
    };
}

//get_of_helper!(keys_of, Vec<UserId>, Vec<(UserId, UserMeta)>);
//get_of_helper!(
//    key_info,
//    Vec<sig::PublicKey>,
//    Vec<(sig::PublicKey, sig::PKMeta)>
//);
//get_of_helper!(keys_exist, Vec<sig::PublicKey>, Vec<bool>);
//get_of_helper!(users_exist, Vec<UserId>, Vec<bool>);
