use anyhow::*;
use herald_common::{protocol::*, *};
use krpc::*;
use location::*;
use once_cell::sync::Lazy;
use rustls::*;
use std::{future::*, marker::PhantomData, sync::Arc};
use tokio::{prelude::*, sync::mpsc::*};
use tokio_rustls::*;

static TLS_CONFIG: Lazy<TlsConnector> = Lazy::new(|| {
    let mut config = rustls::ClientConfig::default();
    config.root_store = rustls_native_certs::load_native_certs()
        .context(loc!())
        .expect("failed to load native certs");
    Arc::new(config).into()
});

enum ClientInitType {
    Register,
    Login,
}

struct ClientInit {
    typ: ClientInitType,
    uid: UserId,
    keys: sig::KeyPair,
    ptx: UnboundedSender<Push>,
}

impl ClientInit {
    fn new_reg(
        uid: UserId,
        ptx: UnboundedSender<Push>,
    ) -> Self {
        ClientInit {
            typ: ClientInitType::Register,
            keys: sig::KeyPair::gen_new(),
            uid,
            ptx,
        }
    }

    fn new_login(
        uid: UserId,
        keys: sig::KeyPair,
        ptx: UnboundedSender<Push>,
    ) -> Self {
        ClientInit {
            typ: ClientInitType::Login,
            uid,
            keys,
            ptx,
        }
    }
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

        Ok(Client { ptx, uid, keys })
    }

    async fn register<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        uid: UserId,
        keys: sig::KeyPair,
        ptx: UnboundedSender<Push>,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        use auth::register::*;

        tx.write_u8(auth::REGISTER)
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
        let ClientInit {
            typ,
            uid,
            keys,
            ptx,
        } = info;

        let this = match typ {
            ClientInitType::Register => Client::register(uid, keys, ptx, tx, rx).await?,
            ClientInitType::Login => Client::login(uid, keys, ptx, tx, rx).await?,
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
