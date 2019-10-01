use crate::{interface::*, ret_err, types::*};
use herald_common::*;
use heraldcore::network;
use heraldcore::types::*;
use std::{
    convert::{TryFrom, TryInto},
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
}

impl EffectsFlags {
    pub fn new() -> Self {
        EffectsFlags {
            net_online: AtomicBool::new(false),
            net_pending: AtomicBool::new(false),
        }
    }
    //pub fn emit_net_down(&self, emit: &mut NetworkHandleEmitter) {
    //    // drop the pending and online flags, we are in a fail state
    //    self.net_online.fetch_and(false, Ordering::Relaxed);
    //    self.net_pending.fetch_or(true, Ordering::Relaxed);
    //    emit.connection_up_changed();
    //    emit.connection_pending_changed();
    //    println!("Net Down!");
    //}
    //pub fn emit_net_up(&self, emit: &mut NetworkHandleEmitter) {
    //    self.net_online.fetch_or(true, Ordering::Relaxed);
    //    self.net_pending.fetch_and(false, Ordering::Relaxed);
    //    emit.connection_up_changed();
    //    emit.connection_pending_changed();
    //    println!("Net Up!")
    //}
    //pub fn emit_net_pending(&self, emit: &mut NetworkHandleEmitter) {
    //    self.net_online.fetch_and(false, Ordering::Relaxed);
    //    self.net_pending.fetch_and(true, Ordering::Relaxed);
    //    emit.connection_up_changed();
    //    emit.connection_pending_changed();
    //    println!("Net Pending!")
    //}
    //pub fn emit_new_msg(&self, emit: &mut NetworkHandleEmitter) {
    //    let old = self.net_new_message.load(Ordering::Relaxed);
    //    self.net_new_message.fetch_xor(old, Ordering::Relaxed);
    //    emit.new_message_changed();
    //}
    //pub fn emit_new_contact(&self, emit: &mut NetworkHandleEmitter) {
    //    let old = self.net_new_message.load(Ordering::Relaxed);
    //    self.net_new_contact.fetch_xor(old, Ordering::Relaxed);
    //    emit.new_contact_changed();
    //}
    //pub fn emit_new_conversation(&self, emit: &mut NetworkHandleEmitter) {
    //    let old = self.net_new_message.load(Ordering::Relaxed);
    //    self.net_new_contact.fetch_xor(old, Ordering::Relaxed);
    //    emit.new_conversation_changed();
    //}
}

pub struct NetworkHandle {
    emit: NetworkHandleEmitter,
    status_flags: Arc<EffectsFlags>,
    new_events: usize,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(emit: NetworkHandleEmitter) -> Self {
        let handle = NetworkHandle {
            emit,
            status_flags: Arc::new(EffectsFlags::new()),
            new_events: 0,
        };
        handle
    }

    fn new_events(&self) -> u64 {
        self.new_events as u64
    }

    /// this is the API exposed to QML
    /// note, currently this function has all together too much copying.
    /// this will be rectified when stupid hanfles fan out.
    fn send_message(&self, body: String, to: FfiConversationIdRef, msg_id: FfiMsgIdRef) -> bool {
        let conv_id = ret_err!(ConversationId::try_from(to), false);

        let msg_id = ret_err!(MsgId::try_from(msg_id), false);

        ret_err!(network::send_text(conv_id, body, msg_id, None), false);
        true
    }

    fn send_add_request(&self, user_id: FfiUserId) -> bool {
        let uid = ret_err!(user_id.as_str().try_into(), false);
        ret_err!(network::send_contact_req(uid), false);
        true
    }

    fn register_new_user(&mut self, user_id: FfiUserId) -> bool {
        let uid = ret_err!(UserId::try_from(user_id.as_str()), false);
        ret_err!(network::register(uid), false);
        true
    }

    fn login(&mut self) -> bool {
        eprintln!("Login is not yet supported");
        true
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
//fn start_worker(
//    mut emit: NetworkHandleEmitter,
//    status_flags: Arc<EffectsFlags>,
//    qtrx: UnboundedReceiver<FuncCall>,
//) {
//    std::thread::spawn(move || {
//        let mut rt = abort_err!(
//            tokio::runtime::current_thread::Runtime::new(),
//            "could not spawn runtime"
//        );
//
//        rt.block_on(async move {
//            status_flags.emit_net_pending(&mut emit);
//            if let Ok((nwrx, sess)) = Session::init().await {
//                status_flags.emit_net_up(&mut emit);
//
//                tokio::spawn(handle_qt_channel(qtrx, sess.clone()));
//                tokio::spawn(handle_nw_channel(nwrx, sess, emit, status_flags));
//            } else {
//                println!("could not connect to server");
//
//                status_flags.emit_net_down(&mut emit);
//            }
//        });
//
//        abort_err!(rt.run());
//    });
//}
//
//async fn handle_qt_channel(mut rx: UnboundedReceiver<FuncCall>, sess: Session) {
//    loop {
//        match rx.recv().await {
//            Some(call) => match call {
//                FuncCall::RegisterDevice => {
//                    abort_err!(sess.register_device().await, "could not register device");
//                }
//                FuncCall::RequestMeta(id) => {
//                    abort_err!(sess.request_meta(id).await, "could not retrieve meta data");
//                }
//                FuncCall::SendMsg { to, msg } => {
//                    abort_err!(sess.send_msg(to, msg).await, "failed to send message");
//                }
//                FuncCall::AddRequest(conversation_id, user_id) => {
//                    abort_err!(
//                        sess.send_msg(user_id, MessageToPeer::AddRequest(conversation_id))
//                            .await,
//                        "failed to send add request"
//                    );
//                }
//            },
//            None => {}
//        };
//        // tokio delay here on return of none
//    }
//}
//async fn handle_nw_channel(
//    mut rx: UnboundedReceiver<Notification>,
//    _sess: Session,
//    mut emit: NetworkHandleEmitter,
//    status_flags: Arc<EffectsFlags>,
//) {
//    use Notification::*;
//    loop {
//        match rx.recv().await {
//            Some(notif) => match notif {
//                Ack(message_id) => {
//                    println!(
//                        "Receiving notification: {:?}",
//                        message_id
//                    );
//                    // TODO update the UI here.
//                }
//                NewMsg(cid) => {
//                    println!("NEW MESSAGE FROM : {:?}", cid);
//                    status_flags.emit_new_msg(&mut emit);
//                }
//                NewContact => {
//                    println!("NEW CONTACT ADDED");
//                    status_flags.emit_new_contact(&mut emit);
//                }
//                NewConversation => {
//                    println!("NEW CONVERSATION ADDED");
//                    status_flags.emit_new_conversation(&mut emit);
//                }
//            },
//            None => println!("print nope"),
//        };
//    }
//}
