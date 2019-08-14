use crate::store::*;
use crate::user::*;
use ccl::dashmap::DashMap;
use crossbeam_queue::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::io::*;

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
    Connect(GlobalId),
    RegisterKey(Signed<RawKey>),
    DeprecateKey(Signed<RawKey>),
    SendMessage { to: UserId, msg: Signed<RawMsg> },
}

pub struct AppState<Sock: AsyncWrite + AsyncRead> {
    open: DashMap<GlobalId, Sock>,
    pending: DashMap<GlobalId, SegQueue<MessageToDevice>>,
    meta: Store,
}
