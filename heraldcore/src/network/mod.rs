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
        w!(helper::push(&push::Req {
            from: gid,
            to: Recip::Many(Recips::Users(users)),
            msg: kson::to_vec(&as_msg).into(),
        }));
    }

    w!(store.commit());

    Ok(res)
}

// /// Adds new key to the server's key registry.
// pub fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
//     let kp = w!(config::keypair());
//     let req = new_key::Req(sign_ser(&kp, to_new));
//     Ok(w!(helper::new_key(&req)).0)
// }

// /// Registers new user on the server.
// pub fn register(
//     uid: UserId,
//     home_server: Option<SocketAddr>,
// ) -> Result<register::Res, HErr> {
//     kcl::init();

//     let home_server = home_server.unwrap_or_else(|| *default_server());

//     let kp = sig::KeyPair::gen_new();
//     let sig = sign_ser(&kp, *kp.public());
//     let req = register::Req(uid, sig);

//     let res = w!(helper::register(&req, home_server));

//     // TODO: retry if this fails?
//     if let register::Res::Success = &res {
//         w!(crate::config::ConfigBuilder::new(uid, kp)
//             .home_server(home_server)
//             .add());
//     }

//     Ok(res)
// }

// /// Sends a message read receipt
// pub fn send_read_receipt(
//     cid: ConversationId,
//     msg_id: MsgId,
// ) -> Result<(), HErr> {
//     w!(send_cmessage(
//         cid,
//         &ConversationMessage::Message(NetContent::Receipt(cmessages::Receipt {
//             of: msg_id,
//             stat: ReceiptStatus::Read,
//         })),
//     ));
//     Ok(())
// }

// /// Sends a typing indicator
// pub fn send_typing_indicator(cid: ConversationId) -> Result<(), HErr> {
//     w!(send_cmessage(
//         cid,
//         &ConversationMessage::Message(NetContent::Typing(Time::now())),
//     ));

//     Ok(())
// }

// /// Sends a user request to `uid` with a proposed conversation id `cid`.
// pub fn send_user_req(
//     uid: UserId,
//     cid: ConversationId,
// ) -> Result<(), HErr> {
//     let ratchet = RatchetState::new();
//     w!(chainkeys::store_state(cid, &ratchet));

//     let req = dmessages::UserReq { cid };

//     send_umessage(uid, &DeviceMessageBody::Req(req))
// }

// pub(crate) fn send_normal_message(
//     cid: ConversationId,
//     msg: cmessages::Msg,
// ) -> Result<SendOutcome, HErr> {
//     let mid = msg.mid;
//     let outcome = w!(send_cmessage(
//         cid,
//         &ConversationMessage::Message(NetContent::Msg(msg))
//     ));

//     if let SendOutcome::Success = outcome {
//         {
//             let conn = w!(crate::db::Database::get());
//             w!(crate::message::db::update_send_status(
//                 &conn,
//                 mid,
//                 coretypes::messages::SendStatus::Ack,
//             ));
//         }
//     }

//     Ok(outcome)
// }

// pub(crate) fn send_group_settings_message(
//     mid: MsgId,
//     cid: ConversationId,
//     expiration: Option<Time>,
//     update: cmessages::GroupSettingsUpdate,
// ) -> Result<(), HErr> {
//     if let SendOutcome::Success = w!(send_normal_message(
//         cid,
//         cmessages::Msg {
//             mid,
//             expiration,
//             content: cmessages::MsgContent::GroupSettings(update),
//         },
//     )) {
//         crate::push(crate::message::OutboundAux::SendDone(cid, mid));
//     }
//     Ok(())
// }

// pub(crate) fn send_profile_update(update: cmessages::ProfileChanged) -> Result<(), HErr> {
//     let conn = w!(crate::db::Database::get());

//     use cmessages::ProfileChanged as P;
//     match update {
//         color @ P::Color(_) => {
//             let cid = w!(crate::config::db::nts_conversation(&conn));
//             let msg = ConversationMessage::Message(NetContent::ProfileChanged(color));

//             w!(send_cmessage(cid, &msg));
//         }
//         other => {
//             let cids = w!(crate::conversation::db::get_all_pairwise_conversations(
//                 &conn
//             ));

//             let msg = ConversationMessage::Message(NetContent::ProfileChanged(other));

//             for cid in cids {
//                 w!(send_cmessage(cid, &msg));
//             }
//         }
//     }

//     Ok(())
// }

// /// Sends a reaction
// pub fn send_reaction(
//     cid: ConversationId,
//     msg_id: MsgId,
//     react_content: crate::message::ReactContent,
// ) -> Result<(), HErr> {
//     w!(send_cmessage(
//         cid,
//         &ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
//             msg_id,
//             react_content,
//             remove: false,
//         })),
//     ));
//     Ok(())
// }

// /// Sends a reaction removal update
// pub fn send_reaction_removal(
//     cid: ConversationId,
//     msg_id: MsgId,
//     react_content: crate::message::ReactContent,
// ) -> Result<(), HErr> {
//     w!(send_cmessage(
//         cid,
//         &ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
//             msg_id,
//             react_content,
//             remove: true,
//         })),
//     ));
//     Ok(())
// }

pub(crate) fn server_url(ext: &str) -> String {
    format!("http://{}/{}", home_server(), ext)
}

// macro_rules! get_of_helper {
//     ($name: tt, $of: ty, $to: ty) => {
//         #[allow(missing_docs)]
//         pub fn $name(of: $of) -> Result<$to, HErr> {
//             Ok(w!(helper::$name(&$name::Req(of))).0)
//         }
//     };
// }

// get_of_helper!(keys_of, Vec<UserId>, Vec<(UserId, UserMeta)>);
// get_of_helper!(
//     key_info,
//     Vec<sig::PublicKey>,
//     Vec<(sig::PublicKey, sig::PKMeta)>
// );
// get_of_helper!(keys_exist, Vec<sig::PublicKey>, Vec<bool>);
// get_of_helper!(users_exist, Vec<UserId>, Vec<bool>);
