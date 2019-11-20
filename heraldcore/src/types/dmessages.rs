use super::*;
use network_types::dmessages::DeviceMessage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A contact request.
pub struct UserReq {
    /// The genesis block for the conversation.
    pub gen: Genesis,
    /// The proposed conversation id.
    pub cid: ConversationId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Types of device message.
pub enum DeviceMessageBody {
    /// A contact request
    Req(dmessages::UserReq),
}

pub(crate) fn seal(
    to: &sig::PublicKey,
    content: &DeviceMessageBody,
) -> Result<DeviceMessage, HErr> {
    let mut content = serde_cbor::to_vec(content)?;

    let pk = spk_to_epk(to)?;

    let kp = crate::config::keypair()?;
    let sk = ssk_to_esk(kp.secret_key())?;

    let nonce = box_::gen_nonce();

    let tag = box_::seal_detached(&mut content, &nonce, &pk, &sk);

    Ok(DeviceMessage {
        from: config::gid()?,
        content,
        nonce,
        tag,
        prekey: None,
    })
}

pub(crate) fn open(message: DeviceMessage) -> Result<(GlobalId, DeviceMessageBody), HErr> {
    // TODO: remove this, handle prekey
    assert!(message.prekey.is_none());

    let DeviceMessage {
        from,
        mut content,
        nonce,
        tag,
        ..
    } = message;

    let pk = spk_to_epk(&from.did)?;

    let kp = config::keypair()?;
    let sk = ssk_to_esk(kp.secret_key())?;

    box_::open_detached(&mut content, &tag, &nonce, &pk, &sk)
        .map_err(|_| HeraldError("Failed to decrypt message to device".into()))?;

    let dm = serde_cbor::from_slice(&content)?;

    Ok((from, dm))
}
