use super::*;
use futures::{
    future::{self, FutureExt, TryFuture, TryFutureExt},
    stream::{StreamExt, TryStreamExt},
};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot,
};
use tokio_tungstenite::WebSocketStream;

struct PendingReq<Req, Res> {
    req: Req,
    out: oneshot::Sender<Res>,
}

pub struct Client<P, K>
where
    P: Protocol,
    K: KrpcClient<P>,
{
    rtx: UnboundedSender<ClientFrame<P::Req, P::PushAck>>,
    awaiting: Arc<Slab<PendingReq<P::Req, P::Res>>>,
    inner: Arc<K>,
}

impl<P: Protocol, K: KrpcClient<P>> Clone for Client<P, K> {
    fn clone(&self) -> Self {
        Client {
            rtx: self.rtx.clone(),
            awaiting: self.awaiting.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<P: Protocol, K: KrpcClient<P>> std::ops::Deref for Client<P, K> {
    type Target = Arc<K>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, K> Client<P, K>
where
    P: Protocol + 'static,
    K: KrpcClient<P>,
{
    pub async fn connect(
        info: K::InitInfo,
        connector: &tokio_rustls::TlsConnector,
        server_name: webpki::DNSNameRef<'_>,
        server_sock: &SocketAddr,
    ) -> Result<(Self, impl TryFuture<Ok = (), Error = Error>), Error> {
        let tcp = tokio::net::TcpStream::connect(server_sock).await?;
        let tls = connector.connect(server_name, tcp).await?;
        let (mut rx, mut tx) = Framed::new(tls).split();
        let inner = K::init(info, &mut tx, &mut rx).await?;
        let tls = Framed::unsplit(rx, tx).into_inner();

        let ws_config = WebSocketConfig {
            max_send_queue: Some(max(P::MAX_CONCURRENT_PUSHES, P::MAX_CONCURRENT_REQS)),
            max_message_size: Some(9 + P::max_item_size()),
            max_frame_size: Some(min(16 * 1024 * 1024, 9 + P::max_item_size())),
        };

        let ws = WebSocketStream::from_raw_socket(tls, Role::Client, Some(ws_config)).await;
        let (wtx, wrx) = ws.split();

        let (cframe_tx, cframe_rx) = unbounded_channel::<ClientFrame<P::Req, P::PushAck>>();

        let acli = Client {
            rtx: cframe_tx.clone(),
            awaiting: Arc::new(Slab::new()),
            inner: Arc::new(inner),
        };

        let send_cframes = cframe_rx
            .map(|f| Ok(Message::Binary(f.to_vec())))
            .forward(wtx)
            .map(|r| r.context("failed to send client frame over websocket"));

        let recv_sframes = wrx
            .map_err(Error::from)
            .try_filter_map(|m| {
                future::ready(match m {
                    Message::Binary(b) => Ok(Some(Bytes::from(b))),
                    Message::Close(c) => Err(anyhow!(ConnectionClosed(c))),
                    Message::Text(_) => Ok(None),
                    Message::Ping(_) => Ok(None),
                    Message::Pong(_) => Ok(None),
                })
            })
            .and_then(|b| future::ready(<ServerFrame<P::Res, P::Push>>::from_bytes(b)))
            .try_for_each_concurrent(P::MAX_CONCURRENT_PUSHES + P::MAX_CONCURRENT_REQS, {
                let client = acli.clone();
                move |s| {
                    let cframe_tx = cframe_tx.clone();
                    let client = client.clone();
                    async move {
                        match s {
                            ServerFrame::Res(u, res) => {
                                if let Some(PendingReq { req, out }) =
                                    client.awaiting.take(u as usize)
                                {
                                    client.inner.on_res(&req, &res).await.with_context(|| {
                                        format!("handling response {:?} to push {:?}", res, req)
                                    })?;
                                    drop(out.send(res));
                                }
                            }
                            ServerFrame::Psh(u, p) => {
                                let ack = client.inner.handle_push(p).await;
                                cframe_tx.clone().send(ClientFrame::Ack(u, ack))?;
                            }
                            ServerFrame::Quit => bail!(ConnectionClosed(None)),
                        }

                        Ok(())
                    }
                }
            });

        let driver = future::try_join(send_cframes, recv_sframes).map_ok(drop);
        // tokio::spawn(driver.unwrap_or_else(|e| eprintln!("{}", e)));

        Ok((acli, driver))
    }
}

impl<P, K> Client<P, K>
where
    P: Protocol,
    K: KrpcClient<P>,
{
    pub fn req(
        &self,
        req: P::Req,
    ) -> Result<oneshot::Receiver<P::Res>, Error>
    where
        P::Req: Clone,
    {
        let (tx, rx) = oneshot::channel();
        let u = self
            .awaiting
            .insert(PendingReq {
                req: req.clone(),
                out: tx,
            })
            .ok_or_else(|| anyhow!("requests slab was at capacity"))?;

        if let Err(e) = self.rtx.send(ClientFrame::Req(u as u64, req.clone())) {
            self.awaiting.remove(u);
            Err(e).with_context(|| format!("failed to sink request {:?}", req))?
        }

        Ok(rx)
    }

    pub fn quit(&self) -> Result<(), Error> {
        self.rtx.send(ClientFrame::Quit)?;
        Ok(())
    }

    pub fn into_inner(self) -> Arc<K> {
        drop(self.quit());
        self.inner
    }
}

// TODO: figure out why this causes crashes
// impl<P: Protocol, K: KrpcClient<P>> Drop for Client<P, K> {
//     fn drop(&mut self) {
//         self.inner.on_close();
//         drop(self.quit())
//     }
// }
