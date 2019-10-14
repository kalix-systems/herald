use super::*;
use sodiumoxide::crypto::sign;
use warp::filters::ws;

pub async fn login<T>(
    active: &DashMap<sign::PublicKey, T>,
    store: &mut Conn,
    ws: &mut ws::WebSocket,
) -> Result<GlobalId, Error> {
    use herald_common::login::*;

    let bytes = UQ::new();

    let g = read_msg::<SignAs>(ws).await?.0;

    if active.contains_key(&g.did) {
        write_msg(&SignAsResponse::SessionExists, ws).await?;
        return Err(LoginFailed);
    } else if !store.key_is_valid(g.did)? {
        write_msg(&SignAsResponse::KeyDeprecated, ws).await?;
        return Err(LoginFailed);
    } else if !store.user_exists(&g.uid)? {
        write_msg(&SignAsResponse::MissingUID, ws).await?;
        return Err(LoginFailed);
    } else {
        let res = SignAsResponse::Sign(bytes);
        write_msg(&res, ws).await?;
    };

    let s = read_msg::<LoginToken>(ws).await?.0;

    if !sign::verify_detached(&s, bytes.as_ref(), &g.did) {
        write_msg(&LoginTokenResponse::BadSig, ws).await?;
        Err(LoginFailed)
    } else {
        write_msg(&LoginTokenResponse::Success, ws).await?;
        Ok(g)
    }
}
