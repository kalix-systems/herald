use herald_common::UserId;
use heraldcore::types::ConversationId;
use heraldcore::user;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use search_pattern::SearchPattern;
use std::collections::HashMap;

/// Concurrent hashmap from `UserId` to `User`. Used to avoid data replication.
static USER_DATA: OnceCell<RwLock<HashMap<UserId, user::User>>> = OnceCell::new();

pub(super) fn user_data() -> &'static RwLock<HashMap<UserId, user::User>> {
    USER_DATA.get_or_init(|| RwLock::new(HashMap::default()))
}

pub fn user_in_cache(uid: &UserId) -> bool {
    user_data().read().contains_key(uid)
}

pub(crate) fn matches(
    uid: &UserId,
    pattern: &SearchPattern,
) -> bool {
    let lock = crate::users::shared::user_data().read();
    match lock.get(&uid) {
        Some(u) => u.matches(pattern),
        None => false,
    }
}

pub(crate) fn color(uid: &UserId) -> Option<u32> {
    Some(user_data().read().get(uid)?.color)
}

pub(crate) fn status(uid: &UserId) -> Option<user::UserStatus> {
    Some(user_data().read().get(uid)?.status)
}

pub(crate) fn name(uid: &UserId) -> Option<String> {
    Some(user_data().read().get(uid)?.name.clone())
}

pub(crate) fn pairwise_cid(uid: &UserId) -> Option<ConversationId> {
    Some(user_data().read().get(uid)?.pairwise_conversation)
}

pub(crate) fn profile_picture(uid: &UserId) -> Option<String> {
    Some(
        user_data()
            .read()
            .get(uid)?
            .profile_picture
            .as_ref()?
            .clone(),
    )
}

#[inline]
pub fn user_ids() -> Vec<UserId> {
    let mut list: Vec<(String, UserId)> = user_data()
        .read()
        .iter()
        .map(|(key, value)| (value.name.clone(), *key))
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
    /// Data has changed for this user
    DataChanged(UserId),
}

impl From<UserUpdate> for crate::Update {
    fn from(update: UserUpdate) -> crate::Update {
        crate::Update::User(update)
    }
}
