use crate::{prelude::*, store::*};

use dashmap::DashMap;
use qutex::Qutex;
use sodiumoxide::crypto::sign;
use tokio::{
    net::TcpStream,
    prelude::*,
    sync::mpsc::{
        unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
    },
};
// use tokio_io::split::{split, WriteHalf};
use womp::womp;

pub struct State {
    active: DashMap<sig::PublicKey, Sender<MessageToClient>>,
    redis: redis::Client,
}

impl State {
    pub fn new<T: redis::IntoConnectionInfo>(redisparams: T) -> Result<Self, Error> {
        sodiumoxide::init().expect("failed to init libsodium");
        Ok(State {
            active: DashMap::default(),
            redis: redis::Client::open(redisparams)?,
        })
    }

    fn new_connection(&self) -> Result<redis::Connection, Error> {
        Ok(self.redis.get_connection()?)
    }

    async fn send_push<C: redis::ConnectionLike>(
        &self,
        con: &mut C,
        to: sign::PublicKey,
        msg: Push,
    ) -> Result<(), Error> {
        if let Some(a) = self.active.async_get(to).await {
            let mut sender = a.clone();
            if let Err(m) = sender.try_send(MessageToClient::Push(msg)) {
                if let MessageToClient::Push(p) = m.into_inner() {
                    con.add_pending(to, p)?;
                }
            }
        } else {
            con.add_pending(to, msg)?;
        }
        Ok(())
    }

    async fn send_response(
        &self,
        to: sign::PublicKey,
        mid: [u8; 32],
        res: Response,
    ) -> Result<(), Error> {
        if let Some(a) = self.active.async_get(to).await {
            a.clone()
                .try_send(MessageToClient::Response(mid, res))
                .map_err(|_| MissingData)
        } else {
            Err(MissingData)
        }
    }
}

impl State {
    fn authenticated_session<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        &'static self,
        gid: GlobalId,
        stream: S,
    ) {
        let (sender, receiver) = channel();
        self.active.insert(gid.did, sender);

        let (reader, writer) = tokio::io::split(stream);
        self.spawn_msg_sender(gid.did, writer, receiver);
        self.recv_messages(gid, reader);
    }

    fn spawn_msg_sender<W: AsyncWrite + Unpin + Send + 'static>(
        &'static self,
        pk: sig::PublicKey,
        mut writer: W,
        mut input: Receiver<MessageToClient>,
    ) {
        tokio::spawn(async move {
            while let Some(msg) = input.recv().await {
                if let Err(e) = send_cbor(&mut writer, &msg).await {
                    eprintln!("failed to write data - assuming connection is closed");
                    eprintln!("error was: {}", e);
                    self.active.remove(&pk);
                    break;
                }
            }

            let mut con = self.new_connection().expect(womp!());
            while let Some(MessageToClient::Push(msg)) = input.recv().await {
                // TODO (HIGH PRIORITY): handle retry logic here
                con.add_pending(pk, msg)
                    .expect("MAJOR ERROR: failed to add pending message");
            }
        });
    }

    fn recv_messages<R: AsyncRead + Unpin + Send + 'static>(
        &'static self,
        gid: GlobalId,
        mut reader: R,
    ) {
        tokio::spawn(async move {
            loop {
                let mut mid = [0u8; 32];
                reader
                    .read_exact(&mut mid)
                    .await
                    .expect("failed to read message id");
                let m = read_cbor(&mut reader)
                    .await
                    .expect("failed to read cbor data");
                let res = self
                    .handle_message_to_server(gid, m)
                    .await
                    .expect(&format!("failed to handle message from gid {:?}", gid));
                self.send_response(gid.did, mid, res).await.expect(&format!(
                    "failed to send response to gid {:?}, though this may not be a problem",
                    gid
                ));
            }
        });
    }
}

#[async_trait]
impl ProtocolHandler for State {
    type Error = Error;
    type From = GlobalId;
    async fn handle_fanout(
        &self,
        from: Self::From,
        fanout: fanout::ToServer,
    ) -> Result<fanout::ServerResponse, Error> {
        use fanout::*;
        let mut con = self.new_connection()?;
        match fanout {
            ToServer::UID { to, msg } => {
                let missing: Vec<UserId> = to
                    .iter()
                    .filter(|u| con.user_exists(u).unwrap_or(false))
                    .map(|u| *u)
                    .collect();
                if missing.is_empty() {
                    Ok(ServerResponse::MissingUIDs(missing))
                } else {
                    let data = Push::NewUMessage { from, msg: msg };
                    for uid in to {
                        for did in con.read_meta(&uid)?.valid_keys() {
                            // TODO: replace this w/a tokio spawn for reliability reasons
                            self.send_push(&mut con, did, data.clone()).await?;
                        }
                    }
                    Ok(ServerResponse::Success)
                }
            }
            ToServer::DID { to, msg } => {
                let missing: Vec<sign::PublicKey> = to
                    .iter()
                    .filter(|d| con.device_exists(d).unwrap_or(false))
                    .map(|d| *d)
                    .collect();
                if missing.is_empty() {
                    Ok(ServerResponse::MissingDIDs(missing))
                } else {
                    for did in to {
                        let data = Push::NewDMessage {
                            from,
                            msg: msg.clone(),
                        };
                        // TODO: replace this w/a tokio spawn for reliability reasons
                        self.send_push(&mut con, did, data).await?;
                    }
                    Ok(ServerResponse::Success)
                }
            }
        }
    }

