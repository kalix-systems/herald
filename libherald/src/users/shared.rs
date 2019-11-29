use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use herald_common::UserId;
use heraldcore::user;
use lazy_static::*;

lazy_static! {
    /// Concurrent hashmap from `UserId` to `User`. Used to avoid data replication.
    pub(super) static ref USER_DATA: DashMap<UserId, user::User> = DashMap::default();
}

pub fn user_in_cache(uid: &UserId) -> bool {
    USER_DATA.contains_key(uid)
}

pub fn get_user(uid: &UserId) -> Option<DashMapRef<UserId, user::User>> {
    USER_DATA.get(uid)
}

pub fn get_user_mut(uid: &UserId) -> Option<DashMapRefMut<UserId, user::User>> {
    USER_DATA.get_mut(uid)
}

pub(crate) fn color(uid: &UserId) -> Option<u32> {
    Some(get_user(&uid)?.color)
}

pub(crate) fn name(uid: &UserId) -> Option<String> {
    let inner = get_user(uid)?;

    Some(inner.name.clone())
}

pub(crate) fn profile_picture(uid: &UserId) -> Option<String> {
    let inner = get_user(uid)?;

    inner.profile_picture.clone()
}

#[inline]
pub fn user_ids() -> Vec<UserId> {
    let mut list: Vec<(String, UserId)> = USER_DATA
        .iter()
        .map(|kv| (kv.value().name.clone(), *kv.key()))
        .collect();

    list.sort_unstable_by(|a, b| a.cmp(b));

    list.into_iter().map(|(_, u)| u).collect()
}

/// User list update
pub enum UserUpdate {
    /// A new user has been added
    NewUser(heraldcore::user::User),
    /// A user request has been responded to
    ReqResp(UserId, bool),
}

impl From<UserUpdate> for crate::Update {
    fn from(update: UserUpdate) -> crate::Update {
        crate::Update::User(update)
    }
}
