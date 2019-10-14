use crossbeam_channel::*;
use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use herald_common::UserId;
use heraldcore::contact;
use lazy_static::*;

lazy_static! {
    /// Concurrent hashmap from `UserId` to `Contact`. Used to avoid data replication.
    pub(super) static ref USER_DATA: DashMap<UserId, contact::Contact> = DashMap::default();
}

pub fn user_in_cache(uid: &UserId) -> bool {
    USER_DATA.contains_key(uid)
}

pub fn get_user(uid: &UserId) -> Option<DashMapRef<UserId, contact::Contact>> {
    USER_DATA.get(uid)
}

pub fn get_user_mut(uid: &UserId) -> Option<DashMapRefMut<UserId, contact::Contact>> {
    USER_DATA.get_mut(uid)
}

use crate::interface::UsersEmitter as Emitter;
use parking_lot::Mutex;

/// User list updates
pub enum UsersUpdates {
    /// A new user has been added
    NewUser(UserId),
    /// A contact request has been responded to
    ReqResp(UserId, bool),
}

/// Channel for global user list updates
pub struct UserChannel {
    pub(crate) rx: Receiver<UsersUpdates>,
    pub(crate) tx: Sender<UsersUpdates>,
}

impl UserChannel {
    /// Creates new `UserChannel`
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

lazy_static! {
    /// Statically initialized instance of `UsersUpdates` used to pass notifications
    /// from the network.
    pub static ref USER_CHANNEL: UserChannel = UserChannel::new();

    /// Users list emitter, filled in when the users list is constructed
    pub static ref USER_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);
}

/// Emits a signal to the QML runtime, returns `None` on failure.
#[must_use]
pub fn users_emit_data_ready() -> Option<()> {
    let mut lock = USER_EMITTER.lock();
    let emitter = lock.as_mut()?;

    emitter.new_data_ready();
    Some(())
}
