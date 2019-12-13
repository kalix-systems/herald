use async_trait::*;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver as Receiver, UnboundedSender as Sender},
    sink::*,
};
use kcl::random::UQ;
use krpc::*;
use kson::prelude::*;
use std::collections::{HashMap, HashSet};
use tokio::sync::*;
use void::Void;

#[derive(Ser, De, Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct User(UQ);

#[derive(Ser, De, Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Topic(UQ);

struct Server {
    sessions: Mutex<HashMap<User, Sender<Push>>>,
    topics: Mutex<HashMap<Topic, HashSet<User>>>,
    subscriptions: Mutex<HashMap<User, HashSet<Topic>>>,
}

#[derive(Ser, De, Debug)]
enum Req {
    Members(Topic),
    SendTo(Topic, Bytes),
    Subscribe(Topic),
    Unsubscribe(Topic),
}

#[derive(Ser, De, Debug)]
enum Resp {
    Members(HashSet<User>),
    Success,
}

#[derive(Ser, De, Debug, Clone)]
enum Push {
    UserJoined(Topic, User),
    UserLeft(Topic, User),
    NewMessage(Topic, User, Bytes),
}

type PushAck = ();

#[async_trait]
impl KrpcServer for Server {
    type InitError = Void;
    type ConnInfo = User;

    const MAX_CONCURRENT_REQS: usize = 1;
    const MAX_CONCURRENT_PUSHES: usize = 1;

    const MAX_REQ_SIZE: usize = 4096;
    const MAX_ACK_SIZE: usize = 4096;

    async fn init(
        &self,
        mut tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self::ConnInfo, KrpcError<Self::InitError>> {
        let u_bytes = rx.read_to_end(4096).await?;
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
        let (tx, rx) = unbounded();
        self.sessions.lock().await.insert(*meta, tx);
        rx
    }

    type PushAck = PushAck;
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
            Req::Members(t) => Resp::Members(
                self.topics
                    .lock()
                    .await
                    .get(&t)
                    .cloned()
                    .unwrap_or(HashSet::new()),
            ),
            Req::SendTo(t, b) => {
                let senders: Vec<Sender<Push>> = {
                    let members = self
                        .topics
                        .lock()
                        .await
                        .get(&t)
                        .cloned()
                        .unwrap_or(HashSet::new());
                    let sessions = self.sessions.lock().await;
                    members
                        .iter()
                        .filter_map(|m| sessions.get(m))
                        .cloned()
                        .collect()
                };

                let push = Push::NewMessage(t, *user, b);
                for mut sender in senders {
                    sender
                        .send(push.clone())
                        .await
                        .expect("failed to send push");
                }

                Resp::Success
            }
            Req::Subscribe(t) => {
                let mut topics = self.topics.lock().await;
                let mut subscriptions = self.subscriptions.lock().await;
                topics.entry(t).or_insert(HashSet::new()).insert(*user);
                subscriptions
                    .entry(*user)
                    .or_insert(HashSet::new())
                    .insert(t);
                Resp::Success
            }
            Req::Unsubscribe(t) => {
                let mut topics = self.topics.lock().await;
                let mut subscriptions = self.subscriptions.lock().await;
                subscriptions.get_mut(user).map(|ts| ts.remove(&t));
                topics.get_mut(&t).map(|s| s.remove(user));
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
        let mut topics = self.topics.lock().await;
        let mut subscriptions = self.subscriptions.lock().await;
        if let Some(subscribed_to) = subscriptions.remove(&user) {
            for sub in &subscribed_to {
                topics.get_mut(sub).map(|m| m.remove(&user));
            }
        }
    }
}

struct ClientInner {
    log: Mutex<HashMap<Topic, Vec<Push>>>,
    members: Mutex<HashMap<Topic, HashSet<User>>>,
}

#[async_trait]
impl KrpcClient for ClientInner {
    const MAX_CONCURRENT_REQS: usize = 1;
    const MAX_CONCURRENT_PUSHES: usize = 1;

    const MAX_RESP_SIZE: usize = 4096;
    const MAX_PUSH_SIZE: usize = 4096;

    type Req = Req;
    type Resp = Resp;
    type Push = Push;
    type PushAck = PushAck;

    type InitInfo = User;
    type InitError = Void;

    async fn init(
        user: Self::InitInfo,
        mut tx: quinn::SendStream,
        rx: quinn::RecvStream,
    ) -> Result<Self, KrpcError<Self::InitError>> {
        tx.write_all(&kson::to_vec(&user)).await?;
        tx.finish().await?;

        rx.read_to_end(0).await?;

        Ok(ClientInner {
            log: Mutex::new(HashMap::new()),
            members: Mutex::new(HashMap::new()),
        })
    }

    async fn handle_push(
        &self,
        push: Self::Push,
    ) -> Self::PushAck {
    }

    async fn on_close(&self) {}
}
