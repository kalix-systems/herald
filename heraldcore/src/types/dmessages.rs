use super::*;
use bytes::*;
pub(crate) use network_types::dmessages::*;

pub(crate) fn seal(
    to: sig::PublicKey,
    content: &DeviceMessageBody,
) -> Result<DeviceMessage, HErr> {
    let mut content = kson::to_vec(content);

    let pk = to.into();
    let sk = kcl::box_::SecretKey::from(config::keypair()?.secret_key().clone());

    let tag = sk.seal(pk, &mut content);

    Ok(DeviceMessage {
        from: config::gid()?,
        content: content.into(),
        tag,
        prekey: None,
    })
}

pub(crate) fn open(message: DeviceMessage) -> Result<(GlobalId, DeviceMessageBody), HErr> {
    // TODO: remove this, handle prekey
    assert!(message.prekey.is_none());

    let DeviceMessage {
        from, content, tag, ..
    } = message;

    let mut content = BytesMut::from(content);

    let pk = from.did.into();
    let sk = kcl::box_::SecretKey::from(config::keypair()?.secret_key().clone());

    if sk.open(pk, tag, &mut content).0 {
        let dm = kson::from_bytes(content.into())?;

        Ok((from, dm))
    } else {
        Err(HeraldError("Failed to decrypt message to device".into()))
    }
}
