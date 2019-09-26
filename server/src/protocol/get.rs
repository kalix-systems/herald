use super::*;
use crate::{prelude::*, store::*};
use bytes::Buf;
use dashmap::DashMap;
use futures::compat::*;
use futures::stream::{Stream, StreamExt};
use sodiumoxide::crypto::sign;
use std::collections::HashMap;
use tokio::sync::mpsc::{
    unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use warp::filters::ws;

pub fn keys_of<S: Store>(store: &mut S, req: keys_of::Req) -> Result<keys_of::Res, Error> {
    use keys_of::*;

    let mut map = HashMap::with_capacity(req.0.len());

    for uid in req.0 {
        let meta = store.read_meta(&uid)?;
        map.insert(uid, meta);
    }

    Ok(Res(map))
}

pub fn key_info<S: Store>(store: &mut S, req: key_info::Req) -> Result<key_info::Res, Error> {
    use key_info::*;

    let mut map = HashMap::with_capacity(req.0.len());

    for key in req.0 {
        let meta = store.read_key(key)?;
        map.insert(key, meta);
    }

    Ok(Res(map))
}

pub fn keys_exist<S: Store>(store: &mut S, req: keys_exist::Req) -> Result<keys_exist::Res, Error> {
    use keys_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for key in req.0 {
        vec.push(store.device_exists(&key)?);
    }

    Ok(Res(vec))
}

pub fn users_exist<S: Store>(
    store: &mut S,
    req: users_exist::Req,
) -> Result<users_exist::Res, Error> {
    use users_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        vec.push(store.user_exists(&uid)?);
    }

    Ok(Res(vec))
}

macro_rules! get_filter {
    ($this: ident, $f: ident) => {
        warp::path(stringify!($f))
            .and(body::concat())
            .map(move |b| {
                $this
                    .get_req_handler(b, $f)
                    .map_err(|e| warp::reject::custom(format!("{:?}", e)))
            })
    };
}

impl State {
    pub(crate) fn get_req_handler<B, I, O, F>(&self, req: B, f: F) -> Result<Vec<u8>, Error>
    where
        B: Buf,
        I: for<'a> Deserialize<'a>,
        O: Serialize,
        F: FnOnce(&mut Conn, I) -> Result<O, Error>,
    {
        let mut con = self.new_connection()?;
        let buf: Vec<u8> = req.collect();
        let req = serde_cbor::from_slice(&buf)?;
        let res = f(&mut con, req)?;
        let res_ser = serde_cbor::to_vec(&res)?;
        Ok(res_ser)
    }

    pub fn handle_get(&'static self) -> impl Filter {
        get_filter!(self, keys_of)
            .or(get_filter!(self, key_info))
            .or(get_filter!(self, keys_exist))
            .or(get_filter!(self, users_exist))
    }
}
