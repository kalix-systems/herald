use super::*;
use crate::types::cmessages;
use crypto_store::prelude as cstore;
use network_types::{
    cmessages::ConversationMessage,
    umessages::{self, UserMessage},
    Substance,
};
use ratchet_chat::protocol as proto;

fn decode_push(
    Push {
        tag,
        timestamp,
        msg,
        gid: from,
    }: Push
) -> Result<(Option<Substance>, Event), HErr> {
    let mut ev = Event::default();
    let mut substance = None;

    let uid = w!(config::id());
    let kp = w!(config::keypair());

    let proto::MsgResult {
        ack,
        forward,
        output,
        response,
    } = {
        get_crypto_conn!(store);

        let msg: proto::Msg = w!(kson::from_bytes(msg));

        let res = w!(proto::handle_incoming(&mut store, &kp, from, msg));

        w!(store.commit());

        res
    };

    if let Some(ack) = ack {
        ev.add_msg_to_device(from.did, proto::Msg::Ack(ack));
    }

    if let Some(forward) = forward {
        w!(ev.add_msg_to_self(forward))
    }

    if let Some(response) = response {
        ev.add_msg_to_user(from.uid, response);
    }

    if let Some(msg) = output {
        match kson::from_bytes(msg) {
            Ok(s) => {
                substance.replace(s);
            }
            Err(e) => {
                todo!();
            }
        }
    }

    Ok((substance, ev))
}

mod content_handlers;
use content_handlers::handle_content;

fn handle_cmessage(
    ts: Time,
    GlobalId { uid, did: _ }: GlobalId,
    cid: ConversationId,
    msg: ConversationMessage,
) -> Result<Event, HErr> {
    use ConversationMessage::*;
    let mut ev = Event::default();

    match msg {
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

            ev.notifications
                .push(Notification::NewConversation(conv.meta));
        }

        Message(content) => w!(handle_content(cid, uid, ts, &mut ev, content)),
    }

    Ok(ev)
}

fn handle_umessage(
    _: Time,
    from: GlobalId,
    msg: UserMessage,
) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let GlobalId { uid, .. } = from;

    match msg {
        UserMessage::Req(cr) => {
            let umessages::UserReq { cid } = cr;
            let (user, conversation) = w!(crate::user::UserBuilder::new(uid)
                .pairwise_conversation(cid)
                .add());

            let coretypes::conversation::Conversation { meta, .. } = conversation;

            ev.notifications
                .push(Notification::NewUser(Box::new((user, meta))));

            w!(ev.push_cm(
                cid,
                ConversationMessage::Message(NetContent::UserReqAck(cmessages::UserReqAck(true))),
            ));
        }
    }

    Ok(ev)
}
