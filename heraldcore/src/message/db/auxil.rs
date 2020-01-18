use super::*;

pub(crate) fn inbound_aux<T: Into<AuxItem>>(
    conn: &mut Conn,
    aux_item: T,
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
    let send_status = SendStatus::Ack;

    let aux_item: AuxItem = aux_item.into();

    let tx = w!(conn.transaction());

    let num_updated = w!(tx.execute_named(
        include_str!("../sql/add_aux.sql"),
        named_params! {
            "@msg_id": mid,
            "@author": uid,
            "@conversation_id": cid,
            "@aux_item": aux_item,
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

    w!(crate::conversation::db::update_last_active(
        &tx,
        time.insertion,
        &cid
    ));

    w!(tx.commit());

    let receipts = get_receipts(&conn, &mid).unwrap_or_default();
    let replies = self::replies(&conn, &mid).unwrap_or_default();
    let reactions = reactions(&conn, &mid).unwrap_or_default();

    Ok(Some(Message {
        message_id: mid,
        author: uid,
        conversation: cid,
        send_status: SendStatus::Ack,
        receipts,
        reactions,
        replies,
        content: aux_item.into(),
        time,
    }))
}

pub(crate) fn outbound_aux<T: Into<AuxItem>>(
    conn: &mut Conn,
    aux: T,
    cid: &ConversationId,
) -> Result<(MsgId, Option<Time>), HErr> {
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

    let send_status = SendStatus::NoAck;
    let mid = MsgId::gen_new();

    let aux_item = aux.into();

    let msg = Message {
        message_id: mid,
        author,
        conversation: *cid,
        send_status: SendStatus::Ack,
        receipts: Default::default(),
        reactions: Default::default(),
        replies: Default::default(),
        content: aux_item.clone().into(),
        time,
    };

    crate::push(OutboundAux::Msg(Box::new(msg)));

    let tx = w!(conn.transaction());

    w!(tx.execute_named(
        include_str!("../sql/add_aux.sql"),
        named_params! {
            "@msg_id": mid,
            "@author": author,
            "@conversation_id": cid,
            "@aux_item": aux_item,
            "@send_status": send_status,
            "@insertion_ts": time.insertion,
            "@server_ts": time.server,
            "@expiration_ts": time.expiration,
        },
    ));

    w!(crate::conversation::db::update_last_active(
        &tx,
        time.insertion,
        cid
    ));

    w!(tx.commit());

    Ok((mid, expiration))
}
