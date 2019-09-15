use crate::interface::*;
use herald_common::*;
use heraldcore::{
    network::*,
    tokio::{self, sync::mpsc::*},
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
}

impl EffectsFlags {
    pub fn new() -> Self {
        EffectsFlags {
            net_online: AtomicBool::new(false),
            net_pending: AtomicBool::new(false),
            net_new_message: AtomicBool::new(false),
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
        self.net_new_message.fetch_and(true, Ordering::Relaxed);
        emit.new_message_changed();
    }
}

/// map to function calls from the herald core session api
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
    fn send_message(&mut self, body: String, to: &[u8], msg_id: &[u8]) -> bool {
        if to.len() != 32 {
            eprintln!("");
            return false;
        }

        // we copy this repeatedly, if this gets slow, put it in an arc.
        let conv_id = match ConversationId::try_from(to) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        let members = match heraldcore::conversation::Conversations::get_members(&conv_id) {
            Ok(vec) => vec.into_iter(),
            Err(e) => {
                eprintln!("Could not get members of conversation {}", e);
                return false;
            }
        };

        let msg_id = match MsgId::try_from(msg_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        for member in members {
            println!("attempting to send to {}", &member);
            match self.tx.try_send(FuncCall::SendMsg {
                msg: MessageToPeer::Message {
                    body: body.clone(),
                    msg_id: msg_id.clone(),
                    conversation_id: conv_id.clone(),
                },
                to: member,
            }) {
                Ok(_) => println!("message queued for send"),
                Err(_e) => {
                    eprintln!("could not send message, error unrpintable");
                    return false;
                }
            }
        }
        true
    }

    fn send_add_request(&mut self, user_id: String, conversation_id: &[u8]) -> bool {
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
    fn request_meta_data(&mut self, of: String) -> bool {
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
        let mut rt =
            tokio::runtime::current_thread::Runtime::new().expect("could not spawn runtime");
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
        rt.run().expect("opops");
    });
}

async fn handle_qt_channel(mut rx: UnboundedReceiver<FuncCall>, sess: Session) {
    loop {
        match rx.recv().await {
            Some(call) => match call {
                FuncCall::RegisterDevice => {
                    sess.register_device()
                        .await
                        .expect("could not register device");
                }
                FuncCall::RequestMeta(id) => {
                    sess.request_meta(id)
                        .await
                        .expect("could not retrieve meta data");
                }
                FuncCall::SendMsg { to, msg } => {
                    sess.send_msg(to, msg)
                        .await
                        .expect("failed to send message");
                }
                FuncCall::AddRequest(conversation_id, user_id) => {
                    sess.send_msg(user_id, MessageToPeer::AddRequest(conversation_id))
                        .await
                        .expect("failed to send add request");
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
    loop {
        match rx.recv().await {
            Some(notif) => match notif {
                Notification::Ack(ClientMessageAck {
                    update_code,
                    message_id,
                }) => {
                    println!(
                        "Receiving notification: {:?}, {:?}",
                        update_code, message_id
                    );
                    // TODO update the UI here.
                }
                Notification::NewMsg(user_id) => {
                    println!("NEW MESSAGE FROM : {}", user_id);
                    status_flags.emit_new_msg(&mut emit);
                }
            },
            None => println!("print nope"),
        };
    }
}
