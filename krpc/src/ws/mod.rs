use super::*;
use futures::{
    future::{self, BoxFuture},
    sink::SinkExt,
    stream::{self, BoxStream, StreamExt, TryStreamExt},
};
use sharded_slab::*;
use std::cmp::min;
use tokio::sync::mpsc::*;
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
    let ws = tokio_tungstenite::accept_async_with_config(conn, Some(ws_config)).await?;
    let (wtx, wrx) = ws.split();

    let awaiting_acks: Slab<P::Push> = Slab::new();
    let (mut sframe_tx, sframe_rx) =
        channel::<ServerFrame<P::Res, P::Push>>(P::MAX_CONCURRENT_PUSHES);

    let send_sframes = sframe_rx
        .map(|s| Ok::<_, Error>(Message::Binary(s.to_vec())))
        .forward(wtx.sink_map_err(Error::from));

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
            let sframe_tx = sframe_tx.clone();
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

    let sink_pushes = async {
        let mut pushes = server.pushes(&cinfo).await;
        while let Some(p) = pushes.next().await {
            let u = awaiting_acks
                .insert(p.clone())
                .ok_or_else(|| anyhow!("failed to insert push into slab"))?
                as u64;
            let frame = ServerFrame::Psh(u, p);
            sframe_tx.send(frame).await?;
        }

        Ok::<(), Error>(())
    };

    future::try_join3(recv_cframes, sink_pushes, send_sframes).await?;

    Ok(())
}

/// `serve(server,socket,tls,out)` listens for connections on `socket`, performs tls handshake
/// using acceptor `tls`, then leaves the stream of futures in a variable `out`, to be handled
/// however the caller decides to.
pub async fn serve<'a, P, S>(
    server: &'a S,
    socket: &'a SocketAddr,
    tls: &'a tokio_rustls::TlsAcceptor,
) -> BoxStream<'a, BoxFuture<'a, Result<(), Error>>>
where
    P: Protocol,
    S: KrpcServer<P>,
    P::Push: Clone,
{
    match tokio::net::TcpListener::bind(socket).await {
        Err(e) => stream::once({ future::ready(future::err(e.into()).boxed()) }).boxed(),
        Ok(listener) => stream::unfold(listener, move |mut listener| {
            async move {
                let (stream, _) = listener.accept().await.ok()?;
                let fut: BoxFuture<'a, Result<(), Error>> = async move {
                    let stream = tls.accept(stream).await?;
                    handle_conn::<P, S, tokio_rustls::server::TlsStream<tokio::net::TcpStream>>(
                        server, stream,
                    )
                    .await
                }
                .boxed();
                Some((fut, listener))
            }
        })
        .boxed(),
    }
}
