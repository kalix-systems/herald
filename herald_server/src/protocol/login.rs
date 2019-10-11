use super::*;
use sodiumoxide::crypto::sign;
use warp::filters::ws;

pub async fn login<T, W, E>(
    active: &DashMap<sign::PublicKey, T>,
    store: &mut Conn,
    ws: &mut W,
) -> Result<GlobalId, Error>
where
    W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
    Error: From<E>,
{
    use herald_common::login::*;

    let bytes = UQ::new();

    let g = read_msg::<SignAs, _, _>(ws).await?.0;

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

    let s = read_msg::<LoginToken, _, _>(ws).await?.0;

    if !sign::verify_detached(&s, bytes.as_ref(), &g.did) {
        write_msg(&LoginTokenResponse::BadSig, ws).await?;
        Err(LoginFailed)
    } else {
        write_msg(&LoginTokenResponse::Success, ws).await?;
        Ok(g)
    }
}

async fn read_msg<T, W, E>(ws: &mut W) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    W: Stream<Item = Result<ws::Message, warp::Error>> + Unpin,
    Error: From<E>,
{
    let m = ws.next().await.ok_or(LoginFailed)??;
    let t = serde_cbor::from_slice::<T>(m.as_bytes())?;
    Ok(t)
}

async fn write_msg<T, W, E>(t: &T, ws: &mut W) -> Result<(), Error>
where
    T: Serialize,
    W: Sink<ws::Message, Error = E> + Unpin,
    Error: From<E>,
{
    let bvec = serde_cbor::to_vec(t)?;
    let msg = ws::Message::binary(bvec);
    ws.send(msg).await?;
    Ok(())
}
