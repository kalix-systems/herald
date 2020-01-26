use super::*;
use futures::{
    future::{self, Future, FutureExt},
    stream::{self, StreamExt, TryStreamExt},
};
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::WebSocketStream;

pub(crate) mod prelude {
    pub use super::{handle_conn, serve, serve_arc};
}

/// Does the server init handshake and sets up request/response + push loops.
/// Does not handle TLS handshake
pub async fn handle_conn<P, S, Conn>(
    server: &S,
    conn: Conn,
) -> Result<(), Error>
where
    S: KrpcServer<P>,
    P: Protocol,
    S::ServePush: Into<P::Push> + Clone,
    Conn: AsyncRead + AsyncWrite + Unpin + Send,
{
    let (mut rx, mut tx) = Framed::new(conn).split();
    let cinfo = server.init(&mut tx, &mut rx).await?;
    let conn = Framed::unsplit(rx, tx).into_inner();

    let ws_config = WebSocketConfig {
        max_send_queue: Some(max(P::MAX_CONCURRENT_PUSHES, P::MAX_CONCURRENT_REQS)),
        max_message_size: Some(9 + P::max_item_size()),
        max_frame_size: Some(min(16 * 1024 * 1024, 9 + P::max_item_size())),
    };

    let ws = WebSocketStream::from_raw_socket(conn, Role::Server, Some(ws_config)).await;
    let (wtx, wrx) = ws.split();

    let awaiting_acks: Slab<_> = Slab::new();
    let (sframe_tx, sframe_rx) = unbounded_channel::<ServerFrame<P::Res, P::Push>>();

    let send_sframes = sframe_rx
        .map(|s| Ok(Message::Binary(s.to_vec())))
        .forward(wtx)
        .map(|r| r.context("failed to send server frame over websocket"));

    let sink_pushes = {
        let sframe_tx = &sframe_tx;
        let cinfo = &cinfo;
        let awaiting_acks = &awaiting_acks;
        async move {
            let mut pushes = server.pushes(cinfo).await?;
            while let Some(p) = pushes.next().await {
                let u = awaiting_acks
                    .insert(p.clone())
                    .ok_or_else(|| anyhow!("failed to insert push into slab"))?
                    as u64;
                let frame = ServerFrame::Psh(u, p.into());
                sframe_tx.send(frame)?;
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
                let sframe_tx = sframe_tx.clone();
                async move {
                    match c {
                        ClientFrame::Req(u, r) => {
                            let r = server.handle_req(cinfo, r).await;
                            let frame = ServerFrame::Res(u, r);
                            sframe_tx.send(frame).with_context(|| {
                                format!(
                                    "failed to sink response to query {} with cinfo {:?}",
                                    u, cinfo
                                )
                            })?;
                            Ok(())
                        }
                        ClientFrame::Ack(u, a) => {
                            if let Some(push) = awaiting.take(u as usize) {
                                server.on_push_ack(cinfo, push, a).await?;
                                Ok(())
                            } else {
                                Ok(())
                            }
                        }
                        ClientFrame::Quit => Err(anyhow!(ConnectionClosed(None))),
                    }
                }
            }
        })
        .boxed();

    if let Err(e) = future::try_join3(recv_cframes, sink_pushes, send_sframes).await {
        let ConnectionClosed(_) = e.downcast()?;
    }

    Ok(())
}

macro_rules! serve_body {
    ($server:expr, $socket:expr, $tls:expr) => {
        stream::unfold(
            tokio::net::TcpListener::bind($socket).await.ok(),
            move |mlistener| {
                let server = $server;
                let tls = $tls.clone();
                async move {
                    let mut listener = mlistener?;
                    let (stream, _) = listener.accept().await.ok()?;
                    let fut = async move {
                        let stream = tls.accept(stream).await?;
                        handle_conn::<P, S, tokio_rustls::server::TlsStream<tokio::net::TcpStream>>(
                            server.deref(),
                            stream,
                        )
                        .await
                    };
                    Some((fut, Some(listener)))
                }
            },
        )
    };
}

pub async fn serve<'a, P, S>(
    server: &'a S,
    socket: SocketAddr,
    tls: tokio_rustls::TlsAcceptor,
) -> impl Stream<Item = impl Future<Output = Result<(), Error>> + 'a> + 'a
where
    P: Protocol,
    S: KrpcServer<P>,
    P::Push: Clone,
    S::ServePush: Into<P::Push> + Clone,
{
    serve_body!(server, socket, tls)
}

pub async fn serve_arc<P, S>(
    server: Arc<S>,
    socket: SocketAddr,
    tls: tokio_rustls::TlsAcceptor,
) -> impl Stream<Item = impl Future<Output = Result<(), Error>>>
where
    P: Protocol,
    S: KrpcServer<P>,
    P::Push: Clone,
    S::ServePush: Into<P::Push> + Clone,
{
    serve_body!(server.clone(), &socket, tls.clone())
}
