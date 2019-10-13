use super::*;

pub async fn login(store: &mut Conn, ws: &mut WebSocket) -> Result<GlobalId, Error> {
    use herald_common::login::*;

    let bytes = UQ::new();

    let g = read_msg::<SignAs>(ws).await?.0;

    let res = if !store.key_is_valid(g.did)? {
        SignAsResponse::KeyDeprecated
    } else if !store.user_exists(&g.uid)? {
        SignAsResponse::MissingUID
    } else {
        SignAsResponse::Sign(bytes)
    };
    write_msg(&res, ws).await?;

    let s = read_msg::<LoginToken>(ws).await?.0;

    let res = if sign::verify_detached(&s, bytes.as_ref(), &g.did) {
        LoginTokenResponse::Success
    } else {
        LoginTokenResponse::BadSig
    };
    write_msg(&res, ws).await?;

    match res {
        LoginTokenResponse::Success => Ok(g),
        LoginTokenResponse::BadSig => Err(LoginFailed),
    }
}
