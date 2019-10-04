use crate::shared::{ConvMsgUpdate, CONV_MSG_RXS};
use crate::{ffi, interface::*, ret_err, ret_none};
use crossbeam_channel::*;
use herald_common::*;
use heraldcore::abort_err;
use heraldcore::network::{self, Notification};
use heraldcore::types::*;
use std::collections::HashMap;
use std::thread;
use std::{
    convert::{TryFrom, TryInto},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

// short type aliases for cleanliness
type Emitter = NetworkHandleEmitter;

/// A bundle of `AtomicBool`s used for signalling
pub struct EffectsFlags {
    net_online: AtomicBool,
    net_pending: AtomicBool,
}

/// A bundle of channel receivers used for receiving notifications from the network.
pub struct NotifRx {
    contact_rx: Receiver<UserId>,
    conversation_rx: Receiver<ConversationId>,
    add_contact_resp_rx: Receiver<(ConversationId, UserId, bool)>,
    add_conv_resp_rx: Receiver<(ConversationId, bool)>,
    emit: Emitter,
}

impl NotifRx {
    fn contact_recv(&mut self) -> Option<UserId> {
        let val = self.contact_rx.recv().ok()?;
        self.emit.new_contact_changed();
        Some(val)
    }

    fn new_contact(&self) -> u64 {
        self.contact_rx.len() as u64
    }

    fn conversation_recv(&mut self) -> Option<ConversationId> {
        let val = self.conversation_rx.recv().ok()?;
        self.emit.new_conversation_changed();
        Some(val)
    }

    fn new_conversation(&self) -> u64 {
        self.conversation_rx.len() as u64
    }

    fn add_contact_resp_recv(&mut self) -> Option<(ConversationId, UserId, bool)> {
        let val = self.add_contact_resp_rx.recv().ok()?;
        self.emit.new_add_contact_resp_changed();
        Some(val)
    }

    fn new_add_contact_resp(&self) -> u64 {
        self.add_contact_resp_rx.len() as u64
    }

    fn add_conv_resp_recv(&mut self) -> Option<(ConversationId, bool)> {
        let val = self.add_conv_resp_rx.recv().ok()?;
        self.emit.new_add_conv_resp_changed();
        Some(val)
    }

    fn new_add_conv_resp(&self) -> u64 {
        self.add_conv_resp_rx.len() as u64
    }
}

/// A bundle of channel senders. This is passed inside of a callback to the login function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifTx {
    contact_tx: Sender<UserId>,
    conversation_tx: Sender<ConversationId>,
    add_contact_resp_tx: Sender<(ConversationId, UserId, bool)>,
    add_conv_resp_tx: Sender<(ConversationId, bool)>,
    conv_senders: HashMap<ConversationId, Sender<ConvMsgUpdate>>,
    conv_data: Arc<AtomicBool>,
    emit: Emitter,
}

impl NotifTx {
    fn send(&mut self, notif: Notification) {
        use Notification::*;
        match notif {
            NewMsg(msg_id, cid) => {
                match self.conv_senders.get(&cid) {
                    Some(tx) => {
                        ret_err!(tx.send(ConvMsgUpdate::Msg(msg_id)));
                    }
                    None => {
                        let (tx, rx) = unbounded();

                        ret_err!(tx.send(ConvMsgUpdate::Msg(msg_id)));
                        self.conv_senders.insert(cid, tx);
                        CONV_MSG_RXS.insert(cid, rx);
                    }
                }
                self.conv_data
                    .fetch_xor(self.conv_data.load(Ordering::Relaxed), Ordering::Acquire);
                self.emit.new_conv_data_changed();
            }
            Ack(msg_id, cid) => {
                match self.conv_senders.get(&cid) {
                    Some(tx) => {
                        ret_err!(tx.send(ConvMsgUpdate::Ack(msg_id)));
                    }
                    None => {
                        let (tx, rx) = unbounded();

                        ret_err!(tx.send(ConvMsgUpdate::Ack(msg_id)));
                        self.conv_senders.insert(cid, tx);
                        CONV_MSG_RXS.insert(cid, rx);
                    }
                }
                self.conv_data
                    .fetch_xor(self.conv_data.load(Ordering::Relaxed), Ordering::Acquire);
                self.emit.new_conv_data_changed();
            }
            NewContact(uid, cid) => {
                ret_err!(self.contact_tx.send(uid));
                self.emit.new_contact_changed();
                ret_err!(self.conversation_tx.send(cid));
                self.emit.new_conversation_changed();
            }
            NewConversation(cid) => {
                ret_err!(self.conversation_tx.send(cid));
                self.emit.new_conversation_changed();
            }
            AddContactResponse(cid, uid, accepted) => {
                ret_err!(self.add_contact_resp_tx.send((cid, uid, accepted)));
                self.emit.new_add_contact_resp_changed();
            }
            AddConversationResponse(cid, accepted) => {
                ret_err!(self.add_conv_resp_tx.send((cid, accepted)));
                self.emit.new_add_conv_resp_changed();
            }
        }
    }
}

