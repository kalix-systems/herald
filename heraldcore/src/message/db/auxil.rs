use super::*;

pub(crate) fn inbound_group_settings(
    conn: &mut Conn,
    update: coretypes::conversation::settings::SettingsUpdate,
    cid: ConversationId,
    mid: MsgId,
    uid: UserId,
    server_ts: Time,
    expiration: Option<Time>,
) -> Result<Option<Message>, HErr> {
    #[cfg(not(test))]
    {
        if let Some(expiration) = expiration {
            // short circuit if message has already expired
            if expiration < Time::now() {
                return Ok(None);
            }
        }
    }

    let time = MessageTime {
        server: server_ts.into(),
        expiration,
        insertion: Time::now(),
    };

    // this can be inferred from the fact that this message was received
    let send_status = MessageSendStatus::Ack;

    let tx = w!(conn.transaction());

    let num_updated = w!(tx.execute_named(
        include_str!("../sql/add_update.sql"),
        named_params! {
            "@msg_id": mid,
            "@author": uid,
            "@conversation_id": cid,
            "@update_item": update,
            "@send_status": send_status,
            "@insertion_ts": time.insertion,
            "@server_ts": time.server,
            "@expiration_ts": time.expiration,
        },
    ));

    // early return on redundant insert
    if num_updated != 1 {
        return Ok(None);
    }

    w!(tx.execute(
        include_str!("../../conversation/sql/update_last_active.sql"),
        params![Time::now(), cid],
    ));

    w!(tx.commit());

    let receipts = get_receipts(&conn, &mid).unwrap_or_default();
    let replies = self::replies(&conn, &mid).unwrap_or_default();
    let reactions = reactions(&conn, &mid).unwrap_or_default();

    Ok(Some(Message {
        message_id: mid,
        author: uid,
        conversation: cid,
        send_status: MessageSendStatus::Ack,
        receipts,
        reactions,
        replies,
        content: coretypes::messages::Item::Update(update).into(),
        time,
    }))
}

pub(crate) fn outbound_group_settings(
    conn: &mut Conn,
    update: coretypes::conversation::settings::SettingsUpdate,
    cid: &ConversationId,
) -> Result<Message, HErr> {
    let author = crate::config::db::id(&conn)?;

    let timestamp = Time::now();
    let expiration_period = w!(expiration_period(&conn, &cid));
    let expiration = match expiration_period.into_millis() {
        Some(period) => Some(timestamp + period),
        None => None,
    };

    let time = MessageTime {
        server: None,
        expiration,
        insertion: timestamp,
    };

    let send_status = MessageSendStatus::NoAck;
    let mid = MsgId::gen_new();

    let tx = w!(conn.transaction());

    w!(tx.execute_named(
        include_str!("../sql/add_update.sql"),
        named_params! {
            "@msg_id": mid,
            "@author": author,
            "@conversation_id": cid,
            "@update_item": update,
            "@send_status": send_status,
            "@insertion_ts": time.insertion,
            "@server_ts": time.server,
            "@expiration_ts": time.expiration,
        },
    ));

    w!(tx.execute(
        include_str!("../../conversation/sql/update_last_active.sql"),
        params![Time::now(), cid],
    ));

    w!(tx.commit());

    let receipts = get_receipts(&conn, &mid).unwrap_or_default();
    let replies = self::replies(&conn, &mid).unwrap_or_default();
    let reactions = reactions(&conn, &mid).unwrap_or_default();

    Ok(Message {
        message_id: mid,
        author,
        conversation: *cid,
        send_status: MessageSendStatus::Ack,
        receipts,
        reactions,
        replies,
        content: coretypes::messages::Item::Update(update).into(),
        time,
    })
}
