use crate::{
    ffi,
    implementation::{conversations::Conversations, messages},
    interface::*,
    ret_err, ret_none,
    shared::{self, AddressedBus, SingletonBus},
};
use crossbeam_channel::*;
use herald_common::*;
use heraldcore::{
    network::{self, Notification},
    types::*,
};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
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
    // Note: these are `AtomicU8`s because it's not obvious
    // how to atomically negate an `AtomicBool`.
    members_data: AtomicU8,
}

/// A bundle of channel senders. This is passed inside of a callback to the login function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifHandler {
    members_senders: HashMap<ConversationId, Sender<shared::members::MemberUpdate>>,
    effects_flags: Arc<EffectsFlags>,
    emit: Emitter,
}

impl NotifHandler {
    fn send(&mut self, notif: Notification) {
        use crate::implementation::conversations::shared::*;
        use crate::implementation::users::shared::*;
        use messages::{shared::MsgUpdate, Messages};
        use Notification::*;

        match notif {
            NewMsg(msg_id, cid) => {
                ret_err!(Messages::push(cid, MsgUpdate::Msg(msg_id)));

                ret_err!(Conversations::push(ConvUpdates::NewActivity(cid)));
            }
            MsgReceipt { mid, cid } => {
                ret_err!(Messages::push(cid, MsgUpdate::Receipt(mid)));
            }
            NewContact(uid, cid) => {
                // add user
                ret_none!(push_user_update(UsersUpdates::NewUser(uid)));

                // add pairwise conversation
                ret_err!(Conversations::push(ConvUpdates::NewConversation(cid)));
            }
            NewConversation(cid) => {
                ret_err!(Conversations::push(ConvUpdates::NewConversation(cid)));
            }
            AddContactResponse(cid, uid, accepted) => {
                // handle response
                ret_none!(push_user_update(UsersUpdates::ReqResp(uid, accepted)));

                // add conversation
                if accepted {
                    ret_err!(Conversations::push(ConvUpdates::NewConversation(cid)));
                }
            }
            AddConversationResponse(cid, uid, accepted) => {
                use shared::members::*;

                let tx = match self.members_senders.get(&cid) {
                    Some(tx) => tx,
                    None => {
                        let (tx, rx) = unbounded();

                        self.members_senders.insert(cid, tx);
                        MEMBER_RXS.insert(cid, rx);
                        ret_none!(self.members_senders.get(&cid))
                    }
                };

                ret_err!(tx.send(MemberUpdate::ReqResp(uid, accepted)));
                self.effects_flags
                    .members_data
                    .fetch_add(1, Ordering::Acquire);
                self.emit.members_data_changed();
            }
        }
    }
    fn new(mut emit: Emitter, effects_flags: Arc<EffectsFlags>) -> Self {
        Self {
            members_senders: HashMap::new(),
            effects_flags,
            emit: emit.clone(),
        }
    }
}

impl EffectsFlags {
    /// Creates a new `EffectsFlags`
    pub fn new() -> Self {
        EffectsFlags {
            net_online: AtomicBool::new(false),
            net_pending: AtomicBool::new(false),
            members_data: AtomicU8::new(0),
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

    fn members_data(&self) -> u8 {
        self.effects_flags.members_data.load(Ordering::Relaxed)
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
