use crate::{ffi, interface::*, ret_err, ret_none, shared};
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
    // The values really just function simple toggles.
    msg_data: AtomicU8,
    members_data: AtomicU8,
}

/// A bundle of channel senders. This is passed inside of a callback to the login function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifHandler {
    msg_senders: HashMap<ConversationId, Sender<shared::messages::MsgUpdate>>,
    members_senders: HashMap<ConversationId, Sender<shared::members::MemberUpdate>>,
    effects_flags: Arc<EffectsFlags>,
    emit: Emitter,
}

impl NotifHandler {
    fn send(&mut self, notif: Notification) {
        use Notification::*;
        match notif {
            NewMsg(msg_id, cid) => {
                use shared::messages::*;
                let tx = match self.msg_senders.get(&cid) {
                    Some(tx) => tx,
                    None => {
                        let (tx, rx) = unbounded();

                        self.msg_senders.insert(cid, tx);
                        MSG_RXS.insert(cid, rx);
                        ret_none!(self.msg_senders.get(&cid))
                    }
                };

                ret_err!(tx.send(MsgUpdate::Msg(msg_id)));
                self.effects_flags.msg_data.fetch_add(1, Ordering::Acquire);
                self.emit.msg_data_changed();

                use crate::shared::conv_global::*;

                ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewActivity(cid)));
                ret_none!(conv_emit_new_data());
            }
            MsgReceipt { mid, cid } => {
                use shared::messages::*;
                let tx = match self.msg_senders.get(&cid) {
                    Some(tx) => tx,
                    None => {
                        let (tx, rx) = unbounded();

                        self.msg_senders.insert(cid, tx);
                        MSG_RXS.insert(cid, rx);
                        ret_none!(self.msg_senders.get(&cid))
                    }
                };

                ret_err!(tx.send(MsgUpdate::Receipt(mid)));
                self.effects_flags.msg_data.fetch_add(1, Ordering::Acquire);
                self.emit.msg_data_changed();
            }
            NewContact(uid, cid) => {
                use crate::implementation::users::shared::*;
                use shared::conv_global::*;

                // add user
                ret_none!(send_user_update(UsersUpdates::NewUser(uid)));

                // add pairwise conversation
                ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewConversation(cid)));
                ret_none!(conv_emit_new_data());
            }
            NewConversation(cid) => {
                use shared::conv_global::*;
                ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewConversation(cid)));
                ret_none!(conv_emit_new_data());
            }
            AddContactResponse(cid, uid, accepted) => {
                use crate::implementation::users::shared::*;
                use shared::conv_global::*;

                // handle response
                ret_none!(send_user_update(UsersUpdates::ReqResp(uid, accepted)));

                // add conversation
                if accepted {
                    ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewConversation(cid)));
                    ret_none!(conv_emit_new_data());
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
            msg_senders: HashMap::new(),
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
            msg_data: AtomicU8::new(0),
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
        let uid = ret_err!(UserId::try_from(user_id.as_str()), false);
        ret_err!(network::register(uid), false);
        true
    }

    fn login(&mut self) -> bool {
        let mut handler = NotifHandler::new(self.emit.clone(), self.effects_flags.clone());

        ret_err!(
            thread::Builder::new().spawn(move || {
                ret_err!(network::login(move |notif: Notification| {
                    handler.send(notif);
                }))
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

    fn msg_data(&self) -> u8 {
        self.effects_flags.msg_data.load(Ordering::Relaxed)
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
