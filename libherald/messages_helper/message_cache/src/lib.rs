use heraldcore::message::MsgData;
use heraldcore::types::MsgId;
use lru::LruCache;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

const CAPACITY: usize = 1024;
const NUM_SHARDS: usize = 32;
const CACHE_SIZE: usize = CAPACITY / NUM_SHARDS;

struct Shard(Mutex<LruCache<MsgId, MsgData>>);

impl Default for Shard {
    fn default() -> Self {
        Self(Mutex::new(LruCache::new(CACHE_SIZE)))
    }
}

pub struct Cache {
    inner: [Shard; NUM_SHARDS],
}

impl Cache {
    fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    fn bucket(
        &self,
        key: &MsgId,
    ) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.as_slice());

        (hasher.finish() % NUM_SHARDS as u64) as usize
    }
}

static CACHE: OnceCell<Cache> = OnceCell::new();

pub fn cache() -> &'static Cache {
    CACHE.get_or_init(Cache::new)
}

pub fn get(mid: &MsgId) -> Option<MsgData> {
    let cache = cache();
    let ix = cache.bucket(mid);
    let maybe = cache.inner[ix].0.lock().get(mid).cloned();

    match maybe {
        data @ Some(_) => data,
        None => db_data(mid),
    }
}

pub fn insert(
    mid: MsgId,
    data: MsgData,
) {
    let cache = cache();
    let ix = cache.bucket(&mid);
    cache.inner[ix].0.lock().put(mid, data);
}

pub fn access<T, F: FnOnce(&MsgData) -> T>(
    mid: &MsgId,
    f: F,
) -> Option<T> {
    let cache = cache();
    let ix = cache.bucket(&mid);

    if let Some(data) = cache.inner[ix].0.lock().get(mid).as_ref() {
        return Some(f(data));
    }

    let data = db_data(mid)?;

    f(&data).into()
}

pub fn update<T, F: FnOnce(&mut MsgData) -> T>(
    mid: &MsgId,
    f: F,
) -> Option<T> {
    let cache = cache();
    let ix = cache.bucket(&mid);

    f(cache.inner[ix].0.lock().get_mut(mid)?).into()
}

pub fn remove(mid: &MsgId) -> Option<MsgData> {
    let cache = cache();
    let ix = cache.bucket(&mid);

    cache.inner[ix].0.lock().pop(mid)
}

fn db_data(mid: &MsgId) -> Option<MsgData> {
    let data = heraldcore::message::message_data(mid).ok().flatten()?;

    if data
        .time
        .expiration
        .map(|exp| exp.as_i64() < herald_common::Time::now().as_i64())
        .unwrap_or(false)
    {
        return None;
    }

    insert(*mid, data.clone());

    Some(data)
}
