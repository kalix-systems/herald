use super::*;
use crate::types::{cmessages, dmessages};
use kdf_ratchet::Cipher;
use network_types::{cmessages::ConversationMessage, dmessages::DeviceMessage};

pub(super) fn handle_cmessage(
    ts: Time,
    cm: Cipher,
) -> Result<Event, HErr> {
    use ConversationMessage::*;
    let mut ev = Event::default();

    let (cid, GlobalId { uid, .. }, msg) = cmessages::open(cm)?;

    match msg {
        NewKey(nk) => crate::user_keys::add_keys(uid, &[nk.0])?,
        DepKey(dk) => crate::user_keys::deprecate_keys(&[dk.0])?,
        AddedToConvo(ac) => {
            use crate::{image_utils::image_path, types::cmessages::AddedToConvo};
            use std::fs;

            let AddedToConvo {
                members,
                cid,
                ratchet,
                title,
                picture,
                expiration_period,
            } = *ac;

            let mut conv_builder = crate::conversation::ConversationBuilder::new();
            conv_builder
                .conversation_id(cid)
                .override_members(members)
                .expiration_period(expiration_period);

            conv_builder.title = title;

            conv_builder.picture = match picture {
                Some(bytes) => {
                    let image_path = image_path();
                    fs::write(&image_path, bytes)?;
                    Some(image_path.into_os_string().into_string()?)
                }
                None => None,
            };

            let mut db = crate::db::Database::get()?;
            let conv = conv_builder.add_db(&mut db)?;

            chainkeys::store_state(cid, &ratchet)?;

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
                ev.notifications.push(Notification::NewMsg(Box::new(msg)));
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
            conversation::settings::apply(&update, &cid)?;

            ev.notifications.push(Notification::Settings(cid, update));
        }
    }

    Ok(ev)
}

pub(super) fn handle_dmessage(
    _: Time,
    msg: DeviceMessage,
) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let (from, msg) = dmessages::open(msg)?;
    let GlobalId { uid, .. } = from;

    match msg {
        DeviceMessageBody::Req(cr) => {
            let dmessages::UserReq { ratchet, cid } = cr;
            let (user, conversation) = crate::user::UserBuilder::new(uid)
                .pairwise_conversation(cid)
                .add()?;

            let coretypes::conversation::Conversation { meta, .. } = conversation;
            chainkeys::store_state(cid, &ratchet)?;

            ev.notifications
                .push(Notification::NewUser(Box::new((user, meta))));

            ev.replies.push((
                cid,
                ConversationMessage::UserReqAck(cmessages::UserReqAck(true)),
            ))
        }
    }

    Ok(ev)
}

fn form_ack(mid: MsgId) -> Result<ConversationMessage, HErr> {
    Ok(ConversationMessage::Ack(cmessages::Ack {
        of: mid,
        stat: MessageReceiptStatus::Received,
    }))
}
