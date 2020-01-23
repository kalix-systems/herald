use super::*;
use nt::{NetMsg, Substance};

pub(super) fn net_msg(
    ev: &mut Event,
    uid: UserId,
    NetMsg { cid, sub }: NetMsg,
    ts: Time,
) -> Result<(), HErr> {
    use network_types::Substance as S;

    match sub {
        S::Init(init) => w!(self::init(ev, cid, init)),
        S::Msg(msg) => w!(self::msg(ev, cid, msg, ts)),
        S::ProfileChanged(change) => w!(profile(ev, uid, change)),
        S::Reaction(reaction) => w!(self::reaction(ev, cid, uid, reaction)),
        S::Typing(time_sent) => self::typing(ev, cid, uid, time_sent, ts),
        S::Receipt(_) => {}
    };

    Ok(())
}

fn init(
    ev: &mut Event,
    cid: ConversationId,
    init: nt::ConversationInit,
) -> Result<(), HErr> {
    todo!()
}

fn receipt(
    ev: &mut Event,
    cid: ConversationId,
    uid: UserId,
    nt::Receipt {
        of: msg_id,
        stat: status,
    }: nt::Receipt,
) -> Result<(), HErr> {
    w!(crate::message::add_receipt(msg_id, uid, status));

    ev.notifications
        .push(Notification::MsgReceipt(message::MessageReceipt {
            msg_id,
            cid,
            recipient: uid,
            status,
        }));

    Ok(())
}

fn msg(
    ev: &mut Event,
    cid: ConversationId,
    msg: nt::Msg,
    ts: Time,
) -> Result<(), HErr> {
    todo!()
}

fn profile(
    ev: &mut Event,
    uid: UserId,
    change: nt::ProfileChanged,
) -> Result<(), HErr> {
    use coretypes::conversation::settings::SettingsUpdate as S;
    use herald_user::UserChange::*;
    use nt::ProfileChanged as U;

    match change {
        U::Color(color) => {
            if uid == crate::config::id()? {
                crate::user::set_color(uid, color)?;
                ev.notifications
                    .push(Notification::UserChanged(uid, Color(color)));
            }
        }

        U::DisplayName(name) => {
            crate::user::set_name(uid, name.as_ref().map(String::as_str))?;

            ev.notifications
                .push(Notification::UserChanged(uid, DisplayName(name.clone())));

            if let Some(cid) = crate::conversation::get_pairwise_conversations(&[uid])?.pop() {
                ev.notifications
                    .push(Notification::Settings(cid, S::Title(name)));
            }
        }

        U::Picture(buf) => {
            let conn = crate::db::Database::get()?;
            let path = crate::user::db::set_profile_picture_buf(
                &conn,
                uid,
                buf.as_ref().map(Vec::as_slice),
            )?;
            ev.notifications
                .push(Notification::UserChanged(uid, Picture(path.clone())));

            if let Some(cid) =
                crate::conversation::db::get_pairwise_conversations(&conn, &[uid])?.pop()
            {
                ev.notifications
                    .push(Notification::Settings(cid, S::Picture(path)));
            }
        }
    }

    Ok(())
}

fn reaction(
    ev: &mut Event,
    cid: ConversationId,
    uid: UserId,
    nt::Reaction {
        react_content,
        msg_id,
        remove,
    }: nt::Reaction,
) -> Result<(), HErr> {
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

    Ok(())
}

fn typing(
    ev: &mut Event,
    cid: ConversationId,
    uid: UserId,
    time_sent: Time,
    server_ts: Time,
) {
    // How old does a typing indicator need to be before being ignored?
    const TYPING_FUZZ: i64 = 10_000;

    let current_time = Time::now();

    // check to make sure the typing indicator was sent recently
    if time_sent.within(TYPING_FUZZ, current_time) && server_ts.within(TYPING_FUZZ, current_time) {
        ev.note(Notification::TypingIndicator(cid, uid));
    }
}
