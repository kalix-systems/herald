use crate::{
    abort_err,
    config::{Config, ConfigBuilder},
    errors::HErr::{self, *},
    members, message, message_status,
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
    net::TcpStream,
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
    NewMsg(ConversationId),
    /// An ack has been received.
    Ack(MsgId),
    /// A new contact has been added
    NewContact,
    /// A new conversation has been added
    NewConversation,
}

#[derive(Clone)]
/// `Session` is the main struct for managing networking. Think of it as the sending handle for
/// interacting with the server. It can make queries and it can send messages.
pub struct Session {
    out: Sender<(QID, MessageToServer)>,
    pending: Arc<DashMap<QID, oneshot::Sender<Response>>>,
    notifications: Sender<Notification>,
}

#[async_trait]
impl ProtocolHandler for Session {
    type Error = HErr;
    type From = ();

    async fn handle_message_to_server(
        &self,
        _: Self::From,
        msg: MessageToServer,
    ) -> Result<Response, Self::Error> {
        let qid = utils::rand_id();
        let (sender, receiver) = oneshot::channel();
        self.pending.insert(qid, sender);
        if let Err(_) = self.out.clone().try_send((qid, msg)) {
            self.pending.remove(&qid);
            return Err(RequestDropped);
        }
        receiver.await.map_err(|_| RequestDropped)
    }

    async fn handle_fanout(
        &self,
        from: Self::From,
        fanout: fanout::ToServer,
    ) -> Result<fanout::ServerResponse, Self::Error> {
        let msg = MessageToServer::Fanout(fanout);
        match self.handle_message_to_server(from, msg).await? {
            Response::Fanout(f) => Ok(f),
            r => Err(BadResponse {
                expected: "fanout",
                found: r,
            }),
        }
    }

    async fn handle_pki(
        &self,
        from: Self::From,
        pki: pubkey::ToServer,
    ) -> Result<pubkey::ServerResponse, Self::Error> {
        let msg = MessageToServer::PKI(pki);
        match self.handle_message_to_server(from, msg).await? {
            Response::PKI(f) => Ok(f),
            r => Err(BadResponse {
                expected: "pki",
                found: r,
            }),
        }
    }

    async fn handle_query(
        &self,
        from: Self::From,
        query: query::ToServer,
    ) -> Result<query::ServerResponse, Self::Error> {
        let msg = MessageToServer::Query(query);
        match self.handle_message_to_server(from, msg).await? {
            Response::Query(f) => Ok(f),
            r => Err(BadResponse {
                expected: "query",
                found: r,
            }),
        }
    }
}

#[async_trait]
impl PushHandler for Session {
    async fn handle_push(&self, push: Push) -> Result<(), HErr> {
        use Push::*;
        let Event {
            reply,
            notification,
        } = match push {
            KeyRegistered(k) => unimplemented!(),
            KeyDeprecated(k) => unimplemented!(),
            NewUMessage {
                timestamp,
                from,
                msg,
            } => {
                let ConversationMessage { cid, body } = self.open_umessage(&from, &msg)?;
                self.handle_umessage(timestamp, from, cid, body)?
            }
            NewDMessage {
                timestamp,
                from,
                msg,
            } => {
                self.handle_dmessage(timestamp, from, self.open_dmessage(&from, &msg)?)
                    .await?
            }
        };

        if let Some(notif) = notification {
            self.notifications.clone().try_send(notif);
        }

        if let Some(reply) = reply {
            let msg = self.seal_umessage(&reply)?;
            // TODO: handle non-success responses here?
            self.handle_fanout((), msg).await?;
        }

        Ok(())
    }
}

impl Session {
    pub async fn login() -> Result<(Receiver<Notification>, Self), HErr> {
        use login::*;
        let mut stream = connect_to_server().await?;
        send_cbor(&mut stream, &SessionType::Login).await?;

        let uid = Config::static_id()?;
        let kp = Config::static_keypair()?;
        let gid = GlobalId {
            uid,
            did: *kp.public_key(),
        };

        let to = ToServer::As(gid);
        send_cbor(&mut stream, &to).await?;

        if let ToClient::Sign(bytes) = read_cbor(&mut stream).await? {
            let sig = kp.raw_sign_detached(&bytes);
            let to = ToServer::Sig(sig);
            send_cbor(&mut stream, &to).await?;

            if ToClient::Success != read_cbor(&mut stream).await? {
                return Err(LoginError);
            }
        } else {
            return Err(LoginError);
        }

        let (notifier, output, this) = Self::new();

        this.catchup(&mut stream).await?;

        let (reader, writer) = tokio::io::split(stream);

        tokio::spawn(async move {
            server_sender(output, writer)
                .await
                .expect("connection died")
        });

        tokio::spawn({
            let this = this.clone();
            async move { this.recv_messages(reader).await.expect("connection died") }
        });

        Ok((notifier, this))
    }

