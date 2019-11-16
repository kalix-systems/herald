use super::*;

pub(super) fn handle_cmessage(ts: Time, cm: ConversationMessage) -> Result<Event, HErr> {
    use ConversationMessageBody::*;
    let mut ev = Event::default();

    let cid = cm.cid();

    let msgs = cm.open()?;

    for (msg, from) in msgs {
        match msg {
            NewKey(nk) => crate::user_keys::add_keys(from.uid, &[nk.0])?,
            DepKey(dk) => crate::user_keys::deprecate_keys(&[dk.0])?,
            AddedToConvo(ac) => {
                let mut db = crate::db::Database::get()?;
                let tx = db.transaction()?;

                let cid = ac.cid;
                let title = ac.title;

                let mut conv_builder = crate::conversation::ConversationBuilder::new();
                conv_builder.conversation_id(cid);

                if let Some(title) = title {
                    conv_builder.title(title);
                }

                conv_builder.add_with_tx(&tx)?;
                crate::members::db::add_members_with_tx(&tx, cid, &ac.members)?;
                tx.commit()?;

                cid.store_genesis(&ac.gen)?;

                ev.notifications.push(Notification::NewConversation(cid));
            }
            UserReqAck(cr) => ev
                .notifications
                .push(Notification::AddUserResponse(cid, from.uid, cr.0)),
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
                    .author(from.uid)
                    .conversation_id(cid)
                    .attachments(attachments)
                    .timestamp(ts);

                if let Some(body) = body {
                    builder.body(body);
                }

                if let Some(op) = op {
                    builder.replying_to(op);
                }

                if let Some(expiration) = expiration {
                    builder.expiration(expiration);
                }

                builder.store()?;

                ev.notifications.push(Notification::NewMsg(mid, cid));
                ev.replies.push((cid, form_ack(mid)?));
            }
            Ack(ack) => {
                crate::message::add_receipt(ack.of, from.uid, ack.stat)?;
                ev.notifications
                    .push(Notification::MsgReceipt { mid: ack.of, cid });
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

    match msg {
        DeviceMessageBody::Req(cr) => {
            let dmessages::UserReq { gen, cid } = cr;
            if gen.verify_sig(&from.did) {
                crate::user::UserBuilder::new(from.uid)
                    .pairwise_conversation(cid)
                    .add()?;

                cid.store_genesis(&gen)?;

                ev.notifications.push(Notification::NewUser(from.uid, cid));

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
