#![allow(warnings)]

use anyhow::*;
use async_trait::*;
use futures::{future::*, stream::*};
use kcl::random::UQ;
use krpc::*;
use kson::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    iter,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::{
    prelude::*,
    sync::{
        mpsc::{
            unbounded_channel as unbounded, UnboundedReceiver as Receiver,
            UnboundedSender as Sender,
        },
        Mutex,
    },
    time,
};

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

#[derive(Ser, De, Clone, Debug, Eq, PartialEq)]
enum Req {
    CheckOnline,
    Send(Bytes),
}

#[derive(Ser, De, Clone, Debug, Eq, PartialEq)]
enum Res {
    Users(HashSet<User>),
    Success,
}

#[derive(Ser, De, Clone, Debug, Eq, PartialEq)]
enum Push {
    Joined(User),
    Left(User),
    Msg(User, Bytes),
}

type PushAck = ();

struct ChatProtocol;

impl Protocol for ChatProtocol {
    type Req = Req;
    type Res = Res;
    type Push = Push;
    type PushAck = ();

    const MAX_CONCURRENT_REQS: usize = 10;
    const MAX_CONCURRENT_PUSHES: usize = 10;

    const MAX_REQ_SIZE: usize = 4096;
    const MAX_ACK_SIZE: usize = 4096;
    const MAX_PUSH_SIZE: usize = 4096;
    const MAX_RESP_SIZE: usize = 4096;
}

#[async_trait]
impl KrpcServer<ChatProtocol> for Server {
    type ConnInfo = User;

