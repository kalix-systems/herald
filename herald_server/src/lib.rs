pub mod http;

use bytes::Buf;
use herald_common::*;
use server_errors::*;
use server_protocol::*;
use server_store::*;
use std::future::Future;

async fn req_handler_store<B, I, O, F, Fut>(state: &State, req: B, f: F) -> Result<Vec<u8>, Error>
where
    B: Buf,
    I: De,
    O: Ser,
    F: FnOnce(Conn, I) -> Fut,
    Fut: Future<Output = Result<O, Error>>,
{
    let con: Conn = state.new_connection().await?;
    let buf: Bytes = req.collect();
    let req: I = kson::from_bytes(buf)?;
    let res: O = f(con, req).await?;
    let res_ser: Vec<u8> = kson::to_vec(&res);
    Ok(res_ser)
}

async fn req_handler_async<'a, B, I, O, F, Fut>(
    state: &'a State,
    req: B,
    f: F,
) -> Result<Vec<u8>, Error>
where
    B: Buf,
    I: De,
    O: Ser,
    F: FnOnce(&'a State, I) -> Fut,
    Fut: Future<Output = Result<O, Error>>,
{
    let buf: Bytes = req.collect();
    let req: I = kson::from_bytes(buf)?;
    let res: O = f(state, req).await?;
    let res_ser: Vec<u8> = kson::to_vec(&res);
    Ok(res_ser)
}
