use crate::{interface::UsersEmitter as Emitter, shared::SingletonBus};
use crossbeam_channel::*;
use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use herald_common::UserId;
use heraldcore::contact;
use heraldcore::{channel_send_err, NE};
use lazy_static::*;
use parking_lot::Mutex;

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

/// User list updates
pub enum UsersUpdates {
    /// A new user has been added
    NewUser(UserId),
    /// A contact request has been responded to
    ReqResp(UserId, bool),
}

/// Channel for global user list updates
pub(crate) struct UserBus {
    pub(super) rx: Receiver<UsersUpdates>,
    pub(super) tx: Sender<UsersUpdates>,
}

impl UserBus {
    /// Creates new `UserBus`
    fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

lazy_static! {
    /// Statically initialized instance of `UsersUpdates` used to pass notifications
    /// from the network.
    pub(super) static ref USER_BUS: UserBus = UserBus::new();

    /// Users list emitter, filled in when the users list is constructed
    pub(super) static ref USER_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);
}

impl SingletonBus for super::Users {
    type Update = UsersUpdates;

    fn push(update: Self::Update) -> Result<(), heraldcore::errors::HErr> {
        USER_BUS
            .tx
            .clone()
            .send(update)
            .map_err(|_| channel_send_err!())?;
        users_emit_new_data().ok_or(NE!())?;
        Ok(())
    }
}

/// Emits a signal to the QML runtime, returns `None` on failure.
#[must_use]
fn users_emit_new_data() -> Option<()> {
    let mut lock = USER_EMITTER.lock();
    let emitter = lock.as_mut()?;

    emitter.new_data_ready();
    Some(())
}
