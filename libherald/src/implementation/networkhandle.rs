use crate::interface::*;
use herald_common::*;
use heraldcore::network::*;
use heraldcore::tokio::{
    self,
    sync::mpsc::*,
    sync::{mpsc, oneshot},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
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
    RequestMeta(UserId),
    RegisterDevice,
}

pub enum HandleMessages {
    ToServer(MessageToServer),
}

pub struct NetworkHandle {
    emit: NetworkHandleEmitter,
    status_flags: Arc<EffectsFlags>,
    tx: UnboundedSender<FuncCall>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(mut emit: NetworkHandleEmitter) -> Self {
        let emitter_clone = emit.clone();
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
    fn send_message(&self, message_body: String, to: String) -> bool {
        false
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
    emit: NetworkHandleEmitter,
    status_flags: Arc<EffectsFlags>,
    qtrx: UnboundedReceiver<FuncCall>,
) {
    std::thread::spawn(move || {
        let mut rt =
            tokio::runtime::current_thread::Runtime::new().expect("could not spawn runtime");
        rt.block_on(async move {
            let (nwrx, sess) = match Session::init().await {
                Ok((nwrx, sess)) => (nwrx, sess),
                Err(e) => {
                    eprintln!("failed to init session! : {}", e);
                    std::process::abort();
                }
            };
            tokio::spawn(handle_qt_channel(qtrx, sess.clone()));
            tokio::spawn(handle_nw_channel(nwrx, sess));
        });
    });
}

async fn handle_qt_channel(mut rx: UnboundedReceiver<FuncCall>, sess: Session) {
    loop {
        match rx.recv().await {
            Some(call) => match call {
                _ => unimplemented!(),
            },
            None => print!("maybe backoff expo here?"),
        };
    }
}
async fn handle_nw_channel(mut rx: UnboundedReceiver<Notification>, sess: Session) {
    loop {
        match rx.recv().await {
            Some(notif) => match notif {
                _ => unimplemented!(),
            },
            None => print!("maybe backoff expo here?"),
        };
    }
}
