use async_trait::*;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver as Receiver, UnboundedSender as Sender},
    future::{self, *},
    sink::*,
};
use kcl::random::UQ;
use krpc::*;
use kson::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    ops::Deref,
    sync::Arc,
    time::Duration,
};
use tokio::{sync::*, time::*};

use void::Void;

#[derive(Ser, De, Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct User(UQ);

struct Server {
    sessions: Mutex<HashMap<User, Sender<Push>>>,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

impl Server {
    async fn senders_except(
        &self,
        except: &User,
    ) -> Vec<Sender<Push>> {
        self.sessions
            .lock()
            .await
            .iter()
            .filter_map(|(u, s)| if u != except { Some(s.clone()) } else { None })
            .collect()
    }
}

#[derive(Ser, De, Debug, Eq, PartialEq)]
enum Req {
    CheckOnline,
    Send(Bytes),
}

#[derive(Ser, De, Debug, Eq, PartialEq)]
enum Resp {
    Users(HashSet<User>),
    Success,
}

#[derive(Ser, De, Debug, Eq, PartialEq)]
enum Push {
    Joined(User),
    Left(User),
    Msg(User, Bytes),
}

type PushAck = ();

#[async_trait]
impl KrpcServer for Server {
    type InitError = Void;
    type ConnInfo = User;

    const MAX_CONCURRENT_REQS: usize = 10;
    const MAX_CONCURRENT_PUSHES: usize = 10;

    const MAX_REQ_SIZE: usize = 4096;
    const MAX_ACK_SIZE: usize = 4096;

    async fn init(
        &self,
        mut tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self::ConnInfo, KrpcError<Self::InitError>> {
        let u_bytes = rx.read_to_end(128).await?;
        let u = kson::from_bytes(u_bytes.into())?;

        tx.finish().await?;
        Ok(u)
    }

    type Push = Push;
    type Pushes = Receiver<Push>;

    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Self::Pushes {
        let (mut tx, rx) = unbounded();

        let mut sess = self.sessions.lock().await;

        for (u, sender) in sess.iter_mut().filter(|(u, _)| *u != meta) {
            future::try_join(sender.send(Push::Joined(*meta)), tx.send(Push::Joined(*u)))
                .await
                .expect("failed to sink join msg");
        }

        sess.insert(*meta, tx);

        rx
    }

    type PushAck = PushAck;
    #[allow(clippy::unit_arg)]
    async fn on_push_ack(
        &self,
        _: Self::Push,
        _: Self::PushAck,
    ) {
    }

    type Req = Req;
    type Resp = Resp;

    async fn handle_req(
        &self,
        user: &Self::ConnInfo,
        req: Self::Req,
    ) -> Self::Resp {
        match req {
            Req::CheckOnline => Resp::Users(self.sessions.lock().await.keys().copied().collect()),
            Req::Send(b) => {
                for mut sender in self.senders_except(user).await {
                    sender
                        .send(Push::Msg(*user, b.clone()))
                        .await
                        .expect("failed to send push");
                }

                Resp::Success
            }
        }
    }

    async fn on_close(
        &self,
        user: Self::ConnInfo,
        _: quinn::Connection,
    ) {
        self.sessions.lock().await.remove(&user);
    }
}

struct Chatter {
    my_id: User,
    online: Mutex<HashSet<User>>,
    msgs: Mutex<Vec<(User, Bytes)>>,
}

#[async_trait]
impl KrpcClient for Chatter {
    const MAX_CONCURRENT_REQS: usize = 10;
    const MAX_CONCURRENT_PUSHES: usize = 10;

    const MAX_PUSH_SIZE: usize = 4096;
    const MAX_RESP_SIZE: usize = 4096;

    type Req = Req;
    type Resp = Resp;
    type Push = Push;
    type PushAck = PushAck;

    type InitInfo = User;
    type InitError = Void;

    async fn init(
        info: Self::InitInfo,
        mut tx: quinn::SendStream,
        _rx: quinn::RecvStream,
    ) -> Result<Self, KrpcError<Self::InitError>> {
        tx.write_all(&kson::to_vec(&info)).await?;
        tx.finish().await?;

        let online = [info].iter().copied().collect();

        Ok(Chatter {
            my_id: info,
            online: Mutex::new(online),
            msgs: Mutex::new(Vec::new()),
        })
    }

    async fn handle_push(
        &self,
        push: Self::Push,
    ) -> Self::PushAck {
        match push {
            Push::Joined(u) => {
                self.online.lock().await.insert(u);
            }
            Push::Left(u) => {
                self.online.lock().await.remove(&u);
            }
            Push::Msg(u, b) => {
                self.msgs.lock().await.push((u, b));
            }
        }
    }

