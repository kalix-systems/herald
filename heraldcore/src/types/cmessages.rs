use super::*;
pub(crate) use network_types::cmessages::*;

/// Seals the messages.
pub fn seal(
    // note: this is only mut because BlockStore thinks it should be
    cid: ConversationId,
    content: &ConversationMessageBody,
) -> Result<(ConversationMessage, BlockHash, ChainKey), HErr> {
    use chainkeys::ChainKeysError;

    let cbytes = serde_cbor::to_vec(content)?;
    let kp = config::keypair()?;
    let from = config::gid()?;
    let (hashes, keys) = chainkeys::get_unused(&cid)?.into_iter().unzip();
    let channel_key = chainkeys::get_channel_key(&cid)?;

    // FIXME: This is terrible. Why don't these functions just return results?
    let SealData { block, key } = Block::seal(kp.secret_key(), &channel_key, &keys, hashes, cbytes)
        .ok_or(HErr::ChainError(ChainKeysError::Chain(
            ChainMailError::CryptoError,
        )))?;
    let hash = block
        .compute_hash()
        .ok_or(HErr::ChainError(ChainKeysError::Chain(
            ChainMailError::CryptoError,
        )))?;

    Ok((
        ConversationMessage {
            cid,
            from,
            body: block,
        },
        hash,
        key,
    ))
}

/// Opens the message.
pub fn open(
    message: ConversationMessage,
) -> Result<Vec<(ConversationMessageBody, GlobalId)>, HErr> {
    let ConversationMessage { cid, from, body } = message;

    let mut out = Vec::new();

    let mut blocks = {
        match chainkeys::open_block(&cid, &from, body)? {
            DecryptionResult::Success(bvec, unlocked) => {
                out.push((serde_cbor::from_slice(&bvec)?, from));
                unlocked
            }
            DecryptionResult::Pending => Vec::new(),
        }
    };

    while let Some((block, from)) = blocks.pop() {
        match chainkeys::open_block(&cid, &from, block)? {
            DecryptionResult::Success(bvec, mut unlocked) => {
                blocks.append(&mut unlocked);
                out.push((serde_cbor::from_slice(&bvec)?, from));
            }
            DecryptionResult::Pending => {
                panic!("this should never happen");
            }
        }
    }

    Ok(out)
}
