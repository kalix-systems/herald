use crate::interface::*;
use herald_common::*;
use std::sync::{
    atomic::{self, AtomicBool},
    mpsc::{channel, Sender},
    Arc,
};
use std::thread;
use std::time::Duration;

pub enum HandleMessages {
    Tx(MessageToServer),
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

        thread::spawn(move || loop {
            use MessageToServer::*;
            match rx.try_recv() {
                Ok(HandleMessages::Tx(message)) => match message {
                    // request from QT to send a message
                    SendMsg { .. } => {}
                    // request from QT to register a device
                    RegisterDevice => {}
                    UpdateBlob { .. } => unimplemented!(),
                    RequestMeta { .. } => unimplemented!(),
                },
                //Ok(HandleMessages::Shutdown) => unimplemented!(),
                Err(_e) => {}
            }

            if let Ok(HandleMessages::Tx(msg)) = rx.try_recv() {
                println!("I'm gettin a message here : {:?} ", msg);
                flag.fetch_xor(false, atomic::Ordering::Relaxed);
            }

            thread::sleep(Duration::from_micros(10));
        });

        handle
    }

    fn send_message(&self, message_body: String, to: String) -> bool {
        let msg = MessageToServer::SendMsg {
            to: UserId::from(&to).unwrap(),
            text: message_body.into(),
        };

        match self.tx.send(HandleMessages::Tx(msg)) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    pub fn headless_send() {}

    #[cfg(test)]
    pub fn headless_receive() {}
}
