use super::{server_url, SocketAddr};
use crate::errors::*;
use coremacros::w;
use herald_common::*;

macro_rules! mk_request {
    ($method: tt, $path: tt) => {
        pub fn $path(req: &$path::Req) -> Result<$path::Res, HErr> {
            use ::coremacros::w;
            use ::std::io::Read;
            let mut res_buf = Vec::new();
            w!(ureq::$method(&server_url(stringify!($path)))
                .send_bytes(&kson::to_vec(req))
                .into_reader()
                .read_to_end(&mut res_buf));
            let res = w!(kson::from_bytes(res_buf.into()));
            Ok(res)
        }
    };
}

mk_request!(get, get_sigchain);
mk_request!(get, recip_exists);
mk_request!(get, new_sig);
mk_request!(get, new_prekeys);
mk_request!(get, get_prekeys);
mk_request!(get, push);
// mk_request!(get, register);

// pub fn register(
//     req: &register::Req,
//     home_server: SocketAddr,
// ) -> Result<register::Res, HErr> {
//     use std::io::Read;

//     let mut res_buf = Vec::new();
//     let url = format!("http://{}/register", home_server);

//     w!(ureq::post(&url)
//         .send_bytes(&kson::to_vec(req))
//         .into_reader()
//         .read_to_end(&mut res_buf));
//     let res = w!(kson::from_bytes(res_buf.into()));
//     Ok(res)
// }
