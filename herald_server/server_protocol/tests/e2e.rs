#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use dashmap::DashMap;
use futures::{
    future::{self, BoxFuture, FutureExt, TryFutureExt},
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
use stream_cancel::*;
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

async fn setup_server<O: Into<Option<usize>>>(
    num_sessions: O
) -> (Trigger, SocketAddr, Vec<u8>, Arc<State>) {
    let (server_config, server_cert) = tls::configure_server().expect("failed to configure server");
    let client_config =
        tls::configure_client(&[server_cert.as_ref()]).expect("failed to configure client");

    let s_port = portpicker::pick_unused_port().expect("failed to pick port");
    let s_socket: SocketAddr = ([127u8, 0, 0, 1], s_port).into();

    let state = Arc::new(State::default());

    dbg!();
    let (trigger, futs) = Valved::new(ws::serve_arc(state.clone(), s_socket, server_config).await);

    tokio::spawn(futs.for_each_concurrent(num_sessions, |s| s.unwrap_or_else(|e| panic!("{}", e))));

    (trigger, s_socket, server_cert, state)
}

type OnPush = Box<dyn Fn(Push) -> BoxFuture<'static, PushAck> + Send + Sync + 'static>;

async fn register_client<F: Fn(Push) -> BoxFuture<'static, PushAck> + Send + Sync + 'static>(
    uid: UserId,
    on_push: F,
    connector: &tokio_rustls::TlsConnector,
    dns: &str,
    socket: &SocketAddr,
) -> Result<(Trigger, ws::Client<HeraldProtocol, TestClient<F>>), Error> {
    let cinit = ClientInit::new_reg(uid, on_push);
    let (client, driver) = ws::Client::connect(
        cinit,
        connector,
        webpki::DNSNameRef::try_from_ascii_str(dns)?,
        socket,
    )
    .await?;
    let (trigger, tripwire) = Tripwire::new();
    tokio::spawn(future::select(driver.boxed(), tripwire).map(drop));
    Ok((trigger, client))
}

async fn login_client<F: Fn(Push) -> BoxFuture<'static, PushAck> + Send + Sync + 'static>(
    uid: UserId,
    keys: sig::KeyPair,
    on_push: F,
    connector: &tokio_rustls::TlsConnector,
    dns: &str,
    socket: &SocketAddr,
) -> Result<(Trigger, ws::Client<HeraldProtocol, TestClient<F>>), Error> {
    let cinit = ClientInit::new_login(uid, keys, on_push);
    let (client, driver) = ws::Client::connect(
        cinit,
        connector,
        webpki::DNSNameRef::try_from_ascii_str(dns)?,
        socket,
    )
    .await?;
    let (trigger, tripwire) = Tripwire::new();
    tokio::spawn(future::select(driver.boxed(), tripwire).map(drop));
    Ok((trigger, client))
}

#[tokio::test]
#[serial]
async fn register() {
    kcl::init();

    let (trigger, socket, cert, state) = setup_server(None).await;
    dbg!();
    state.reset().await.expect("failed to reset db");
    dbg!();

    let connector = tls::configure_client(&[&cert]).expect("failed to configure client-side TLS");

    time::delay_for(Duration::from_secs(1)).await;
    let uids = vec!["u1", "u2"]
        .into_iter()
        .map(UserId::try_from)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to make userids");

    let items = stream::iter(uids.iter().map(|uid| {
        let (tx, rx) = channel();
        let on_push = move |p| {
            let tx = tx.clone();
            async move { tx.send(p).is_ok() }.boxed()
        };
        register_client(*uid, on_push, &connector, "localhost", &socket).map_ok(|(t, c)| (t, c, rx))
    }))
    .buffered(uids.len())
    .try_collect::<Vec<_>>()
    .await
    .expect("failed to register clients");

    let rxs = stream::iter(items)
        .map(|(trigger, client, rx)| {
            async move {
                client.quit().expect("client failed to quit");
                trigger.cancel();
                rx
            }
        })
        .buffered(uids.len())
        .collect::<Vec<_>>()
        .await;

    for rx in rxs {
        let pushes = rx.collect::<Vec<_>>().await;
        assert_eq!(pushes, vec![]);
    }

    trigger.cancel();
}

#[tokio::test]
#[serial]
async fn broadcast() {
    kcl::init();

    let (trigger, socket, cert, state) = setup_server(None).await;
    dbg!();
    state.reset().await.expect("failed to reset db");
    dbg!();

    let connector = tls::configure_client(&[&cert]).expect("failed to configure client-side TLS");

    time::delay_for(Duration::from_secs(1)).await;
    let uids = vec!["u1", "u2", "u3", "u4"]
        .into_iter()
        .map(UserId::try_from)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to make userids");

    let items = stream::iter(uids.iter().map(|uid| {
        let (tx, rx) = channel();
        let on_push = move |p| {
            let tx = tx.clone();
            async move { tx.send(p).is_ok() }.boxed()
        };
        register_client(*uid, on_push, &connector, "localhost", &socket).map_ok(|(t, c)| (t, c, rx))
    }))
    .buffered(uids.len())
    .try_collect::<Vec<_>>()
    .await
    .expect("failed to register clients");

    let mut triggers = Vec::with_capacity(items.len());
    let mut clients = Vec::with_capacity(items.len());
    let mut rxs = Vec::with_capacity(items.len());

    for (t, c, r) in items {
        triggers.push(t);
        clients.push(c);
        rxs.push(r);
    }

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

    time::delay_for(Duration::from_millis(100)).await;

    for (client, trigger) in clients.into_iter().zip(triggers) {
        client.quit().expect("client failed to quit");
        trigger.cancel();
    }

    let mut pushsets: Vec<Vec<_>> = Vec::with_capacity(uids.len());
    for rx in rxs {
        pushsets.push(rx.collect().await);
    }

    for (p1, p2) in pushsets.iter().zip(&pushsets[1..]) {
        assert_eq!(p1, p2);
    }

    trigger.cancel();
}

#[tokio::test]
#[serial]
async fn login() {
    kcl::init();

    let (trigger, socket, cert, state) = setup_server(None).await;
    dbg!();
    state.reset().await.expect("failed to reset db");
    dbg!();

    let connector = tls::configure_client(&[&cert]).expect("failed to configure client-side TLS");

    time::delay_for(Duration::from_secs(1)).await;

    let uid = UserId::try_from("u").expect("failed to create userid");
    let (t1, c1) = register_client(
        uid,
        |p| future::ready(true).boxed(),
        &connector,
        "localhost",
        &socket,
    )
    .await
    .expect("failed to register");

    let new_keypair = sig::KeyPair::gen_new();
    let update = sig::SigUpdate::Endorse(new_keypair.sign(uid));
    let signed = c1.keys.sign(update);

    let resp = c1
        .req(Request::NewSig(signed))
        .expect("failed to sink request")
        .await
        .expect("failed to receive response");

    assert_eq!(
        resp,
        Response::NewSig(PKIResponse::Success),
        "server failed to add key"
    );

    let (t2, c2) = login_client(
        uid,
        new_keypair,
        |p| future::ready(true).boxed(),
        &connector,
        "localhost",
        &socket,
    )
    .await
    .expect("failed to login with new keys");

    c1.quit().expect("failed to quit c1");
    c2.quit().expect("failed to quit c2");
    t1.cancel();
    t2.cancel();
    trigger.cancel();

    // let signed = c1.keys.private_key().sign(new_keypair);
    // c1.req(
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

    fn new_login(
        uid: UserId,
        keys: sig::KeyPair,
        on_push: F,
    ) -> Self {
        ClientInit {
            typ: ClientInitType::Login,
            keys,
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
