use crate::{config::Config, errors::HErr, message as db};
use chrono::prelude::*;
use herald_common::{
    read_cbor, send_cbor, ClientMessageAck, GlobalId, MessageStatus, MessageToClient,
    MessageToPeer, MessageToServer, Response, UserId,
};
use lazy_static::*;
use qutex::Qutex;
use std::{
    collections::HashMap,
    env,
    net::{SocketAddr, SocketAddrV4},
    ops::DerefMut,
};
use tokio::{
    net::{tcp::split::TcpStreamWriteHalf, *},
    prelude::*,
    sync::{mpsc, oneshot},
};

const DEFAULT_PORT: u16 = 8000;
const DEFAULT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

lazy_static! {
    static ref SERVER_ADDR: SocketAddr = match env::var("SERVER_ADDR") {
        Ok(addr) => addr.parse().unwrap_or_else(|e| {
            eprintln!("Provided address {} is invalid: {}", addr, e);
            std::process::abort();
        }),
        Err(_) => SocketAddr::V4(SocketAddrV4::new(
            DEFAULT_SERVER_IP_ADDR.into(),
            DEFAULT_PORT
        )),
    };
}

#[derive(Debug)]
pub enum Notification {
    // TODO: include conversation id's here
    // issue: #15
    NewMsg(UserId),
    Ack(ClientMessageAck),
}

fn form_ack(update_code: MessageStatus, message_id: i64) -> MessageToPeer {
    let ack = ClientMessageAck {
        update_code,
        message_id,
    };
    MessageToPeer::Ack(ack)
}

// note: this should never fail, but I'm returning a result until I read `serde_cbor` more closely
fn form_push(to: UserId, msg: MessageToPeer) -> Result<MessageToServer, HErr> {
    Ok(MessageToServer::SendMsg {
        to,
        body: serde_cbor::to_vec(&msg)?.into(),
    })
}

fn handle_msg(from: UserId, body: String, time: DateTime<Utc>) -> Result<Event, HErr> {
    let recipient = Config::static_id()?;
    let (row, _) = db::Messages::add_message(
        &from,
        &recipient,
        &body,
        Some(time),
        MessageStatus::Inbound, //all messages are inbound
    )?;
    let reply = form_push(from.clone(), form_ack(MessageStatus::RecipReceivedAck, row))?;
    let notification = Notification::NewMsg(from);
    Ok(Event {
        reply: Some(reply),
        notification: Some(notification),
    })
}

fn handle_ack(from: UserId, ack: ClientMessageAck) -> Result<Event, HErr> {
    let ClientMessageAck {
        message_id,
        update_code,
    } = ack;
    db::Messages::update_status(from.as_str(), message_id, update_code)?;
    Ok(Event {
        reply: None,
        notification: None,
    })
}

fn handle_push(from: GlobalId, body: MessageToPeer, time: DateTime<Utc>) -> Result<Event, HErr> {
    use MessageToPeer::*;
    match body {
        Message(s) => handle_msg(from.uid, s, time),
        Ack(a) => handle_ack(from.uid, a),
    }
}

struct Event {
    reply: Option<MessageToServer>,
    notification: Option<Notification>,
}

#[derive(Clone)]
pub struct Session {
    writer: Qutex<TcpStreamWriteHalf>,
    id: GlobalId,
    pending: Qutex<HashMap<MessageToServer, oneshot::Sender<Response>>>,
    notifications: mpsc::UnboundedSender<Notification>,
}

/// Login
async fn login<S: AsyncWrite + Unpin>(stream: &mut S) -> Result<GlobalId, HErr> {
    let gid = GlobalId {
        did: 0,
        uid: Config::static_id()?,
    };
    send_cbor(stream, &gid).await?;
    Ok(gid)
}

fn error_abort<E: std::fmt::Display, T>(e: E) -> T {
    eprintln!("an impossible error happend at {}:{}", file!(), line!());
    eprintln!("message was: {}", e);
    eprintln!("what have you done?");
    std::process::abort()
}

impl Session {
    /// Initializes connection with the server.
    pub async fn init() -> Result<(mpsc::UnboundedReceiver<Notification>, Self), HErr> {
        println!("Client connecting to {}", *SERVER_ADDR);
        let stream = TcpStream::connect(*SERVER_ADDR).await?;
        let (mut reader, mut writer) = stream.split();
        let id = login(&mut writer).await?;
        let (sender, receiver) = mpsc::unbounded_channel();
        let sess = Session {
            writer: Qutex::new(writer),
            id,
            // start with capacity because capacity 32 is basically 0 from a load persepective,
            // and more than we're likely to ever need
            pending: Qutex::new(HashMap::with_capacity(32)),
            notifications: sender,
        };
        tokio::spawn({
            let sess = sess.clone();
            async move {
                let comp: Result<(), HErr> = try {
                    loop {
                        sess.handle_server_msg(read_cbor(&mut reader).await?)
                            .await?;
                    }
                };
                if let Err(e) = comp {
                    eprintln!("session ended, code {}", e);
                }
            }
        });
        Ok((receiver, sess))
    }

    pub async fn send_msg(&self, to: UserId, msg: MessageToPeer) -> Result<(), HErr> {
        let push = form_push(to, msg)?;
        self.send_to_server(&push).await
    }

    pub async fn request_meta(&self, of: UserId) -> Result<oneshot::Receiver<Response>, HErr> {
        let query = MessageToServer::RequestMeta { of };
        self.send_query(query).await
    }

    pub async fn register_device(&self) -> Result<oneshot::Receiver<Response>, HErr> {
        let query = MessageToServer::RegisterDevice;
        self.send_query(query).await
    }

    async fn send_to_server(&self, msg: &MessageToServer) -> Result<(), HErr> {
        let mut writer = self
            .writer
            .clone()
            .lock_async()
            .await
            .unwrap_or_else(error_abort);
        send_cbor(writer.deref_mut(), msg).await?;
        Ok(())
    }

    async fn send_query(
        &self,
        query: MessageToServer,
    ) -> Result<oneshot::Receiver<Response>, HErr> {
        let (sender, receiver) = oneshot::channel();
        // this clone looks unnecessary - it's not!
        // insert before send so that we don't have to worry about what happens if a query is
        // responded to before this future is executed again
        self.pending
            .clone()
            .lock_async()
            .await
            .unwrap_or_else(error_abort)
            .insert(query.clone(), sender);
        let mut w = self
            .writer
            .clone()
            .lock_async()
            .await
            .unwrap_or_else(error_abort);
        send_cbor(w.deref_mut(), &query).await?;
        Ok(receiver)
    }

    async fn handle_server_msg(&self, msg: MessageToClient) -> Result<(), HErr> {
        use MessageToClient::*;
        match msg {
            Push { from, body, time } => {
                let push = serde_cbor::from_slice(&body)?;
                let Event {
                    reply,
                    notification,
                } = handle_push(from, push, time)?;
                if let Some(n) = notification {
                    drop(self.notifications.clone().try_send(n));
                }
                if let Some(r) = reply {
                    self.send_to_server(&r).await?;
                }
            }
            QueryResponse { res, query } => self.handle_response(res, &query).await,
        }
        Ok(())
    }

    async fn handle_response(&self, res: Response, query: &MessageToServer) {
        if let Some(s) = self
            .pending
            .clone()
            .lock_async()
            .await
            .unwrap_or_else(error_abort)
            .remove(query)
        {
            drop(s.send(res));
        }
    }
}
