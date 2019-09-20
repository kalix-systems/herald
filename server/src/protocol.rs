use crate::{prelude::*, store::*};

use dashmap::DashMap;
use qutex::Qutex;
use sodiumoxide::crypto::sign;
use tokio::{net::TcpStream, prelude::*, sync::mpsc};
use tokio_io::split::{split, WriteHalf};
use womp::womp;

pub struct State {
    active: DashMap<sig::PublicKey, Qutex<WriteHalf<TcpStream>>>,
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
}

#[async_trait]
impl ProtocolHandler for State {
    type Error = Error;
    async fn handle_fanout<'a>(
        &'a self,
        fanout: fanout::ToServer<'a>,
    ) -> Result<fanout::ServerResponse, Error> {
        unimplemented!()
    }

    async fn handle_pki(&self, msg: pubkey::ToServer) -> Result<pubkey::ServerResponse, Error> {
        unimplemented!()
    }

    async fn handle_query(&self, query: query::ToServer) -> Result<query::ServerResponse, Error> {
        unimplemented!()
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

// pub fn spawn_msg_sender<W: AsyncWrite + Unpin + Send + 'static>(
//     &'static self,
//     pk: sig::PublicKey,
//     mut writer: W,
//     mut input: mpsc::UnboundedReceiver<MessageToClient>,
// ) {
//     tokio::spawn(async move {
//         while let Some(msg) = input.recv().await {
//             if let Err(e) = send_cbor(&mut writer, &msg).await {
//                 eprintln!("failed to write data - assuming connection is closed");
//                 eprintln!("error was: {}", e);
//                 self.active.remove(&pk);
//                 break;
//             }
//         }

//         let mut con = self.new_connection().expect(womp!());
//         while let Some(MessageToClient::Push(msg)) = input.recv().await {
//             con.add_pending(pk, msg).expect(womp!());
//         }
//     });
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
