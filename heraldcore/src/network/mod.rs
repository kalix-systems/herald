use crate::{
    chainkeys,
    config::Config,
    conversation::settings,
    errors::HErr::{self, *},
    message::MessageReceiptStatus,
    pending,
    types::*,
};
use chainmail::block::*;
use herald_common::*;
use lazy_static::*;
use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::atomic::{AtomicBool, Ordering},
};
use websocket::{message::OwnedMessage as WMessage, sync::client as wsclient};

mod statics;
use statics::*;

mod login_imp;
pub use login_imp::login;

mod message_handlers;
use message_handlers::*;

mod message_senders;
use message_senders::*;

mod event;
use event::*;

mod helper;

#[derive(Clone, Debug)]
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
    /// The conversation settings have been updated
    Settings(ConversationId, settings::SettingsUpdate),
}

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
    picture: Option<String>,
) -> Result<ConversationId, HErr> {
    use crate::conversation;

    let pairwise = conversation::get_pairwise_conversations(members)?;

    let mut db = crate::db::Database::get()?;
    let tx = db.transaction()?;

    let mut conv_builder = conversation::ConversationBuilder::new();

    if let Some(title) = title.as_ref() {
        conv_builder.title(title.clone());
    }

    if let Some(picture) = picture.as_ref() {
        conv_builder.picture(picture.clone());
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

pub(crate) fn send_normal_message(cid: ConversationId, msg: cmessages::Msg) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessageBody::Msg(msg))
}

pub(crate) fn send_conversation_settings_update(
    cid: ConversationId,
    update: settings::SettingsUpdate,
) -> Result<(), HErr> {
    send_cmessage(cid, &ConversationMessageBody::Settings(update))
}

pub(crate) fn server_url(ext: &str) -> String {
    format!("http://{}/{}", *SERVER_ADDR, ext)
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
