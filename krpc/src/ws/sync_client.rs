use super::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::net::*;

fn spawn<F: FnOnce() + Send + 'static>(f: F) -> Result<(), Error> {
    std::thread::Builder::new().spawn(move || f())?;
    Ok(())
}

struct PendingReq<Req, Res> {
    req: Req,
    out: Sender<Res>,
}

pub struct Client<P, C>
where
    P: Protocol,
    C: SyncClient<P>,
{
    rtx: Sender<ClientFrame<P::Req, P::PushAck>>,
    awaiting: Arc<Slab<PendingReq<P::Req, P::Res>>>,
    inner: Arc<C>,
}

impl<P: Protocol, C: SyncClient<P>> Clone for Client<P, C> {
    fn clone(&self) -> Self {
        Client {
            rtx: self.rtx.clone(),
            awaiting: self.awaiting.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<P, C> Client<P, C>
where
    P: Protocol,
    C: SyncClient<P>,
{
    pub fn connect(
        info: C::InitInfo,
        tls_config: &'static rustls::ClientConfig,
        server_name: webpki::DNSNameRef<'_>,
        server_sock_addr: &SocketAddr,
    ) -> Result<Self, Error> {
        let mut sock = TcpStream::connect(server_sock_addr)?;
        let mut sess = rustls::ClientSession::new(&Arc::new(tls_config.clone()), server_name);
        let mut tls = rustls::Stream::new(&mut sess, &mut sock);

        let mut framed = Framed::new(tls);

        let inner = C::init(info, &mut framed)?;

        let tls = framed.into_inner();

        let ws_config = WebSocketConfig {
            max_send_queue: max(P::MAX_CONCURRENT_PUSHES, P::MAX_CONCURRENT_REQS).into(),
            max_message_size: (9 + P::max_item_size()).into(),
            max_frame_size: min(16 * 1024 * 1024, 9 + P::max_item_size()).into(),
        };

        WebSocket::from_raw_socket(tls, Role::Client, ws_config.into());

        let (cframe_tx, cframe_rx) = unbounded::<ClientFrame<P::Req, P::PushAck>>();

        let acli = Client {
            rtx: cframe_tx.clone(),
            awaiting: Arc::new(Slab::new()),
            inner: Arc::new(inner),
        };

        todo!()
    }
}

//impl<P, C> Client<P, C>
//where
//    P: Protocol,
//    C: SyncClient<P>,
//{
//    pub fn req(
//        &self,
//        req: P::Req,
//    ) -> Result<oneshot::Receiver<P::Res>, Error>
//    where
//        P::Req: Clone,
//    {
//        let (tx, rx) = oneshot::channel();
//        let u = self
//            .awaiting
//            .insert(PendingReq {
//                req: req.clone(),
//                out: tx,
//            })
//            .ok_or(anyhow!("requests slab was at capacity"))?;
//
//        if let Err(e) = self.rtx.send(ClientFrame::Req(u as u64, req.clone())) {
//            self.awaiting.remove(u);
//            Err(e).with_context(|| format!("failed to sink request {:?}", req))?
//        }
//
//        Ok(rx)
//    }
//
//    pub fn quit(&self) -> Result<(), Error> {
//        self.rtx.send(ClientFrame::Quit)?;
//        Ok(())
//    }
//
//    pub fn into_inner(self) -> Arc<K> {
//        drop(self.quit());
//        self.inner
//    }
//}
