use super::*;
use kdf_ratchet::Cipher;
pub(crate) use network_types::cmessages::*;

#[derive(Ser, De)]
pub struct CAd {
    cid: ConversationId,
    from: GlobalId,
    gen: u32,
}

/// Seals the messages.
pub fn seal(
    // note: this is only mut because BlockStore thinks it should be
    cid: ConversationId,
    content: &ConversationMessage,
) -> Result<Cipher, HErr> {
    let cbytes = kson::to_vec(content).into();
    let from = config::gid()?;

    chainkeys::db::with_tx(move |tx| {
        let gen = tx.get_generation(cid, from.did)?;
        let ad = kson::to_vec(&CAd { cid, from, gen }).into();

        let cipher = chainkeys::seal_msg(cid, from.did, ad, cbytes)?;

        Ok(cipher)
    })
}

/// Opens the message.
pub fn open(cipher: Cipher) -> Result<(ConversationId, GlobalId, ConversationMessage), HErr> {
    let CAd { cid, from, gen } = kson::from_bytes(cipher.ad.clone())?;
    let decrypted =
        chainkeys::open_msg(cid, from.did, gen, cipher)?.ok_or(ChainKeysError::DecryptionFailed)?;
    let parsed = kson::from_bytes(decrypted.pt.into())?;

    Ok((cid, from, parsed))
}
