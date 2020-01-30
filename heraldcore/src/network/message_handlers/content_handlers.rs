use super::*;
use coremacros::w;

/// How old does a typing indicator need to be before being ignored?
const TYPING_FUZZ: i64 = 10_000;

pub(super) fn handle_content(
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

        Msg(msg) => {
            // items that appear in linear message history
            w!(handle_msg(cid, uid, ts, msg, ev));
        }

        Receipt(receipt) => {
            let cmessages::Receipt {
                of: msg_id,
                stat: status,
            } = receipt;

            w!(crate::message::add_receipt(msg_id, uid, status));
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
                w!(crate::message::remove_reaction(
                    &msg_id,
                    &uid,
                    &react_content
                ));
            } else {
                w!(crate::message::add_reaction(&msg_id, &uid, &react_content));
            }
            ev.notifications.push(Notification::Reaction {
                cid,
                msg_id,
                reactionary: uid,
                content: react_content,
                remove,
            });
        }

        ProfileChanged(change) => {
            // changes to a user's profile
            w!(profile_change(uid, change, ev));
        }

        Typing(time_sent) => {
            let current_time = Time::now();

            // check to make sure the typing indicator was sent recently
            if time_sent.within(TYPING_FUZZ, current_time) && ts.within(TYPING_FUZZ, current_time) {
                crate::push(Notification::TypingIndicator(cid, uid));
            }
        }
    };

    Ok(())
}

fn handle_msg(
    cid: ConversationId,
    uid: UserId,
    ts: Time,
    cmessages::Msg {
        mid,
        content,
        expiration,
    }: cmessages::Msg,
    ev: &mut Event,
) -> Result<(), HErr> {
    match content {
        // normal message
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

            if let Some(msg) = w!(builder.store()) {
                ev.notifications.push(Notification::NewMsg(Box::new(msg)));
                ev.replies.push((cid, form_ack(mid)));
            }
        }

        // group settings update
        cmessages::MsgContent::GroupSettings(settings) => {
            let mut conn = w!(crate::db::Database::get());

            let update = w!(crate::conversation::settings::db::apply_inbound(
                &conn, settings, &cid
            ));

            let msg = w!(crate::message::db::inbound_aux(
                &mut conn,
                update.clone(),
                cid,
                mid,
                uid,
                ts,
                expiration,
            ));

            if let Some(msg) = msg {
                ev.notifications.push(Notification::NewMsg(Box::new(msg)));
                ev.notifications.push(Notification::Settings(cid, update));
            }
        }

        // group membership update
        cmessages::MsgContent::NewMembers(nm) => {
            let mut conn = w!(crate::db::Database::get());
            let tx = w!(conn.transaction());
            w!(crate::members::db::add_members_with_tx(&tx, cid, &nm.0));
            w!(tx.commit());

            let msg = w!(crate::message::db::inbound_aux(
                &mut conn, nm, cid, mid, uid, ts, expiration
            ));

            if let Some(msg) = msg {
                ev.notifications.push(Notification::NewMsg(Box::new(msg)));
            }
        }
    };

    Ok(())
}

fn profile_change(
    uid: UserId,
    change: cmessages::ProfileChanged,
    ev: &mut Event,
) -> Result<(), HErr> {
    use cmessages::ProfileChanged as U;
    use coretypes::conversation::settings::SettingsUpdate as S;
    use herald_user::UserChange::*;

    match change {
        U::Color(color) => {
            if uid == w!(crate::config::id()) {
                w!(crate::user::set_color(uid, color));
                ev.notifications
                    .push(Notification::UserChanged(uid, Color(color)));
            }
        }

        U::DisplayName(name) => {
            w!(crate::user::set_name(
                uid,
                name.as_ref().map(String::as_str)
            ));

            ev.notifications
                .push(Notification::UserChanged(uid, DisplayName(name.clone())));

            if let Some(cid) = w!(crate::conversation::get_pairwise_conversations(&[uid])).pop() {
                ev.notifications
                    .push(Notification::Settings(cid, S::Title(name)));
            }
        }

        U::Picture(buf) => {
            let conn = w!(crate::db::Database::get());
            let path = w!(crate::user::db::set_profile_picture_buf(
                &conn,
                uid,
                buf.as_ref().map(Vec::as_slice),
            ));
            ev.notifications
                .push(Notification::UserChanged(uid, Picture(path.clone())));

            if let Some(cid) = w!(crate::conversation::db::get_pairwise_conversations(
                &conn,
                &[uid]
            ))
            .pop()
            {
                ev.notifications
                    .push(Notification::Settings(cid, S::Picture(path)));
            }
        }
    }
    Ok(())
}