    async fn init<Tx, Rx>(
        &self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<Self::ConnInfo, Error>
    where
        Tx: AsyncWrite + Send + Unpin,
        Rx: AsyncRead + Send + Unpin,
    {
        let mut buf = [0u8];
        rx.read_exact(&mut buf).await?;
        let len = u8::from_le_bytes(buf);
        assert!(len <= 128);

        let mut buf = vec![0u8; len as usize];
        rx.read_exact(&mut buf).await?;
        let u = kson::from_bytes(buf.into())?;

        Ok(u)
    }

    type Pushes = Receiver<Push>;

    async fn pushes(
        &self,
        meta: &Self::ConnInfo,
    ) -> Self::Pushes {
        let (tx, rx) = unbounded();

        let mut sess = self.sessions.lock().await;

        for (u, sender) in sess.iter_mut().filter(|(u, _)| *u != meta) {
            sender
                .send(Push::Joined(*meta))
                .expect("failed to sink join msg");
            tx.send(Push::Joined(*u)).expect("failed to sink join msg");
        }

        sess.insert(*meta, tx);

        rx
    }

    #[allow(clippy::unit_arg)]
    async fn on_push_ack(
        &self,
        _: Push,
        _: PushAck,
    ) {
    }

    async fn handle_req(
        &self,
        user: &Self::ConnInfo,
        req: Req,
    ) -> Res {
        match req {
            Req::CheckOnline => Res::Users(self.sessions.lock().await.keys().copied().collect()),
            Req::Send(b) => {
                for sender in self.senders_except(user).await {
                    sender
                        .send(Push::Msg(*user, b.clone()))
                        .expect("failed to send push");
                }

                Res::Success
            }
        }
    }

    async fn on_close(
        &self,
        user: Self::ConnInfo,
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
impl KrpcClient<ChatProtocol> for Chatter {
    type InitInfo = User;

    async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        info: Self::InitInfo,
        tx: &mut Tx,
        _rx: &mut Rx,
    ) -> Result<Self, Error> {
        dbg!();
        let bytes = kson::to_vec(&info);
        dbg!();
        let len = bytes.len() as u8;
        dbg!();
        tx.write_all(&len.to_le_bytes()).await?;
        dbg!();
        tx.write_all(&bytes).await?;
        dbg!();

        let online = iter::once(info).collect();

        Ok(Chatter {
            my_id: info,
            online: Mutex::new(online),
            msgs: Mutex::new(Vec::new()),
        })
    }

    async fn handle_push(
        &self,
        push: Push,
    ) -> PushAck {
        // dbg!(self.my_id, &push);
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

    async fn on_res(
        &self,
        req: &Req,
        res: &Res,
    ) -> Result<(), Error> {
        match (req, res) {
            (Req::CheckOnline, Res::Users(s)) => {
                let mut online = self.online.lock().await;
                for u in s {
                    online.insert(*u);
                }
            }
            (Req::Send(b), Res::Success) => {
                self.msgs.lock().await.push((self.my_id, b.clone()));
            }
            (req, res) => {
                panic!("bad resonse\nreq:  {:?}\nres: {:?}", req, res);
            }
        }
        Ok(())
    }

    fn on_close(&self) {}
}

const DELAY: Duration = Duration::from_millis(10);

mod quic_ {
    use super::*;
    use quic::*;

    fn start_server() -> (SocketAddr, Vec<u8>, impl Future<Output = ()>) {
        let e_config = quinn_proto::EndpointConfig::default();
        let (s_config, cert) = tls::configure_server().expect("failed to generate server config");

        let server = Arc::new(Server::default());
        let s_port = portpicker::pick_unused_port().expect("failed to pick port");
        let s_socket: SocketAddr = ([127u8, 0, 0, 1], s_port).into();

        let driver = async move {
            serve_arc(server, e_config, s_config, &s_socket)
                .unwrap_or_else(|e| panic!("Server error: {}", e))
                .await
        };
        (s_socket, cert, driver)
    }

    async fn start_client(
        info: <Chatter as KrpcClient<ChatProtocol>>::InitInfo,
        s_socket: &SocketAddr,
        cert: &[u8],
    ) -> Client<ChatProtocol, Chatter> {
        let e_config = quinn_proto::EndpointConfig::default();

        let c_config = tls::configure_client(&[cert]).expect("failed to generate client config");
        // let c_config = tls::configure_client(&[]).expect("failed to generate client config");
        let c_port = portpicker::pick_unused_port().expect("failed to pick client port");
        let c_socket = ([127u8, 0, 0, 1], c_port).into();

        Client::connect(info, e_config, c_config, &c_socket, s_socket, "localhost")
            .await
            .expect("failed to connect client")
    }

    #[tokio::test(threaded_scheduler)]
    async fn ping_pong() {
        kcl::init();

        let u1 = User(UQ::gen_new());
        let u2 = User(UQ::gen_new());

        let (sock, cert, driver) = start_server();
        tokio::spawn(driver);

        let c1 = start_client(u1, &sock, &cert).await;
        dbg!();

        let c2 = start_client(u2, &sock, &cert).await;
        dbg!();

        time::delay_for(DELAY).await;

        let c1_were_online = c1.online.lock().await.clone();
        dbg!();

        let c1_found_online = c1
            .req(&Req::CheckOnline)
            .await
            .expect("c1 failed to check who was online");
        dbg!();

        let c1_send_res = c1
            .req(&Req::Send("msg1".into()))
            .await
            .expect("c1 failed to send msg");
        dbg!();

        let c2_were_online = c2.online.lock().await.clone();
        dbg!();

        let c2_found_online = c2
            .req(&Req::CheckOnline)
            .await
            .expect("c2 failed to check who was online");
        dbg!();

        let c2_send_res = c2
            .req(&Req::Send("msg2".into()))
            .await
            .expect("c2 failed to send msg");
        dbg!();

        assert_eq!(c1_were_online, c2_were_online);
        assert_eq!(c1_found_online, c2_found_online);
        assert_eq!(c1_send_res, c2_send_res);

        assert_eq!(c1_were_online, [u1, u2].iter().copied().collect());
        assert_eq!(c1_found_online, Res::Users(c1_were_online));
        assert_eq!(c1_send_res, Res::Success);

        time::delay_for(DELAY).await;

        let c1_log = c1.msgs.lock().await.clone();
        dbg!();
        let c2_log = c2.msgs.lock().await.clone();
        dbg!();

        assert_eq!(c1_log, c2_log);
        assert_eq!(c1_log, vec![(u1, "msg1".into()), (u2, "msg2".into())]);
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
        pub(super) fn configure_client(
            server_certs: &[&[u8]]
        ) -> Result<ClientConfig, Box<dyn Error>> {
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
}

mod ws_ {
    use super::*;
    use ws::*;

    async fn start_server() -> (SocketAddr, Vec<u8>, impl Future<Output = ()>) {
        let (s_config, cert) = tls::configure_server().expect("failed to generate server config");
        let tls = s_config;

        let server = Arc::new(Server::default());
        let s_port = portpicker::pick_unused_port().expect("failed to pick port");
        let s_socket: SocketAddr = ([127u8, 0, 0, 1], s_port).into();

        let driver = {
            serve_arc(server, s_socket, tls).await.for_each(|f| {
                tokio::spawn(f.unwrap_or_else(|e| panic!("connection closed: {}", e)));
                ready(())
            })
        };
        (s_socket, cert, driver)
    }

    async fn start_client(
        info: <Chatter as KrpcClient<ChatProtocol>>::InitInfo,
        s_socket: &SocketAddr,
        cert: &[u8],
    ) -> Client<ChatProtocol, Chatter> {
        let c_config = tls::configure_client(&[cert]).expect("failed to generate client config");
        // let c_config = tls::configure_client(&[]).expect("failed to generate client config");

        Client::connect(
            info,
            &c_config,
            webpki::DNSNameRef::try_from_ascii_str("localhost")
                .expect("failed to parse localhost as dns name"),
            s_socket,
        )
        .await
        .expect("failed to connect client")
    }

    #[tokio::test(threaded_scheduler)]
    async fn ping_pong() {
        kcl::init();

        let u1 = User(UQ::gen_new());
        let u2 = User(UQ::gen_new());

        let (sock, cert, driver) = start_server().await;
        tokio::spawn(driver);

        let c1 = start_client(u1, &sock, &cert).await;

        time::delay_for(DELAY).await;

        let c2 = start_client(u2, &sock, &cert).await;

        time::delay_for(DELAY).await;

        let c1_were_online = c1.online.lock().await.clone();

        let c1_found_online = c1
            .req(Req::CheckOnline)
            .await
            .expect("c1 failed to check who was online")
            .await
            .expect("c1 failed to check who was online");

        let c1_send_res = c1
            .req(Req::Send("msg1".into()))
            .await
            .expect("c1 failed to send msg")
            .await
            .expect("c1 failed to send msg");

        let c2_were_online = c2.online.lock().await.clone();

        let c2_found_online = c2
            .req(Req::CheckOnline)
            .await
            .expect("c2 failed to check who was online")
            .await
            .expect("c2 failed to check who was online");

        let c2_send_res = c2
            .req(Req::Send("msg2".into()))
            .await
            .expect("c2 failed to send msg")
            .await
            .expect("c2 failed to send msg");

        assert_eq!(c1_were_online, c2_were_online);
        assert_eq!(c1_found_online, c2_found_online);
        assert_eq!(c1_send_res, c2_send_res);

        assert_eq!(c1_were_online, [u1, u2].iter().copied().collect());
        assert_eq!(c1_found_online, Res::Users(c1_were_online));
        assert_eq!(c1_send_res, Res::Success);

        time::delay_for(DELAY).await;

        let c1_log = c1.msgs.lock().await.clone();
        let c2_log = c2.msgs.lock().await.clone();

        assert_eq!(c1_log, c2_log);
        assert_eq!(c1_log, vec![(u1, "msg1".into()), (u2, "msg2".into())]);
    }

    mod tls {
        use super::*;
        use rustls::*;
        use tokio_rustls::*;

        /// Builds default rustls client config and trusts given certificates.
        ///
        /// ## Args
        ///
        /// - server_certs: a list of trusted certificates in DER format.
        pub(super) fn configure_client(server_certs: &[&[u8]]) -> Result<TlsConnector, Error> {
            let mut config = rustls::ClientConfig::default();
            // config.root_store = rustls_native_certs::load_native_certs()?;

            let anchors: Result<Vec<_>, _> = server_certs
                .iter()
                .map(|cert| webpki::trust_anchor_util::cert_der_as_trust_anchor(cert))
                .collect();

            config
                .root_store
                .add_server_trust_anchors(&webpki::TLSServerTrustAnchors(anchors?.as_slice()));

            Ok(Arc::new(config).into())
        }

        /// Returns default server configuration along with its certificate.
        pub(super) fn configure_server() -> Result<(TlsAcceptor, Vec<u8>), Error> {
            let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
            let cert_der = cert.serialize_der().unwrap();
            let priv_key = cert.serialize_private_key_der();
            let priv_key = PrivateKey(priv_key);

            let mut config = rustls::ServerConfig::new(Arc::new(NoClientAuth));
            config.set_single_cert(vec![Certificate(cert_der.clone())], priv_key)?;

            Ok((Arc::new(config).into(), cert_der))
        }
    }
}
