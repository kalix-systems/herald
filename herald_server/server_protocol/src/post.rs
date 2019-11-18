use super::*;

pub async fn register(mut store: Conn, req: register::Req) -> Result<register::Res, Error> {
    use register::*;

    match req.1.verify_sig() {
        SigValid::Yes => store.register_user(req.0, req.1).await,
        s => Ok(Res::BadSig(s)),
    }
}

pub async fn new_key(mut store: Conn, req: new_key::Req) -> Result<new_key::Res, Error> {
    use new_key::*;

    match req.0.verify_sig() {
        SigValid::Yes => store.add_key(req.0).await.map(Res),
        s => Ok(Res(PKIResponse::BadSig(s))),
    }
}

pub async fn dep_key(mut store: Conn, req: dep_key::Req) -> Result<dep_key::Res, Error> {
    use dep_key::*;

    match req.0.verify_sig() {
        SigValid::Yes => store.deprecate_key(req.0).await.map(Res),
        s => Ok(Res(PKIResponse::BadSig(s))),
    }
}
