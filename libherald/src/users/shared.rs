use dashmap::{DashMap, DashMapRef, DashMapRefMut};
use herald_common::UserId;
use heraldcore::user;
use once_cell::sync::OnceCell;

/// Concurrent hashmap from `UserId` to `User`. Used to avoid data replication.
static USER_DATA: OnceCell<DashMap<UserId, user::User>> = OnceCell::new();

pub(super) fn user_data() -> &'static DashMap<UserId, user::User> {
    USER_DATA.get_or_init(|| DashMap::default())
}

pub fn user_in_cache(uid: &UserId) -> bool {
    user_data().contains_key(uid)
}

pub fn get_user(uid: &UserId) -> Option<DashMapRef<UserId, user::User>> {
    user_data().get(uid)
}

pub fn get_user_mut(uid: &UserId) -> Option<DashMapRefMut<UserId, user::User>> {
    user_data().get_mut(uid)
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
    let mut list: Vec<(String, UserId)> = user_data()
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
