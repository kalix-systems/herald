use crate::{prelude::*, store::*};
use dashmap::DashMap;
use sodiumoxide::{crypto::sign, randombytes::randombytes_into};
use std::convert::TryInto;
use tokio::{
    prelude::*,
    sync::mpsc::{
        unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
    },
};

pub struct State {
    active: DashMap<sig::PublicKey, Sender<MessageToClient>>,
    pool: Pool,
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
                    .copied()
                    .collect();
                if missing.is_empty() {
                    Ok(ServerResponse::MissingUIDs(missing))
                } else {
                    let data = Push::NewUMessage {
                        from,
                        msg,
                        timestamp: Utc::now(),
                    };
                    for uid in to {
                        for did in con.read_meta(&uid)?.valid_keys() {
                            // TODO: replace this w/a tokio spawn for reliability reasons
                            self.send_push(did, data.clone()).await?;
                        }
                    }
                    Ok(ServerResponse::Success)
                }
            }
            ToServer::DID { to, msg } => {
                let missing: Vec<sign::PublicKey> = to
                    .iter()
                    .filter(|d| con.device_exists(d).unwrap_or(false))
                    .copied()
                    .collect();
                if missing.is_empty() {
                    Ok(ServerResponse::MissingDIDs(missing))
                } else {
                    let timestamp = Utc::now();
                    for did in to {
                        let data = Push::NewDMessage {
                            from,
                            msg: msg.clone(),
                            timestamp,
                        };
                        // TODO: replace this w/a tokio spawn for reliability reasons
                        self.send_push(did, data).await?;
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
                    con.add_key(from.uid, spk)
                } else {
                    Ok(BadSignature)
                }
            }
            DeprecateKey(spk) => {
                if from.did == *spk.signed_by() && spk.verify_sig() {
                    con.deprecate_key(spk)
                } else {
                    Ok(BadSignature)
                }
            }
            RegisterPrekey(spk) => {
                if from.did == *spk.signed_by() && spk.verify_sig() {
                    con.add_prekey(from.did, spk)
                } else {
                    Ok(BadSignature)
                }
            }
        }
    }

    async fn handle_query(
        &self,
        _: Self::From,
        query: query::ToServer,
    ) -> Result<query::ServerResponse, Error> {
        use query::{ServerResponse::*, ToServer::*};
        let mut con = self.new_connection()?;
        match query {
            UserExists(uid) => con.user_exists(&uid).map(Exists),
            UserKeys(uid) => Ok(con.read_meta(&uid).map(Keys).unwrap_or(MissingData)),
            GetKeyMeta(_uid, pk) => Ok(con.read_key(pk).map(KeyMeta).unwrap_or(MissingData)),
            GetPrekey(pk) => Ok(con.get_prekey(pk).map(PreKey).unwrap_or(MissingData)),
        }
    }
}

impl State {
    pub fn new() -> Result<Self, Error> {
        sodiumoxide::init().expect("failed to init libsodium");
        let pool = init_pool();
        Ok(State {
            active: DashMap::default(),
            pool,
        })
    }

    pub async fn handle_stream<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        &'static self,
        mut stream: S,
    ) -> Result<(), Error> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).await?;
        let gid = match buf[0].try_into().map_err(BadSessionType)? {
            SessionType::Register => self.register(&mut stream).await?,
            SessionType::Login => self.login(&mut stream).await?,
        };
        self.authenticated_session(gid, stream);
        Ok(())
    }

    // TODO #18: make these loops timeout?
    async fn register<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        &self,
        stream: &mut S,
    ) -> Result<GlobalId, Error> {
        use register::*;
        let mut con = self.new_connection()?;
        let mut uid = None;
        let mut key = None;
        while uid.is_none() || key.is_none() {
            // TODO: lock uid,key in the body of this loop?
            let msg = match read_cbor(stream).await? {
                ToServer::RequestUID(u) => {
                    if con.user_exists(&u)? {
                        ToClient::UIDTaken
                    } else {
                        uid = Some(u);
                        ToClient::UIDReady
                    }
                }
                ToServer::UseKey(k) => {
                    if con.device_exists(k.data())? {
                        ToClient::KeyTaken
                    } else if !k.verify_sig() {
                        ToClient::BadSig
                    } else {
                        key = Some(k);
                        ToClient::KeyReady
                    }
                }
            };
            send_cbor(stream, &msg).await?;
        }
        let uid = uid.unwrap();
        let key = key.unwrap();
        con.add_key(uid, key)?;
        Ok(GlobalId {
            uid,
            did: *key.data(),
        })
    }

    // TODO #18: make these loops timeout?
    async fn login<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        &self,
        stream: &mut S,
    ) -> Result<GlobalId, Error> {
        use login::*;
        let mut con = self.new_connection()?;
        let mut gid = None;
        let mut to_sign = None;
        loop {
            match read_cbor(stream).await? {
                ToServer::As(g) => {
                    let msg = if con.key_is_valid(g.did)? {
                        gid = Some(g);

                        let mut bytes = [0u8; 32];
                        randombytes_into(&mut bytes);
                        to_sign = Some(bytes);

                        ToClient::Sign(bytes)
                    } else {
                        ToClient::BadGID
                    };
                    send_cbor(stream, &msg).await?;
                }
                ToServer::Sig(sig) => {
                    if gid.is_none() || to_sign.is_none() {
                        send_cbor(stream, &ToClient::BadGID).await?;
                    } else if sign::verify_detached(&sig, &to_sign.unwrap(), &gid.unwrap().did) {
                        send_cbor(stream, &ToClient::Success).await?;
                        break;
                    } else {
                        send_cbor(stream, &ToClient::BadSig).await?;
                    }
                }
            }
        }
        gid.ok_or(LoginFailed)
    }

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

            let mut con = self
                .new_connection()
                .expect("MAJOR ERROR: failed to connect to database - messages may have dropped");
            while let Some(MessageToClient::Push(msg)) = input.recv().await {
                // TODO (HIGH PRIORITY): handle retry logic here
                // TODO take advantage of batching
                con.add_pending(vec![pk], msg)
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
                let m: MessageToServer = read_cbor(&mut reader)
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

    fn new_connection(&self) -> Result<Conn, Error> {
        // TODO add error type
        Ok(Conn(self.pool.get().unwrap()))
    }

    async fn send_push(&self, to: sign::PublicKey, msg: Push) -> Result<(), Error> {
        let mut conn = self.new_connection()?;
        if let Some(a) = self.active.async_get(to).await {
            let mut sender = a.clone();
            if let Err(m) = sender.try_send(MessageToClient::Push(msg)) {
                if let MessageToClient::Push(p) = m.into_inner() {
                    // TODO take advantage of batching
                    conn.add_pending(vec![to], p)?;
                }
            }
        } else {
            // TODO take advantage of batching
            conn.add_pending(vec![to], msg)?;
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
