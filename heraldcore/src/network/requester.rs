use super::*;
use anyhow::anyhow;
use herald_network::Requester;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

static HANDLE: OnceCell<Mutex<Requester>> = OnceCell::new();

pub(super) fn update(handle: Requester) {
    // try to set the client
    if let Err(handle) = HANDLE.set(Mutex::new(handle)) {
        // if it fails, this means the handle is already set, so we should replace it
        if let Some(slot) = HANDLE.get() {
            let mut lock = slot.lock();
            *lock = handle.into_inner();
        }
    }
}

pub(super) fn send<F: FnMut(Result<Response, HErr>) + Send + 'static>(
    req: Request,
    f: F,
) -> Result<(), HErr> {
    let handle: &Mutex<Requester> = HANDLE
        .get()
        .ok_or_else(|| anyhow!("Request handle not set"))?;

    handle.lock().send(req, f)?;

    Ok(())
}

/// Sends a message read receipt
pub fn send_read_receipt(
    cid: ConversationId,
    msg_id: MsgId,
) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Receipt(cmessages::Receipt {
    //        of: msg_id,
    //        stat: ReceiptStatus::Read,
    //    })),
    //)?;
    //Ok(())
}

/// Sends a typing indicator
pub fn send_typing_indicator(cid: ConversationId) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Typing(Time::now())),
    //)?;

    //Ok(())
}

/// Sends a user request to `uid` with a proposed conversation id `cid`.
pub fn send_user_req(
    uid: UserId,
    cid: ConversationId,
) -> Result<(), HErr> {
    todo!()
    //let ratchet = RatchetState::new();
    //chainkeys::store_state(cid, &ratchet)?;

    //let req = dmessages::UserReq { ratchet, cid };

    //send_umessage(uid, &DeviceMessageBody::Req(req))
}

pub(crate) fn send_normal_message(
    cid: ConversationId,
    msg: cmessages::Msg,
) -> Result<SendOutcome, HErr> {
    todo!()
    //let mid = msg.mid;
    //let outcome = send_cmessage(cid, &ConversationMessage::Message(NetContent::Msg(msg)))?;

    //if let SendOutcome::Success = outcome {
    //    {
    //        let conn = crate::db::Database::get()?;
    //        crate::message::db::update_send_status(
    //            &conn,
    //            mid,
    //            coretypes::messages::SendStatus::Ack,
    //        )?;
    //    }
    //}

    //Ok(outcome)
}

pub(crate) fn send_group_settings_message(
    mid: MsgId,
    cid: ConversationId,
    expiration: Option<Time>,
    update: cmessages::GroupSettingsUpdate,
) -> Result<(), HErr> {
    if let SendOutcome::Success = send_normal_message(
        cid,
        cmessages::Msg {
            mid,
            expiration,
            content: cmessages::MsgContent::GroupSettings(update),
        },
    )? {
        crate::push(crate::message::OutboundAux::SendDone(cid, mid));
    }
    Ok(())
}

pub(crate) fn send_profile_update(update: cmessages::ProfileChanged) -> Result<(), HErr> {
    todo!()
    //let conn = crate::db::Database::get()?;

    //use cmessages::ProfileChanged as P;
    //match update {
    //    color @ P::Color(_) => {
    //        let cid = crate::config::db::nts_conversation(&conn)?;
    //        let msg = ConversationMessage::Message(NetContent::ProfileChanged(color));

    //        send_cmessage(cid, &msg)?;
    //    }
    //    other => {
    //        let cids = crate::conversation::db::get_all_pairwise_conversations(&conn)?;

    //        let msg = ConversationMessage::Message(NetContent::ProfileChanged(other));

    //        for cid in cids {
    //            send_cmessage(cid, &msg)?;
    //        }
    //    }
    //}

    //Ok(())
}

/// Sends a reaction
pub fn send_reaction(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
    //        msg_id,
    //        react_content,
    //        remove: false,
    //    })),
    //)?;
    //Ok(())
}

/// Sends a reaction removal update
pub fn send_reaction_removal(
    cid: ConversationId,
    msg_id: MsgId,
    react_content: crate::message::ReactContent,
) -> Result<(), HErr> {
    todo!()
    //send_cmessage(
    //    cid,
    //    &ConversationMessage::Message(NetContent::Reaction(cmessages::Reaction {
    //        msg_id,
    //        react_content,
    //        remove: true,
    //    })),
    //)?;
    //Ok(())
}

/// Outcome of sending a `ConversationMessage`
pub enum SendOutcome {
    /// Message sent succesfully
    Success,
    /// Message was placed in pending
    Pending,
}
//mod event;
//use event::*;

//mod helper;

/// Deprecates key on server.
pub fn dep_key(to_dep: sig::PublicKey) -> Result<PKIResponse, HErr> {
    todo!()
    //    let kp = config::keypair()?;
    //    let req = dep_key::Req(kp.sign(to_dep));
    //    Ok(helper::dep_key(&req)?.0)
}

/// Adds new key to the server's key registry.
pub fn new_key(to_new: sig::PublicKey) -> Result<PKIResponse, HErr> {
    todo!()
    //let kp = config::keypair()?;
    //let req = new_key::Req(kp.sign(to_new));
    //Ok(helper::new_key(&req)?.0)
}
