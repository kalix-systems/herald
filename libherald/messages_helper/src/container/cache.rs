use super::*;
use lru::LruCache;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

static CACHE: OnceCell<Mutex<LruCache<MsgId, MsgData>>> = OnceCell::new();

pub(super) fn cache() -> &'static Mutex<LruCache<MsgId, MsgData>> {
    CACHE.get_or_init(|| Mutex::new(LruCache::new(1024)))
}

pub fn get(mid: &MsgId) -> Option<MsgData> {
    let maybe = cache().lock().get(mid).cloned();

    match maybe {
        data @ Some(_) => data,
        None => db_data(mid),
    }
}

pub(super) fn insert(
    mid: MsgId,
    data: MsgData,
) {
    cache().lock().put(mid, data);
}

pub fn access<T, F: FnOnce(&MsgData) -> T>(
    mid: &MsgId,
    f: F,
) -> Option<T> {
    if let Some(data) = cache().lock().get(mid).as_ref() {
        return Some(f(data));
    }

    let data = db_data(mid)?;

    Some(f(&data))
}

pub fn update<T, F: FnOnce(&mut MsgData) -> T>(
    mid: &MsgId,
    f: F,
) -> Option<T> {
    Some(f(cache().lock().get_mut(mid)?))
}

pub(super) fn remove(mid: &MsgId) -> Option<MsgData> {
    cache().lock().pop(mid)
}

fn db_data(mid: &MsgId) -> Option<MsgData> {
    let data = heraldcore::message::message_data(mid).ok()?;

    insert(*mid, data.clone());

    Some(data)
}
