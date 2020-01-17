use anyhow::*;
use futures::{future::TryFutureExt, stream::StreamExt};
use herald_common::{protocol::*, *};
use krpc::*;
use once_cell::sync::Lazy;
use std::{net::ToSocketAddrs, sync::Arc};
use tokio::sync::{mpsc::*, oneshot};
use tokio_rustls::*;

static TLS_CONFIG: Lazy<TlsConnector> = Lazy::new(|| {
    let mut config = rustls::ClientConfig::default();
    config.root_store = rustls_native_certs::load_native_certs()
        .context(loc!())
        .expect("failed to load native certs");
    Arc::new(config).into()
});

enum ClientInit {
    Register {
        names: Receiver<auth::register::ClientEvent>,
        resps: Sender<auth::register::ServeEvent>,
        keys: sig::KeyPair,
        ptx: UnboundedSender<Push>,
    },
    Login {
        uid: UserId,
        keys: sig::KeyPair,
        ptx: UnboundedSender<Push>,
    },
}

async fn registration_loop<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
    mut names: Receiver<auth::register::ClientEvent>,
    mut resps: Sender<auth::register::ServeEvent>,
    tx: &mut Framed<Tx>,
    rx: &mut Framed<Rx>,
) -> Result<UserId, Error> {
    use auth::register::*;

    while let Some(cev) = names.next().await {
        tx.write_ser(&cev).await?;
        let sev = rx.read_de().await?;
        resps.send(sev).await?;

        if let (ClientEvent::Claim(s), ServeEvent::Success) = (cev, sev) {
            return Ok(*s.data());
        }
    }

    Err(anyhow!("registration failed"))
}

struct Client {
    ptx: UnboundedSender<Push>,
    uid: UserId,
    keys: sig::KeyPair,
}

impl Client {
    async fn login<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        uid: UserId,
        keys: sig::KeyPair,
        ptx: UnboundedSender<Push>,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        use auth::login_types::*;

        tx.write_u8(auth::LOGIN)
            .await
            .context("failed to write auth method")?;

        tx.write_ser(keys.public())
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

        let sig = keys.secret().sign(&cbuf);
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

        Ok(Client { ptx, uid, keys })
    }

    async fn register<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        names: Receiver<auth::register::ClientEvent>,
        resps: Sender<auth::register::ServeEvent>,
        keys: sig::KeyPair,
        ptx: UnboundedSender<Push>,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        tx.write_u8(auth::REGISTER)
            .await
            .context("failed to write auth method")?;

        let uid = registration_loop(names, resps, tx, rx).await?;

        Ok(Client { ptx, uid, keys })
    }

    async fn catchup<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        &self,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<(), Error> {
        while rx.read_u8().await? == 0 {
            let pushes: Vec<Push> = rx.read_de().await?;
            for push in pushes {
                self.ptx.send(push)?;
            }
            tx.write_u8(1).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl KrpcClient<HeraldProtocol> for Client {
    type InitInfo = ClientInit;

    async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        info: Self::InitInfo,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        let this = match info {
            ClientInit::Login { uid, keys, ptx } => Client::login(uid, keys, ptx, tx, rx).await?,
            ClientInit::Register {
                names,
                resps,
                keys,
                ptx,
            } => Client::register(names, resps, keys, ptx, tx, rx).await?,
        };

        this.catchup(tx, rx).await?;

        Ok(this)
    }

    async fn handle_push(
        &self,
        push: <HeraldProtocol as Protocol>::Push,
    ) -> <HeraldProtocol as Protocol>::PushAck {
        self.ptx.send(push).is_ok()
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
            (Request::Push(_), Response::Push(_)) => {}
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

pub struct HClient {
    inner: ws::Client<HeraldProtocol, Client>,
}

impl HClient {
    pub fn uid(&self) -> UserId {
        self.inner.uid
    }

    pub fn keys(&self) -> &sig::KeyPair {
        &self.inner.keys
    }

    pub async fn login(
        uid: UserId,
        keys: sig::KeyPair,
        server_dns: &str,
        server_port: u16,
    ) -> Result<(Self, UnboundedReceiver<Push>, oneshot::Receiver<Error>), Error> {
        let (ptx, prx) = unbounded_channel();
        let init = ClientInit::Login { uid, keys, ptx };
        let (inner, driver) = ws::Client::connect(
            init,
            &TLS_CONFIG,
            webpki::DNSNameRef::try_from_ascii_str(server_dns)?,
            &(server_dns, server_port)
                .to_socket_addrs()
                .with_context(|| format!("host {} failed to resolve", server_dns))?
                .next()
                .ok_or_else(|| anyhow!("host {} resolved to no IPs", server_dns))?,
        )
        .await?;

        let (etx, erx) = oneshot::channel();
        tokio::spawn(driver.unwrap_or_else(|e| drop(etx.send(e))));
        let out = HClient { inner };
        Ok((out, prx, erx))
    }

    pub async fn register(
        names: Receiver<auth::register::ClientEvent>,
        resps: Sender<auth::register::ServeEvent>,
        keys: sig::KeyPair,
        server_dns: &str,
        server_port: u16,
    ) -> Result<(Self, UnboundedReceiver<Push>, oneshot::Receiver<Error>), Error> {
        let (ptx, prx) = unbounded_channel();
        let init = ClientInit::Register {
            names,
            resps,
            keys,
            ptx,
        };
        let (inner, driver) = ws::Client::connect(
            init,
            &TLS_CONFIG,
            webpki::DNSNameRef::try_from_ascii_str(server_dns)?,
            &(server_dns, server_port)
                .to_socket_addrs()
                .with_context(|| format!("host {} failed to resolve", server_dns))?
                .next()
                .ok_or_else(|| anyhow!("host {} resolved to no IPs", server_dns))?,
        )
        .await?;

        let (etx, erx) = oneshot::channel();
        tokio::spawn(driver.unwrap_or_else(|e| drop(etx.send(e))));
        let out = HClient { inner };
        Ok((out, prx, erx))
    }

    pub fn req(
        &self,
        req: Request,
    ) -> Result<oneshot::Receiver<Response>, Error> {
        self.inner.req(req)
    }

    pub fn quit(&self) -> Result<(), Error> {
        self.inner.quit()
    }
}
