use super::*;
use crate::types::cmessages;
use crypto_store::prelude as cstore;
use ratchet_chat::protocol as proto;

/// Outcome of sending a `ConversationMessage`
pub enum SendOutcome {
    /// Message sent succesfully
    Success,
    /// Message was placed in pending
    Pending,
}

pub(crate) fn prepare_send_cmessage(
    cid: ConversationId,
    msg: ConversationMessage,
) -> Result<Vec<(Recip, proto::Msg)>, HErr> {
    let kp = w!(config::keypair());

    get_crypto_conn!(store);

    let substance = network_types::Substance::Cm { cid, msg };
    let payload = proto::Payload::from(kson::to_vec(&substance));

    let members = w!(crate::members::members(&cid));
    let mut out = Vec::with_capacity(members.len());

    for user in members {
        let pairs = w!(proto::prepare_send_to_user(
            &mut store,
            &kp,
            user,
            payload.clone()
        ));

        out.extend(
            pairs
                .into_iter()
                .map(|(k, m)| (Recip::One(SingleRecip::Key(k)), m)),
        );
    }

    // todo: consider committing later
    w!(store.commit());

    Ok(out)
}

pub(crate) fn send_cmessage(
    cid: ConversationId,
    content: ConversationMessage,
) -> Result<SendOutcome, HErr> {
    let prepared = w!(prepare_send_cmessage(cid, content.clone()));
    let from: GlobalId = w!(crate::config::gid());

    if CAUGHT_UP.load(Ordering::Acquire) {
        for (to, msg) in prepared {
            let req = push::Req {
                from,
                to,
                msg: kson::to_vec(&msg).into(),
            };
            match helper::push(&req) {
                Ok(push::Res::Success(ts)) => {}
                Ok(push::Res::Missing(missing)) => {
                    return Err(HeraldError(format!(
                        "tried to send messages to nonexistent users {:?}",
                        missing
                    )));
                }
                Err(e) => {
                    CAUGHT_UP.store(false, Ordering::Release);
                    w!(pending::add_to_pending(cid, &content));
                    return Ok(SendOutcome::Pending);
                }
            }
        }
        Ok(SendOutcome::Success)
    } else {
        w!(pending::add_to_pending(cid, &content));
        Ok(SendOutcome::Pending)
    }
}

pub(crate) fn prepare_send_umessage(
    uid: UserId,
    um: UserMessage,
) -> Result<Vec<(Recip, proto::Msg)>, HErr> {
    let kp = w!(config::keypair());

    get_crypto_conn!(store);

    let substance = network_types::Substance::Um(um);
    let payload = proto::Payload::from(kson::to_vec(&substance));

    let prepared = w!(proto::prepare_send_to_user(&mut store, &kp, uid, payload));

    // todo: consider committing later
    w!(store.commit());

    Ok(prepared
        .into_iter()
        .map(|(k, m)| (Recip::One(SingleRecip::Key(k)), m))
        .collect())
}

pub(super) fn send_umessage(
    uid: UserId,
    msg: UserMessage,
) -> Result<(), HErr> {
    let prepared = w!(prepare_send_umessage(uid, msg));
    let from: GlobalId = w!(crate::config::gid());
    for (to, msg) in prepared {
        let req = push::Req {
            from,
            to,
            msg: kson::to_vec(&msg).into(),
        };
        w!(helper::push(&req));
    }

    Ok(())
}