    async fn handle_pki(
        &self,
        from: Self::From,
        msg: pubkey::ToServer,
    ) -> Result<pubkey::ServerResponse, Error> {
        use pubkey::{ServerResponse::*, ToServer::*};
        let mut con = self.new_connection()?;
        match msg {
            RegisterKey(spk) => {
                if from.did == *spk.signed_by() && spk.verify_sig() {
                    if con.add_key(&from.uid, spk)? {
                        Ok(Success)
                    } else {
                        Ok(Redundant)
                    }
                } else {
                    Ok(BadSignature)
                }
            }
            DeprecateKey(spk) => {
                if from.did == *spk.signed_by() && spk.verify_sig() {
                    if con.deprecate_key(&from.uid, spk)? {
                        Ok(Success)
                    } else {
                        Ok(Redundant)
                    }
                } else {
                    Ok(BadSignature)
                }
            }
            RegisterPrekey(spk) => {
                if from.did == *spk.signed_by() && spk.verify_sig() {
                    if con.add_prekey(from.did, spk)? {
                        Ok(Success)
                    } else {
                        Ok(Redundant)
                    }
                } else {
                    Ok(BadSignature)
                }
            }
        }
    }

    async fn handle_query(
        &self,
        from: Self::From,
        query: query::ToServer,
    ) -> Result<query::ServerResponse, Error> {
        use query::{ServerResponse::*, ToServer::*};
        let mut con = self.new_connection()?;
        match query {
            UserExists(uid) => con.user_exists(&uid).map(Exists),
            UserKeys(uid) => Ok(con.read_meta(&uid).map(Keys).unwrap_or(MissingData)),
            GetKeyMeta(uid, pk) => Ok(con.read_key(&uid, pk).map(KeyMeta).unwrap_or(MissingData)),
            GetPrekey(pk) => Ok(con.get_prekey(pk).map(PreKey).unwrap_or(MissingData)),
        }
    }
}

// impl State {
// pub async fn send_message(
//     &self,
//     con: &mut redis::Connection,
//     to: &GlobalId,
//     msg: MessageToClient,
// ) -> Result<(), Error> {
//     use MessageToClient::*;
//     if let Some(a) = self.active.async_get(to.did).await {
//         let mut sender = a.clone();
//         if let Err(m) = sender.try_send(msg) {
//             if let Push(p) = m.into_inner() {
//                 con.add_pending(to.did, p)?;
//             }
//         }
//     } else if con.user_exists(&to.uid)? {
//         if let Push(p) = msg {
//             con.add_pending(to.did, p)?;
//         } else if let Catchup(_) = msg {
//             return Err(CatchupFailed);
//         }
//     } else {
//         return Err(UnknownUser(to.uid.clone()));
//     }
//     Ok(())
// }

// pub async fn handle_stream(&'static self, mut stream: TcpStream) -> Result<(), Error> {
//     let mut buf = [0u8; 1];
//     stream.read_exact(&mut buf).await?;
//     match buf[0].try_into() {
//         Ok(SessionType::Register) => self.registration_session(stream).await,
//         Ok(SessionType::Login) => self.login_session(stream).await,
//         Err(_) => {
//             stream
//                 .write_all(b"invalid session type - expected 0 or 1")
//                 .await?;
//             Ok(())
//         }
//     }
// }

// async fn registration_session(&'static self, mut stream: TcpStream) -> Result<(), Error> {
//     let mut con = self.new_connection()?;
//     let mut done = false;
//     let gid = {
//         let mut gid: Option<GlobalId> = None;
//         let mut comp: Result<(), Error> = Ok(());
//         while !done && comp.is_ok() {
//             let uid: UserId = read_cbor(&mut stream).await?;
//             // TODO: actually make this transactional
//             comp = try {
//                 if con.user_exists(&uid)? {
//                     stream.write_all(b"ERR").await?;
//                 } else {
//                     stream.write_all(b"YES").await?;
//                     let s: Signed<sig::PublicKey> = read_cbor(&mut stream).await?;
//                     if s.verify_sig() {
//                         let p = *s.data();
//                         con.add_key(&uid, s)?;
//                         done = true;
//                         gid = Some(GlobalId { uid, did: p });
//                     }
//                 }
//             };
//         }
//         gid.ok_or(CommandFailed)?
//     };
//     stream.write_all(b"YES").await?;

