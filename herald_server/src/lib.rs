pub mod http;

use anyhow::*;
use bytes::Buf;
use herald_common::*;
use server_protocol::*;
use server_store::*;
use std::future::Future;

async fn req_handler_async<'a, I, O, F, Fut>(
    state: &'a State,
    buf: Bytes,
    f: F,
) -> Result<Vec<u8>, Error>
where
    I: De,
    O: Ser,
    F: FnOnce(&'a State, I) -> Fut,
    Fut: Future<Output = Result<O, Error>>,
{
    let req: I = kson::from_bytes(buf)?;
    let res: O = f(state, req).await?;
    let res_ser: Vec<u8> = kson::to_vec(&res);
    Ok(res_ser)
}
