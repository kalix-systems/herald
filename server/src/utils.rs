use crate::prelude::*;
use crate::store::Conn;
use bytes::Buf;
use serde::*;

pub(crate) fn req_handler<B, I, O, F>(con: &mut Conn, req: B, f: F) -> Result<Vec<u8>, Error>
where
    B: Buf,
    I: for<'a> Deserialize<'a>,
    O: Serialize,
    F: FnOnce(&mut Conn, I) -> Result<O, Error>,
{
    let buf: Vec<u8> = req.collect();
    let req = serde_cbor::from_slice(&buf)?;
    let res = f(con, req)?;
    let res_ser = serde_cbor::to_vec(&res)?;
    Ok(res_ser)
}
