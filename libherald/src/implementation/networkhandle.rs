use crate::interface::*;
use herald_common::*;
use heraldcore::network::*;
use std::{
    sync::{
        atomic::{self, AtomicBool},
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

pub enum HandleMessages {
    ToServer(MessageToServer),
    //Shutdown,
}

pub struct NetworkHandle {
    emit: NetworkHandleEmitter,
    message_received: Arc<AtomicBool>,
    tx: Sender<HandleMessages>,
}

impl NetworkHandleTrait for NetworkHandle {
    fn new(emit: NetworkHandleEmitter) -> Self {
        let (tx, rx) = channel::<HandleMessages>();

        let handle = NetworkHandle {
            emit,
            message_received: Arc::new(AtomicBool::new(false)),
            tx,
        };

        let flag = handle.message_received.clone();

        thread::spawn(move || {
            let mut stream = match open_connection() {
                Ok(stream) => stream,
                Err(e) => {
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
                    },
                    //Ok(HandleMessages::Shutdown) => unimplemented!(),
                    Err(_e) => {}
                }

                // check os queue for tcp messages, they are inserted into
                // the db. set flag on the QML side...
                flag.fetch_xor(
                    read_from_server(&mut stream).is_ok(),
                    atomic::Ordering::Relaxed,
                );
                thread::sleep(Duration::from_micros(10));
            }
        });

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
        self.message_received
            .fetch_and(true, atomic::Ordering::Relaxed)
    }

    fn emit(&mut self) -> &mut NetworkHandleEmitter {
        &mut self.emit
    }
}

impl NetworkHandle {}

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
