use crate::{
    abort_err,
    config::{Config, ConfigBuilder},
    errors::HErr::{self, *},
    message, message_status,
    types::*,
    utils,
};
use chrono::prelude::*;
use dashmap::DashMap;
use herald_common::*;
use lazy_static::*;
use sodiumoxide::{
    crypto::{box_, generichash as hash, sealedbox, sign},
    randombytes::randombytes_into,
};
use std::{
    collections::HashMap,
    env,
    net::{SocketAddr, SocketAddrV4},
    ops::DerefMut,
    sync::Arc,
};
use tokio::{
    net::{tcp::split::TcpStreamWriteHalf, *},
    prelude::*,
    sync::{
        mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender},
        oneshot,
    },
};

pub type QID = [u8; 32];

#[derive(Debug)]
/// `Notification`s contain info about what updates were made to the db.
pub enum Notification {
    /// A new message has been received.
    NewMsg(UserId),
    /// An ack has been received.
    Ack(MessageReceipt),
    /// A new contact has been added
    NewContact,
    /// A new conversation has been added
    NewConversation,
}

#[derive(Clone)]
/// `Session` is the main struct for managing networking. Think of it as the sending handle for
/// interacting with the server. It can make queries and it can send messages.
pub struct Session {
    out: Sender<MessageToServer>,
    pending: Arc<DashMap<QID, oneshot::Sender<Response>>>,
    notifications: Sender<Notification>,
}

impl Session {
    pub async fn new() -> Result<(Receiver<Notification>, Receiver<MessageToServer>, Self), HErr> {
        let (server_sender, server_receiver) = mpsc::unbounded_channel();
        let (notif_sender, notif_receiver) = mpsc::unbounded_channel();
        let sess = Session {
            out: server_sender,
            pending: Arc::new(DashMap::default()),
            notifications: notif_sender,
        };
        Ok((notif_receiver, server_receiver, sess))
    }

    pub fn setup_streams(self, mut outgoing: Receiver<MessageToServer>, stream: TcpStream) {
        let (mut reader, mut writer) = stream.split();
        tokio::spawn({
            async move {
                let comp: Result<(), HErr> = try {
                    let mut res = Ok(());
                    while res.is_ok() {
                        res = self.handle_server_msg(read_cbor(&mut reader).await?).await;
                    }
                    res?;
                };
                if let Err(e) = comp {
                    eprintln!("session ended, code {}", e);
                }
            }
        });
        tokio::spawn(async move {
            while let Some(msg) = outgoing.recv().await {
                if let Err(e) = send_cbor(&mut writer, &msg).await {
                    eprintln!("failed to write data - assuming connection is closed");
                    eprintln!("error was: {}", e);
                }
            }
        });
    }

    pub async fn login<S: AsyncWrite + AsyncRead + Unpin>(
        &self,
        stream: &mut S,
    ) -> Result<(), HErr> {
        send_cbor(stream, &SessionType::Login).await?;
        let sk = Config::static_secretkey()?;
        let uid = Config::static_id()?;
        let did = Config::static_publickey()?;
        let gid = GlobalId { uid, did };
        send_cbor(stream, &gid).await?;

        let mut buf = [0u8; 3];
        stream.read_exact(&mut buf).await?;
        if buf != *b"YES" {
            return Err(LoginError);
        }

        let mut buf = [0u8; 32];
        stream.read_exact(&mut buf).await?;
        let sig = sign::sign_detached(&buf, &sk);
        stream.write_all(sig.as_ref()).await?;

        let mut buf = [0u8; 3];
        stream.read_exact(&mut buf).await?;
        if buf != *b"YES" {
            return Err(LoginError);
        }

        // TODO: this should probably be a single db transaction
        if let MessageToClient::Catchup(p) = read_cbor(stream).await? {
            let mut replies = Vec::new();
            for push in p {
                replies.append(&mut self.handle_push(push).await?);
            }
            self.send_to_server(MessageToServer::CaughtUp).await?;
            for reply in replies {
                let msg = self.reply_to_server_msg(&reply).await?;
                self.send_to_server(msg).await?;
            }
        } else {
            return Err(HeraldError("missing catchup".into()));
        }

        Ok(())
    }

