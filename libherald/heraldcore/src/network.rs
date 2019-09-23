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

impl Session {
    fn new() -> Result<
        (
            Receiver<Notification>,
            Receiver<(QID, MessageToServer)>,
            Self,
        ),
        HErr,
    > {
        let (server_sender, server_receiver) = mpsc::unbounded_channel();
        let (notif_sender, notif_receiver) = mpsc::unbounded_channel();
        let sess = Session {
            out: server_sender,
            pending: Arc::new(DashMap::default()),
            notifications: notif_sender,
        };
        Ok((notif_receiver, server_receiver, sess))
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

async fn connect_to_server() -> Result<TcpStream, HErr> {
    Ok(TcpStream::connect(*SERVER_ADDR).await?)
}
