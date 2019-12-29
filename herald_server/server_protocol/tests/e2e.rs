use dashmap::DashMap;
use futures::future::{BoxFuture, FutureExt};
use herald_common::{
    protocol::{auth::*, *},
    *,
};
use krpc::*;
use server_protocol::*;
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

type OnPush = Box<dyn Fn(Push) -> BoxFuture<'static, PushAck> + Send + Sync + 'static>;

struct TestClient<F> {
    on_push: F,
    my_uid: UserId,
    my_keys: sig::KeyPair,
}

impl<F> TestClient<F> {
    async fn login<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        my_uid: UserId,
        my_keys: sig::KeyPair,
        on_push: F,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        use login_types::*;

        tx.write_u8(LOGIN)
            .await
            .context("failed to write auth method")?;

        tx.write_ser(my_keys.public_key())
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
        let sig = my_keys.secret_key().sign(&cbuf);
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

        Ok(TestClient {
            on_push,
            my_uid,
            my_keys,
        })
    }

    async fn register<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        my_uid: UserId,
        my_keys: sig::KeyPair,
        on_push: F,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        use register::*;

        tx.write_u8(REGISTER)
            .await
            .context("failed to write auth method")?;

        tx.write_ser(&ClientEvent::Check(my_uid)).await?;
        ensure!(
            ServeEvent::Available == rx.read_de().await?,
            "username taken"
        );

        let signed = my_keys.sign(my_uid);
        tx.write_ser(&ClientEvent::Claim(signed)).await?;
        ensure!(
            ServeEvent::Success == rx.read_de().await?,
            "registration failed"
        );

        Ok(TestClient {
            on_push,
            my_uid,
            my_keys,
        })
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
    Reg,
    Login,
}

struct ClientInit {
    typ: ClientInitType,
    uid: UserId,
    keys: sig::KeyPair,
    on_push: OnPush,
}

#[async_trait]
impl KrpcClient<HeraldProtocol> for TestClient<OnPush> {
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
            on_push,
        } = info;

        let this = match typ {
            ClientInitType::Reg => TestClient::register(uid, keys, on_push, tx, rx).await?,
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
