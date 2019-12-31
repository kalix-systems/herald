use crate::{
    config::Config, conversation_builder::ConversationBuilder, conversations::Conversations, err,
    errors::Errors, ffi, interface::*, message_search::MessageSearch, push_err, spawn,
    users::Users, users_search::UsersSearch, Loadable,
};
use herald_common::*;
use heraldcore::{
    config, db,
    message::gc,
    network::{self as net},
    updates::Notification,
};
use std::convert::TryFrom;

type Emitter = HeraldEmitter;

mod imp;
mod notif_handler;
mod shared;
mod trait_imp;
pub(crate) mod utils;
use self::utils::Utils;
pub(crate) use shared::{push, Update};
pub(crate) mod underscore;

/// Application state
pub struct Herald {
    emit: HeraldEmitter,
    errors: Errors,
    message_search: MessageSearch,
    users_search: UsersSearch,
    utils: Utils,
    load_props: imp::LoadProps,

    registration_failure_code: Option<shared::RegistrationFailureCode>,
}
