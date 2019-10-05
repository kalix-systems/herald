use crate::{ffi, interface::*, ret_err, ret_none, shared::*};
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
    users_data: AtomicU8,
    members_data: AtomicU8,
    conv_data: AtomicU8,
}

/// A bundle of channel senders. This is passed inside of a callback to the login function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifHandler {
    msg_senders: HashMap<ConversationId, Sender<MsgUpdate>>,
    members_senders: HashMap<ConversationId, Sender<MemberUpdate>>,
    effects_flags: Arc<EffectsFlags>,
    emit: Emitter,
}

impl NotifHandler {
    fn send(&mut self, notif: Notification) {
        use Notification::*;
        match notif {
            NewMsg(msg_id, cid) => {
                match self.msg_senders.get(&cid) {
                    Some(tx) => {
                        ret_err!(tx.send(MsgUpdate::Msg(msg_id)));
                    }
                    None => {
                        let (tx, rx) = unbounded();

                        ret_err!(tx.send(MsgUpdate::Msg(msg_id)));
                        self.msg_senders.insert(cid, tx);
                        MSG_RXS.insert(cid, rx);
                    }
                }
                self.effects_flags.msg_data.fetch_add(1, Ordering::Acquire);
                self.emit.msg_data_changed();
            }
            Ack(msg_id, cid) => {
                match self.msg_senders.get(&cid) {
                    Some(tx) => {
                        ret_err!(tx.send(MsgUpdate::Ack(msg_id)));
                    }
                    None => {
                        let (tx, rx) = unbounded();

                        ret_err!(tx.send(MsgUpdate::Ack(msg_id)));
                        self.msg_senders.insert(cid, tx);
                        MSG_RXS.insert(cid, rx);
                    }
                }
                self.effects_flags.msg_data.fetch_add(1, Ordering::Acquire);
                self.emit.msg_data_changed();
            }
            NewContact(uid, cid) => {
                // add user
                ret_err!(USER_CHANNEL.tx.send(UsersUpdates::NewUser(uid)));
                self.effects_flags
                    .users_data
                    .fetch_add(1, Ordering::Acquire);
                self.emit.users_data_changed();

                // add pairwise conversation
                ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewConversation(cid)));
                self.effects_flags.conv_data.fetch_add(1, Ordering::Acquire);
                self.emit.conv_data_changed();
            }
            NewConversation(cid) => {
                ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewConversation(cid)));
                self.effects_flags.conv_data.fetch_add(1, Ordering::Acquire);
                self.emit.conv_data_changed();
            }
            AddContactResponse(cid, uid, accepted) => {
                // handle response
                ret_err!(USER_CHANNEL.tx.send(UsersUpdates::ReqResp(uid, accepted)));
                self.effects_flags
                    .users_data
                    .fetch_add(1, Ordering::Acquire);

                // add conversation
                if accepted {
                    ret_err!(CONV_CHANNEL.tx.send(ConvUpdates::NewConversation(cid)));
                    self.effects_flags.conv_data.fetch_add(1, Ordering::Acquire);
                    self.emit.conv_data_changed();
                    self.emit.users_data_changed();
                }
            }
            AddConversationResponse(cid, uid, accepted) => {
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
                self.emit.conv_data_changed();
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
            conv_data: AtomicU8::new(0),
            users_data: AtomicU8::new(0),
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

    //fn send_message(
    //    &self,
    //    body: String,
    //    to: ffi::ConversationIdRef,
    //    msg_id: ffi::MsgIdRef,
    //) -> bool {
    //    let conv_id = ret_err!(ConversationId::try_from(to), false);

    //    let msg_id = ret_err!(MsgId::try_from(msg_id), false);

    //    ret_err!(
    //        thread::Builder::new().spawn(move || {
    //            ret_err!(network::send_text(conv_id, body, msg_id, None));
    //        }),
    //        false
    //    );
    //    true
    //}

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

    fn conv_data(&self) -> u8 {
        self.effects_flags.conv_data.load(Ordering::Relaxed)
    }

    fn members_data(&self) -> u8 {
        self.effects_flags.members_data.load(Ordering::Relaxed)
    }

    fn msg_data(&self) -> u8 {
        self.effects_flags.msg_data.load(Ordering::Relaxed)
    }

    fn users_data(&self) -> u8 {
        self.effects_flags.users_data.load(Ordering::Relaxed)
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
