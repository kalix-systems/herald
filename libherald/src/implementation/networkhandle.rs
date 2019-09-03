use crate::interface::*;
use herald_common::*;
use heraldcore::network::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct NetworkFlags {
    net_online: AtomicBool,
    net_pending: AtomicBool,
    net_new_message: AtomicBool,
}

impl NetworkFlags {
    pub fn new() -> Self {
        NetworkFlags {
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

pub enum HandleMessages {
    ToServer(MessageToServer),
    //Shutdown,
}

struct NetworkHandle {
    emit: NetworkHandleEmitter,
    status_flags: Arc<NetworkFlags>,
    tx: Sender<HandleMessages>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(mut emit: NetworkHandleEmitter) -> Self {
        let (tx, rx) = channel::<HandleMessages>();
        let emitter_clone = emit.clone();

        let handle = NetworkHandle {
            emit,
            status_flags: Arc::new(NetworkFlags::new()),
            tx,
        };

        let flag = handle.status_flags.clone();
        NetworkHandle::connect_to_server(flag, rx, emitter_clone);
        handle
    }

    fn send_message(&self, message_body: String, to: String) -> bool {
        let to = UserId::from(&to);

        let msg = MessageToServer::SendMsg {
            to,
            text: message_body.into(),
        };

        match self.tx.send(HandleMessages::ToServer(msg)) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("{}", e);
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

    fn emit(&mut self) -> &mut NetworkHandleEmitter {
        &mut self.emit
    }
}

impl NetworkHandle {
    pub fn connect_to_server(
        flag: Arc<NetworkFlags>,
        rx: Receiver<HandleMessages>,
        mut emit: NetworkHandleEmitter,
    ) {
        thread::spawn(move || {
            flag.emit_net_pending(&mut emit);
            let mut stream = match open_connection() {
                Ok(stream) => {
                    flag.emit_net_up(&mut emit);
                    stream
                }
                Err(e) => {
                    flag.emit_net_down(&mut emit);
                    eprintln!("{}", e);
                    return;
                }
            };

            loop {
                use MessageToServer::*;
                match rx.try_recv() {
                    Ok(HandleMessages::ToServer(message)) => match message {
                        // request from Qt to send a message
                        SendMsg { to, text } => {
                            send_message(to, text, &mut stream).unwrap();
                        }
                        // request from Qt to register a device
                        RegisterDevice => unimplemented!(),
                        UpdateBlob { .. } => unimplemented!(),
                        RequestMeta { .. } => unimplemented!(),
                        // request from the network thread to
                        // ack that a message has been received and or read
                        ClientMessageAck {
                            to,
                            update_code,
                            message_id,
                        } => {
                            send_ack(to, update_code, message_id, &mut stream).unwrap();
                        }
                    },
                    //Ok(HandleMessages::Shutdown) => unimplemented!(),
                    Err(_e) => {}
                }

                // check os queue for tcp messages, they are inserted into db
                if let Ok(()) = read_from_server(&mut stream) {
                    flag.emit_new_msg(&mut emit);
                }

                thread::sleep(Duration::from_micros(10));
                // check and repair dead connection
                // retry logic should go here, this should infinite
                // loop until the net comes back
                let mut buf = [0; 8];
                if let Ok(0) = stream.peek(&mut buf) {
                    flag.emit_net_pending(&mut emit);
                    stream = match open_connection() {
                        Ok(stream) => {
                            flag.emit_net_up(&mut emit);
                            stream
                        }
                        Err(e) => {
                            flag.emit_net_down(&mut emit);
                            eprintln!("{}", e);
                            return;
                        }
                    };
                }
            }
        });
    }
}
