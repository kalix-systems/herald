use super::*;
use kdf_ratchet::{Cipher, RatchetState};
pub(crate) use network_types::amessages::*;

#[derive(Ser, De)]
struct AuxAD {
    cid: ConversationId,
    to: UserId,
    from: GlobalId,
    gen: u32,
}

/// Seals the messages.
pub fn seal(
    to: UserId,
    content: &AuxMessage,
) -> Result<(u32, Cipher, Option<RatchetState>), HErr> {
    let cid = crate::user::by_user_id(to)?.pairwise_conversation;
    let cbytes = kson::to_vec(content).into();
    let from = config::gid()?;

    chainkeys::db::with_tx(move |tx| {
        let gen = tx.get_generation(cid, from.did)?;
        let ad = kson::to_vec(&AuxAD { cid, to, from, gen }).into();

        tx.seal_msg(cid, from.did, ad, cbytes).map_err(HErr::from)
    })
}

/// Opens the message.
pub fn open(cipher: Cipher) -> Result<(ConversationId, GlobalId, AuxMessage), HErr> {
    let AuxAD { cid, to, from, gen } = kson::from_bytes(cipher.ad.clone())?;

    if to != config::id()? {
        return Err(HeraldError(format!(
            "auxiliary message was sent by {} to {}, who is not you!",
            from.uid, to
        )));
    }

    let decrypted =
        chainkeys::open_msg(cid, from.did, gen, cipher)?.ok_or(ChainKeysError::DecryptionFailed)?;

    let parsed = kson::from_bytes(decrypted.pt.into())?;

    Ok((cid, from, parsed))
}
