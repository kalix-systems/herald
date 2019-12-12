use crate::{
    config::Config, conversation_builder::ConversationBuilder, conversations::Conversations, err,
    errors::Errors, ffi, interface::*, message_search::MessageSearch, push_err, spawn,
    users::Users, users_search::UsersSearch, Loadable,
};
use herald_common::*;
use heraldcore::{
    config, db,
    message::gc,
    network::{self as net, Notification},
};
use std::{
    convert::TryFrom,
    sync::{atomic::Ordering, Arc},
};

type Emitter = HeraldEmitter;

mod network;
use network::*;
mod imp;
mod shared;
mod trait_imp;
pub(crate) mod utils;
use self::utils::Utils;
pub(crate) use shared::{push, Update};
pub(crate) mod underscore;

/// Application state
pub struct Herald {
    emit: HeraldEmitter,
    effects_flags: Arc<EffectsFlags>,
    errors: Errors,
    message_search: MessageSearch,
    users_search: UsersSearch,
    utils: Utils,
    load_props: imp::LoadProps,
}
