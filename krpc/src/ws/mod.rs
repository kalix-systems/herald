use super::*;
use futures::{
    future::{self, TryFuture},
    stream::{self, StreamExt, TryStreamExt},
};
use sharded_slab::*;
use std::{cmp::min, sync::Arc};
use tokio::sync::{mpsc::*, oneshot};
use tokio_tungstenite::WebSocketStream;
use tungstenite::{protocol::*, Message};

mod frames;
use frames::*;

trait WsProtocol: Protocol {
    fn max_item_size() -> usize {
        max(
            max(Self::MAX_REQ_SIZE, Self::MAX_ACK_SIZE),
            max(Self::MAX_RESP_SIZE, Self::MAX_PUSH_SIZE),
        )
    }
}

impl<P: Protocol> WsProtocol for P {}

#[derive(Debug)]
struct ConnectionClosed(Option<CloseFrame<'static>>);

impl std::fmt::Display for ConnectionClosed {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}

impl std::error::Error for ConnectionClosed {}

/// Does the server init handshake and sets up request/response + push loops.
/// Does not handle TLS handshake
pub async fn handle_conn<P, S, Conn>(
    server: &S,
    conn: Conn,
) -> Result<(), Error>
where
    S: KrpcServer<P>,
    P: Protocol,
    P::Push: Clone,
    Conn: AsyncRead + AsyncWrite + Unpin + Send,
{
    let (mut rx, mut tx) = tokio::io::split(conn);
    let cinfo = server.init(&mut tx, &mut rx).await?;

    let ws_config = WebSocketConfig {
        max_send_queue: Some(max(P::MAX_CONCURRENT_PUSHES, P::MAX_CONCURRENT_REQS)),
        max_message_size: Some(9 + P::max_item_size()),
        max_frame_size: Some(min(16 * 1024 * 1024, 9 + P::max_item_size())),
    };

    let conn = rx.unsplit(tx);
    let ws = WebSocketStream::from_raw_socket(conn, Role::Server, Some(ws_config)).await;
    let (wtx, wrx) = ws.split();

    let awaiting_acks: Slab<P::Push> = Slab::new();
    let (sframe_tx, sframe_rx) = channel::<ServerFrame<P::Res, P::Push>>(max(
        P::MAX_CONCURRENT_PUSHES,
        P::MAX_CONCURRENT_REQS,
    ));

    let send_sframes = sframe_rx
        .map(|s| Ok(Message::Binary(s.to_vec())))
        .forward(wtx)
        .map(|r| r.context("failed to send server frame over websocket"));

    let sink_pushes = {
        let mut sframe_tx = sframe_tx.clone();
        let cinfo = &cinfo;
        let awaiting_acks = &awaiting_acks;
        async move {
            let mut pushes = server.pushes(cinfo).await;
            while let Some(p) = pushes.next().await {
                let u = awaiting_acks
                    .insert(p.clone())
                    .ok_or_else(|| anyhow!("failed to insert push into slab"))?
                    as u64;
                let frame = ServerFrame::Psh(u, p);
                sframe_tx.send(frame).await?;
            }

            Ok::<(), Error>(())
        }
    };

    let recv_cframes = wrx
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
        .and_then(|b| future::ready(<ClientFrame<P::Req, P::PushAck>>::from_bytes(b)))
        .try_for_each_concurrent(P::MAX_CONCURRENT_REQS, {
            let sframe_tx = &sframe_tx;
            let cinfo = &cinfo;
            let awaiting = &awaiting_acks;
            move |c| {
                let mut sframe_tx = sframe_tx.clone();
                async move {
                    match c {
                        ClientFrame::Req(u, r) => {
                            let r = server.handle_req(cinfo, r).await;
                            let frame = ServerFrame::Res(u, r);
                            sframe_tx.send(frame).await.with_context(|| {
                                format!(
                                    "failed to sink response to query {} with cinfo {:?}",
                                    u, cinfo
                                )
                            })?;
                            Ok(())
                        }
                        ClientFrame::Ack(u, a) => {
                            if let Some(push) = awaiting.take(u as usize) {
                                server.on_push_ack(push, a).await;
                                Ok(())
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
            }
        })
        .boxed();

    future::try_join3(recv_cframes, sink_pushes, send_sframes).await?;

    Ok(())
}

pub async fn serve<'a, P, S>(
    server: &'a S,
    socket: &'a SocketAddr,
    tls: &'a tokio_rustls::TlsAcceptor,
) -> impl Stream<Item = impl TryFuture<Ok = (), Error = Error> + 'a> + 'a
where
    P: Protocol,
    S: KrpcServer<P>,
    P::Push: Clone,
{
    stream::unfold(
        tokio::net::TcpListener::bind(socket).await.ok(),
        move |mlistener| {
            async move {
                let mut listener = mlistener?;
                let (stream, _) = listener.accept().await.ok()?;
                let fut = async move {
                    let stream = tls.accept(stream).await?;
                    handle_conn::<P, S, tokio_rustls::server::TlsStream<tokio::net::TcpStream>>(
                        server, stream,
                    )
                    .await
                };
                Some((fut, Some(listener)))
            }
        },
    )
}

struct PendingReq<Req, Res> {
    req: Req,
    out: oneshot::Sender<Res>,
}

pub struct Client<P, K>
where
    P: Protocol,
    K: KrpcClient<P>,
{
    rtx: Sender<ClientFrame<P::Req, P::PushAck>>,
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
    ) -> Result<Self, Error> {
        let tcp = tokio::net::TcpStream::connect(server_sock).await?;
        let tls = connector.connect(server_name, tcp).await?;
        let (mut rx, mut tx) = tokio::io::split(tls);
        let inner = K::init(info, &mut tx, &mut rx).await?;
        let tls = rx.unsplit(tx);

        let ws_config = WebSocketConfig {
            max_send_queue: Some(max(P::MAX_CONCURRENT_PUSHES, P::MAX_CONCURRENT_REQS)),
            max_message_size: Some(9 + P::max_item_size()),
            max_frame_size: Some(min(16 * 1024 * 1024, 9 + P::max_item_size())),
        };

        let ws = WebSocketStream::from_raw_socket(tls, Role::Client, Some(ws_config)).await;
        let (wtx, wrx) = ws.split();

        let (cframe_tx, cframe_rx) = channel::<ClientFrame<P::Req, P::PushAck>>(max(
            P::MAX_CONCURRENT_PUSHES,
            P::MAX_CONCURRENT_REQS,
        ));

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
            .try_for_each_concurrent(max(P::MAX_CONCURRENT_PUSHES, P::MAX_CONCURRENT_REQS), {
                let cframe_tx = cframe_tx.clone();
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
                                cframe_tx.clone().send(ClientFrame::Ack(u, ack)).await?;
                            }
                        }

                        Ok(())
                    }
                }
            });

        let driver = future::try_join(send_cframes, recv_sframes).map_ok(drop);
        tokio::spawn(driver.unwrap_or_else(|e| eprintln!("{}", e)));

        Ok(acli)
    }

    pub async fn req(
        &self,
        req: P::Req,
    ) -> Result<oneshot::Receiver<P::Res>, Error>
    where
        P::Req: Clone,
    {
        let mut rtx = self.rtx.clone();
        let (tx, rx) = oneshot::channel();
        let u = self
            .awaiting
            .insert(PendingReq {
                req: req.clone(),
                out: tx,
            })
            .ok_or(anyhow!("requests slab was at capacity"))?;

        if let Err(e) = rtx.send(ClientFrame::Req(u as u64, req.clone())).await {
            self.awaiting.remove(u);
            Err(e).with_context(|| format!("failed to sink request {:?}", req))?
        }

        Ok(rx)
    }
}
