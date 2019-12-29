#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use dashmap::DashMap;
use futures::{
    future::{self, BoxFuture, FutureExt},
    stream::{self, StreamExt, TryStreamExt},
};
use herald_common::{
    protocol::{auth::*, *},
    *,
};
use krpc::*;
use serial_test_derive::serial;
use server_protocol::*;
use std::{
    collections::HashSet, convert::TryFrom, net::SocketAddr, ops::Deref, sync::Arc, time::Duration,
};
use tokio::{
    prelude::*,
    sync::{
        mpsc::{
            unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
        },
        oneshot, Semaphore,
    },
    time,
};

#[tokio::test(threaded_scheduler)]
#[serial]
async fn register() {
    let (server_config, server_cert) = tls::configure_server().expect("failed to configure server");
    let client_config =
        tls::configure_client(&[server_cert.as_ref()]).expect("failed to configure client");

    let s_port = portpicker::pick_unused_port().expect("failed to pick port");
    let s_socket: SocketAddr = ([127u8, 0, 0, 1], s_port).into();

    let run_server = async move {
        let state = Arc::new(State::default());
        dbg!();
        state
            .new_connection()
            .await
            .expect("failed to get postgres connection")
            .reset_all()
            .await
            .expect("failed to reset db");

        dbg!();
        let mut futs = ws::serve_arc(state, s_socket, server_config)
            .await
            .map(Ok)
            .try_buffer_unordered(2)
            .boxed();
        for i in 0u8..2 {
            dbg!(i);
            futs.next()
                .await
                .expect(&format!("never received connection {}", i))
                .with_context(|| format!("server failed in connection {}", i))
                .unwrap();
        }
    };

    let run_clients = async {
        time::delay_for(Duration::from_secs(1)).await;
        let uid_strs = vec!["u1", "u2"];
        let uids = uid_strs
            .into_iter()
            .map(UserId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("failed to make userids");

        let (cinits, crecvs) = uids
            .iter()
            .map(|u| {
                let (tx, rx) = channel();
                let cinit = ClientInit::new_reg(*u, move |p| {
                    let tx = tx.clone();
                    Box::pin(async move { tx.clone().send(p).is_ok() })
                });
                (cinit, rx)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let clients: Vec<ws::Client<HeraldProtocol, TestClient<_>>> = stream::iter(cinits)
            .map(Ok)
            .and_then(|cinit| {
                dbg!();
                ws::Client::connect(
                    cinit,
                    &client_config,
                    webpki::DNSNameRef::try_from_ascii_str("localhost")
                        .expect("failed to parse localhost as dns name"),
                    &s_socket,
                )
            })
            .map_ok(|(c, d)| {
                tokio::spawn(d.map(drop));
                c
            })
            .boxed()
            .try_collect()
            .await
            .expect("failed to connect clients");

        dbg!();
        for client in clients {
            client.quit().expect("client failed to quit");
        }
        dbg!();
    };

    future::join(run_server, run_clients).await;
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn ping_pong() {
    let (server_config, server_cert) = tls::configure_server().expect("failed to configure server");
    let client_config =
        tls::configure_client(&[server_cert.as_ref()]).expect("failed to configure client");

    let s_port = portpicker::pick_unused_port().expect("failed to pick port");
    let s_socket: SocketAddr = ([127u8, 0, 0, 1], s_port).into();

    let run_server = async move {
        let state = Arc::new(State::default());
        dbg!();
        state
            .new_connection()
            .await
            .expect("failed to get postgres connection")
            .reset_all()
            .await
            .expect("failed to reset db");

        dbg!();
        let mut futs = ws::serve_arc(state, s_socket, server_config)
            .await
            .map(Ok)
            .try_buffer_unordered(4)
            .boxed();
        for i in 0u8..4 {
            dbg!(i);
            futs.next()
                .await
                .expect(&format!("never received connection {}", i))
                .with_context(|| format!("server failed in connection {}", i))
                .unwrap();
        }
    };

    let run_clients = async {
        time::delay_for(Duration::from_secs(1)).await;
        let uid_strs = vec!["u1", "u2", "u3", "u4"];
        let uids = uid_strs
            .into_iter()
            .map(UserId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("failed to make userids");

        let (cinits, crecvs) = uids
            .iter()
            .map(|u| {
                let (tx, rx) = channel();
                let cinit = ClientInit::new_reg(*u, move |p| {
                    let tx = tx.clone();
                    Box::pin(async move { tx.clone().send(p).is_ok() })
                });
                (cinit, rx)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let clients: Vec<ws::Client<HeraldProtocol, TestClient<_>>> = stream::iter(cinits)
            .map(Ok)
            .and_then(|cinit| {
                dbg!(cinit.uid.as_str());
                ws::Client::connect(
                    cinit,
                    &client_config,
                    webpki::DNSNameRef::try_from_ascii_str("localhost")
                        .expect("failed to parse localhost as dns name"),
                    &s_socket,
                )
            })
            .map_ok(|(c, d)| {
                dbg!();
                tokio::spawn(d.map(drop));
                dbg!();
                c
            })
            .boxed()
            .try_collect()
            .await
            .expect("failed to connect clients");

        dbg!();
        let recip = Recip::Many(Recips::Users(uids.clone()));

        for client in &clients {
            dbg!(client.uid.as_str());

            let push = Request::Push(push::Req {
                to: recip.clone(),
                msg: Bytes::copy_from_slice(client.uid.as_str().as_bytes()),
            });

            let res = client
                .req(push)
                .expect(&format!(
                    "client {} failed to sink push",
                    client.deref().uid
                ))
                .await
                .expect(&format!(
                    "client {} failed to receive response",
                    client.deref().uid
                ));
            dbg!(&res);
        }

        for client in clients {
            client.quit().expect("client failed to quit");
        }

        let mut pushsets: Vec<Vec<_>> = Vec::with_capacity(uids.len());
        for recv in crecvs {
            pushsets.push(recv.collect().await);
        }

        for (p1, p2) in pushsets.iter().zip(&pushsets[1..]) {
            assert_eq!(p1, p2);
        }
    };

    future::join(run_server, run_clients).await;
}

struct TestClient<F> {
    on_push: F,
    uid: UserId,
    keys: sig::KeyPair,
}

struct ClientInit<F> {
    typ: ClientInitType,
    uid: UserId,
    keys: sig::KeyPair,
    on_push: F,
}

impl<F: Fn(Push) -> BoxFuture<'static, PushAck> + Send + Sync + 'static> ClientInit<F> {
    fn new_reg(
        uid: UserId,
        on_push: F,
    ) -> Self {
        ClientInit {
            typ: ClientInitType::Register,
            keys: sig::KeyPair::gen_new(),
            on_push,
            uid,
        }
    }
}

impl<F> TestClient<F> {
    async fn login<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        uid: UserId,
        keys: sig::KeyPair,
        on_push: F,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        use login_types::*;

        tx.write_u8(LOGIN)
            .await
            .context("failed to write auth method")?;

        tx.write_ser(keys.public_key())
            .await
            .context("failed to write public key")?;
        let res: ClaimResponse = rx.read_de().await.context("failed to read ClaimResponse")?;
        ensure!(
            res == ClaimResponse::Challenge,
            "ClaimResponse was not Challenge"
        );

        let mut cbuf = [0u8; 32];
        rx.read_exact(&mut cbuf)
            .await
            .context("failed to read challenge")?;
        let sig = keys.secret_key().sign(&cbuf);
        tx.write_all(sig.as_ref())
            .await
            .context("failed to write challenge response")?;

        let res: ChallengeResult = rx
            .read_de()
            .await
            .context("failed to read ChallengeResult")?;
        ensure!(
            res == ChallengeResult::Success,
            "ChallengeResult was not Success"
        );

        Ok(TestClient { on_push, uid, keys })
    }

    async fn register<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        uid: UserId,
        keys: sig::KeyPair,
        on_push: F,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        use register::*;

        tx.write_u8(REGISTER)
            .await
            .context("failed to write auth method")?;

        tx.write_ser(&ClientEvent::Check(uid)).await?;
        ensure!(
            ServeEvent::Available == rx.read_de().await?,
            "username taken"
        );

        let signed = keys.sign(uid);
        tx.write_ser(&ClientEvent::Claim(signed)).await?;
        ensure!(
            ServeEvent::Success == rx.read_de().await?,
            "registration failed"
        );

        Ok(TestClient { on_push, uid, keys })
    }

    async fn catchup<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        &self,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<(), Error>
    where
        F: Fn(Push) -> BoxFuture<'static, PushAck>,
    {
        while rx.read_u8().await? == 0 {
            let pushes: Vec<Push> = rx.read_de().await?;
            for push in pushes {
                (self.on_push)(push).await;
            }
            tx.write_u8(1).await?;
        }

        Ok(())
    }
}

enum ClientInitType {
    Register,
    Login,
}

#[async_trait]
impl<F: Fn(Push) -> BoxFuture<'static, PushAck> + Send + Sync + 'static> KrpcClient<HeraldProtocol>
    for TestClient<F>
{
    type InitInfo = ClientInit<F>;

    async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        info: Self::InitInfo,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        let ClientInit {
            typ,
            uid,
            keys,
            on_push,
        } = info;

        let this = match typ {
            ClientInitType::Register => TestClient::register(uid, keys, on_push, tx, rx).await?,
            ClientInitType::Login => TestClient::login(uid, keys, on_push, tx, rx).await?,
        };

        this.catchup(tx, rx).await?;

        Ok(this)
    }

    async fn handle_push(
        &self,
        push: <HeraldProtocol as Protocol>::Push,
    ) -> <HeraldProtocol as Protocol>::PushAck {
        (self.on_push)(push).await
    }

    async fn on_res(
        &self,
        req: &<HeraldProtocol as Protocol>::Req,
        res: &<HeraldProtocol as Protocol>::Res,
    ) -> Result<(), Error> {
        match (req, res) {
            (req, Response::Err(e)) => {
                return Err(anyhow!(e.clone()))
                    .with_context(|| format!("error handling request {:#?}", req));
            }
            (Request::GetSigchain(_), Response::GetSigchain(_)) => {}
            (Request::RecipExists(_), Response::RecipExists(_)) => {}
            (Request::NewSig(_), Response::NewSig(_)) => {}
            (Request::NewPrekey(_), Response::NewPrekey(_)) => {}
            (Request::GetPrekey(_), Response::GetPrekey(_)) => {}
            (Request::Push(push::Req { to, msg }), Response::Push(push::Res::Success(ts))) => {
                let push = Push {
                    tag: to.tag(),
                    timestamp: *ts,
                    msg: msg.clone(),
                    gid: GlobalId {
                        uid: self.uid,
                        did: *self.keys.public_key(),
                    },
                };

                (self.on_push)(push).await;
            }
            // (Request::Push(_), Response::Push(_)) => {}
            (req, res) => {
                return Err(anyhow!("type mismatch")).with_context(|| {
                    format!(
                        "request was:\n\
                         {:#?}\n\
                         response was:\n\
                         {:#?}",
                        req, res
                    )
                });
            }
        }
        Ok(())
    }

    fn on_close(&self) {}
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