//     self.authenticated_session(con, gid, stream).await
// }

// async fn login_session(&'static self, mut stream: TcpStream) -> Result<(), Error> {
//     let gid: GlobalId = read_cbor(&mut stream).await?;
//     if !self.new_connection()?.key_is_valid(&gid.uid, gid.did)? {
//         stream.write_all(b"ERR").await?;
//         return Err(InvalidKey);
//     } else {
//         stream.write_all(b"YES").await?;
//     }

//     let mut dat = [0u8; 32];
//     sodiumoxide::randombytes::randombytes_into(&mut dat);
//     stream.write_all(&dat).await?;

//     let mut buf = [0u8; sign::SIGNATUREBYTES];
//     stream.read_exact(&mut buf).await?;
//     let sig = sign::Signature(buf);

//     if !sign::verify_detached(&sig, &dat, &gid.did) {
//         stream.write_all(b"ERR").await?;
//         return Err(InvalidSig);
//     } else {
//         stream.write_all(b"YES").await?;
//     }

//     let mut con = self.new_connection()?;
//     let pending = con.get_pending(gid.did)?;
//     let push = MessageToClient::Catchup(pending);
//     send_cbor(&mut stream, &push).await?;
//     if MessageToServer::CaughtUp != read_cbor(&mut stream).await? {
//         return Err(CatchupFailed);
//     }
//     con.expire_pending(gid.did)?;

//     self.authenticated_session(con, gid, stream).await
// }

// async fn authenticated_session(
//     &'static self,
//     con: redis::Connection,
//     gid: GlobalId,
//     stream: TcpStream,
// ) -> Result<(), Error> {
//     let (sender, receiver) = mpsc::unbounded_channel();
//     self.active.insert(gid.did, sender);

//     let (reader, writer) = stream.split();
//     self.spawn_msg_sender(gid.did, writer, receiver);
//     self.recv_messages(con, gid, reader).await?;
//     Ok(())
// }

// pub async fn recv_messages<R: AsyncRead + Unpin>(
//     &self,
//     mut con: redis::Connection,
//     from: GlobalId,
//     mut reader: R,
// ) -> Result<(), Error> {
//     use MessageToServer::*;

//     loop {
//         match read_cbor(&mut reader).await? {
//             SendBlock { to, msg } => {
//                 let push = MessageToClient::Push(Push::NewBlock {
//                     from: from.clone(),
//                     time: Utc::now(),
//                     body: msg,
//                 });
//                 for uid in to.into_iter() {
//                     let meta = con.read_meta(&uid)?;
//                     let mut to = GlobalId {
//                         uid,
//                         did: sign::PublicKey([0u8; 32]),
//                     };
//                     for key in meta.valid_keys() {
//                         to.did = key;
//                         self.send_message(&mut con, &to, push.clone()).await?;
//                     }
//                 }
//             }
//             SendBlob { to, msg } => {
//                 let push = MessageToClient::Push(Push::NewBlob {
//                     from: from.clone(),
//                     time: Utc::now(),
//                     body: msg,
//                 });
//                 for gid in to.iter() {
//                     self.send_message(&mut con, gid, push.clone()).await?;
//                 }
//             }
//             RequestMeta { qid, of } => {
//                 let res = Response::Meta(con.read_meta(&of)?);
//                 let push = MessageToClient::QueryResponse { qid, res };
//                 self.send_message(&mut con, &from, push).await?;
//             }
//             RegisterDevice { qid, key } => {
//                 let res = if key.verify_sig() && *key.signed_by() == from.did {
//                     let k = *key.data();
//                     con.add_key(&from.uid, key)?;
//                     Response::DeviceRegistered(k)
//                 } else {
//                     Response::InvalidRequest
//                 };
//                 let push = MessageToClient::QueryResponse { qid, res };
//                 self.send_message(&mut con, &from, push).await?;
//             }
//             RequestPrekey { qid, did } => {
//                 let res = Response::Prekey(con.get_prekey(did)?);
//                 let push = MessageToClient::QueryResponse { qid, res };
//                 self.send_message(&mut con, &from, push).await?;
//             }
//             UserExists { qid, of } => {
//                 let res = Response::Exists(con.user_exists(&of)?);
//                 let push = MessageToClient::QueryResponse { qid, res };
//                 self.send_message(&mut con, &from, push).await?;
//             }
//             CaughtUp => {}
//             Quit => break,
//         }
//     }
//     Ok(())
// }
// }