    pub async fn register(
        uid: UserId,
        key: Signed<sign::PublicKey>,
    ) -> Result<(Receiver<Notification>, Self), HErr> {
        use register::*;
        let mut stream = connect_to_server().await?;
        send_cbor(&mut stream, &SessionType::Register).await?;

        let to = ToServer::RequestUID(uid);
        send_cbor(&mut stream, &to).await?;
        if ToClient::UIDReady != read_cbor(&mut stream).await? {
            return Err(RegistrationError);
        }

        let to = ToServer::UseKey(key);
        send_cbor(&mut stream, &to).await?;
        if ToClient::Success != read_cbor(&mut stream).await? {
            return Err(RegistrationError);
        }

        let (notifier, output, this) = Self::new();

        let (reader, writer) = tokio::io::split(stream);
        tokio::spawn(async move {
            server_sender(output, writer)
                .await
                .expect("connection died")
        });
        tokio::spawn({
            let this = this.clone();
            async move { this.recv_messages(reader).await.expect("connection died") }
        });

        Ok((notifier, this))
    }

    pub async fn send_umessage(
        &self,
        msg: &ConversationMessage,
    ) -> Result<fanout::ServerResponse, HErr> {
        let fout = self.seal_umessage(msg)?;
        self.handle_fanout((), fout).await
    }

    pub async fn send_dmessage(
        &self,
        msg: &MessageToDevice,
    ) -> Result<fanout::ServerResponse, HErr> {
        unimplemented!()
        // let fout = self.seal_dmessage(msg)?;
        // self.handle_fanout((), fout).await
    }

    async fn catchup<S: AsyncWrite + AsyncRead + Unpin>(&self, s: &mut S) -> Result<(), HErr> {
        use catchup::*;

        send_cbor(s, &ToServer::CatchMeUp).await?;

        let ToClient::Catchup(ps) = read_cbor(s).await?;

        for push in ps {
            self.handle_push(push).await?;
        }

        Ok(())
    }

    fn new() -> (
        Receiver<Notification>,
        Receiver<(QID, MessageToServer)>,
        Self,
    ) {
        let (server_sender, server_receiver) = mpsc::unbounded_channel();
        let (notif_sender, notif_receiver) = mpsc::unbounded_channel();
        let sess = Session {
            out: server_sender,
            pending: Arc::new(DashMap::default()),
            notifications: notif_sender,
        };
        (notif_receiver, server_receiver, sess)
    }

    fn open_umessage(&self, _: &GlobalId, msg: &[u8]) -> Result<ConversationMessage, HErr> {
        Ok(serde_cbor::from_slice(msg)?)
    }

    fn seal_umessage(&self, msg: &ConversationMessage) -> Result<fanout::ToServer, HErr> {
        let to = members::members(&msg.cid)?;
        let msg = Bytes::from(serde_cbor::to_vec(msg)?);
        Ok(fanout::ToServer::UID { to, msg })
    }

    fn open_dmessage(&self, _: &GlobalId, msg: &[u8]) -> Result<MessageToDevice, HErr> {
        Ok(serde_cbor::from_slice(msg)?)
    }

    fn handle_umessage(
        &self,
        t: DateTime<Utc>,
        from: GlobalId,
        cid: ConversationId,
        body: ConversationMessageBody,
    ) -> Result<Event, HErr> {
        use ConversationMessageBody::*;
        let mut event = Event {
            reply: None,
            notification: None,
        };

        match body {
            Message {
                body,
                msg_id,
                op_msg_id,
            } => {
                message::add_message(Some(msg_id), from.uid, &cid, &body, Some(t), &op_msg_id)?;
                event.reply.replace(ConversationMessage {
                    cid,
                    body: Ack(MessageReceipt {
                        update_code: MessageReceiptStatus::Received,
                        message_id: msg_id,
                    }),
                });
            }
            AddRequest => {
                use crate::contact::{self, ContactBuilder};
                event.notification = match contact::by_user_id(from.uid) {
                    Ok(contact) => {
                        if cid != contact.pairwise_conversation {
                            crate::conversation::add_conversation_db(Some(&cid), None, false)?;
                            crate::members::add_member(&cid, from.uid)?;
                            Some(Notification::NewConversation)
                        } else {
                            None
                        }
                    }
                    Err(_) => {
                        ContactBuilder::new(from.uid)
                            .pairwise_conversation(cid)
                            .add()?;
                        Some(Notification::NewContact)
                    }
                };
            }
            AddResponse(accepted) => unimplemented!(),
            Ack(receipt) => {
                message_status::set_message_status(receipt.message_id, cid, receipt.update_code)?;
                event
                    .notification
                    .replace(Notification::Ack(receipt.message_id));
            }
        }

        Ok(event)
    }

