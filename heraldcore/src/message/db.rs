use super::*;
use crate::{conversation::db::expiration_period, message::MessageTime, w};
use rusqlite::{named_params, Connection as Conn};
use std::collections::HashSet;

/// Get message by message id
pub(crate) fn get_message(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<Message, HErr> {
    let receipts = get_receipts(conn, msg_id)?;
    let replies = self::replies(conn, msg_id)?;
    let attachments = crate::message::attachments::db::get(conn, &msg_id)?;
    let reactions = reactions(conn, msg_id)?;

    let mut stmt = conn.prepare_cached(include_str!("sql/get_message.sql"))?;

    Ok(w!(stmt.query_row_named(
        named_params! {
            "@msg_id": msg_id
        },
        |row| {
            let time = MessageTime {
                insertion: row.get("insertion_ts")?,
                server: row.get("server_ts")?,
                expiration: row.get("expiration_ts")?,
            };

            let is_reply: bool = row.get("is_reply")?;
            let op: Option<MsgId> = row.get("op_msg_id")?;

            let op = (op, is_reply).into();

            Ok(Message {
                message_id: row.get("msg_id")?,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                body: row.get("body")?,
                op,
                send_status: row.get("send_status")?,
                attachments,
                time,
                receipts,
                replies,
                reactions,
            })
        },
    )))
}

/// Get message metadata by message id
pub(crate) fn message_meta(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<MessageMeta, HErr> {
    let mut stmt = conn.prepare_cached(include_str!("sql/message_meta.sql"))?;
    Ok(w!(stmt.query_row_named(
        named_params! { "@msg_id": msg_id },
        |row| {
            Ok(MessageMeta {
                insertion_time: row.get("insertion_ts")?,
                msg_id: *msg_id,
                match_status: MatchStatus::NotMatched,
            })
        }
    )))
}

/// Get message data by message id
pub(crate) fn message_data(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<MsgData, HErr> {
    let mut stmt = conn.prepare_cached(include_str!("sql/message_data.sql"))?;

    let receipts = get_receipts(conn, msg_id)?;
    let replies = replies(conn, msg_id)?;
    let reactions = reactions(conn, msg_id)?;
    let attachments = crate::message::attachments::db::get(conn, &msg_id)?;

    Ok(w!(stmt.query_row_named(
        named_params! { "@msg_id": msg_id },
        |row| {
            let time = MessageTime {
                insertion: row.get("insertion_ts")?,
                server: row.get("server_ts")?,
                expiration: row.get("expiration_ts")?,
            };

            let is_reply: bool = row.get("is_reply")?;
            let op: Option<MsgId> = row.get("op_msg_id")?;
            let op = (op, is_reply).into();

            Ok(MsgData {
                author: row.get("author")?,
                body: row.get("body")?,
                send_status: row.get("send_status")?,
                op,
                attachments,
                time,
                receipts,
                replies,
                reactions,
            })
        }
    )))
}

/// Get message by message id
pub(crate) fn get_message_opt(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<Option<Message>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/get_message.sql")));

    let mut res = stmt.query_map_named(
        named_params! {
            "@msg_id": msg_id
        },
        |row| {
            let receipts = get_receipts(conn, msg_id)?;
            let replies = self::replies(conn, msg_id)?;
            let attachments = crate::message::attachments::db::get(conn, &msg_id)?;
            let reactions = reactions(conn, msg_id)?;

            let time = MessageTime {
                insertion: row.get("insertion_ts")?,
                server: row.get("server_ts")?,
                expiration: row.get("expiration_ts")?,
            };

            let is_reply: bool = row.get("is_reply")?;
            let op: Option<MsgId> = row.get("op_msg_id")?;

            let op = (op, is_reply).into();

            Ok(Message {
                message_id: row.get("msg_id")?,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                body: row.get("body")?,
                op,
                send_status: row.get("send_status")?,
                attachments,
                time,
                receipts,
                replies,
                reactions,
            })
        },
    )?;

    match res.next() {
        Some(res) => Ok(Some(res?)),
        None => Ok(None),
    }
}

/// Sets the message status of an item in the database
pub(crate) fn update_send_status(
    conn: &Conn,
    msg_id: MsgId,
    status: MessageSendStatus,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("sql/update_send_status.sql"),
        params![status, msg_id],
    ));

    Ok(())
}

pub(crate) fn get_receipts(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<HashMap<UserId, MessageReceiptStatus>, rusqlite::Error> {
    let mut get_stmt = w!(conn.prepare_cached(include_str!("sql/get_receipts.sql")));

    let res = w!(get_stmt.query_map(params![msg_id], |row| Ok((row.get(0)?, row.get(1)?))));
    Ok(w!(res.collect()))
}

pub(crate) fn replies(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<HashSet<MsgId>, rusqlite::Error> {
    let mut get_stmt = w!(conn.prepare_cached(include_str!("sql/replies.sql")));

    let res = w!(
        get_stmt.query_map_named(named_params!("@parent_msg_id": msg_id), |row| {
            Ok(row.get("msg_id")?)
        })
    );

    Ok(w!(res.collect()))
}

pub(crate) fn reactions(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<Option<Reactions>, rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/reactions.sql")));

    let res = w!(
        stmt.query_map_named(named_params!("@msg_id": msg_id), |row| {
            let reactionary = row.get("reactionary")?;
            let react_content = row.get("react_content")?;
            let time = row.get("insertion_ts")?;

            Ok(Reaction {
                reactionary,
                time,
                react_content,
            })
        })
    );

    res.collect::<Result<Vec<_>, _>>().map(Reactions::from_vec)
}

pub(crate) fn add_reaction(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
    reactionary: &UserId,
    react_content: &str,
) -> Result<(), rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/add_reaction.sql")));

    stmt.execute_named(named_params!(
        "@msg_id": msg_id,
        "@reactionary": reactionary,
        "@react_content": react_content,
        "@insertion_ts": Time::now()
    ))?;

    Ok(())
}

pub(crate) fn remove_reaction(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
    reactionary: &UserId,
    react_content: &str,
) -> Result<(), rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/remove_reaction.sql")));

    stmt.execute_named(named_params!(
        "@msg_id": msg_id,
        "@reactionary": reactionary,
        "@react_content": react_content,
    ))?;

    Ok(())
}

pub(crate) fn add_receipt(
    conn: &Conn,
    msg_id: MsgId,
    recip: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/add_receipt.sql")));
    w!(stmt.execute(params![msg_id, recip, receipt_status]));
    Ok(())
}

/// Gets messages by `MessageSendStatus`
pub(crate) fn by_send_status(
    conn: &Conn,
    send_status: MessageSendStatus,
) -> Result<Vec<Message>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/by_send_status.sql")));
    let res = w!(
        stmt.query_map_named(named_params! { "@send_status": send_status }, |row| {
            let message_id = row.get("msg_id")?;
            let receipts = get_receipts(conn, &message_id)?;
            let replies = self::replies(conn, &message_id)?;
            let attachments = crate::message::attachments::db::get(conn, &message_id)?;
            let reactions = reactions(conn, &message_id)?;

            let time = MessageTime {
                insertion: row.get("insertion_ts")?,
                server: row.get("server_ts")?,
                expiration: row.get("expiration_ts")?,
            };

            let is_reply: bool = row.get("is_reply")?;
            let op: Option<MsgId> = row.get("op_msg_id")?;

            let op = (op, is_reply).into();

            Ok(Message {
                message_id,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                body: row.get("body")?,
                op,
                send_status: row.get("send_status")?,
                attachments,
                time,
                receipts,
                replies,
                reactions,
            })
        })
    );

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Deletes a message
pub(crate) fn delete_message(
    conn: &Conn,
    id: &MsgId,
) -> Result<(), HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/delete_message.sql")));
    w!(stmt.execute_named(named_params! { "@msg_id": id }));
    super::attachments::db::gc(conn)?;
    Ok(())
}

