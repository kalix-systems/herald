use super::*;
use crate::types::{cmessages, dmessages};
use kdf_ratchet::Cipher;
use network_types::{cmessages::ConversationMessage, dmessages::DeviceMessage};
use std::ops::DerefMut;

impl Event {
    pub fn handle_push(
        &mut self,
        push: Push,
    ) {
        self.with_comp(|ev| {
            let Push {
                tag,
                timestamp,
                msg,
            } = push;

            match tag {
                PushTag::User => {
                    let cmsg = kson::from_bytes(msg)?;
                    ev.handle_cmessage(timestamp, cmsg)
                }
                PushTag::Device => {
                    let dmsg = kson::from_bytes(msg)?;
                    ev.handle_dmessage(timestamp, dmsg)
                }
                PushTag::Aux => {
                    let amsg = kson::from_bytes(msg)?;
                    ev.handle_amessage(timestamp, amsg)
                }
            }

            Ok(())
        })
    }

    pub(super) fn handle_cmessage(
        &mut self,
        ts: Time,
        cm: Cipher,
    ) {
        use ConversationMessage::*;

        self.with_comp(|ev| {
            let (cid, GlobalId { uid, did }, msg) = cmessages::open(cm)?;

            match msg {
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
                        ev.add_notif(Notification::NewMsg(Box::new(msg)));
                    }

                    ev.add_creply(cid, form_ack(mid));
                }
                Ack(ack) => {
                    let cmessages::Ack {
                        of: msg_id,
                        stat: status,
                    } = ack;

                    ev.with_simple_comp(|| crate::message::add_receipt(msg_id, uid, status));
                    ev.add_notif(Notification::MsgReceipt(message::MessageReceipt {
                        msg_id,
                        cid,
                        recipient: uid,
                        status,
                    }));
                }
                Settings(update) => {
                    ev.with_simple_comp(|| conversation::settings::apply(&update, &cid));

                    ev.add_notif(Notification::Settings(cid, update));
                }
                Leave => unimplemented!(),
            }

            Ok(())
        });
    }

    pub(super) fn handle_dmessage(
        &mut self,
        _: Time,
        msg: DeviceMessage,
    ) {
        self.with_comp(|ev| {
            let (from, msg) = dmessages::open(msg)?;
            let GlobalId { uid, did } = from;

            match msg {
                DeviceMessageBody::Req(dmessages::UserReq { ratchet, cid }) => {
                    let (user, conversation) = crate::user::UserBuilder::new(uid)
                        .pairwise_conversation(cid)
                        .add()?;

                    chainkeys::store_state(cid, did, 0, &ratchet)?;

                    ev.add_notif(Notification::NewUser(Box::new((user, conversation.meta))));

                    ev.add_areply(
                        from.uid,
                        AuxMessage::UserReqAck(amessages::UserReqAck(true)),
                    );
                }
                DeviceMessageBody::NewRatchet(dmessages::NewRatchet { gen, ratchet }) => {
                    let cid = crate::user::by_user_id(uid)?.pairwise_conversation;
                    chainkeys::store_state(cid, did, gen, &ratchet)?;
                }
            }

            Ok(())
        });
    }

    pub(super) fn handle_amessage(
        &mut self,
        ts: Time,
        msg: Cipher,
    ) {
        self.with_comp(move |ev| {
            let (cid, from, msg) = amessages::open(msg)?;
            match msg {
                AuxMessage::NewKey(nk) => crate::user_keys::add_keys(from.uid, &[nk.0])?,
                AuxMessage::DepKey(dk) => crate::user_keys::deprecate_keys(&[dk.0])?,
                AuxMessage::AddedToConvo(ac) => {
                    use crate::{image_utils::image_path, types::amessages::AddedToConvo};
                    use std::fs;

                    let AddedToConvo {
                        ratchets,
                        members,
                        cid,
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

                    let conv = conv_builder.add_db(crate::db::Database::get()?.deref_mut())?;

                    for (did, (gen, ratchet)) in ratchets {
                        chainkeys::store_state(cid, did, gen, &ratchet)?;
                    }

                    ev.add_notif(Notification::NewConversation(conv.meta));
                }
                // UserReqAck(cr) => ev
                //     .notifications
                //     .push(Notification::AddUserResponse(cid, uid, cr.0)),
                _ => {}
            }
            unimplemented!()
        })
    }
}

fn form_ack(mid: MsgId) -> ConversationMessage {
    ConversationMessage::Ack(cmessages::Ack {
        of: mid,
        stat: MessageReceiptStatus::Received,
    })
}
