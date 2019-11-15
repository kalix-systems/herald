use super::*;
use sodiumoxide::crypto::sign;

pub async fn login<T>(
    active: &DashMap<sign::PublicKey, T>,
    store: &mut Conn,
    wtx: &mut WTx,
    rrx: &mut Receiver<Vec<u8>>,
) -> Result<GlobalId, Error> {
    use herald_common::login::*;

    let bytes = UQ::new();

    let g: GlobalId = read_msg::<SignAs>(rrx).await?.0;

    if active.contains_key(&g.did) {
        write_msg(&SignAsResponse::SessionExists, wtx, rrx).await?;
        return Err(LoginFailed);
    } else if !store.key_is_valid(g.did).await? {
        write_msg(&SignAsResponse::KeyDeprecated, wtx, rrx).await?;
        return Err(LoginFailed);
    } else if !store.user_exists(&g.uid).await? {
        write_msg(&SignAsResponse::MissingUID, wtx, rrx).await?;
        return Err(LoginFailed);
    } else {
        let res = SignAsResponse::Sign(bytes);
        write_msg(&res, wtx, rrx).await?;
    };

    let s: sign::Signature = read_msg::<LoginToken>(rrx).await?.0;

    if !sign::verify_detached(&s, bytes.as_ref(), &g.did) {
        write_msg(&LoginTokenResponse::BadSig, wtx, rrx).await?;
        Err(LoginFailed)
    } else {
        write_msg(&LoginTokenResponse::Success, wtx, rrx).await?;
        Ok(g)
    }
}
