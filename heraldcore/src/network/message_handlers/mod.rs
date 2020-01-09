use super::*;
use crate::types::{cmessages, dmessages};
use channel_ratchet::Cipher;
use network_types::{cmessages::ConversationMessage, dmessages::DeviceMessage};

mod content_handlers;
use content_handlers::handle_content;

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
                Some(bytes) => Some(image_utils::update_picture_buf(&bytes)?),
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

fn form_ack(mid: MsgId) -> ConversationMessage {
    ConversationMessage::Message(NetContent::Receipt(cmessages::Receipt {
        of: mid,
        stat: MessageReceiptStatus::Received,
    }))
}