/// Testing utility
#[cfg(test)]
pub(crate) fn test_outbound_text(
    db: &mut Conn,
    msg: &str,
    conv: ConversationId,
) -> (MsgId, Time) {
    use std::convert::TryInto;

    let mut builder = OutboundMessageBuilder::default();

    builder.conversation_id(conv).body(
        msg.try_into()
            .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!())),
    );
    let out = builder
        .store_and_send_blocking_db(db)
        .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!()));

    (out.message_id, out.time.insertion)
}

impl OutboundMessageBuilder {
    pub(crate) fn store_and_send_db<F: FnMut(StoreAndSend) + Send + 'static>(
        self,
        db: &mut Conn,
        mut callback: F,
    ) {
        macro_rules! e {
            ($res: expr) => {
                match $res {
                    Ok(val) => val,
                    Err(e) => {
                        callback(StoreAndSend::Error {
                            error: e.into(),
                            location: loc!(),
                        });
                        return;
                    }
                }
            };
        }

        let Self {
            conversation,
            body,
            op,
            attachments,
        } = self;

        use MissingOutboundMessageField::*;

        if attachments.is_empty() && body.is_none() {
            return e!(Err(MissingBody));
        }

        let conversation_id = e!(conversation.ok_or(MissingConversationId));
        let msg_id = MsgId::gen_new();
        let timestamp = Time::now();
        let author = e!(crate::config::db::id(&db));
        let expiration_period = e!(expiration_period(&db, &conversation_id));

        let expiration = match expiration_period.into_millis() {
            Some(period) => Some(timestamp + period),
            None => None,
        };

        let send_status = MessageSendStatus::NoAck;

        let time = MessageTime {
            server: None,
            expiration,
            insertion: timestamp,
        };

        let msg = Message {
            message_id: msg_id,
            author,
            body: (&body).clone(),
            op: op.into(),
            conversation: conversation_id,
            time,
            send_status,
            attachments: vec![].into(),
            receipts: HashMap::new(),
            replies: HashSet::new(),
            reactions: None,
        };

        callback(StoreAndSend::Msg(Box::new(msg)));

        let attachments: Result<Vec<Attachment>, HErr> = attachments
            .into_iter()
            .map(|path| {
                let attach: Attachment = Attachment::new(&path)?;

                attach.save()?;

                Ok(attach)
            })
            .collect();

        let attachments = e!(attachments);

        let tx = e!(db.transaction());

        e!(tx.execute_named(
            include_str!("sql/add.sql"),
            named_params![
                "@msg_id": msg_id,
                "@author": author,
                "@conversation_id": conversation_id,
                "@body": body,
                "@send_status": send_status,
                "@insertion_ts": time.insertion,
                "@server_ts": time.server,
                "@expiration_ts": time.expiration,
                "@is_reply": op.is_some()
            ],
        ));

        e!(tx.execute(
            include_str!("../conversation/sql/update_last_active.sql"),
            params![timestamp, conversation_id],
        ));

        if let Some(op) = op {
            e!(tx.execute_named(
                include_str!("sql/add_reply.sql"),
                named_params! { "@msg_id": msg_id, "@op": op }
            ));
        }

        let attachment_meta = if !attachments.is_empty() {
            e!(attachments::db::add(
                &tx,
                &msg_id,
                attachments.iter().map(Attachment::hash_dir)
            ))
        } else {
            Default::default()
        };

        e!(tx.commit());

        callback(StoreAndSend::StoreDone(msg_id, attachment_meta));

        let content = cmessages::Message {
            body,
            attachments,
            expiration,
        };

        let msg = cmessages::Msg {
            mid: msg_id,
            content,
            op,
        };

        e!(crate::network::send_normal_message(conversation_id, msg));

        callback(StoreAndSend::SendDone(msg_id));
    }

    #[cfg(test)]
    pub(crate) fn store_and_send_blocking_db(
        self,
        db: &mut Conn,
    ) -> Result<Message, HErr> {
        use crate::channel_recv_err;
        use crossbeam_channel::*;

        let (tx, rx) = unbounded();
        self.store_and_send_db(db, move |m| {
            tx.send(m).unwrap_or_else(|_| panic!("Send error"));
        });

        let out = match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::Msg(msg) => msg,
            StoreAndSend::Error { error, .. } => return Err(error),
            other => {
                panic!("Unexpected  variant {:?}", other);
            }
        };

        match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::StoreDone(..) => {}
            other => {
                panic!("Unexpected variant {:?}", other);
            }
        }

        match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::SendDone(_) => Ok(*out),
            other => {
                panic!("Unexpected variant {:?}", other);
            }
        }
    }
}

