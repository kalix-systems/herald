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
    spawn, Loadable,
};
use crossbeam_channel::{Receiver, Sender};
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
use imp::LoadProps;
mod shared;

/// Application state
pub struct Herald {
    emit: HeraldEmitter,
    effects_flags: Arc<EffectsFlags>,
    message_search: MessageSearch,
    errors: Errors,
    users_search: UsersSearch,
    utils: Utils,
    load_props: LoadProps,
    rx: Receiver<shared::Update>,
    tx: Sender<shared::Update>,
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

macro_rules! load_props {
    ($( $field: ident, $mut: ident, $ret: ty),*) => {
       $(
       fn $field(&self) -> &$ret {
            &self.load_props.$field
       }

       fn $mut(&mut self) -> &mut $ret {
            &mut self.load_props.$field
       }
       )*
    }
}

impl HeraldTrait for Herald {
    fn new(
        emit: HeraldEmitter,
        _: HeraldList,
        config: Config,
        conversation_builder: ConversationBuilder,
        conversations: Conversations,
        errors: Errors,
        message_search: MessageSearch,
        users: Users,
        users_search: UsersSearch,
        utils: Utils,
    ) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut herald = Herald {
            emit,
            effects_flags: Arc::new(EffectsFlags::new()),
            message_search,
            load_props: LoadProps {
                config,
                conversation_builder,
                conversations,
                users,
            },
            errors,
            users_search,
            utils,
            tx,
            rx,
        };

        if config::id().is_ok() {
            herald.load_props.setup();
        } else {
            // If this fails, the file system is in a very bad place.
            // This probably cannot be recovered from, and there's not meaningful
            // sense in which the application can work.
            push_err!(db::init(), "Couldn't initialize storage");
        };

        herald
    }

    fn config_init(&self) -> bool {
        self.load_props.config.loaded()
    }

    fn register_new_user(
        &mut self,
        user_id: ffi::UserId,
    ) {
        use register::*;

        let uid = ret_err!(UserId::try_from(user_id.as_str()));

        let mut emit = self.emit.clone();
        let tx = self.tx.clone();

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
                ret_err!(tx.send(shared::Update::RegistrationSuccess));
                emit.new_data_ready();
            }
        });
    }

    fn can_fetch_more(&self) -> bool {
        !self.rx.is_empty()
    }

    fn fetch_more(&mut self) {
        use shared::Update::*;

        for update in self.rx.try_iter() {
            match update {
                RegistrationSuccess => {
                    self.load_props.setup();
                    self.emit.config_init_changed();
                }
            }
        }
    }

    fn login(&mut self) -> bool {
        self.login_()
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

    load_props! {
        config, config_mut, Config,
        conversation_builder, conversation_builder_mut, ConversationBuilder,
        conversations, conversations_mut, Conversations,
        users, users_mut, Users
    }

    props! {
        errors, errors_mut, Errors,
        message_search, message_search_mut, MessageSearch,
        users_search, users_search_mut, UsersSearch,
        utils, utils_mut, Utils
    }

    fn row_count(&self) -> usize {
        0
    }
}
