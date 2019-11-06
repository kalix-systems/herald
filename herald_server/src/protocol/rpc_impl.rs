use super::*;

use std::pin::Pin;

async fn keys_of(mut store: Conn, req: keys_of::Req) -> Result<keys_of::Res, Error> {
    use keys_of::*;

    let mut map = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        let meta = store.read_meta(&uid).await?;
        map.push((uid, meta));
    }

    Ok(Res(map))
}

async fn key_info(mut store: Conn, req: key_info::Req) -> Result<key_info::Res, Error> {
    use key_info::*;

    let mut map = Vec::with_capacity(req.0.len());

    for key in req.0 {
        let meta = store.read_key(key).await?;
        map.push((key, meta));
    }

    Ok(Res(map))
}

async fn keys_exist(mut store: Conn, req: keys_exist::Req) -> Result<keys_exist::Res, Error> {
    use keys_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for key in req.0 {
        vec.push(store.device_exists(&key).await?);
    }

    Ok(Res(vec))
}

async fn users_exist(mut store: Conn, req: users_exist::Req) -> Result<users_exist::Res, Error> {
    use users_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        vec.push(store.user_exists(&uid).await?);
    }

    Ok(Res(vec))
}

async fn register(mut store: Conn, req: register::Req) -> Result<register::Res, Error> {
    use register::*;

    match req.1.verify_sig() {
        SigValid::Yes => store.register_user(req.0, req.1).await,
        s => Ok(Res::BadSig(s)),
    }
}

async fn new_key(mut store: Conn, req: new_key::Req) -> Result<new_key::Res, Error> {
    use new_key::*;

    match req.0.verify_sig() {
        SigValid::Yes => store.add_key(req.0).await.map(Res),
        s => Ok(Res(PKIResponse::BadSig(s))),
    }
}

async fn dep_key(mut store: Conn, req: dep_key::Req) -> Result<dep_key::Res, Error> {
    use dep_key::*;

    match req.0.verify_sig() {
        SigValid::Yes => store.deprecate_key(req.0).await.map(Res),
        s => Ok(Res(PKIResponse::BadSig(s))),
    }
}

type Fut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
use RequestError::*;

macro_rules! serve_req {
    (store, $lifetime: tt, $tname:ident, $mname:ident) => {
        type $tname = Fut<$lifetime, Result<$mname::Res, RequestError>>;
        fn $mname(
            self,
            _cx: tarpc_lib::context::Context,
            req: $mname::Req,
        ) -> <Self as HeraldService>::$tname {
            Box::pin(async move {
                let con = self.new_connection().await.map_err(|_| UnknownError)?;
                $mname(con, req).await.map_err(|e| match e {
                    Error::PgError(p) => p
                        .code()
                        .map_or_else(|| UnknownError, |s| DbError(s.code().into())),
                    _ => UnknownError,
                })
            })
        }
    };
    (self, $lifetime: tt, $tname:ident, $mname:ident) => {
        type $tname = Fut<$lifetime, Result<$mname::Res, RequestError>>;
        fn $mname(
            self,
            _cx: tarpc_lib::context::Context,
            req: $mname::Req,
        ) -> <Self as HeraldService>::$tname {
            Box::pin(async move {
                self.$mname(req).await.map_err(|e| match e {
                    Error::PgError(p) => p
                        .code()
                        .map_or_else(|| UnknownError, |s| DbError(s.code().into())),
                    _ => UnknownError,
                })
            })
        }
    };
}

impl<'a> HeraldService for &'a State {
    serve_req!(store, 'a, KeysOfFut, keys_of);
    serve_req!(store, 'a, KeyInfoFut, key_info);
    serve_req!(store, 'a, KeysExistFut, keys_exist);
    serve_req!(store, 'a, UsersExistFut, users_exist);
    serve_req!(store, 'a, NewKeyFut, new_key);
    serve_req!(store, 'a, DepKeyFut, dep_key);
    serve_req!(store, 'a, RegisterFut, register);
    // serve_req!(store, 'a, AddPrekeysFut, add_prekeys);
    // serve_req!(store, 'a, GetPrekeysFut, get_prekeys);

    serve_req!(self, 'a, PushUsersFut, push_users);
    serve_req!(self, 'a, PushDevicesFut, push_devices);
}
