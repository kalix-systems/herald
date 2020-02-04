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

    let (cid, GlobalId { uid, .. }, msg) = w!(cmessages::open(cm));

    match msg {
        NewKey(nk) => w!(crate::user_keys::add_keys(uid, &[nk.0])),
        DepKey(dk) => w!(crate::user_keys::deprecate_keys(&[dk.0])),
        AddedToConvo { info } => {
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
                Some(bytes) => Some(w!(image_utils::update_picture_buf(&bytes))),
                None => None,
            };

            let mut db = w!(crate::db::Database::get());
            let conv = w!(conv_builder.add_db(&mut db));

            w!(chainkeys::store_state(cid, &ratchet));

            ev.notifications
                .push(Notification::NewConversation(conv.meta));
        }

        Message(content) => w!(handle_content(cid, uid, ts, &mut ev, content)),
    }

    Ok(ev)
}

pub(super) fn handle_dmessage(
    _: Time,
    msg: DeviceMessage,
) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let (from, msg) = w!(dmessages::open(msg));
    let GlobalId { uid, .. } = from;

    match msg {
        DeviceMessageBody::Req(cr) => {
            let dmessages::UserReq { cid } = cr;
            let (user, conversation) = w!(crate::user::UserBuilder::new(uid)
                .pairwise_conversation(cid)
                .add());

            let coretypes::conversation::Conversation { meta, .. } = conversation;
            w!(chainkeys::store_state(cid, &ratchet));

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
        stat: ReceiptStatus::Received,
    }))
}
