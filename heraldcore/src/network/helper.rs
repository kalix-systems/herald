use super::server_url;
use crate::errors::*;
use herald_common::*;

macro_rules! mk_request {
    ($method: tt, $path: tt) => {
        pub fn $path(req: &$path::Req) -> Result<$path::Res, HErr> {
            let res_reader = ureq::$method(&server_url(stringify!($path)))
                .send_bytes(&serde_cbor::to_vec(req)?)
                .into_reader();
            let res = serde_cbor::from_reader(res_reader)?;
            Ok(res)
        }
    };
}

mk_request!(get, keys_of);
mk_request!(get, key_info);
mk_request!(get, keys_exist);
mk_request!(get, users_exist);
mk_request!(post, register);
mk_request!(post, new_key);
mk_request!(post, dep_key);
mk_request!(post, push_users);
mk_request!(post, push_devices);