    async fn on_resp(
        &self,
        req: &Self::Req,
        resp: &Self::Resp,
    ) -> Result<(), KrpcError<Self::InitError>> {
        match (req, resp) {
            (Req::CheckOnline, Resp::Users(s)) => {
                let mut online = self.online.lock().await;
                for u in s {
                    online.insert(*u);
                }
            }
            (Req::Send(b), Resp::Success) => {
                self.msgs.lock().await.push((self.my_id, b.clone()));
            }
            (req, resp) => {
                panic!("bad response\nreq:  {:?}\nresp: {:?}", req, resp);
            }
        }
        Ok(())
    }

    async fn on_close(&self) {}
}

const DELAY: Duration = Duration::from_millis(10);

#[tokio::test(threaded_scheduler)]
async fn ping_pong() {
    kcl::init();

    let u1 = User(UQ::gen_new());
    let u2 = User(UQ::gen_new());

    let (sock, cert) = start_server().await;
    dbg!();

    let c1 = start_client(u1, &sock, &cert).await;
    dbg!();

    let c2 = start_client(u2, &sock, &cert).await;
    dbg!();

    delay_for(DELAY).await;
    dbg!();

    assert_eq!(
        c1.online.lock().await.deref(),
        c2.online.lock().await.deref()
    );
    dbg!();

    let m1 = c1
        .req(&Req::CheckOnline)
        .await
        .expect("c1 failed to check who was online");
    dbg!();

    let m2 = c2
        .req(&Req::CheckOnline)
        .await
        .expect("c2 failed to check who was online");
    dbg!();

    delay_for(DELAY).await;
    dbg!();

    assert_eq!(Resp::Users(c1.online.lock().await.clone()), m1);
    dbg!();

    assert_eq!(Resp::Users(c2.online.lock().await.clone()), m2);
    dbg!();

    assert_eq!(m1, m2);

    assert_eq!(
        c1.req(&Req::Send("msg1".into()))
            .await
            .expect("c1 failed to send msg"),
        Resp::Success
    );
    dbg!();

    delay_for(DELAY).await;
    dbg!();

    assert_eq!(
        c2.req(&Req::Send("msg2".into()))
            .await
            .expect("c2 failed to send msg"),
        Resp::Success
    );
    dbg!();

    delay_for(DELAY).await;
    dbg!();

    assert_eq!(c1.msgs.lock().await.deref(), c2.msgs.lock().await.deref());
}

async fn start_client(
    info: <Chatter as KrpcClient>::InitInfo,
    s_socket: &SocketAddr,
    cert: &[u8],
) -> Client<Chatter> {
    let e_config = quinn_proto::EndpointConfig::default();

    let c_config = tls::configure_client(&[cert]).expect("failed to generate client config");
    let c_port = portpicker::pick_unused_port().expect("failed to pick client port");
    let c_socket = ([127u8, 0, 0, 1], c_port).into();

    Client::connect(info, e_config, c_config, &c_socket, s_socket, "localhost")
        .await
        .expect("failed to connect client")
}

async fn start_server() -> (SocketAddr, Vec<u8>) {
    let e_config = quinn_proto::EndpointConfig::default();
    let (s_config, cert) = tls::configure_server().expect("failed to generate server config");

    let server = Arc::new(Server::default());
    let s_port = portpicker::pick_unused_port().expect("failed to pick port");
    let s_socket: SocketAddr = ([127u8, 0, 0, 1], s_port).into();

    tokio::spawn(async move {
        krpc::serve_arc(server, e_config, s_config, &s_socket)
            .unwrap_or_else(|e| panic!("Server error: {}", e))
            .await
    });
    (s_socket, cert)
}

/// Shamelessly copied from the quinn examples
mod tls {
    use super::*;

    use quinn::*;
    use std::error::Error;

    /// Builds default quinn client config and trusts given certificates.
    ///
    /// ## Args
    ///
    /// - server_certs: a list of trusted certificates in DER format.
    pub(super) fn configure_client(server_certs: &[&[u8]]) -> Result<ClientConfig, Box<dyn Error>> {
        let mut cfg_builder = ClientConfigBuilder::default();
        for cert in server_certs {
            cfg_builder.add_certificate_authority(Certificate::from_der(&cert)?)?;
        }
        Ok(cfg_builder.build())
    }

    /// Returns default server configuration along with its certificate.
    pub(super) fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let cert_der = cert.serialize_der().unwrap();
        let priv_key = cert.serialize_private_key_der();
        let priv_key = PrivateKey::from_der(&priv_key)?;

        let server_config = ServerConfig {
            transport: Arc::new(TransportConfig {
                stream_window_uni: 0,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut cfg_builder = ServerConfigBuilder::new(server_config);
        let cert = Certificate::from_der(&cert_der)?;
        cfg_builder.certificate(CertificateChain::from_certs(vec![cert]), priv_key)?;

        Ok((cfg_builder.build(), cert_der))
    }
}