    async fn handle_dmessage(
        &self,
        t: DateTime<Utc>,
        from: GlobalId,
        msg: MessageToDevice,
    ) -> Result<Event, HErr> {
        match msg {}
    }

    async fn recv_messages<R: AsyncRead + Unpin>(&self, mut reader: R) -> Result<(), HErr> {
        loop {
            match read_cbor(&mut reader).await? {
                MessageToClient::Push(p) => self.handle_push(p).await?,
                MessageToClient::Response(mid, res) => {
                    if let Some((_, sender)) = self.pending.remove(&mid) {
                        sender.send(res);
                    }
                }
            }
        }
        Ok(())
    }
}

async fn server_sender<W: AsyncWrite + Unpin>(
    mut to_send: Receiver<(QID, MessageToServer)>,
    mut writer: W,
) -> Result<(), HErr> {
    while let Some((q, msg)) = to_send.recv().await {
        writer.write_all(&q).await?;
        let out = serde_cbor::to_vec(&msg)?;
        writer.write_all(&out).await?;
    }
    Ok(())
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

async fn connect_to_server() -> Result<TcpStream, HErr> {
    Ok(TcpStream::connect(*SERVER_ADDR).await?)
}

struct Event {
    reply: Option<ConversationMessage>,
    notification: Option<Notification>,
}

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
//     message::add_message(
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
//     use crate::contact::ContactBuilder;

//     let notification = match crate::contact::by_user_id(from.as_str()) {
//         Ok(contact) => {
//             if conversation_id != contact.pairwise_conversation {
//                 crate::conversation::add_conversation(Some(&conversation_id), None)?;
//                 crate::members::add_member(&conversation_id, from.as_str())?;
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
//     message_status::set_message_status(message_id, conv_id, update_code)?;
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

// async fn login<S: AsyncWrite + Unpin>(stream: &mut S) -> Result<GlobalId, HErr> {
//     // TODO: replace this with a static global_id instead
//     let gid = GlobalId {
//         did: 0,
//         uid: Config::static_id()?,
//     };
//     send_cbor(stream, &gid).await?;
//     Ok(gid)
// }

// impl Session {
//     async fn send_to_server(&self, msg: &MessageToServer) -> Result<(), HErr> {
//         let mut writer = abort_err!(self.writer.clone().lock_async().await);
//         send_cbor(writer.deref_mut(), msg).await?;
//         Ok(())
//     }

//     async fn send_query(&self, query: MessageToServer) -> Result<Response, HErr> {
//         let (sender, receiver) = oneshot::channel();
//         // this clone looks unnecessary - it's not!
//         // insert before send so that we don't have to worry about what happens if a query is
//         // responded to before this future is executed again
//         {
//             let mut p = abort_err!(self.pending.clone().lock_async().await);
//             p.insert(query.clone(), sender);
//         }
//         {
//             let mut w = abort_err!(self.writer.clone().lock_async().await);
//             send_cbor(w.deref_mut(), &query).await?;
//         }
//         receiver.await.map_err(|e| {
//             HeraldError(format!(
//                 "query sender was dropped, query was {:?}, error was {}",
//                 query, e,
//             ))
//         })
//     }

//     async fn handle_server_msg(&self, msg: MessageToClient) -> Result<(), HErr> {
//         use MessageToClient::*;
//         match msg {
//             Push { from, body, time } => {
//                 let push = serde_cbor::from_slice(&body)?;
//                 let Event {
//                     reply,
//                     notification,
//                 } = handle_push(from, push, time)?;
//                 if let Some(n) = notification {
//                     drop(self.notifications.clone().try_send(n));
//                 }
//                 if let Some(r) = reply {
//                     self.send_to_server(&r).await?;
//                 }
//             }
//             QueryResponse { res, query } => self.handle_response(res, &query).await,
//         }
//         Ok(())
//     }

//     async fn handle_response(&self, res: Response, query: &MessageToServer) {
//         if let Some(s) = abort_err!(self.pending.clone().lock_async().await).remove(query) {
//             drop(s.send(res));
//         }
//     }
// }
