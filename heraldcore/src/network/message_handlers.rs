use super::*;
use crate::types::{cmessages, dmessages};
use channel_ratchet::Cipher;
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
        AddedToConvo { info, ratchet } => {
            use crate::types::cmessages::AddedToConvo;

            let AddedToConvo {
                members,
                cid,
                title,
                picture,
                expiration_period,
            } = *info;

            let mut conv_builder = crate::conversation::ConversationBuilder::new();
            conv_builder
                .conversation_id(cid)
                .override_members(members)
                .expiration_period(expiration_period);

            conv_builder.title = title;

            conv_builder.picture = match picture {
                Some(bytes) => Some(image_utils::update_picture_buf(&bytes, None::<&str>)?),
                None => None,
            };

            let mut db = crate::db::Database::get()?;
            let conv = conv_builder.add_db(&mut db)?;

            chainkeys::store_state(cid, &ratchet)?;

            ev.notifications
                .push(Notification::NewConversation(conv.meta));
        }
        Message(content) => handle_content(cid, uid, ts, &mut ev, content)?,
    }

    Ok(ev)
}

fn handle_content(
    cid: ConversationId,
    uid: UserId,
    ts: Time,
    ev: &mut Event,
    content: NetContent,
) -> Result<(), HErr> {
    use NetContent::*;
    match content {
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
            let cmessages::Msg {
                mid,
                content,
                expiration,
            } = msg;

            match content {
                cmessages::MsgContent::Normal(cmessages::Message {
                    body,
                    attachments,
                    op,
                }) => {
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
                        ev.replies.push((cid, form_ack(mid)?));
                    }
                }
                cmessages::MsgContent::GroupSettings(settings) => {
                    let mut conn = crate::db::Database::get()?;

                    let update =
                        crate::conversation::settings::db::apply_inbound(&conn, settings, &cid)?;

                    let msg = crate::message::db::inbound_group_settings(
                        &mut conn,
                        update.clone(),
                        cid,
                        mid,
                        uid,
                        ts,
                        expiration,
                    )?;

                    if let Some(msg) = msg {
                        ev.notifications.push(Notification::NewMsg(Box::new(msg)));
                        ev.notifications.push(Notification::Settings(cid, update));
                    }
                }
            }
        }
        Receipt(receipt) => {
            let cmessages::Receipt {
                of: msg_id,
                stat: status,
            } = receipt;

            crate::message::add_receipt(msg_id, uid, status)?;
            ev.notifications
                .push(Notification::MsgReceipt(message::MessageReceipt {
                    msg_id,
                    cid,
                    recipient: uid,
                    status,
                }));
        }
        Reaction(cmessages::Reaction {
            react_content,
            msg_id,
            remove,
        }) => {
            if remove {
                crate::message::remove_reaction(&msg_id, &uid, &react_content)?;
            } else {
                crate::message::add_reaction(&msg_id, &uid, &react_content)?;
            }
            ev.notifications.push(Notification::Reaction {
                cid,
                msg_id,
                reactionary: uid,
                content: react_content,
                remove,
            });
        }
        Settings(update) => {
            conversation::settings::apply(&update, &cid)?;

            ev.notifications.push(Notification::Settings(cid, update));
        }
    };

    Ok(())
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
                ConversationMessage::Message(NetContent::UserReqAck(cmessages::UserReqAck(true))),
            ))
        }
    }

    Ok(ev)
}

fn form_ack(mid: MsgId) -> Result<ConversationMessage, HErr> {
    Ok(ConversationMessage::Message(NetContent::Receipt(
        cmessages::Receipt {
            of: mid,
            stat: MessageReceiptStatus::Received,
        },
    )))
}
