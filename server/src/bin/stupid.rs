#![feature(async_await, async_closure, try_blocks)]
/// this is the stupidest version of the server I know how to write
/// offline delivery only works as long as the server is online.
///
/// Nothing is encrypted, except maybe eventually with TLS.
use ccl::dashmap::DashMap;
use herald_common::*;
use crossbeam_queue::SegQueue;
use failure::*;
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::net;
use tokio::prelude::*;

use MessageToServer::*;

pub struct AppState<Sock: AsyncWrite> {
    meta: Arc<DashMap<UserId, User>>,
    open: Arc<DashMap<GlobalId, Sock>>,
    pending: Arc<DashMap<GlobalId, SegQueue<MessageToClient>>>,
}

impl<S: AsyncWrite> Clone for AppState<S> {
    fn clone(&self) -> Self {
        AppState {
            meta: self.meta.clone(),
            open: self.open.clone(),
            pending: self.pending.clone(),
        }
    }
}
impl<Sock: AsyncWrite> AppState<Sock> {
    pub fn new() -> Self {
        AppState {
            meta: Arc::new(DashMap::default()),
            open: Arc::new(DashMap::default()),
            pending: Arc::new(DashMap::default()),
        }
    }

    // TODO implement this
    #[allow(unused_variables)]
    pub async fn send_msg(&self, to: UserId, msg: RawMsg) -> Result<(), Error> {
        unimplemented!()
    }
}

const PORT: u16 = 8000;

async fn read_datagram<S: AsyncRead + Unpin, T: for<'a> Deserialize<'a>>(
    s: &mut S,
) -> Result<T, Error> {
    let mut buf = [0u8; 8];
    s.read_exact(&mut buf).await?;
    let len = u64::from_le_bytes(buf) as usize;
    let mut buf = vec![0u8; len];
    s.read_exact(&mut buf).await?;
    let res = serde_cbor::from_slice(&buf)?;
    Ok(res)
}

async fn send_datagram<S: AsyncWrite + Unpin, T: Serialize>(s: &mut S, t: &T) -> Result<(), Error> {
    let vec = serde_cbor::to_vec(t)?;
    let len = u64::to_le_bytes(vec.len() as u64);
    s.write_all(&len).await?;
    s.write_all(&vec).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let state: AppState<net::tcp::split::TcpStreamWriteHalf> = AppState::new();
    let addr = format!("0.0.0.0:{}", PORT).parse().unwrap();

    let mut listener = net::TcpListener::bind(&addr).expect("unable to bind TCP listener");
    while let Ok((stream, addr)) = listener.accept().await {
        let state = state.clone();
        tokio::spawn(async move {
            let comp: Result<(), Error> = try {
                let (mut reader, writer) = stream.split();
                let uid: GlobalId = read_datagram(&mut reader).await?;
                state.open.insert(uid, writer);
                if let Some((_, p)) = state.pending.remove(&uid) {
                    if let Some(mut w) = state.open.async_get_mut(uid).await {
                        while !p.is_empty() {
                            let msg = p.pop()?;
                            send_datagram(w.deref_mut(), &msg).await?;
                        }
                    }
                }
                loop {
                    let e: Result<MessageToServer, _> = read_datagram(&mut reader).await;
                    match e {
                        Ok(Send { to, text }) => state.send_msg(to, text).await?,
                        Err(e) => {
                            eprintln!("connection to {} closing with msg {:?}", addr, e);
                            break;
                        }
                    }
                }
            };
            if let Err(e) = comp {
                eprintln!("connection to {} failed, error was: {:?}", addr, e);
            }
        });
    }
    let server = listener.incoming().for_each(move |rsock| {
        let state = state.clone();
        async move {}
    });
}
