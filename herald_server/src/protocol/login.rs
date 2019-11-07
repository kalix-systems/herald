use super::*;
use sodiumoxide::crypto::sign;

pub async fn login<S, T>(
    active: &DashMap<sign::PublicKey, T>,
    store: &mut Conn,
    stream: &mut Framed<S>,
) -> Result<GlobalId, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    use herald_common::login::*;

    let bytes = UQ::new();

    let g: GlobalId = stream.read::<SignAs>().await?.0;

    if active.contains_key(&g.did) {
        stream
            .write_packeted(&SignAsResponse::SessionExists)
            .await?;
        return Err(LoginFailed);
    } else if !store.user_exists(&g.uid).await? {
        stream.write_packeted(&SignAsResponse::MissingUID).await?;
        return Err(LoginFailed);
    } else if !store.key_is_valid(g.did).await? {
        stream
            .write_packeted(&SignAsResponse::KeyDeprecated)
            .await?;
        return Err(LoginFailed);
    } else {
        stream.write_packeted(&SignAsResponse::Sign(bytes)).await?;
    };

    let s: sign::Signature = stream.read::<LoginToken>().await?.0;

    if !sign::verify_detached(&s, bytes.as_ref(), &g.did) {
        stream.write_packeted(&LoginTokenResponse::BadSig).await?;
        Err(LoginFailed)
    } else {
        stream.write_packeted(&LoginTokenResponse::Success).await?;
        Ok(g)
    }
}
