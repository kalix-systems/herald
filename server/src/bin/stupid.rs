#![feature(async_await, async_closure, try_blocks)]
/// this is the stupidest version of the server I know how to write
/// offline delivery only works as long as the server is online.
///
/// Nothing is encrypted, except maybe eventually with TLS.
use ccl::dashmap::DashMap;
use chrono::prelude::*;
use crossbeam_queue::SegQueue;
use failure::*;
use herald_common::*;
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::net;
use tokio::prelude::*;

use MessageToClient::*;
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

impl<Sock: AsyncWrite + Unpin> AppState<Sock> {
    pub fn new() -> Self {
        AppState {
            meta: Arc::new(DashMap::default()),
            open: Arc::new(DashMap::default()),
            pending: Arc::new(DashMap::default()),
        }
    }

    // TODO implement this
    #[allow(unused_variables)]
    pub async fn send_msg(&self, to: UserId, msg: MessageToClient) -> Result<(), Error> {
        let u = self
            .meta
            .async_get(to)
            .await
            .ok_or(format_err!("couldn't find user {}", to))?;
        for d in 0..u.num_devices {
            let gid = GlobalId { uid: to, did: d };
            if let Some(mut s) = self.open.async_get_mut(gid).await {
                let raw = serde_cbor::to_vec(&msg)?;
                let len = u64::to_le_bytes(raw.len() as u64);
                s.write_all(&len).await?;
                s.write_all(&raw).await?;
            } else if let Some(q) = self.pending.async_get(gid).await {
                // TODO: consider removing cloning here?
                q.push(msg.clone());
            } else {
                let q = self.pending.get_or_insert_with(&gid, || SegQueue::new());
                q.push(msg.clone());
            }
        }
        Ok(())
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

    println!("Listening on: {}", addr);
    let mut listener = net::TcpListener::bind(&addr).expect("unable to bind TCP listener");
    while let Ok((stream, addr)) = listener.accept().await {
        let state = state.clone();
        // todo: factor this into functions rather than the main from hell
        tokio::spawn(async move {
            let comp: Result<(), Error> = try {
                let (mut reader, writer) = stream.split();
                let gid: GlobalId = read_datagram(&mut reader).await?;
                state.open.insert(gid, writer);
                if let Some(mut u) = state.meta.async_get_mut(gid.uid).await {
                    let devs = std::cmp::max(u.num_devices, gid.did + 1);
                    u.num_devices = devs;
                } else {
                    state.meta.insert(
                        gid.uid.clone(),
                        User {
                            num_devices: gid.did + 1,
                        },
                    );
                }
                if let Some((_, p)) = state.pending.remove(&gid) {
                    if let Some(mut w) = state.open.async_get_mut(gid).await {
                        while !p.is_empty() {
                            let msg = p.pop()?;
                            send_datagram(w.deref_mut(), &msg).await?;
                        }
                    }
                }
                loop {
                    let d = read_datagram(&mut reader).await;
                    if let Err(e) = d {
                        eprintln!("connection to {} closing with msg {:?}", addr, e);
                        break;
                    };
                    match d.unwrap() {
                        SendMsg { to, text } => {
                            state
                                .send_msg(
                                    to,
                                    NewMessage {
                                        from: gid,
                                        text: text,
                                        time: Utc::now(),
                                    },
                                )
                                .await?
                        }
                        RequestMeta { of } => {
                            let reply = match state.meta.async_get(of).await {
                                Some(m) => Response::Meta(m.clone()),
                                None => Response::DataNotFound,
                            };
                            let msg = QueryResponse {
                                res: reply,
                                query: RequestMeta { of },
                            };
                            if let Some(mut w) = state.open.async_get_mut(gid).await {
                                send_datagram(w.deref_mut(), &msg).await?;
                            }
                        }
                        RegisterDevice => {
                            let reply = match state.meta.async_get_mut(gid.uid).await {
                                Some(mut m) => {
                                    let id = m.num_devices;
                                    m.num_devices += 1;
                                    Response::DeviceRegistered(id)
                                }
                                None => Response::DataNotFound,
                            };
                            let msg = MessageToClient::QueryResponse {
                                res: reply,
                                query: RegisterDevice,
                            };
                            if let Some(mut w) = state.open.async_get_mut(gid).await {
                                send_datagram(w.deref_mut(), &msg).await?;
                            }
                        }
                    }
                }
                state.open.remove(&gid);
            };
            if let Err(e) = comp {
                eprintln!("connection to {} failed, error was: {:?}", addr, e);
            }
        });
    }
}
