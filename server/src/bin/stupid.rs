//! this is the stupidest version of the server I know how to write
//! offline delivery only works as long as the server is online.
//!
//! Nothing is encrypted, except maybe eventually with TLS.

#![feature(async_await, async_closure, try_blocks)]
use bytes::Bytes;
use ccl::dashmap::DashMap;
use chrono::prelude::*;
use crossbeam_queue::SegQueue;
use failure::*;
use herald_common::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
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

    pub async fn send_msg(&self, from: GlobalId, to: UserId, body: RawMsg) -> Result<(), Error> {
        let wrapped = Push {
            from,
            body,
            time: Utc::now(),
        };
        let u = self
            .meta
            .async_get(to.clone())
            .await
            .ok_or(format_err!("couldn't find user {}", to.clone()))?;

        for d in 0..u.num_devices {
            let gid = GlobalId {
                uid: to.clone(),
                did: d.try_into()?,
            };
            if let Some(mut s) = self.open.async_get_mut(gid.clone()).await {
                let raw = serde_cbor::to_vec(&wrapped)?;
                let len = u64::to_le_bytes(raw.len() as u64);
                s.write_all(&len).await?;
                s.write_all(&raw).await?;
            } else if let Some(q) = self.pending.async_get(gid.clone()).await {
                // TODO: consider removing cloning here?
                q.push(wrapped.clone());
            } else {
                let q = self.pending.get_or_insert_with(&gid, || SegQueue::new());
                q.push(wrapped.clone());
            }
        }
        Ok(())
    }

    pub async fn request_meta(&self, of: &UserId) -> Result<Response, Error> {
        try {
            match self.meta.async_get(of.clone()).await {
                Some(m) => Response::Meta(m.clone()),
                None => Response::DataNotFound,
            }
        }
    }

    pub async fn register_device(&self, uid: UserId) -> Result<Response, Error> {
        try {
            match self.meta.async_get_mut(uid).await {
                Some(mut m) => {
                    let id = m.num_devices;
                    m.num_devices += 1;
                    Response::DeviceRegistered(id.try_into()?)
                }
                None => Response::DataNotFound,
            }
        }
    }

    pub async fn update_blob(&self, uid: UserId, blob: Bytes) -> Result<(), Error> {
        if let Some(mut u) = self.meta.async_get_mut(uid.clone()).await {
            u.blob = blob;
        } else {
            eprintln!("user tried to set blob but found no metadata");
            eprintln!("this should never happen");
            eprintln!("uid was {}", uid);
        }
        Ok(())
    }

    pub async fn login(&self, gid: &GlobalId, writer: Sock) -> Result<(), Error> {
        let device_id: usize = (gid.did + 1) as usize;
        if let Some(mut u) = self.meta.async_get_mut(gid.uid.clone()).await {
            let devs = std::cmp::max(u.num_devices, device_id);
            u.num_devices = devs;
        } else {
            self.meta.insert(
                gid.uid.clone(),
                User {
                    num_devices: device_id,
                    blob: Bytes::new(),
                },
            );
        }
        self.open.insert(gid.clone(), writer);
        if let Some((_, p)) = self.pending.remove(&gid) {
            if let Some(mut w) = self.open.async_get_mut(gid.clone()).await {
                while !p.is_empty() {
                    let msg = p.pop()?;
                    send_datagram(w.deref_mut(), &msg).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn handle_msg(&self, gid: &GlobalId, msg: MessageToServer) -> Result<(), Error> {
        let reply = match &msg {
            SendMsg { to, body } => {
                self.send_msg(gid.clone(), to.clone(), body.clone()).await?;
                None
            }
            RequestMeta { of } => Some(self.request_meta(of).await?),
            RegisterDevice => Some(self.register_device(gid.uid.clone()).await?),
            UpdateBlob { blob } => {
                self.update_blob(gid.uid.clone(), blob.clone()).await?;
                None
            }
        };
        if let Some(res) = reply {
            let wrapped = QueryResponse {
                res,
                query: msg.clone(),
            };
            if let Some(mut w) = self.open.async_get_mut(gid.clone()).await {
                send_datagram(w.deref_mut(), &wrapped).await?;
            }
        }
        Ok(())
    }
}

const PORT: u16 = 8000;

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
                state.login(&gid, writer).await?;
                loop {
                    let d = read_datagram(&mut reader).await;
                    if let Err(e) = d {
                        dbg!("invalid msg", addr, e);
                        break;
                    };
                    state.handle_msg(&gid, d.unwrap()).await?;
                }
                dbg!("closing connection", &gid);
                state.open.remove(&gid);
                let open: Vec<GlobalId> = state.open.iter().map(|p| p.key().clone()).collect();
                dbg!("current open connections are", &open);
            };
            if let Err(e) = comp {
                eprintln!("session with {} failed, error was: {:?}", addr, e);
            }
        });
    }
}
