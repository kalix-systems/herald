use crate::{interface::*, ret_err, types::*};
use herald_common::*;
use heraldcore::{
    abort_err,
    network::*,
    tokio::{self, sync::mpsc::*},
    types::*,
};
use std::{
    convert::TryFrom,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
// short type aliases for cleanliness
type Emitter = NetworkHandleEmitter;

pub struct EffectsFlags {
    net_online: AtomicBool,
    net_pending: AtomicBool,
    net_new_message: AtomicBool,
    net_new_contact: AtomicBool,
    net_new_conversation: AtomicBool,
}

impl EffectsFlags {
    pub fn new() -> Self {
        EffectsFlags {
            net_online: AtomicBool::new(false),
            net_pending: AtomicBool::new(false),
            net_new_message: AtomicBool::new(false),
            net_new_contact: AtomicBool::new(false),
            net_new_conversation: AtomicBool::new(false),
        }
    }
    pub fn emit_net_down(&self, emit: &mut NetworkHandleEmitter) {
        // drop the pending and online flags, we are in a fail state
        self.net_online.fetch_and(false, Ordering::Relaxed);
        self.net_pending.fetch_and(true, Ordering::Relaxed);
        emit.connection_up_changed();
        emit.connection_pending_changed();
        println!("Net Down!");
    }
    pub fn emit_net_up(&self, emit: &mut NetworkHandleEmitter) {
        self.net_online.fetch_and(true, Ordering::Relaxed);
        self.net_pending.fetch_and(false, Ordering::Relaxed);
        emit.connection_up_changed();
        emit.connection_pending_changed();
        println!("Net Up!")
    }
    pub fn emit_net_pending(&self, emit: &mut NetworkHandleEmitter) {
        self.net_online.fetch_and(false, Ordering::Relaxed);
        self.net_pending.fetch_and(true, Ordering::Relaxed);
        emit.connection_up_changed();
        emit.connection_pending_changed();
        println!("Net Pending!")
    }
    pub fn emit_new_msg(&self, emit: &mut NetworkHandleEmitter) {
        let old = self.net_new_message.load(Ordering::Relaxed);
        self.net_new_message.fetch_xor(old, Ordering::Relaxed);
        emit.new_message_changed();
    }
    pub fn emit_new_contact(&self, emit: &mut NetworkHandleEmitter) {
        let old = self.net_new_message.load(Ordering::Relaxed);
        self.net_new_contact.fetch_xor(old, Ordering::Relaxed);
        emit.new_contact_changed();
    }
    pub fn emit_new_conversation(&self, emit: &mut NetworkHandleEmitter) {
        let old = self.net_new_message.load(Ordering::Relaxed);
        self.net_new_contact.fetch_xor(old, Ordering::Relaxed);
        emit.new_conversation_changed();
    }
}

/// map to function calls from the herald core session api
#[derive(Debug)]
pub enum FuncCall {
    SendMsg { to: UserId, msg: MessageToPeer },
    AddRequest(ConversationId, UserId),
    RequestMeta(UserId),
    RegisterDevice,
}

pub struct NetworkHandle {
    emit: NetworkHandleEmitter,
    status_flags: Arc<EffectsFlags>,
    tx: UnboundedSender<FuncCall>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(emit: NetworkHandleEmitter) -> Self {
        let (tx, rx) = unbounded_channel();

        let mut handle = NetworkHandle {
            emit,
            status_flags: Arc::new(EffectsFlags::new()),
            tx,
        };
        start_worker(handle.emit.clone(), handle.status_flags.clone(), rx);
        handle
    }

    /// this is the API exposed to QML
    /// note, currently this function has all together too much copying.
    /// this will be rectified when stupid hanfles fan out.
    fn send_message(
        &mut self,
        body: String,
        to: FfiConversationIdRef,
        msg_id: FfiMsgIdRef,
    ) -> bool {
        if to.len() != 32 {
            eprintln!("");
            return false;
        }

        // we copy this repeatedly, if this gets slow, put it in an arc.
        let conv_id = ret_err!(ConversationId::try_from(to), false);

        let handle = ret_err!(heraldcore::conversation::Conversations::new(), false);
        let members = ret_err!(handle.members(&conv_id), false);

        let msg_id = ret_err!(MsgId::try_from(msg_id), false);

        for member in members {
            println!("attempting to send to {}", &member);

            ret_err!(
                self.tx.try_send(FuncCall::SendMsg {
                    msg: MessageToPeer::Message {
                        body: body.clone(),
                        msg_id: msg_id.clone(),
                        conversation_id: conv_id.clone(),
                        op_msg_id: None
                    },
                    to: member,
                }),
                false
            );

            println!("message queued for send");
        }
        true
    }

    fn send_add_request(&mut self, user_id: UserId, conversation_id: FfiConversationIdRef) -> bool {
        if conversation_id.len() != 32 {
            eprintln!("Invalid conversation_id");
            return false;
        }

        let conversation_id = match ConversationId::try_from(conversation_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        match self
            .tx
            .try_send(FuncCall::AddRequest(conversation_id, user_id))
        {
            Ok(_) => return true,
            Err(_) => {
                eprintln!("failed to send");
                return false;
            }
        }
    }

    fn register_device(&mut self) -> bool {
        match self.tx.try_send(FuncCall::RegisterDevice) {
            Ok(_) => true,
            Err(_e) => {
                eprintln!("could not register device, error unrpintable");
                false
            }
        }
    }

    /// this is the API exposed to QML
    fn request_meta_data(&mut self, of: UserId) -> bool {
        match self.tx.try_send(FuncCall::RequestMeta(of)) {
            Ok(_) => true,
            Err(_e) => {
                eprintln!("could not get meta data, error unrpintable");
                false
            }
        }
    }

    fn new_message(&self) -> bool {
        self.status_flags.net_new_message.load(Ordering::Relaxed)
    }

    fn new_contact(&self) -> bool {
        self.status_flags.net_new_contact.load(Ordering::Relaxed)
    }

    fn new_conversation(&self) -> bool {
        self.status_flags
            .net_new_conversation
            .load(Ordering::Relaxed)
    }

    fn connection_up(&self) -> bool {
        self.status_flags.net_online.load(Ordering::Relaxed)
    }

    fn connection_pending(&self) -> bool {
        self.status_flags.net_pending.load(Ordering::Relaxed)
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}

// qtrx: receive from qt
fn start_worker(
    mut emit: NetworkHandleEmitter,
    status_flags: Arc<EffectsFlags>,
    qtrx: UnboundedReceiver<FuncCall>,
) {
    std::thread::spawn(move || {
        let mut rt = abort_err!(
            tokio::runtime::current_thread::Runtime::new(),
            "could not spawn runtime"
        );

        rt.block_on(async move {
            status_flags.emit_net_pending(&mut emit);

            if let Ok((nwrx, sess)) = Session::init().await {
                status_flags.emit_net_up(&mut emit);

                tokio::spawn(handle_qt_channel(qtrx, sess.clone()));
                tokio::spawn(handle_nw_channel(nwrx, sess, emit, status_flags));
            } else {
                println!("could not connect to server");

                status_flags.emit_net_down(&mut emit);
            }
        });

        abort_err!(rt.run());
    });
}

async fn handle_qt_channel(mut rx: UnboundedReceiver<FuncCall>, sess: Session) {
    loop {
        match rx.recv().await {
            Some(call) => match call {
                FuncCall::RegisterDevice => {
                    abort_err!(sess.register_device().await, "could not register device");
                }
                FuncCall::RequestMeta(id) => {
                    abort_err!(sess.request_meta(id).await, "could not retrieve meta data");
                }
                FuncCall::SendMsg { to, msg } => {
                    abort_err!(sess.send_msg(to, msg).await, "failed to send message");
                }
                FuncCall::AddRequest(conversation_id, user_id) => {
                    abort_err!(
                        sess.send_msg(user_id, MessageToPeer::AddRequest(conversation_id))
                            .await,
                        "failed to send add request"
                    );
                }
            },
            None => {}
        };
        // tokio delay here on return of none
    }
}
async fn handle_nw_channel(
    mut rx: UnboundedReceiver<Notification>,
    _sess: Session,
    mut emit: NetworkHandleEmitter,
    status_flags: Arc<EffectsFlags>,
) {
    use Notification::*;
    loop {
        match rx.recv().await {
            Some(notif) => match notif {
                Ack(MessageReceipt {
                    update_code,
                    message_id,
                }) => {
                    println!(
                        "Receiving notification: {:?}, {:?}",
                        update_code, message_id
                    );
                    // TODO update the UI here.
                }
                NewMsg(user_id) => {
                    println!("NEW MESSAGE FROM : {}", user_id);
                    status_flags.emit_new_msg(&mut emit);
                }
                NewContact => {
                    println!("NEW CONTACT ADDED");
                    status_flags.emit_new_contact(&mut emit);
                }
                NewConversation => {
                    println!("NEW CONVERSATION ADDED");
                    status_flags.emit_new_conversation(&mut emit);
                }
            },
            None => println!("print nope"),
        };
    }
}
