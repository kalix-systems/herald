use super::*;

pub(super) fn handle_cmessage(ts: Time, cm: ConversationMessage) -> Result<Event, HErr> {
    use ConversationMessageBody::*;
    let mut ev = Event::default();

    let cid = cm.cid();

    let msgs = cm.open()?;

    for (msg, GlobalId { uid, .. }) in msgs {
        match msg {
            NewKey(nk) => crate::user_keys::add_keys(uid, &[nk.0])?,
            DepKey(dk) => crate::user_keys::deprecate_keys(&[dk.0])?,
            AddedToConvo(ac) => {
                use crate::types::cmessages::AddedToConvo;

                let AddedToConvo {
                    members,
                    cid,
                    gen,
                    title,
                } = *ac;

                let mut conv_builder = crate::conversation::ConversationBuilder::new();
                conv_builder.conversation_id(cid).override_members(members);
                conv_builder.title = title;

                let mut db = crate::db::Database::get()?;
                let conv = conv_builder.add_db(&mut db)?;

                cid.store_genesis(&gen)?;

                ev.notifications
                    .push(Notification::NewConversation(conv.meta));
            }
            UserReqAck(cr) => ev
                .notifications
                .push(Notification::AddUserResponse(cid, uid, cr.0)),
            NewMembers(nm) => {
                let mut db = crate::db::Database::get()?;
                let tx = db.transaction()?;
                crate::members::db::add_members_with_tx(&tx, cid, &nm.0)?;
                tx.commit()?;
            }
            Msg(msg) => {
                let cmessages::Msg { mid, content, op } = msg;
                let cmessages::Message {
                    body,
                    attachments,
                    expiration,
                } = content;

                let mut builder = crate::message::InboundMessageBuilder::default();

                builder
                    .id(mid)
                    .author(uid)
                    .conversation_id(cid)
                    .attachments(attachments)
                    .timestamp(ts);

                builder.body = body;
                builder.op = op;
                builder.expiration = expiration;

                if let Some(msg) = builder.store()? {
                    ev.notifications.push(Notification::NewMsg(msg));
                }
                ev.replies.push((cid, form_ack(mid)?));
            }
            Ack(ack) => {
                let cmessages::Ack {
                    of: msg_id,
                    stat: status,
                } = ack;

                crate::message::add_receipt(msg_id, uid, status)?;
                ev.notifications
                    .push(Notification::MsgReceipt(message::MessageReceipt {
                        msg_id,
                        cid,
                        recipient: uid,
                        status,
                    }));
            }
            Settings(update) => {
                update.apply(&cid)?;
                ev.notifications.push(Notification::Settings(cid, update));
            }
        }
    }

    Ok(ev)
}

pub(super) fn handle_dmessage(_: Time, msg: DeviceMessage) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let (from, msg) = msg.open()?;
    let GlobalId { did, uid } = from;

    match msg {
        DeviceMessageBody::Req(cr) => {
            let dmessages::UserReq { gen, cid } = cr;
            if gen.verify_sig(&did) {
                let (user, conversation) = crate::user::UserBuilder::new(uid)
                    .pairwise_conversation(cid)
                    .add()?;

                let conversation::Conversation { meta, .. } = conversation;
                cid.store_genesis(&gen)?;

                ev.notifications.push(Notification::NewUser(user, meta));

                ev.replies.push((
                    cid,
                    ConversationMessageBody::UserReqAck(cmessages::UserReqAck(true)),
                ))
            }
        }
    }

    Ok(ev)
}

fn form_ack(mid: MsgId) -> Result<ConversationMessageBody, HErr> {
    Ok(ConversationMessageBody::Ack(cmessages::Ack {
        of: mid,
        stat: MessageReceiptStatus::Received,
    }))
}
