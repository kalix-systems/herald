use super::*;

pub async fn login(
    active: &ActiveSessions,
    store: &mut Conn,
    wtx: &mut WTx,
    rrx: &mut Receiver<Vec<u8>>,
) -> Result<GlobalId, Error> {
    use herald_common::login::*;

    let bytes = UQ::gen_new();

    let g: GlobalId = read_msg::<SignAs>(rrx).await?.0;

    if !store.key_is_valid(g.did).await? {
        write_msg(&SignAsResponse::KeyDeprecated, wtx, rrx).await?;
        return Err(LoginFailed);
    } else if !store.user_exists(&g.uid).await? {
        write_msg(&SignAsResponse::MissingUID, wtx, rrx).await?;
        return Err(LoginFailed);
    } else {
        let res = SignAsResponse::Sign(bytes);
        write_msg(&res, wtx, rrx).await?;
    };

    let s: sig::Signature = read_msg::<LoginToken>(rrx).await?.0;

    if !g.did.verify(bytes.as_ref(), s) {
        write_msg(&LoginTokenResponse::BadSig, wtx, rrx).await?;
        Err(LoginFailed)
    } else {
        if let Some((_, sess)) = active.remove(&g.did) {
            sess.interrupt().await;
        }

        write_msg(&LoginTokenResponse::Success, wtx, rrx).await?;
        Ok(g)
    }
}
