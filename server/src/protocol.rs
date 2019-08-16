use crate::store::*;
use crate::user::*;
use ccl::dashmap::DashMap;
use crossbeam_queue::*;
use failure::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::prelude::*;

#[derive(Serialize, Deserialize, Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlobalId {
    uid: UserId,
    did: DeviceId,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToDevice {
    NewKey {
        index: GlobalId,
        sig: Signed<RawKey>,
    },
    DeprecateKey {
        from: UserId,
        sig: Signed<DeviceId>,
    },
    Message {
        from: UserId,
        msg: Signed<RawMsg>,
    },
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    RegisterKey(Signed<RawKey>),
    DeprecateKey(Signed<RawKey>),
    SendMessage { to: UserId, msg: Signed<RawMsg> },
}

pub struct AppState<Sock: AsyncWrite + AsyncRead> {
    open: DashMap<GlobalId, Sock>,
    pending: DashMap<GlobalId, SegQueue<MessageToDevice>>,
    meta: Store,
}

impl<S: AsyncWrite + AsyncRead + Unpin> AppState<S> {
    pub async fn handle_incoming(&self, mut incoming: S) -> Result<(), Error> {
        let mut buf = [0u8; 8];
        incoming.read_exact(&mut buf).await?;
        let len = u64::from_le_bytes(buf) as usize;

        Ok(())
    }
}
