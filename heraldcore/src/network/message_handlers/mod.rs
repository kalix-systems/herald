use super::*;
use crate::types::cmessages;
use crypto_store::prelude as cstore;
use network_types::{cmessages::ConversationMessage, dmessages::DeviceMessage, Substance};
use ratchet_chat::protocol as proto;

#[derive(Default, Debug)]
struct PushResult {
    substance: Option<Substance>,
    outbox: Vec<(Recip, proto::Msg)>,
}

impl PushResult {
    fn add_msg_to_self(
        &mut self,
        msg: proto::Msg,
    ) -> Result<(), HErr> {
        let uid = w!(config::id());
        self.outbox.push((Recip::One(SingleRecip::User(uid)), msg));
        Ok(())
    }

    fn add_msg_to_user(
        &mut self,
        to: UserId,
        msg: proto::Msg,
    ) -> Result<(), HErr> {
        self.outbox.push((Recip::One(SingleRecip::User(to)), msg));
        Ok(())
    }

    fn add_msg_to_device(
        &mut self,
        to: sig::PublicKey,
        msg: proto::Msg,
    ) -> Result<(), HErr> {
        self.outbox.push((Recip::One(SingleRecip::Key(to)), msg));
        Ok(())
    }
}

fn decode_push(
    Push {
        tag,
        timestamp,
        msg,
        gid: from,
    }: Push
) -> Result<PushResult, HErr> {
    let mut res = PushResult::default();

    let uid = w!(config::id());
    let kp = w!(config::keypair());

    let proto::MsgResult {
        ack,
        forward,
        output,
        response,
    } = {
        let raw = cstore::raw_conn();
        let mut lock = raw.lock();
        let mut store = w!(cstore::as_conn(&mut lock));

        let msg: proto::Msg = w!(kson::from_bytes(msg));

        let res = w!(proto::handle_incoming(&mut store, &kp, from, msg));

        w!(store.commit());

        res
    };

    if let Some(ack) = ack {
        w!(res.add_msg_to_device(from.did, proto::Msg::Ack(ack)));
    }

    if let Some(forward) = forward {
        w!(res.add_msg_to_self(forward))
    }

    if let Some(response) = response {
        w!(res.add_msg_to_user(from.uid, response));
    }

    if let Some(msg) = output {
        match kson::from_bytes(msg) {
            Ok(s) => {
                res.substance.replace(s);
            }
            Err(e) => {
                todo!();
            }
        }
    }

    Ok(res)
}

// mod content_handlers;
// use content_handlers::handle_content;

// pub(super) fn handle_push(ts:Time,msg:Bytes,

// pub(super) fn handle_cmessage(
//     ts: Time,
//     msg: Bytes,
// ) -> Result<Event, HErr> {
//     use ConversationMessage::*;
//     let mut ev = Event::default();

//     let (cid, GlobalId { uid, .. }, msg) = w!(cmessages::open(cm));

//     match msg {
//         NewKey(nk) => w!(crate::user_keys::add_keys(uid, &[nk.0])),
//         DepKey(dk) => w!(crate::user_keys::deprecate_keys(&[dk.0])),
//         AddedToConvo { info } => {
//             use crate::types::cmessages::AddedToConvo;

//             let AddedToConvo {
//                 members,
//                 cid,
//                 title,
//                 picture,
//                 expiration_period,
//             } = *info;

//             let mut conv_builder = crate::conversation::ConversationBuilder::new();
//             conv_builder
//                 .conversation_id(cid)
//                 .override_members(members)
//                 .expiration_period(expiration_period);

//             conv_builder.title = title;

//             conv_builder.picture = match picture {
//                 Some(bytes) => Some(w!(image_utils::update_picture_buf(&bytes))),
//                 None => None,
//             };

//             let mut db = w!(crate::db::Database::get());
//             let conv = w!(conv_builder.add_db(&mut db));

//             w!(chainkeys::store_state(cid, &ratchet));

//             ev.notifications
//                 .push(Notification::NewConversation(conv.meta));
//         }

//         Message(content) => w!(handle_content(cid, uid, ts, &mut ev, content)),
//     }

//     Ok(ev)
// }

// pub(super) fn handle_dmessage(
//     _: Time,
//     msg: DeviceMessage,
// ) -> Result<Event, HErr> {
//     let mut ev = Event::default();

//     let (from, msg) = w!(dmessages::open(msg));
//     let GlobalId { uid, .. } = from;

//     match msg {
//         DeviceMessageBody::Req(cr) => {
//             let dmessages::UserReq { cid } = cr;
//             let (user, conversation) = w!(crate::user::UserBuilder::new(uid)
//                 .pairwise_conversation(cid)
//                 .add());

//             let coretypes::conversation::Conversation { meta, .. } = conversation;
//             w!(chainkeys::store_state(cid, &ratchet));

//             ev.notifications
//                 .push(Notification::NewUser(Box::new((user, meta))));

//             ev.replies.push((
//                 cid,
//                 ConversationMessage::Message(NetContent::UserReqAck(cmessages::UserReqAck(true))),
//             ))
//         }
//     }

//     Ok(ev)
// }

// fn form_ack(mid: MsgId) -> ConversationMessage {
//     ConversationMessage::Message(NetContent::Receipt(cmessages::Receipt {
//         of: mid,
//         stat: ReceiptStatus::Received,
//     }))
// }
