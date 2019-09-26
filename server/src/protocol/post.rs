use super::*;
use crate::{prelude::*, store::*};
use dashmap::DashMap;
use futures::compat::*;
use futures::stream::{Stream, StreamExt};
use sodiumoxide::crypto::sign;
use std::collections::VecDeque;
use tokio::sync::mpsc::{
    unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use warp::filters::ws;

pub async fn register<S: Store>(_: &mut S, _: register::Req) -> Result<register::Res, Error> {
    unimplemented!()
}

pub async fn new_key<S: Store>(store: &mut S, req: new_key::Req) -> Result<new_key::Res, Error> {
    use new_key::*;

    let res = if req.0.verify_sig() {
        drop(store);
        // store.add_key(req.0)?
        unimplemented!()
    } else {
        PKIResponse::BadSignature
    };

    Ok(Res(res))
}

pub async fn dep_key<S: Store>(store: &mut S, req: dep_key::Req) -> Result<dep_key::Res, Error> {
    use dep_key::*;

    let res = if req.0.verify_sig() {
        drop(store);
        // store.deprecate_key(req.0)?
        unimplemented!()
    } else {
        PKIResponse::BadSignature
    };

    Ok(Res(res))
}
