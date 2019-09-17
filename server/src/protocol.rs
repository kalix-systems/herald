use crate::{prelude::*, store::*};

use dashmap::DashMap;
use tokio::{net::TcpStream, prelude::*, sync::mpsc};
use womp::womp;

pub struct Streams {
    active: DashMap<sig::PublicKey, mpsc::UnboundedSender<MessageToClient>>,
    redis: redis::Client,
}

impl Streams {
    pub fn new<T: redis::IntoConnectionInfo>(redisparams: T) -> Result<Self, Error> {
        sodiumoxide::init().expect("failed to init libsodium");
        Ok(Streams {
            active: DashMap::default(),
            redis: redis::Client::open(redisparams)?,
        })
    }

    fn new_connection(&self) -> Result<redis::Connection, Error> {
        Ok(self.redis.get_connection()?)
    }

    pub async fn send_message(
        &self,
        con: &mut redis::Connection,
        to: sig::PublicKey,
        msg: MessageToClient,
    ) -> Result<(), Error> {
        if let Some(a) = self.active.async_get(to).await {
            let mut sender = a.clone();
            if let Err(m) = sender.try_send(msg) {
                con.add_pending(to, m.into_inner())?;
            }
        } else {
            con.add_pending(to, msg)?;
        }
        Ok(())
    }

    pub async fn handle_stream(&'static self, mut stream: TcpStream) -> Result<(), Error> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).await?;
        match buf[0].try_into() {
            Ok(SessionType::Register) => self.registration_session(stream).await,
            Ok(SessionType::Login) => self.login_session(stream).await,
            Err(_) => {
                stream
                    .write_all(b"invalid session type - expected 0 or 1")
                    .await?;
                Ok(())
            }
        }
    }

    async fn registration_session(&'static self, mut stream: TcpStream) -> Result<(), Error> {
        let mut con = self.new_connection()?;
        let mut done = false;
        let gid = {
            let mut gid: Option<GlobalId> = None;
            let mut comp: Result<(), Error> = Ok(());
            while !done && comp.is_ok() {
                let uid: UserId = read_cbor(&mut stream).await?;
                redis::cmd("watch").arg(&uid).query(&mut con)?;
                comp = try {
                    if con.user_exists(&uid)? {
                        stream.write_all(b"ERR").await?;
                    } else {
                        stream.write_all(b"YES").await?;
                        let s: Signed<sig::PublicKey> = read_cbor(&mut stream).await?;
                        if s.verify_sig() {
                            let p = *s.data();
                            con.add_key(&uid, s)?;
                            done = true;
                            gid = Some(GlobalId { uid, did: p });
                        }
                    }
                };
                redis::cmd("unwatch").query(&mut con)?;
            }
            gid.ok_or(CommandFailed)?
        };
        stream.write_all(b"YES").await?;
        self.authenticated_session(gid, stream).await
    }

    async fn login_session(&'static self, mut stream: TcpStream) -> Result<(), Error> {
        use sodiumoxide::crypto::sign;

        let gid: GlobalId = read_cbor(&mut stream).await?;
        if !self.new_connection()?.key_is_valid(&gid.uid, gid.did)? {
            stream.write_all(b"ERR").await?;
            return Err(InvalidKey);
        }

        let mut dat = [0u8; 32];
        sodiumoxide::randombytes::randombytes_into(&mut dat);
        stream.write_all(&dat).await?;

        let mut buf = [0u8; sign::SIGNATUREBYTES];
        stream.read_exact(&mut buf).await?;
        let sig = sign::Signature(buf);

        if !sign::verify_detached(&sig, &dat, &gid.did) {
            stream.write_all(b"ERR").await?;
            return Err(InvalidSig);
        }

        self.authenticated_session(gid, stream).await
    }

    async fn authenticated_session(
        &'static self,
        gid: GlobalId,
        mut stream: TcpStream,
    ) -> Result<(), Error> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.active.insert(gid.did, sender);

        let mut con = self.new_connection()?;
        let pending = con.get_pending(gid.did)?;
        let push = MessageToClient::Catchup(pending);
        send_cbor(&mut stream, &push).await?;
        con.expire_pending(gid.did)?;

        let (reader, writer) = stream.split();
        self.spawn_msg_sender(gid.did, writer, receiver);
        self.recv_messages(con, gid, reader).await?;
        Ok(())
    }

    pub fn spawn_msg_sender<W: AsyncWrite + Unpin + Send + 'static>(
        &'static self,
        pk: sig::PublicKey,
        mut writer: W,
        mut input: mpsc::UnboundedReceiver<MessageToClient>,
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
            while let Some(msg) = input.recv().await {
                con.add_pending(pk, msg).expect(womp!());
            }
        });
    }

    pub async fn recv_messages<R: AsyncRead + Unpin>(
        &self,
        mut con: redis::Connection,
        from: GlobalId,
        mut reader: R,
    ) -> Result<(), Error> {
        use MessageToServer::*;

        loop {
            match read_cbor(&mut reader).await? {
                SendBlock { to, msg } => {
                    let push = MessageToClient::NewBlock {
                        from: from.clone(),
                        time: Utc::now(),
                        body: msg,
                    };
                    for uid in to.iter() {
                        let meta = con.read_meta(uid)?;
                        for key in meta.valid_keys() {
                            self.send_message(&mut con, key, push.clone()).await?;
                        }
                    }
                }
                SendBlob { to, msg } => {
                    let push = MessageToClient::NewBlob {
                        from: from.clone(),
                        time: Utc::now(),
                        body: msg,
                    };
                    for key in to {
                        self.send_message(&mut con, key, push.clone()).await?;
                    }
                }
                RequestMeta { qid, of } => {
                    let res = Response::Meta(con.read_meta(&of)?);
                    let push = MessageToClient::QueryResponse { qid, res };
                    self.send_message(&mut con, from.did, push).await?;
                }
                RegisterDevice { qid, key } => {
                    let res = if key.verify_sig() && *key.signed_by() == from.did {
                        let k = *key.data();
                        con.add_key(&from.uid, key)?;
                        Response::DeviceRegistered(k)
                    } else {
                        Response::InvalidRequest
                    };
                    let push = MessageToClient::QueryResponse { qid, res };
                    self.send_message(&mut con, from.did, push).await?;
                }
                Quit => break,
            }
        }
        Ok(())
    }
}