impl InboundMessageBuilder {
    pub(crate) fn store_db(
        self,
        conn: &mut rusqlite::Connection,
    ) -> Result<Option<Message>, HErr> {
        let Self {
            message_id,
            author,
            conversation,
            body,
            server_timestamp,
            expiration,
            op,
            attachments,
        } = self;

        use MissingInboundMessageField::*;

        #[cfg(not(test))]
        {
            if let Some(expiration) = expiration {
                // short circuit if message has already expired
                if expiration < Time::now() {
                    return Ok(None);
                }
            }
        }

        let conversation_id = w!(conversation.ok_or(MissingConversationId));
        let msg_id = w!(message_id.ok_or(MissingMessageId));
        let server_timestamp = w!(server_timestamp.ok_or(MissingTimestamp));
        let author = w!(author.ok_or(MissingAuthor));

        let res: Result<Vec<String>, HErr> = attachments
            .into_iter()
            .map(|a| Ok(a.save()?.into()))
            .collect();

        let attachment_paths = res?;

        // this can be inferred from the fact that this message was received
        let send_status = MessageSendStatus::Ack;

        let time = MessageTime {
            insertion: Time::now(),
            server: Some(server_timestamp),
            expiration,
        };

        let mut tx = w!(conn.transaction());

        w!(tx.execute_named(
            include_str!("sql/add.sql"),
            named_params! {
                "@msg_id": msg_id,
                "@author": author,
                "@conversation_id": conversation_id,
                "@body": body,
                "@send_status": send_status,
                "@insertion_ts": time.insertion,
                "@server_ts": time.server,
                "@expiration_ts": time.expiration,
                "@is_reply": op.is_some()
            },
        ));

        w!(tx.execute(
            include_str!("../conversation/sql/update_last_active.sql"),
            params![Time::now(), conversation_id],
        ));

        let op = if let Some(op) = op {
            // what if you receive a reply to message you don't have?
            let sp = w!(tx.savepoint());

            // this succeeds in the happy case
            let res = sp.execute(include_str!("sql/add_reply.sql"), params![msg_id, op]);

            // and if it doesn't try making it a dangling reply
            match res {
                Ok(_) => {
                    w!(sp.commit());
                    ReplyId::Known(op)
                }
                Err(rusqlite::Error::SqliteFailure(..)) => {
                    let none_msg_id: Option<MsgId> = None;
                    w!(sp.execute(
                        include_str!("sql/add_reply.sql"),
                        params![msg_id, none_msg_id],
                    ));
                    w!(sp.commit());
                    ReplyId::Dangling
                }
                Err(e) => return Err(e.into()),
            }
        } else {
            ReplyId::None
        };

        if !attachment_paths.is_empty() {
            attachments::db::add(&tx, &msg_id, attachment_paths.iter().map(|s| s.as_str()))?;
        }

        w!(tx.commit());

        let receipts = get_receipts(&conn, &msg_id).unwrap_or_default();
        let replies = self::replies(&conn, &msg_id).unwrap_or_default();
        let reactions = reactions(&conn, &msg_id).unwrap_or_default();

        let attachments = crate::message::attachments::db::get(&conn, &msg_id).unwrap_or_default();

        Ok(Some(Message {
            message_id: msg_id,
            author,
            body,
            attachments,
            conversation: conversation_id,
            send_status: MessageSendStatus::Ack,
            op,
            time,
            receipts,
            replies,
            reactions,
        }))
    }
}
