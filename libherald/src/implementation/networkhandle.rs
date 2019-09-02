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
    fn emit_net_down(&self) {}
    fn emit_net_up(&self) {}
    fn emit_net_pending(&self) {}
    fn emit_new_msg(&self) {}
}
// i'm kind of assuming that these emit a change
// they may very well not, I doubt that these poll
// across the FFI boundary
impl NetworkFlag for FlagType {
    #[inline(always)]
    fn emit_net_down(&self) {
        // drop the pending and online flags, we are in a fail state
        self.fetch_nand(NET_ONLINE | NET_PENDING, Ordering::Relaxed);
    }
    #[inline(always)]
    fn emit_net_up(&self) {
        // drops pending, start connection retries
        self.fetch_and(NET_ONLINE | !NET_PENDING, Ordering::Relaxed);
    }
    #[inline(always)]
    fn emit_net_pending(&self) {
        // sets pending, drops online
        self.fetch_and(!NET_ONLINE | NET_PENDING, Ordering::Relaxed);
    }
    #[inline(always)]
    fn emit_new_msg(&self) {
        self.fetch_and(NET_NEW_MSG, Ordering::Relaxed);
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
    fn new(emit: NetworkHandleEmitter) -> Self {
        let (tx, rx) = channel::<HandleMessages>();

        let handle = NetworkHandle {
            emit,
            status_flag: Arc::new(FlagType::new(NET_PENDING)),
            tx,
        };

        let flag = handle.status_flag.clone();

        NetworkHandle::connect_to_server(flag, rx);
        handle
    }

    fn send_message(&self, message_body: String, to: String) -> bool {
        if self.connection_pending() {
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
    pub fn connect_to_server(flag: Arc<FlagType>, rx: Receiver<HandleMessages>) {
        thread::spawn(move || {
            flag.emit_net_pending();
            let mut stream = match open_connection() {
                Ok(stream) => {
                    flag.emit_net_up();
                    stream
                }
                Err(e) => {
                    flag.emit_net_down();
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
                        RegisterDevice => {}
                        UpdateBlob { .. } => unimplemented!(),
                        RequestMeta { .. } => unimplemented!(),
                        // request from the network thread to
                        // ack that a message has been received and or read
                        ClientMessageAck { .. } => unimplemented!(),
                        _ => unimplemented!(),
                    },
                    //Ok(HandleMessages::Shutdown) => unimplemented!(),
                    Err(_e) => {}
                }

                // check os queue for tcp messages, they are inserted into db
                if let Ok(()) = read_from_server(&mut stream) {
                    flag.emit_new_msg();
                }

                thread::sleep(Duration::from_micros(10));
                // check and repair dead connection
                // retry logic should go here, this should infinite
                // loop until the net comes back
                let mut buf = [0; 1];
                if let Ok(0) = stream.peek(&mut buf) {
                    flag.emit_net_pending();
                    stream = match open_connection() {
                        Ok(stream) => {
                            flag.emit_net_up();
                            stream
                        }
                        Err(e) => {
                            flag.emit_net_down();
                            eprintln!("{}", e);
                            return;
                        }
                    };
                }
            }
        });
    }
}

// TODO add these
//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[cfg(test)]
//    pub fn headless_send() {}
//
//    #[cfg(test)]
//    pub fn headless_receive() {}
//}
