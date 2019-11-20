use crate::{
    ffi,
    imp::{conversations::Conversations, messages},
    interface::*,
    push_err, ret_err,
    shared::{AddressedBus, SingletonBus},
    spawn,
};
use herald_common::*;
use heraldcore::network::{self as net, Notification};
use heraldcore::{config, db, message::gc};
use std::{
    convert::TryFrom,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

type Emitter = HeraldStateEmitter;

mod network;
use network::*;

/// Global state for the application that can't easily be included
/// in another model. Currently only used to distinguish initial registration
/// from logins.
pub struct HeraldState {
    config_init: Arc<AtomicBool>,
    emit: HeraldStateEmitter,
    effects_flags: Arc<EffectsFlags>,
}

impl HeraldStateTrait for HeraldState {
    fn new(emit: HeraldStateEmitter) -> Self {
        let config_init = if config::id().is_ok() {
            // If this fails, it's because a thread couldn't be spawned.
            // This implies the OS is in a very bad place.
            push_err!(
                gc::init(move |update| {
                    gc_handler(update);
                }),
                "Couldn't start GC thread"
            );

            Arc::new(AtomicBool::new(true))
        } else {
            // If this fails, the file system is in a very bad place.
            // This probably cannot be recovered from, and there's not meaningful
            // sense in which the application can work.
            push_err!(db::init(), "Couldn't initialize storage");

            Arc::new(AtomicBool::new(false))
        };

        HeraldState {
            emit,
            config_init,
            effects_flags: Arc::new(EffectsFlags::new()),
        }
    }

    fn config_init(&self) -> bool {
        self.config_init.load(Ordering::Acquire)
    }

    fn register_new_user(&mut self, user_id: ffi::UserId) {
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
                        gc_handler(update);
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

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }
}

fn gc_handler(update: gc::GCUpdate) {
    use crate::imp::messages::{shared::MsgUpdate, Messages};
    use gc::GCUpdate::*;
    use heraldcore::errors::HErr;
    match update {
        StaleConversations(convs) => {
            for (cid, mids) in convs {
                push_err!(
                    Messages::push(cid, MsgUpdate::ExpiredMessages(mids)),
                    "Couldn't expire messages"
                );
            }
        }
        GCError(e) => {
            push_err!(Err::<(), HErr>(e), "Error deleting expired messages");
        }
    }
}
