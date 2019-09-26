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

use register::*;
pub async fn register<S: Store>(_: &mut S, _: Req) -> Result<Res, Error> {
    unimplemented!()
}