fn notif_channel(mut emit: Emitter, conv_data: Arc<AtomicBool>) -> (NotifTx, NotifRx) {
    let (contact_tx, contact_rx) = unbounded();
    let (conversation_tx, conversation_rx) = unbounded();
    let (add_contact_resp_tx, add_contact_resp_rx) = unbounded();
    let (add_conv_resp_tx, add_conv_resp_rx) = unbounded();

    let tx = NotifTx {
        contact_tx,
        conversation_tx,
        add_contact_resp_tx,
        add_conv_resp_tx,
        conv_senders: HashMap::new(),
        conv_data,
        emit: emit.clone(),
    };

    let rx = NotifRx {
        contact_rx,
        conversation_rx,
        add_contact_resp_rx,
        add_conv_resp_rx,
        emit: emit.clone(),
    };

    (tx, rx)
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
    status_flags: Arc<EffectsFlags>,
    new_conv_data: Arc<AtomicBool>,
    notif_rx: Option<NotifRx>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(emit: NetworkHandleEmitter) -> Self {
        let handle = NetworkHandle {
            emit,
            status_flags: Arc::new(EffectsFlags::new()),
            new_conv_data: Arc::new(AtomicBool::new(false)),
            notif_rx: None,
        };
        handle
    }

    fn send_message(
        &self,
        body: String,
        to: ffi::ConversationIdRef,
        msg_id: ffi::MsgIdRef,
    ) -> bool {
        let conv_id = ret_err!(ConversationId::try_from(to), false);

        let msg_id = ret_err!(MsgId::try_from(msg_id), false);

        ret_err!(
            thread::Builder::new().spawn(move || {
                ret_err!(network::send_text(conv_id, body, msg_id, None));
            }),
            false
        );
        true
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
        let (mut tx, rx) = notif_channel(self.emit.clone(), self.new_conv_data.clone());

        self.notif_rx.replace(rx);

        ret_err!(
            network::login(move |notif: Notification| {
                tx.send(notif);
            }),
            false
        );
        true
    }

    fn connection_up(&self) -> bool {
        self.status_flags.net_online.load(Ordering::Relaxed)
    }

    fn connection_pending(&self) -> bool {
        self.status_flags.net_pending.load(Ordering::Relaxed)
    }

    fn new_conv_data(&self) -> bool {
        self.new_conv_data.load(Ordering::Relaxed)
    }

    fn new_add_contact_resp(&self) -> u64 {
        ret_none!(&self.notif_rx, 0).new_add_contact_resp()
    }

    fn new_add_conv_resp(&self) -> u64 {
        ret_none!(&self.notif_rx, 0).new_add_conv_resp()
    }

    fn new_contact(&self) -> u64 {
        ret_none!(&self.notif_rx, 0).new_contact()
    }

    fn new_conversation(&self) -> u64 {
        ret_none!(&self.notif_rx, 0).new_conversation()
    }

    /// Returns a `(ConversationId, UserId, bool)` serialized as CBOR,
    /// or `None` serialized as CBOR if the queue is exhausted.
    fn next_add_contact_resp(&mut self) -> Vec<u8> {
        let res = ret_none!(
            ret_none!(&mut self.notif_rx, cbor_none()).add_contact_resp_recv(),
            cbor_none()
        );

        ret_err!(serde_cbor::to_vec(&res), cbor_none())
    }

    /// Returns a `(ConversationId, bool)` serialized as CBOR,
    /// or `None` serialized as CBOR if the queue is exhausted.
    fn next_add_conversation_resp(&mut self) -> Vec<u8> {
        let res = ret_none!(
            ret_none!(&mut self.notif_rx, cbor_none()).add_conv_resp_recv(),
            cbor_none()
        );

        ret_err!(serde_cbor::to_vec(&res), cbor_none())
    }

    /// Returns a string representation of a `UserId`, or an empty string if the queue
    /// is exhausted
    fn next_new_contact(&mut self) -> ffi::UserId {
        ret_none!(
            ret_none!(&mut self.notif_rx, "".into()).contact_recv(),
            "".into()
        )
        .to_string()
    }

    /// Returns a `ConversationId`, or an empty vector if the queue
    /// is exhausted
    fn next_new_conversation(&mut self) -> ffi::ConversationId {
        ret_none!(
            ret_none!(&mut self.notif_rx, vec![]).conversation_recv(),
            vec![]
        )
        .to_vec()
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}

fn cbor_none() -> Vec<u8> {
    abort_err!(serde_cbor::to_vec(&serde_cbor::Value::Null))
}
