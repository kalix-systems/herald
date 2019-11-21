#[macro_use]
mod macros;

pub mod http;

use bytes::Buf;
use herald_common::*;
use server_errors::*;
use server_protocol::*;
use server_store::*;
use std::future::Future;

async fn req_handler_store<B, I, O, F, Fut>(
    state: &State,
    req: B,
    f: F,
) -> Result<Vec<u8>, Error>
where
    B: Buf,
    I: for<'a> Deserialize<'a>,
    O: Serialize,
    F: FnOnce(Conn, I) -> Fut,
    Fut: Future<Output = Result<O, Error>>,
{
    let con: Conn = state.new_connection().await?;
    let buf: Vec<u8> = req.collect();
    let req: I = serde_cbor::from_slice(&buf)?;
    let res: O = f(con, req).await?;
    let res_ser: Vec<u8> = serde_cbor::to_vec(&res)?;
    Ok(res_ser)
}

async fn req_handler_async<'a, B, I, O, F, Fut>(
    state: &'a State,
    req: B,
    f: F,
) -> Result<Vec<u8>, Error>
where
    B: Buf,
    I: for<'b> Deserialize<'b>,
    O: Serialize,
    F: FnOnce(&'a State, I) -> Fut,
    Fut: Future<Output = Result<O, Error>>,
{
    let buf: Vec<u8> = req.collect();
    let req: I = serde_cbor::from_slice(&buf)?;
    let res: O = f(state, req).await?;
    let res_ser: Vec<u8> = serde_cbor::to_vec(&res)?;
    Ok(res_ser)
}
