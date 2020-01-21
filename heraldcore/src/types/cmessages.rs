use super::*;
pub(crate) use network_types::cmessages::*;

/// Seals the messages.
pub fn seal(
    // note: this is only mut because BlockStore thinks it should be
    cid: ConversationId,
    content: &ConversationMessage,
) -> Result<(), HErr> {
    todo!();
    //    let cbytes = kson::to_vec(content).into();
    //    let from = config::gid()?;
    //
    //    let ad = kson::to_vec(&(cid, from)).into();
    //
    //    let cipher = chainkeys::seal_msg(cid, ad, cbytes)?;
    //
    //    Ok(cipher)
}

/// Opens the message.
pub fn open(cipher: u8) -> Result<(ConversationId, GlobalId, ConversationMessage), HErr> {
    todo!()
    //let (cid, from) = kson::from_bytes(cipher.ad.clone())?;
    //let decrypted = chainkeys::open_msg(cid, cipher)?.ok_or(ChainKeysError::DecryptionFailed)?;
    //let parsed = kson::from_bytes(decrypted.pt.into())?;

    //Ok((cid, from, parsed))
}
