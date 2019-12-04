use super::{server_url, SocketAddr};
use crate::errors::*;
use herald_common::*;

macro_rules! mk_request {
    ($method: tt, $path: tt) => {
        pub fn $path(req: &$path::Req) -> Result<$path::Res, HErr> {
            use ::std::io::Read;
            let mut res_buf = Vec::new();
            ureq::$method(&server_url(stringify!($path)))
                .send_bytes(&kson::to_vec(req))
                .into_reader()
                .read_to_end(&mut res_buf)?;
            let res = kson::from_bytes(res_buf.into())?;
            Ok(res)
        }
    };
}

mk_request!(get, keys_of);
mk_request!(get, key_info);
mk_request!(get, keys_exist);
mk_request!(get, users_exist);
mk_request!(post, new_key);
mk_request!(post, dep_key);
mk_request!(post, push_users);
mk_request!(post, push_devices);
mk_request!(post, push_aux);

pub fn register(
    req: &register::Req,
    home_server: SocketAddr,
) -> Result<register::Res, HErr> {
    use ::std::io::Read;

    let mut res_buf = Vec::new();
    let url = format!("http://{}/register", home_server);

    ureq::post(&url)
        .send_bytes(&kson::to_vec(req))
        .into_reader()
        .read_to_end(&mut res_buf)?;
    let res = kson::from_bytes(res_buf.into())?;
    Ok(res)
}
