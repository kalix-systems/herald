use super::*;
use kdf_ratchet::{Cipher, RatchetState};
pub(crate) use network_types::cmessages::*;

#[derive(Ser, De)]
pub struct CAd {
    cid: ConversationId,
    from: GlobalId,
    gen: u32,
}

/// Seals the messages.
pub fn seal(
    cid: ConversationId,
    content: &ConversationMessage,
) -> Result<(u32, Cipher, Option<RatchetState>), HErr> {
    let cbytes = kson::to_vec(content).into();
    let from = config::gid()?;

    chainkeys::db::with_tx(move |tx| {
        let gen = tx.get_generation(cid, from.did)?;
        let ad = kson::to_vec(&CAd { cid, from, gen }).into();

        tx.seal_msg(cid, from.did, ad, cbytes).map_err(HErr::from)
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
