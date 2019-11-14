use crate::errors::*;
use herald_common::*;

macro_rules! mk_request {
    ($path: tt) => {
        pub async fn $path(req: $path::Req) -> Result<$path::Res, HErr> {
            let mut client = super::get_client().await?;
            let res = client.$path(tarpc::context::current(), req).await??;
            Ok(res)
        }
    };
}

mk_request!(keys_of);
mk_request!(key_info);
mk_request!(keys_exist);
mk_request!(users_exist);
mk_request!(register);
mk_request!(new_key);
mk_request!(dep_key);
mk_request!(push_users);
mk_request!(push_devices);
