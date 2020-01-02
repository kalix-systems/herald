use super::*;
use coretypes::messages::PlainItem;

impl OutboundMessageBuilder {
    pub(crate) fn store_and_send_db(
        self,
        db: &mut Conn,
    ) {
        // this is a macro rather than a closure to provide a line number
        macro_rules! e {
            ($res: expr) => {
                match $res {
                    Ok(val) => val,
                    Err(e) => {
                        $crate::err(e);
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
            content: Item::Plain(PlainItem {
                body: body.clone(),
                op: op.into(),
                attachments: vec![].into(),
            }),
            conversation: conversation_id,
            time,
            send_status,
            receipts: HashMap::new(),
            replies: HashSet::new(),
            reactions: None,
        };

        crate::push(StoreAndSend::Msg(conversation_id, Box::new(msg)));

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

        let num_updated = e!(tx.execute_named(
            include_str!("../sql/add.sql"),
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

        // early return on redundant insert
        if num_updated != 1 {
            return;
        }

        e!(tx.execute(
            include_str!("../../conversation/sql/update_last_active.sql"),
            params![timestamp, conversation_id],
        ));

        if let Some(op) = op {
            e!(tx.execute_named(
                include_str!("../sql/add_reply.sql"),
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

        crate::push(StoreAndSend::StoreDone(
            conversation_id,
            msg_id,
            attachment_meta,
        ));

        let content = cmessages::MsgContent::Normal(cmessages::Message {
            body,
            attachments,
            op,
        });

        let msg = cmessages::Msg {
            mid: msg_id,
            content,
            expiration,
        };

        e!(crate::network::send_normal_message(conversation_id, msg));

        crate::push(StoreAndSend::SendDone(conversation_id, msg_id));
    }

    pub(crate) fn store_db(
        self,
        conn: &mut rusqlite::Connection,
    ) -> Result<Message, HErr> {
        let Self {
            conversation,
            body,
            op,
            attachments,
        } = self;

        use MissingOutboundMessageField::*;

        if attachments.is_empty() && body.is_none() {
            return Err(MissingBody.into());
        }

        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id = MsgId::gen_new();
        let timestamp = Time::now();
        let author = crate::config::db::id(&conn)?;
        let expiration_period = expiration_period(&conn, &conversation_id)?;

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

        let attachments: Result<Vec<Attachment>, HErr> = attachments
            .into_iter()
            .map(|path| {
                let attach: Attachment = Attachment::new(&path)?;

                attach.save()?;

                Ok(attach)
            })
            .collect();

        let attachments = attachments?;

        let tx = conn.transaction()?;

        tx.execute_named(
            include_str!("../sql/add.sql"),
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
        )?;

        tx.execute(
            include_str!("../../conversation/sql/update_last_active.sql"),
            params![timestamp, conversation_id],
        )?;

        if let Some(op) = op {
            tx.execute_named(
                include_str!("../sql/add_reply.sql"),
                named_params! { "@msg_id": msg_id, "@op": op },
            )?;
        }

        let attachment_meta = if !attachments.is_empty() {
            attachments::db::add(&tx, &msg_id, attachments.iter().map(Attachment::hash_dir))?
        } else {
            Default::default()
        };

        tx.commit()?;

        Ok(Message {
            message_id: msg_id,
            author,
            content: Item::Plain(PlainItem {
                body,
                attachments: attachment_meta,
                op: op.into(),
            }),
            conversation: conversation_id,
            time,
            send_status,
            receipts: HashMap::new(),
            replies: HashSet::new(),
            reactions: None,
        })
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

        let num_updated = w!(tx.execute_named(
            include_str!("../sql/add.sql"),
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

        // early return on redundant insert
        if num_updated != 1 {
            return Ok(None);
        }

        w!(tx.execute(
            include_str!("../../conversation/sql/update_last_active.sql"),
            params![Time::now(), conversation_id],
        ));

        let op = if let Some(op) = op {
            // what if you receive a reply to message you don't have?
            let sp = w!(tx.savepoint());

            // this succeeds in the happy case
            let res = sp.execute(include_str!("../sql/add_reply.sql"), params![msg_id, op]);

            // and if it doesn't try making it a dangling reply
            match res {
                Ok(_) => {
                    w!(sp.commit());
                    ReplyId::Known(op)
                }
                Err(rusqlite::Error::SqliteFailure(..)) => {
                    let none_msg_id: Option<MsgId> = None;
                    w!(sp.execute(
                        include_str!("../sql/add_reply.sql"),
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
            content: Item::Plain(PlainItem {
                body,
                attachments,
                op,
            }),
            conversation: conversation_id,
            send_status: MessageSendStatus::Ack,
            time,
            receipts,
            replies,
            reactions,
        }))
    }
}
