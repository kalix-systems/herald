use crate::{
    ffi,
    imp::{
        config::Config, conversation_builder::ConversationBuilder, conversations::Conversations,
        errors::Errors, message_search::MessageSearch, users::Users, users_search::UsersSearch,
        utils::Utils,
    },
    interface::*,
    push_err, ret_err,
    shared::{AddressedBus, SingletonBus},
    spawn,
};
use herald_common::*;
use heraldcore::{
    config, db,
    message::gc,
    network::{self as net, Notification},
};
use std::{
    convert::TryFrom,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

type Emitter = HeraldEmitter;

mod network;
use network::*;
mod imp;

/// Application state
pub struct Herald {
    config_init: Arc<AtomicBool>,
    emit: HeraldEmitter,
    effects_flags: Arc<EffectsFlags>,
    message_search: MessageSearch,
    config: Config,
    conversation_builder: ConversationBuilder,
    conversations: Conversations,
    errors: Errors,
    users: Users,
    users_search: UsersSearch,
    utils: Utils,
}

macro_rules! props {
    ($( $field: ident, $mut: ident, $ret: ty),*) => {
       $(
       fn $field(&self) -> &$ret {
            &self.$field
       }

       fn $mut(&mut self) -> &mut $ret {
            &mut self.$field
       }
       )*
    }
}

impl HeraldTrait for Herald {
    fn new(
        emit: HeraldEmitter,
        mut config: Config,
        conversation_builder: ConversationBuilder,
        conversations: Conversations,
        errors: Errors,
        message_search: MessageSearch,
        users: Users,
        users_search: UsersSearch,
        utils: Utils,
    ) -> Self {
        let config_init = if config::id().is_ok() {
            // If this fails, it's because a thread couldn't be spawned.
            // This implies the OS is in a very bad place.
            push_err!(
                gc::init(move |update| {
                    imp::gc_handler(update);
                }),
                "Couldn't start GC thread"
            );

            push_err!(config.try_load(), "Couldn't load Config");

            Arc::new(AtomicBool::new(true))
        } else {
            // If this fails, the file system is in a very bad place.
            // This probably cannot be recovered from, and there's not meaningful
            // sense in which the application can work.
            push_err!(db::init(), "Couldn't initialize storage");

            Arc::new(AtomicBool::new(false))
        };

        Herald {
            emit,
            config_init,
            effects_flags: Arc::new(EffectsFlags::new()),
            message_search,
            config,
            conversation_builder,
            conversations,
            errors,
            users,
            users_search,
            utils,
        }
    }

    fn config_init(&self) -> bool {
        self.config.loaded()
    }

    fn register_new_user(
        &mut self,
        user_id: ffi::UserId,
    ) {
        use register::*;

        let uid = ret_err!(UserId::try_from(user_id.as_str()));

        let config_init = self.config_init.clone();
        let mut emit = self.emit.clone();

        spawn!(match ret_err!(net::register(uid)) {
            Res::UIDTaken => {
                eprintln!("UID taken!");
            }
            Res::KeyTaken => {
                eprintln!("Key taken!");
            }
            Res::BadSig(s) => {
                eprintln!("Bad sig: {:?}", s);
            }
            Res::Success => {
                config_init.fetch_xor(true, Ordering::Acquire);
                // If this fails, it's because a thread couldn't be spawned.
                // This implies the OS is in a very bad place.
                push_err!(
                    gc::init(move |update| {
                        imp::gc_handler(update);
                    }),
                    "Couldn't start GC thread"
                );

                emit.config_init_changed();
            }
        });
    }

    fn login(&mut self) -> bool {
        use heraldcore::errors::HErr;

        let mut handler = NotifHandler::new(self.emit.clone(), self.effects_flags.clone());

        spawn!(
            ret_err!(net::login(
                move |notif: Notification| {
                    handler.send(notif);
                },
                move |herr: HErr| {
                    ret_err!(Err::<(), HErr>(herr));
                }
            )),
            false
        );
        true
    }

    fn connection_up(&self) -> bool {
        self.effects_flags.net_online.load(Ordering::Relaxed)
    }

    fn connection_pending(&self) -> bool {
        self.effects_flags.net_pending.load(Ordering::Relaxed)
    }

    fn emit(&mut self) -> &mut HeraldEmitter {
        &mut self.emit
    }

    props! {
        config, config_mut, Config,
        conversation_builder, conversation_builder_mut, ConversationBuilder,
        conversations, conversations_mut, Conversations,
        errors, errors_mut, Errors,
        message_search, message_search_mut, MessageSearch,
        users, users_mut, Users,
        users_search, users_search_mut, UsersSearch,
        utils, utils_mut, Utils
    }
}
