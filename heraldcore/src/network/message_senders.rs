use super::*;
use crate::types::cmessages;
use crate::types::dmessages;

pub(crate) fn send_cmessage(
    cid: ConversationId,
    content: &ConversationMessageBody,
) -> Result<(), HErr> {
    if CAUGHT_UP.load(Ordering::Acquire) {
        let (cm, hash, key) = cmessages::seal(cid, &content)?;

        let to = crate::members::members(&cid)?;
        let exc = *crate::config::keypair()?.public_key();
        let msg = Bytes::from(serde_cbor::to_vec(&cm)?);
        let req = push_users::Req { to, exc, msg };

        let mut db = chainkeys::CK_CONN.lock();
        let mut tx = db.transaction()?;

        let unlocked = chainkeys::db::store_key(&mut tx, cid, hash, &key)?;
        debug_assert!(unlocked.is_empty());
        // TODO: replace used with probably_used here
        // in general we probably want a slightly smarter system for dealing with scenarios where
        // we thought a message wasn't sent but it was
        chainkeys::db::mark_used(&mut tx, cid, cm.body().parent_hashes().iter())?;

        match helper::push_users(&req) {
            Ok(push_users::Res::Success) => {
                tx.commit()?;
                Ok(())
            }
            Ok(push_users::Res::Missing(missing)) => Err(HeraldError(format!(
                "tried to send messages to nonexistent users {:?}",
                missing
            ))),
            Err(e) => {
                chainkeys::db::mark_used(&mut tx, cid, [hash].iter())?;
                tx.commit()?;

                // TODO: maybe try more than once?
                // maybe have some mechanism to send a signal that more things have gone wrong?
                eprintln!(
                    "failed to send message {:?}, error was {}\n\
                     assuming failed session and adding to pending now",
                    req, e
                );

                CAUGHT_UP.store(false, Ordering::Release);

                pending::add_to_pending(cid, content)
            }
        }
    } else {
        pending::add_to_pending(cid, content)
    }
}

pub(super) fn send_dmessage(to: sig::PublicKey, dm: &DeviceMessageBody) -> Result<(), HErr> {
    let msg = Bytes::from(serde_cbor::to_vec(&dmessages::seal(&to, dm)?)?);

    let req = push_devices::Req { to: vec![to], msg };

    // TODO retry logic? for now, things go to the void
    match helper::push_devices(&req)? {
        push_devices::Res::Success => Ok(()),
        push_devices::Res::Missing(missing) => Err(HeraldError(format!(
            "tried to send messages to nonexistent keys {:?}",
            missing
        ))),
    }
}

pub(super) fn send_umessage(uid: UserId, msg: &DeviceMessageBody) -> Result<(), HErr> {
    let meta = match keys_of(vec![uid])?.pop() {
        Some((u, m)) => {
            if u == uid {
                Ok(m)
            } else {
                Err(HErr::HeraldError(format!(
                    "Response returned keys not associated with uid {}\n\
                     failed at line {}",
                    uid,
                    line!()
                )))
            }
        }
        None => Err(HErr::HeraldError(format!(
            "No keys associated with {}\n\
             failed at line {}",
            uid,
            line!()
        ))),
    }?;

    let keys: Vec<sig::PublicKey> = meta.keys.into_iter().map(|(k, _)| k).collect();
    for key in keys {
        send_dmessage(key, msg)?;
    }

    Ok(())
}
