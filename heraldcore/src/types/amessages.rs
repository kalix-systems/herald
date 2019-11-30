use super::*;
use kdf_ratchet::Cipher;
pub(crate) use network_types::amessages::*;

/// Seals the messages.
pub fn seal(
    // note: this is only mut because BlockStore thinks it should be
    to: UserId,
    content: &AuxMessage,
) -> Result<Cipher, HErr> {
    let cid = crate::user::by_user_id(to)?.pairwise_conversation;
    let cbytes = kson::to_vec(content).into();
    let from = config::gid()?;

    let ad = kson::to_vec(&(cid, to, from)).into();

    let cipher = chainkeys::seal_msg(cid, from.did, ad, cbytes)?;

    Ok(cipher)
}

/// Opens the message.
pub fn open(cipher: Cipher) -> Result<(ConversationId, GlobalId, ConversationMessage), HErr> {
    let (cid, to, from) =
        kson::from_bytes::<(ConversationId, UserId, GlobalId)>(cipher.ad.clone())?;

    if to != config::id()? {
        return Err(HeraldError(format!(
            "auxiliary message was sent by {} to {}, who is not you!",
            from.uid, to
        )));
    }

    let decrypted =
        chainkeys::open_msg(cid, from.did, cipher)?.ok_or(ChainKeysError::DecryptionFailed)?;

    let parsed = kson::from_bytes(decrypted.pt.into())?;

    Ok((cid, from, parsed))
}
