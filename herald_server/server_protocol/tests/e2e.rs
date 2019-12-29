use dashmap::DashMap;
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

struct TestClient {
    sender: Sender<Push>,
    my_uid: UserId,
    my_keys: sig::KeyPair,
}

impl TestClient {
    async fn login<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        my_uid: UserId,
        my_keys: sig::KeyPair,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<(Self, Receiver<Push>), Error> {
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

        let (sender, receiver) = channel();
        Ok((
            TestClient {
                sender,
                my_uid,
                my_keys,
            },
            receiver,
        ))
    }
}

enum ClientInit {
    Reg(UserId, sig::KeyPair),
    Login(UserId, sig::KeyPair),
}

#[async_trait]
impl KrpcClient<HeraldProtocol> for TestClient {
    type InitInfo = ClientInit;

    async fn init<Tx: AsyncWrite + Send + Unpin, Rx: AsyncRead + Send + Unpin>(
        info: Self::InitInfo,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<Self, Error> {
        todo!()
    }

    async fn handle_push(
        &self,
        push: <HeraldProtocol as Protocol>::Push,
    ) -> <HeraldProtocol as Protocol>::PushAck {
        todo!()
    }

    async fn on_res(
        &self,
        req: &<HeraldProtocol as Protocol>::Req,
        res: &<HeraldProtocol as Protocol>::Res,
    ) -> Result<(), Error> {
        todo!()
    }

    fn on_close(&self) {
        todo!()
    }
}
