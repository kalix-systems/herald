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
        S::Init(init) => w!(self::init(ev, cid, uid, init)),

        S::Msg(msg) => w!(self::msg(ev, cid, msg, uid, ts)),

        S::ProfileChanged(change) => w!(profile(ev, uid, change)),

        S::Reaction(reaction) => w!(self::reaction(ev, cid, uid, reaction)),

        S::Typing(time_sent) => self::typing(ev, cid, uid, time_sent, ts),

        S::Receipt(receipt) => w!(self::receipt(ev, cid, uid, receipt)),
    };

    Ok(())
}

fn init(
    ev: &mut Event,
    cid: ConversationId,
    uid: UserId,
    init: nt::ConversationInit,
) -> Result<(), HErr> {
    match init {
        nt::ConversationInit::Pairwise => w!(pairwise_init(ev, cid, uid)),
        nt::ConversationInit::Group {
            members,
            title,
            picture,
            expiration_period,
        } => {
            use crate::conversation as c;
            let mut conv_builder = c::ConversationBuilder::new();
            conv_builder
                .conversation_id(cid)
                .override_members(members)
                .expiration_period(expiration_period);

            conv_builder.title = title;

            conv_builder.picture = match picture {
                Some(bytes) => image_utils::update_picture_buf(&bytes).ok(),
                None => None,
            };

            let mut db = w!(crate::db::Database::get());
            let conv = w!(conv_builder.add_db(&mut db));

            ev.note(Notification::NewConversation(conv.meta));
        }
    };

    Ok(())
}

fn pairwise_init(
    ev: &mut Event,
    cid: ConversationId,
    uid: UserId,
) -> Result<(), HErr> {
    use crate::user;

    let builder = user::UserBuilder::new(uid).pairwise_conversation(cid);
    let (user, conv) = w!(builder.add());

    ev.note(Notification::NewUser(Box::new((user, conv.meta))));

    Ok(())
}

fn msg(
    ev: &mut Event,
    cid: ConversationId,
    nt::Msg {
        mid,
        content,
        expiration,
    }: nt::Msg,
    uid: UserId,
    ts: Time,
) -> Result<(), HErr> {
    use nt::MsgContent as M;
    match content {
        M::Normal(nt::Message {
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
                ev.note(Notification::NewMsg(Box::new(msg)));
            }
        }

        M::GroupSettings(settings) => {
            let mut conn = crate::db::Database::get()?;

            let update = crate::conversation::settings::db::apply_inbound(&conn, settings, &cid)?;

            let msg = crate::message::db::inbound_aux(
                &mut conn,
                update.clone(),
                cid,
                mid,
                uid,
                ts,
                expiration,
            )?;

            if let Some(msg) = msg {
                ev.note(Notification::NewMsg(Box::new(msg)));
                ev.note(Notification::Settings(cid, update));
            }
        }

        M::Membership(m) => {
            use coretypes::messages::Membership as M;
            match m {
                M::Added { members, added_by } => todo!(),
                M::Left(_) => todo!(),
            }
        }
    };

    let mut raw = raw_conn().lock();
    let mut conn: Conn = w!(raw.transaction()).into();
    let tx = &mut conn;

    let kp = w!(crate::config::keypair());

    let replies = w!(prepare_send_to_convo(
        tx,
        &kp,
        cid,
        kson::to_bytes(&nt::Receipt {
            of: mid,
            stat: ReceiptStatus::Received,
        }),
    ));

    w!(conn.commit());

    ev.replies(replies);

    Ok(())
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

    ev.note(Notification::MsgReceipt(message::MessageReceipt {
        msg_id,
        cid,
        recipient: uid,
        status,
    }));

    Ok(())
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
                ev.note(Notification::UserChanged(uid, Color(color)));
            }
        }

        U::DisplayName(name) => {
            crate::user::set_name(uid, name.as_ref().map(String::as_str))?;

            ev.note(Notification::UserChanged(uid, DisplayName(name.clone())));

            if let Some(cid) = crate::conversation::get_pairwise_conversations(&[uid])?.pop() {
                ev.note(Notification::Settings(cid, S::Title(name)));
            }
        }

        U::Picture(buf) => {
            let conn = crate::db::Database::get()?;
            let path = crate::user::db::set_profile_picture_buf(
                &conn,
                uid,
                buf.as_ref().map(Vec::as_slice),
            )?;

            ev.note(Notification::UserChanged(uid, Picture(path.clone())));

            if let Some(cid) =
                crate::conversation::db::get_pairwise_conversations(&conn, &[uid])?.pop()
            {
                ev.note(Notification::Settings(cid, S::Picture(path)));
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

    ev.note(Notification::Reaction {
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
