use crate::{
    conversation::settings,
    errors::HErr::{self, *},
    message::MessageReceiptStatus,
    pending,
    types::*,
    *,
};
use chainkeys;
use channel_ratchet::RatchetState;
use coretypes::conversation::ConversationMeta;
use herald_common::*;
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
pub(crate) use message_senders::send_cmessage;
use message_senders::*;

mod event;
use event::*;

mod helper;

#[derive(Clone, Debug)]
/// `Notification`s contain info about what updates were made to the database.
pub enum Notification {
    /// A new message has been received.
    NewMsg(Box<message::Message>),
    /// A message has been received.
    MsgReceipt(message::MessageReceipt),
    /// A new user has been added
    NewUser(Box<(coretypes::user::User, ConversationMeta)>),
    /// A new conversation has been added
    NewConversation(ConversationMeta),
    /// Response to user request.
    AddUserResponse(ConversationId, UserId, bool),
    /// Response to request to join conversation.
    AddConversationResponse(ConversationId, UserId, bool),
    /// The conversation settings have been updated
    Settings(ConversationId, settings::SettingsUpdate),
}

/// Deprecates key on server.
pub fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = config::keypair()?;
    let req = dep_key::Req(kp.sign(to_dep));
    Ok(helper::dep_key(&req)?.0)
}

/// Adds new key to the server's key registry.
pub fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    let kp = config::keypair()?;
    let req = new_key::Req(kp.sign(to_new));
    Ok(helper::new_key(&req)?.0)
}

/// Registers new user on the server.
pub fn register(
    uid: UserId,
    home_server: Option<SocketAddr>,
) -> Result<register::Res, HErr> {
    kcl::init();

    let home_server = home_server.unwrap_or_else(|| *default_server());

    let kp = sig::KeyPair::gen_new();
    let sig = kp.sign(*kp.public_key());
    let req = register::Req(uid, sig);

    let res = helper::register(&req, home_server)?;

    // TODO: retry if this fails?
    if let register::Res::Success = &res {
        crate::config::ConfigBuilder::new(uid, kp)
            .home_server(home_server)
            .add()?;
    }

    Ok(res)
}

/// Sends a message read receipt
pub fn send_read_receipt(
    cid: ConversationId,
    msg_id: MsgId,
) -> Result<(), HErr> {
    send_cmessage(
        cid,
        &ConversationMessage::Receipt(cmessages::Receipt {
            of: msg_id,
            stat: MessageReceiptStatus::Read,
        }),
    )
}

/// Sends a user request to `uid` with a proposed conversation id `cid`.
pub fn send_user_req(
    uid: UserId,
    cid: ConversationId,
) -> Result<(), HErr> {
    let ratchet = RatchetState::new();
    chainkeys::store_state(cid, &ratchet)?;

    let req = dmessages::UserReq { ratchet, cid };

    send_umessage(uid, &DeviceMessageBody::Req(req))
}

pub(crate) fn send_normal_message(
    cid: ConversationId,
    msg: cmessages::Msg,
) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessage::Msg(msg))
}

/// Sends a reaction
pub fn send_reaction(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    send_cmessage(
        cid,
        &ConversationMessage::Reaction(cmessages::Reaction {
            msg_id,
            react_content,
        }),
    )
}

pub(crate) fn send_conversation_settings_update(
    cid: ConversationId,
    update: settings::SettingsUpdate,
) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessage::Settings(update))
}

pub(crate) fn server_url(ext: &str) -> String {
    format!("http://{}/{}", home_server(), ext)
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
