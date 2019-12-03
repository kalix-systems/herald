use dashmap::DashMap;
use futures::stream::*;
use herald_common::*;
use server_errors::*;
use server_store::*;
use std::time::Duration;
use stream_cancel::{Trigger, Tripwire, Valved};
use tokio::{
    prelude::*,
    sync::{
        mpsc::{
            unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
        },
        oneshot,
    },
    timer::Timeout,
};
use warp::filters::ws::{self, WebSocket};

type WTx = SplitSink<WebSocket, ws::Message>;

pub struct ActiveSession {
    interrupt: Trigger,
    done: Tripwire,
    emitter: Sender<()>,
}

impl ActiveSession {
    #[allow(clippy::unneeded_field_pattern)]
    pub async fn interrupt(self) -> bool {
        let ActiveSession {
            interrupt,
            done,
            emitter: _,
        } = self;

        interrupt.cancel();
        done.await
    }

    pub fn emit(&self) -> Result<(), tokio::sync::mpsc::error::UnboundedTrySendError<()>> {
        self.emitter.clone().try_send(())
    }
}

type ActiveSessions = DashMap<sig::PublicKey, ActiveSession>;

#[derive(Default)]
pub struct State {
    pub active: ActiveSessions,
    pub pool: Pool,
}

pub mod get;
pub mod login;
pub mod post;

impl State {
    pub fn new() -> Self {
        State {
            active: DashMap::default(),
            pool: Pool::new(),
        }
    }

    pub async fn new_connection(&self) -> Result<Conn, Error> {
        Ok(self.pool.get().await?)
    }

    pub async fn handle_login(
        &'static self,
        ws: WebSocket,
    ) -> Result<(), Error> {
        let mut store: Conn = self.new_connection().await?;

        // all the channels we'll need for plumbing
        // first we split the websocket
        let (mut wtx, mut wrx) = ws.split();
        // bytevec messages received from the socket
        let (mut rtx, mut rrx) = channel::<Vec<u8>>();
        // push emitter which will be stored in the active sessions dashmap
        let (ptx, prx) = channel::<()>();
        // session-close emitter
        let (close, closed) = oneshot::channel::<()>();

        // on graceful exit, notify runtime to close channel
        // we set things up this way so that the rrx channel
        // will be populated before we call login, hence before
        // we know the gid
        tokio::spawn(async move {
            while let Some(Ok(m)) = wrx.next().await {
                if m.is_close() || (m.is_binary() && rtx.send(m.into_bytes()).await.is_err()) {
                    break;
                }
            }

            close.send(()).ok();
        });

        let gid: GlobalId = login::login(&self.active, &mut store, &mut wtx, &mut rrx).await?;

        let (interrupt, prx) = Valved::new(prx);
        let (trigger_done, done) = Tripwire::new();
        let sess = ActiveSession {
            interrupt,
            done,
            emitter: ptx,
        };
        self.active.insert(gid.did, sess);

        // remove active session on graceful exit
        tokio::spawn(async move {
            drop(closed.await);
            if let Some((_, s)) = self.active.remove(&gid.did) {
                s.interrupt().await;
            }
        });

        // TODO: handle this error somehow?
        // for now we're just dropping it
        if catchup(gid.did, &mut store, &mut wtx, &mut rrx)
            .await
            .is_ok()
        {
            let mut prx: Timeout<Valved<Receiver<()>>> = prx.timeout(Duration::from_secs(60));
            drop(
                self.send_pushes(&mut store, &mut wtx, &mut rrx, &mut prx, gid.did)
                    .await,
            );
            trigger_done.cancel();
        }

        if let Some((_, s)) = self.active.remove(&gid.did) {
            s.interrupt().await;
        }

        Ok(())
    }

    pub async fn push_users(
        &self,
        req: push_users::Req,
    ) -> Result<push_users::Res, Error> {
        let push_users::Req { to, exc, msg } = req;
        let msg: Push = Push {
            tag: PushTag::User,
            timestamp: Time::now(),
            msg,
        };

        let mut missing_users: Vec<UserId> = Vec::new();
        let mut to_devs: Vec<sig::PublicKey> = Vec::new();
        let mut conn: Conn = self.new_connection().await?;

        for user in to {
            if !conn.user_exists(&user).await? {
                missing_users.push(user);
            } else {
                for dev in conn.valid_keys(&user).await? {
                    if dev != exc {
                        to_devs.push(dev);
                    }
                }
            }
        }

        Ok(if !missing_users.is_empty() {
            push_users::Res::Missing(missing_users)
        } else {
            self.send_push_to_devices(&mut conn, to_devs, msg).await?;
            push_users::Res::Success
        })
    }

