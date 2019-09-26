use super::*;
use crate::{prelude::*, store::*};
use dashmap::DashMap;
use futures::{
    compat::*,
    stream::{Stream, StreamExt},
};
use sodiumoxide::crypto::sign;
use std::collections::VecDeque;
use tokio::sync::mpsc::{
    unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use warp::{filters::ws, Future as WFut, Stream as WStream};

pub async fn login<W, E>(store: &mut Conn, ws: &mut W) -> Result<GlobalId, Error>
where
    W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
    Error: From<E>,
{
    use herald_common::login::*;

    let bytes = UQ::new();

    let m = ws.next().await.ok_or(LoginFailed)??;
    let g = serde_cbor::from_slice::<SignAs>(m.as_bytes())?.0;

    let res = if !store.key_is_valid(g.did)? {
        SignAsResponse::KeyDeprecated
    } else if !store.user_exists(&g.uid)? {
        SignAsResponse::MissingUID
    } else {
        SignAsResponse::Sign(bytes)
    };
    ws.send(ws::Message::binary(serde_cbor::to_vec(&res)?))
        .await?;

    let m = ws.next().await.ok_or(LoginFailed)??;
    let s = serde_cbor::from_slice::<LoginToken>(m.as_bytes())?.0;

    let res = if sign::verify_detached(&s, bytes.as_ref(), &g.did) {
        LoginTokenResponse::Success
    } else {
        LoginTokenResponse::BadSig
    };
    ws.send(ws::Message::binary(serde_cbor::to_vec(&res)?))
        .await?;

    match res {
        LoginTokenResponse::Success => Ok(g),
        LoginTokenResponse::BadSig => Err(LoginFailed),
    }
}
