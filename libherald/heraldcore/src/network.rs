use crate::{config::Config, errors::HErr, message as db};
// use ccl::dashmap::DashMap;
use chrono::prelude::*;
use herald_common::{
    read_cbor, send_cbor, ClientMessageAck, GlobalId, MessageStatus, MessageToClient,
    MessageToPeer, MessageToServer, RawMsg, Response, UserId,
};
use lazy_static::*;
use serde::Serialize;
use std::sync::Arc;
use std::{
    collections::HashMap,
    env,
    net::{SocketAddr, SocketAddrV4},
};
use tokio::{
    net::*,
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

// TODO: consider making this async?
// sqlite is fast so might not be worth it on an ssd
fn handle_msg(from: UserId, body: String, time: DateTime<Utc>) -> Result<MessageToServer, HErr> {
    let recipient = Config::static_id()?;
    let (row, _) = db::Messages::add_message(
        &from,
        &recipient,
        &body,
        Some(time),
        MessageStatus::Inbound, //all messages are inbound
    )?;
    form_push(from, form_ack(MessageStatus::RecipReceivedAck, row))
}

fn handle_ack(from: UserId, ack: ClientMessageAck) -> Result<(), HErr> {
    let ClientMessageAck {
        message_id,
        update_code,
    } = ack;
    db::Messages::update_status(from.as_str(), message_id, update_code)?;
    Ok(())
}

fn handle_push(
    from: GlobalId,
    body: MessageToPeer,
    time: DateTime<Utc>,
) -> Result<Option<MessageToServer>, HErr> {
    use MessageToPeer::*;
    match body {
        Message(s) => handle_msg(from.uid, s, time).map(Some),
        Ack(a) => handle_ack(from.uid, a).map(|_| None),
    }
}

// TODO: make this handle request responses - that'll end up making this more object-y
pub fn handle_server_msg(msg: MessageToClient) -> Result<Option<MessageToServer>, HErr> {
    use MessageToClient::*;
    match msg {
        Push { from, body, time } => {
            let msg = serde_cbor::from_slice(&body)?;
            handle_push(from, msg, time)
        }
        _ => unimplemented!(),
    }
}

pub enum Notification {
    // TODO: include conversation id's here
    // issue: #15
    NewMsg(UserId),
    Ack(ClientMessageAck),
}

pub struct Session {
    connection: TcpStream,
    id: GlobalId,
    pending: HashMap<MessageToServer, oneshot::Sender<Response>>,
    notifications: mpsc::UnboundedSender<Notification>,
}

/// Login
pub async fn login<S: AsyncWrite + Unpin>(stream: &mut S) -> Result<GlobalId, HErr> {
    let gid = GlobalId {
        did: 0,
        uid: Config::static_id()?,
    };
    send_cbor(stream, &gid).await?;
    Ok(gid)
}

impl Session {
    /// Initializes connection with the server.
    pub async fn init() -> Result<(mpsc::UnboundedReceiver<Notification>, Self), HErr> {
        println!("Client connecting to {}", *SERVER_ADDR);
        let mut connection = TcpStream::connect(*SERVER_ADDR).await?;
        let id = login(&mut connection).await?;
        let (sender, receiver) = mpsc::unbounded_channel();
        Ok((
            receiver,
            Session {
                connection,
                id,
                pending: HashMap::new(),
                notifications: sender,
            },
        ))
    }

    fn handle_response(&mut self, res: Response, query: &MessageToServer) {
        if let Some(s) = self.pending.remove(query) {
            s.send(res);
        }
    }

    async fn send_query(
        &mut self,
        query: MessageToServer,
    ) -> Result<oneshot::Receiver<Response>, HErr> {
        let (sender, receiver) = oneshot::channel();
        // this clone looks unnecessary - it's not!
        // insert before send so that we don't have to worry about what happens if a query is
        // responded to before this future is executed again
        self.pending.insert(query.clone(), sender);
        send_cbor(&mut self.connection, &query).await?;
        Ok(receiver)
    }

    pub async fn request_meta(&mut self, of: UserId) -> Result<oneshot::Receiver<Response>, HErr> {
        let query = MessageToServer::RequestMeta { of };
        self.send_query(query).await
    }

    pub async fn RegisterDevice(&mut self) -> Result<oneshot::Receiver<Response>, HErr> {
        let query = MessageToServer::RegisterDevice;
        self.send_query(query).await
    }
}