    pub async fn try_register<S: AsyncWrite + AsyncRead + Unpin>(
        &self,
        stream: &mut S,
        uid: UserId,
        pair: sig::KeyPair,
    ) -> Result<bool, HErr> {
        send_cbor(stream, &uid).await?;
        let mut buf = [0u8; 3];
        stream.read_exact(&mut buf).await?;
        if buf != *b"YES" {
            return Ok(false);
        }
        let signed = pair.sign(*pair.public_key());
        send_cbor(stream, &signed).await?;
        if buf != *b"YES" {
            return Ok(false);
        }
        ConfigBuilder::new().id(uid).keypair(pair).add()?;
        Ok(true)
    }

    /// Sends a `MessageToPeer` through the server.
    pub async fn send_msg(&self, to: UserId, msg: MessageToPeer) -> Result<(), HErr> {
        let push = unimplemented!();
        self.send_to_server(push).await
    }

    /// Requests the metadata of a user, asynchronously returns the response.
    pub async fn request_meta(&self, of: UserId) -> Result<UserMeta, HErr> {
        let qid = utils::rand_id();
        match self
            .send_query(
                qid,
                MessageToServer::RequestMeta {
                    qid,
                    of: of.clone(),
                },
            )
            .await?
        {
            Response::Meta(u) => Ok(u),
            Response::DataNotFound => Err(InvalidUserId(of)),
            r => Err(HeraldError(format!(
                "bad response to metadata request from server - response was {:?}",
                r
            ))),
        }
    }

