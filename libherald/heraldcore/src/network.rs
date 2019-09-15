use crate::{
    config::Config,
    errors::HErr::{self, *},
    message, message_status,
    types::*,
};
use chrono::prelude::*;
use herald_common::*;
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

#[derive(Debug)]
/// `Notification`s contain info about what updates were made to the db.
pub enum Notification {
    /// A new message has been received.
    NewMsg(UserId),
    /// An ack has been received.
    Ack(MessageReceipt),
}

#[derive(Clone)]
/// `Session` is the main struct for managing networking. Think of it as the sending handle for
/// interacting with the server. It can make queries and it can send messages.
pub struct Session {
    writer: Qutex<TcpStreamWriteHalf>,
    id: GlobalId,
    pending: Qutex<HashMap<MessageToServer, oneshot::Sender<Response>>>,
    notifications: mpsc::UnboundedSender<Notification>,
}

impl Session {
    /// Initalizes connection with the server and login.
    #[allow(unreachable_code)]
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

    /// Sends a `MessageToPeer` through the server.
    pub async fn send_msg(&self, to: UserId, msg: MessageToPeer) -> Result<(), HErr> {
        let push = form_push(to, msg)?;
        self.send_to_server(&push).await
    }

    /// Requests the metadata of a user, asynchronously returns the response.
    pub async fn request_meta(&self, of: UserId) -> Result<User, HErr> {
        match self
            .send_query(MessageToServer::RequestMeta { of: of.clone() })
            .await?
        {
            Response::Meta(u) => Ok(u),
            Response::DataNotFound => Err(InvalidUserId(of)),
            r => Err(HeraldError(format!(
                "bad response to metadata request from server from user {} - response was {:?}",
                of, r
            ))),
        }
    }

    /// Registers a new device and returns a future which will contain the new `DeviceId`.
    pub async fn register_device(&self) -> Result<DeviceId, HErr> {
        match self.send_query(MessageToServer::RegisterDevice).await? {
            Response::DeviceRegistered(d) => Ok(d),
            r => Err(HeraldError(format!(
                "bad response to device registry request from server - respones was {:?}",
                r
            ))),
        }
    }
}

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

struct Event {
    reply: Option<MessageToServer>,
    notification: Option<Notification>,
}

fn form_ack(update_code: MessageReceiptStatus, message_id: MsgId) -> MessageToPeer {
    let ack = MessageReceipt {
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

fn handle_msg(
    msg_id: MsgId,
    author: UserId,
    conversation_id: ConversationId,
    body: String,
    time: DateTime<Utc>,
    op_msg_id: Option<MsgId>,
) -> Result<Event, HErr> {
    message::Messages::add_message(
        Some(msg_id),
        &author,
        &conversation_id,
        &body,
        Some(time),
        &op_msg_id,
    )?;

    let reply = form_push(
        author.clone(),
        form_ack(MessageReceiptStatus::Received, msg_id),
    )?;
    let notification = Notification::NewMsg(author);
    Ok(Event {
        reply: Some(reply),
        notification: Some(notification),
    })
}

fn handle_add_request(from: UserId, conversation_id: ConversationId) -> Result<Event, HErr> {
    use crate::contact::ContactBuilder;

    let contact = ContactBuilder::new((&from).clone())
        .pairwise_conversation(conversation_id)
        .add()?;

    let reply = Some(form_push(
        from.clone(),
        MessageToPeer::AddResponse(contact.pairwise_conversation, true),
    )?);

    Ok(Event {
        reply,
        notification: None,
    })
}

// TODO should this do something?
fn handle_add_response(_: ConversationId, _: bool) -> Result<Event, HErr> {
    Ok(Event {
        reply: None,
        notification: None,
    })
}

fn handle_ack(from: UserId, ack: MessageReceipt) -> Result<Event, HErr> {
    let MessageReceipt {
        message_id,
        update_code,
    } = ack;
    message_status::MessageStatus::set_message_status(message_id, from.as_str(), update_code)?;
    Ok(Event {
        reply: None,
        notification: None,
    })
}

fn handle_push(author: GlobalId, body: MessageToPeer, time: DateTime<Utc>) -> Result<Event, HErr> {
    use MessageToPeer::*;
    match body {
        Message {
            body,
            msg_id,
            op_msg_id,
            conversation_id,
        } => handle_msg(msg_id, author.uid, conversation_id, body, time, op_msg_id),
        AddRequest(conversation_id) => handle_add_request(author.uid, conversation_id),
        AddResponse(_conversation_id, _accepted) => {
            handle_add_response(_conversation_id, _accepted)
        }
        Ack(a) => handle_ack(author.uid, a),
    }
}

async fn login<S: AsyncWrite + Unpin>(stream: &mut S) -> Result<GlobalId, HErr> {
    // TODO: replace this with a static global_id instead
    let gid = GlobalId {
        did: 0,
        uid: Config::static_id()?,
    };
    send_cbor(stream, &gid).await?;
    Ok(gid)
}

// TODO: replace this with a macro?
fn impossible_error<E: std::fmt::Display, T>(e: E) -> T {
    eprintln!("an impossible error happened");
    eprintln!("message was: {}", e);
    eprintln!("what have you done?");
    std::process::abort()
}

impl Session {
    async fn send_to_server(&self, msg: &MessageToServer) -> Result<(), HErr> {
        let mut writer = self
            .writer
            .clone()
            .lock_async()
            .await
            .unwrap_or_else(impossible_error);
        send_cbor(writer.deref_mut(), msg).await?;
        Ok(())
    }

    async fn send_query(&self, query: MessageToServer) -> Result<Response, HErr> {
        let (sender, receiver) = oneshot::channel();
        // this clone looks unnecessary - it's not!
        // insert before send so that we don't have to worry about what happens if a query is
        // responded to before this future is executed again
        {
            let mut p = self
                .pending
                .clone()
                .lock_async()
                .await
                .unwrap_or_else(impossible_error);
            p.insert(query.clone(), sender);
        }
        {
            let mut w = self
                .writer
                .clone()
                .lock_async()
                .await
                .unwrap_or_else(impossible_error);
            send_cbor(w.deref_mut(), &query).await?;
        }
        receiver.await.map_err(|e| {
            HeraldError(format!(
                "query sender was dropped, query was {:?}, error was {}",
                query, e,
            ))
        })
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
            .unwrap_or_else(impossible_error)
            .remove(query)
        {
            drop(s.send(res));
        }
    }
}
