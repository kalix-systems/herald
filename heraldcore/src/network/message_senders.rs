use super::*;
use crate::types::{amessages, cmessages, dmessages};
use network_types::*;

pub(crate) fn send_cmessage(
    cid: ConversationId,
    content: &ConversationMessage,
) -> Result<(), HErr> {
    if CAUGHT_UP.load(Ordering::Acquire) {
        let (gen, cm, new) = cmessages::seal(cid, &content)?;

        let to = crate::members::members(&cid)?;

        if let Some(ratchet) = new {
            let msg = amessages::AuxMessage::NewRatchets(amessages::NewRatchets(vec![(
                cid, gen, ratchet,
            )]));
            for uid in to.iter() {
                dbg!(uid);
                send_amessage(*uid, &msg)?;
            }
        }

        let exc = *crate::config::keypair()?.public_key();
        let msg = kson::to_vec(&cm).into();

        let req = push_users::Req { to, exc, msg };

        match helper::push_users(&req) {
            Ok(push_users::Res::Success) => Ok(()),
            Ok(push_users::Res::Missing(missing)) => Err(HeraldError(format!(
                "tried to send messages to nonexistent users {:?}",
                missing
            ))),
            Err(e) => {
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

pub(crate) fn send_dmessage(
    to: sig::PublicKey,
    dm: &DeviceMessageBody,
) -> Result<(), HErr> {
    let msg = kson::to_vec(&dmessages::seal(to, dm)?).into();

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

pub(crate) fn send_umessage(
    uid: UserId,
    msg: &DeviceMessageBody,
) -> Result<(), HErr> {
    for key in crate::user_keys::get_valid_keys(uid)? {
        send_dmessage(key, msg)?;
    }

    Ok(())
}

pub(crate) fn send_amessage(
    uid: UserId,
    msg: &AuxMessage,
) -> Result<(), HErr> {
    if CAUGHT_UP.load(Ordering::Acquire) {
        let (gen, am, new) = amessages::seal(uid, &msg)?;
        let exc = *crate::config::keypair()?.public_key();
        let me = crate::config::id()?;

        if let Some(ratchet) = new {
            dbg!(gen);

            let dm = DeviceMessageBody::NewRatchet(dmessages::NewRatchet { gen, ratchet });

            for key in crate::user_keys::get_valid_keys(me)? {
                if key != exc {
                    send_dmessage(key, &dm)?;
                }
            }

            send_umessage(uid, &dm)?;
        }

        let msg = kson::to_vec(&am).into();

        let req = push_aux::Req {
            to: vec![uid, me],
            exc,
            msg,
        };

        match helper::push_aux(&req) {
            Ok(push_aux::Res::Success) => Ok(()),
            Ok(push_aux::Res::Missing(missing)) => Err(HeraldError(format!(
                "tried to send messages to nonexistent aux {:?}",
                missing
            ))),
            Err(e) => {
                // TODO: maybe try more than once?
                // maybe have some mechanism to send a signal that more things have gone wrong?
                eprintln!(
                    "failed to send message {:?}, error was {}\n\
                     assuming failed session and adding to pending now",
                    req, e
                );

                CAUGHT_UP.store(false, Ordering::Release);

                // pending::add_to_pending(cid, content)
                Ok(())
            }
        }
    } else {
        // pending::add_to_pending(cid, content)
        Ok(())
    }
}