    /// Registers a new device and returns a future which will contain the new `DeviceId`.
    pub async fn register_device(
        &self,
        key: Signed<sig::PublicKey>,
    ) -> Result<sign::PublicKey, HErr> {
        let qid = utils::rand_id();
        match self
            .send_query(qid, MessageToServer::RegisterDevice { qid, key })
            .await?
        {
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

// struct Event {
//     reply: Option<MessageToServer>,
//     notification: Option<Notification>,
// }

// fn form_ack(
//     update_code: MessageReceiptStatus,
//     conv_id: ConversationId,
//     message_id: MsgId,
// ) -> MessageToPeer {
//     let ack = MessageReceipt {
//         update_code,
//         message_id,
//     };
//     MessageToPeer::Ack(conv_id, ack)
// }

// // note: this should never fail, but I'm returning a result until I read `serde_cbor` more closely
// fn form_push(to: UserId, msg: MessageToPeer) -> Result<MessageToServer, HErr> {
//     Ok(MessageToServer::SendMsg {
//         to,
//         body: serde_cbor::to_vec(&msg)?.into(),
//     })
// }

// fn handle_msg(
//     msg_id: MsgId,
//     author: UserId,
//     conversation_id: ConversationId,
//     body: String,
//     time: DateTime<Utc>,
//     op_msg_id: Option<MsgId>,
// ) -> Result<Event, HErr> {
//     let db = crate::db::Database::get()?;
//     message::add_message(
//         &db,
//         Some(msg_id),
//         &author,
//         &conversation_id,
//         &body,
//         Some(time),
//         &op_msg_id,
//     )?;

//     let reply = form_push(
//         author.clone(),
//         form_ack(MessageReceiptStatus::Received, conversation_id, msg_id),
//     )?;
//     let notification = Notification::NewMsg(author);
//     Ok(Event {
//         reply: Some(reply),
//         notification: Some(notification),
//     })
// }

// fn handle_add_request(from: UserId, conversation_id: ConversationId) -> Result<Event, HErr> {
//     use crate::{
//         contact::{ContactBuilder, ContactsHandle},
//         conversation::Conversations,
//     };
//     let handle = ContactsHandle::new()?;

//     let notification = match handle.by_user_id(from.as_str()) {
//         Ok(contact) => {
//             if conversation_id != contact.pairwise_conversation {
//                 let conv_handle = Conversations::new()?;
//                 conv_handle.add_conversation(Some(&conversation_id), None)?;
//                 conv_handle.add_member(&conversation_id, from.as_str())?;
//                 Some(Notification::NewConversation)
//             } else {
//                 None
//             }
//         }
//         Err(_) => {
//             ContactBuilder::new((&from).clone())
//                 .pairwise_conversation(conversation_id)
//                 .add()?;
//             Some(Notification::NewContact)
//         }
//     };

//     let reply = Some(form_push(
//         from.clone(),
//         MessageToPeer::AddResponse(conversation_id, true),
//     )?);

//     Ok(Event {
//         reply,
//         notification,
//     })
// }

// // TODO this should do something
// fn handle_add_response(_: ConversationId, _: bool) -> Result<Event, HErr> {
//     Ok(Event {
//         reply: None,
//         notification: None,
//     })
// }

// fn handle_ack(conv_id: ConversationId, ack: MessageReceipt) -> Result<Event, HErr> {
//     let MessageReceipt {
//         message_id,
//         update_code,
//     } = ack;
//     let db = crate::db::Database::get()?;
//     message_status::set_message_status(&db, message_id, conv_id, update_code)?;
//     Ok(Event {
//         reply: None,
//         notification: None,
//     })
// }

// fn handle_push(author: GlobalId, body: MessageToPeer, time: DateTime<Utc>) -> Result<Event, HErr> {
//     use MessageToPeer::*;
//     match body {
//         Message {
//             body,
//             msg_id,
//             op_msg_id,
//             conversation_id,
//         } => handle_msg(msg_id, author.uid, conversation_id, body, time, op_msg_id),
//         AddRequest(conversation_id) => handle_add_request(author.uid, conversation_id),
//         AddResponse(_conversation_id, _accepted) => {
//             handle_add_response(_conversation_id, _accepted)
//         }
//         Ack(conv_id, a) => handle_ack(conv_id, a),
//     }
// }

impl Session {
    async fn handle_push(&self, push: Push) -> Result<Vec<Reply>, HErr> {
        unimplemented!()
    }

    /// Encrypts `msg` as a `Block` or `Blob`, depending on the value of `msg`.
    async fn reply_to_server_msg(&self, msg: &Reply) -> Result<MessageToServer, HErr> {
        unimplemented!()
    }

    async fn send_to_server(&self, msg: MessageToServer) -> Result<(), HErr> {
        self.out
            .clone()
            .try_send(msg)
            // TODO: more sensible error here???
            .map_err(|_| HeraldError("network closed".into()))?;
        Ok(())
    }

    async fn send_query(&self, qid: QID, query: MessageToServer) -> Result<Response, HErr> {
        let (sender, receiver) = oneshot::channel();
        // this clone looks unnecessary - it's not!
        // insert before send so that we don't have to worry about what happens if a query is
        // responded to before this future is executed again
        self.pending.insert(qid, sender);
        self.send_to_server(query).await?;
        receiver
            .await
            .map_err(|e| HeraldError(format!("query sender was dropped, error was {}", e,)))
    }

    async fn handle_server_msg(&self, msg: MessageToClient) -> Result<(), HErr> {
        unimplemented!()
        // use MessageToClient::*;
        // match msg {
        //     Push { from, body, time } => {
        //         let push = serde_cbor::from_slice(&body)?;
        //         let Event {
        //             reply,
        //             notification,
        //         } = handle_push(from, push, time)?;
        //         if let Some(n) = notification {
        //             drop(self.notifications.clone().try_send(n));
        //         }
        //         if let Some(r) = reply {
        //             self.send_to_server(&r).await?;
        //         }
        //     }
        //     QueryResponse { res, query } => self.handle_response(res, &query).await,
        // }
        // Ok(())
    }

    async fn handle_response(&self, qid: QID, res: Response, query: &MessageToServer) {
        if let Some(s) = self.pending.remove(&qid) {
            drop(s.1.send(res));
        }
    }
}
