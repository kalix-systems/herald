use crate::interface::*;
use herald_common::*;
use heraldcore::network::*;
use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

type FlagType = AtomicU16;
type FlagPrimitive = u16;
// indicates connection is down
static NET_ONLINE: FlagPrimitive = 0x01;
// if connection status is low, and this is low,
// indicates network errors
static NET_PENDING: FlagPrimitive = 0x02;
// indicates that a new message is available
static NET_NEW_MSG: FlagPrimitive = 0x04;

trait NetworkFlag {
    fn emit_net_down(&self, emit: &mut NetworkHandleEmitter);
    fn emit_net_up(&self, emit: &mut NetworkHandleEmitter);
    fn emit_net_pending(&self, emit: &mut NetworkHandleEmitter);
    fn emit_new_msg(&self, emit: &mut NetworkHandleEmitter);
}
// i'm kind of assuming that these emit a change
// they may very well not, I doubt that these poll
// across the FFI boundary
impl NetworkFlag for FlagType {
    #[inline(always)]
    fn emit_net_down(&self, emit: &mut NetworkHandleEmitter) {
        // drop the pending and online flags, we are in a fail state
        self.fetch_and(!NET_ONLINE, Ordering::Relaxed);
        self.fetch_or(NET_PENDING, Ordering::Relaxed);
        emit.connection_up_changed();
        emit.connection_pending_changed();
        println!("Net Down!");
    }
    #[inline(always)]
    fn emit_net_up(&self, emit: &mut NetworkHandleEmitter) {
        // drops pending, set online
        self.fetch_or(NET_ONLINE, Ordering::Relaxed);
        self.fetch_and(!NET_PENDING, Ordering::Relaxed);
        emit.connection_up_changed();
        emit.connection_pending_changed();
        println!("Net up!");
    }
    #[inline(always)]
    fn emit_net_pending(&self, emit: &mut NetworkHandleEmitter) {
        // sets pending, drops online
        self.fetch_and(!NET_ONLINE, Ordering::Relaxed);
        self.fetch_or(NET_PENDING, Ordering::Relaxed);
        emit.connection_up_changed();
        emit.connection_pending_changed();
        println!("Net pending?!");
    }
    #[inline(always)]
    fn emit_new_msg(&self, emit: &mut NetworkHandleEmitter) {
        self.fetch_or(NET_NEW_MSG, Ordering::Relaxed);
        emit.new_message_changed();
    }
}

pub enum HandleMessages {
    ToServer(MessageToServer),
    //Shutdown,
}

pub struct NetworkHandle {
    emit: NetworkHandleEmitter,
    status_flag: Arc<FlagType>,
    tx: Sender<HandleMessages>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(mut emit: NetworkHandleEmitter) -> Self {
        let (tx, rx) = channel::<HandleMessages>();
        let emitter_clone = emit.clone();

        let handle = NetworkHandle {
            emit,
            status_flag: Arc::new(FlagType::new(0)),
            tx,
        };

        let flag = handle.status_flag.clone();
        NetworkHandle::connect_to_server(flag, rx, emitter_clone);
        handle
    }

    fn send_message(&self, message_body: String, to: String) -> bool {
        if !self.connection_up() {
            println!("you are literally not connected to the server.");
            return false;
        }

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
        self.status_flag.fetch_and(NET_NEW_MSG, Ordering::Relaxed) != 0
    }

    fn connection_up(&self) -> bool {
        self.status_flag.fetch_and(NET_ONLINE, Ordering::Relaxed) != 0
    }

    fn connection_pending(&self) -> bool {
        self.status_flag.fetch_and(NET_PENDING, Ordering::Relaxed) != 0
    }

    fn emit(&mut self) -> &mut NetworkHandleEmitter {
        &mut self.emit
    }
}

impl NetworkHandle {
    pub fn connect_to_server(
        flag: Arc<FlagType>,
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
                        ClientMessageAck { .. } => unimplemented!(),
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

#[cfg(test)]
mod test {
    use super::*;

    // just copy pasting logic out of the interior
    // here because I can't instantiate the emitter
    #[test]
    fn bit_flags_work() {
        let flag: FlagType = FlagType::new(0);

        // emit net down and pending
        flag.fetch_and(!NET_ONLINE, Ordering::Relaxed);
        flag.fetch_or(NET_PENDING, Ordering::Relaxed);
        assert_eq!(0b10u16, flag.fetch_and(0u16, Ordering::Relaxed));

        // emit net up
        flag.fetch_or(NET_ONLINE, Ordering::Relaxed);
        flag.fetch_and(!NET_PENDING, Ordering::Relaxed);
        assert_eq!(0b01u16, flag.fetch_and(0u16, Ordering::Relaxed));

        // emit net down, forever
        flag.fetch_and(!NET_ONLINE, Ordering::Relaxed);
        flag.fetch_and(!NET_PENDING, Ordering::Relaxed);
        assert_eq!(0u16, flag.fetch_and(0u16, Ordering::Relaxed));
        // new message
        flag.fetch_or(NET_NEW_MSG, Ordering::Relaxed);
        assert_eq!(0b100u16, flag.fetch_and(0u16, Ordering::Relaxed));
    }
}
