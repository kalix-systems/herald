use crate::{
    ffi,
    imp::{conversations::Conversations, messages},
    interface::*,
    ret_err,
    shared::{AddressedBus, SingletonBus},
};
use herald_common::*;
use heraldcore::network::{self, Notification};
use std::{
    convert::{TryFrom, TryInto},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

// short type aliases for cleanliness
type Emitter = NetworkHandleEmitter;

/// A bundle of `AtomicBool`s used for signalling
pub struct EffectsFlags {
    net_online: AtomicBool,
    net_pending: AtomicBool,
}

/// This is passed inside of a callback to the login function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifHandler {
    _effects_flags: Arc<EffectsFlags>,
    _emit: Emitter,
}

impl NotifHandler {
    fn send(&mut self, notif: Notification) {
        use crate::imp::conversations::shared::*;
        use crate::imp::users::{shared::*, Users};
        use messages::{shared::MsgUpdate, Messages};
        use Notification::*;

        match notif {
            NewMsg(msg_id, cid) => {
                ret_err!(Messages::push(cid, MsgUpdate::Msg(msg_id)));
            }
            MsgReceipt { mid, cid } => {
                ret_err!(Messages::push(cid, MsgUpdate::Receipt(mid)));
            }
            NewContact(uid, cid) => {
                // add user
                ret_err!(Users::push(UsersUpdates::NewUser(uid)));

                // add pairwise conversation
                ret_err!(Conversations::push(ConvUpdates::NewConversation(cid)));
            }
            NewConversation(cid) => {
                ret_err!(Conversations::push(ConvUpdates::NewConversation(cid)));
            }
            AddContactResponse(cid, uid, accepted) => {
                // handle response
                ret_err!(Users::push(UsersUpdates::ReqResp(uid, accepted)));

                // add conversation
                if accepted {
                    ret_err!(Conversations::push(ConvUpdates::NewConversation(cid)));
                }
            }
            AddConversationResponse(cid, uid, accepted) => {
                use crate::imp::members::{shared::*, Members};
                ret_err!(Members::push(cid, MemberUpdate::ReqResp(uid, accepted)));
            }
        }
    }
    fn new(emit: Emitter, _effects_flags: Arc<EffectsFlags>) -> Self {
        Self {
            _effects_flags,
            _emit: emit,
        }
    }
}

impl EffectsFlags {
    /// Creates a new `EffectsFlags`
    pub fn new() -> Self {
        EffectsFlags {
            net_online: AtomicBool::new(false),
            net_pending: AtomicBool::new(false),
        }
    }
}

/// This struct provides an interface to interact with the network and receive
/// notifications.
pub struct NetworkHandle {
    emit: NetworkHandleEmitter,
    effects_flags: Arc<EffectsFlags>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(emit: NetworkHandleEmitter) -> Self {
        let handle = NetworkHandle {
            emit,
            effects_flags: Arc::new(EffectsFlags::new()),
        };
        handle
    }

    fn send_add_request(&self, user_id: ffi::UserId, cid: ffi::ConversationIdRef) -> bool {
        let uid = ret_err!(user_id.as_str().try_into(), false);
        let cid = ret_err!(cid.try_into(), false);

        ret_err!(
            thread::Builder::new().spawn(move || {
                ret_err!(network::send_contact_req(uid, cid));
            }),
            false
        );

        true
    }

    fn register_new_user(&mut self, user_id: ffi::UserId) -> bool {
        use register::*;

        let uid = ret_err!(UserId::try_from(user_id.as_str()), false);
        match ret_err!(network::register(uid), false) {
            Res::UIDTaken => {
                eprintln!("UID taken!");
                false
            }
            Res::KeyTaken => {
                eprintln!("Key taken!");
                false
            }
            Res::BadSig(s) => {
                eprintln!("Bad sig: {:?}", s);
                false
            }
            Res::Success => true,
        }
    }

    fn login(&mut self) -> bool {
        use heraldcore::errors::HErr;

        let mut handler = NotifHandler::new(self.emit.clone(), self.effects_flags.clone());

        ret_err!(
            thread::Builder::new().spawn(move || {
                ret_err!(network::login(
                    move |notif: Notification| {
                        handler.send(notif);
                    },
                    move |herr: HErr| {
                        ret_err!(Err::<(), HErr>(herr));
                    }
                ))
            }),
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

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