    pub async fn push_aux(
        &self,
        req: push_aux::Req,
    ) -> Result<push_aux::Res, Error> {
        let push_aux::Req { to, exc, msg } = req;
        let push = Push {
            tag: PushTag::Aux,
            timestamp: Time::now(),
            msg,
        };

        let mut conn: Conn = self.new_connection().await?;

        let mut missing_users: Vec<UserId> = Vec::with_capacity(to.len());

        for user in to.iter() {
            if !conn.user_exists(user).await? {
                missing_users.push(user.clone());
            }
        }

        Ok(if !missing_users.is_empty() {
            push_aux::Res::Missing(missing_users)
        } else {
            let mut to_devs: Vec<sig::PublicKey> = Vec::with_capacity(2 * to.len());
            for user in to {
                for dev in conn.valid_keys(&user).await? {
                    if dev != exc {
                        to_devs.push(dev);
                    }
                }
            }

            self.send_push_to_devices(&mut conn, to_devs, push).await?;

            push_aux::Res::Success
        })
    }

    pub async fn push_devices(
        &self,
        req: push_devices::Req,
    ) -> Result<push_devices::Res, Error> {
        let push_devices::Req { to, msg } = req;
        let msg = Push {
            tag: PushTag::Device,
            timestamp: Time::now(),
            msg,
        };

        let mut conn = self.new_connection().await?;
        let mut missing_devs: Vec<sig::PublicKey> = Vec::new();

        for dev in to.iter() {
            if !conn.device_exists(dev).await? {
                missing_devs.push(*dev);
            }
        }

        Ok(if !missing_devs.is_empty() {
            push_devices::Res::Missing(missing_devs)
        } else {
            self.send_push_to_devices(&mut conn, to, msg).await?;
            push_devices::Res::Success
        })
    }

    async fn send_push_to_devices(
        &self,
        con: &mut Conn,
        to_devs: Vec<sig::PublicKey>,
        msg: Push,
    ) -> Result<(), Error> {
        con.add_pending(to_devs.clone(), &[msg]).await?;

        for dev in to_devs {
            if let Some(s) = self.active.async_get(dev).await {
                drop(s.emit());
            }
        }

        Ok(())
    }

    async fn send_pushes(
        &self,
        store: &mut Conn,
        wtx: &mut WTx,
        rrx: &mut Receiver<Vec<u8>>,
        rx: &mut Timeout<Valved<Receiver<()>>>,
        did: sig::PublicKey,
    ) -> Result<(), Error> {
        while let Some(p) = rx.next().await {
            if p.is_ok() {
                catchup(did, store, wtx, rrx).await?;
            } else {
                wtx.send(ws::Message::ping(vec![0u8]))
                    .timeout(Duration::from_secs(5))
                    .await??;
            }
        }

        Ok(())
    }
}

async fn catchup(
    did: sig::PublicKey,
    s: &mut Conn,
    wtx: &mut WTx,
    rrx: &mut Receiver<Vec<u8>>,
) -> Result<(), Error> {
    use catchup::*;

    loop {
        let pending: Vec<Push> = s.get_pending(did, CHUNK_SIZE).await?;
        if pending.is_empty() {
            break;
        } else {
            let len = pending.len() as u64;
            let msg = Catchup::Messages(pending);

            loop {
                write_msg(&msg, wtx, rrx).await?;

                if CatchupAck(len) == read_msg(rrx).await? {
                    s.expire_pending(did, len as u32).await?;
                    break;
                }
            }
        }
    }

    write_msg(&Catchup::Done, wtx, rrx).await?;

    Ok(())
}

const TIMEOUT_DUR: std::time::Duration = Duration::from_secs(10);

async fn read_msg<T>(rx: &mut Receiver<Vec<u8>>) -> Result<T, Error>
where
    T: De,
{
    let m = rx.next().await.ok_or(StreamDied)?;
    let t = kson::from_slice(&m)?;
    Ok(t)
}

fn ser_msg<T: Ser>(t: &T) -> ws::Message {
    ws::Message::binary(kson::to_vec(t))
}

async fn write_msg<T>(
    t: &T,
    wtx: &mut WTx,
    rrx: &mut Receiver<Vec<u8>>,
) -> Result<(), Error>
where
    T: Ser,
{
    let bvec = Bytes::from(kson::to_vec(t));
    let packets = Packet::from_bytes(bvec);
    let len = packets.len() as u64;

    loop {
        wtx.send(ser_msg(&len)).timeout(TIMEOUT_DUR).await??;

        if len == read_msg::<u64>(rrx).timeout(TIMEOUT_DUR).await?? {
            wtx.send(ser_msg(&PacketResponse::Success))
                .timeout(TIMEOUT_DUR)
                .await??;
            break;
        } else {
            wtx.send(ser_msg(&PacketResponse::Retry))
                .timeout(TIMEOUT_DUR)
                .await??;
        }
    }

    loop {
        for packet in packets.iter() {
            wtx.send(ser_msg(packet)).timeout(TIMEOUT_DUR).await??;
        }

        match read_msg(rrx).timeout(TIMEOUT_DUR).await?? {
            PacketResponse::Success => break,
            PacketResponse::Retry => {}
        }
    }

    Ok(())
}
