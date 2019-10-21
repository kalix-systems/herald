use super::*;
use rusqlite::Connection as Conn;
use std::ops::{Deref, DerefMut};

/// Get message read receipts by message id
pub(crate) fn get_message_receipts(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<HashMap<UserId, MessageReceiptStatus>, HErr> {
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;
    let receipts: HashMap<UserId, MessageReceiptStatus> = {
        let data = get_stmt.query_row(params![msg_id], |row| row.get::<_, Vec<u8>>(0))?;
        serde_cbor::from_slice(&data)?
    };
    Ok(receipts)
}

/// Get message by message id
pub(crate) fn get_message(conn: &Conn, msg_id: &MsgId) -> Result<Message, HErr> {
    Ok(conn.query_row(
        include_str!("sql/get_message.sql"),
        params![msg_id],
        Message::from_db,
    )?)
}

/// Sets the message status of an item in the database
pub(crate) fn update_send_status(
    conn: &Conn,
    msg_id: MsgId,
    status: MessageSendStatus,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/update_send_status.sql"),
        params![status, msg_id],
    )?;
    Ok(())
}

pub(crate) fn get_receipts(
    conn: &rusqlite::Connection,
    msg_id: MsgId,
) -> Result<Option<HashMap<UserId, MessageReceiptStatus>>, HErr> {
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;

    let mut res = get_stmt.query(params![msg_id])?;

    match res.next()? {
        Some(row) => match row.get::<_, Option<Vec<u8>>>(0)? {
            Some(data) => Ok(Some(serde_cbor::from_slice(&data)?)),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub(crate) fn add_receipt(
    conn: &mut Conn,
    msg_id: MsgId,
    recip: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let tx = conn.transaction()?;

    // check if message exists yet
    let mut receipts = get_receipts(&tx, msg_id)?;

    match receipts {
        Some(ref mut receipts) => {
            // update receipts

            match receipts.get(&recip) {
                Some(old_status) => {
                    if (*old_status as u8) < (receipt_status as u8) {
                        receipts.insert(recip, receipt_status);
                    }
                }
                None => {
                    receipts.insert(recip, receipt_status);
                }
            }

            receipts.insert(recip, receipt_status);
            let data = serde_cbor::to_vec(&receipts)?;

            let mut put_stmt = tx.prepare(include_str!("sql/set_receipts.sql"))?;
            put_stmt.execute(params![data, msg_id])?;
        }
        None => {
            let mut receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::new();
            receipts.insert(recip, receipt_status);
            let data = serde_cbor::to_vec(&receipts)?;

            let mut put_pending_stmt = tx.prepare(include_str!("sql/add_pending_receipt.sql"))?;
            put_pending_stmt.execute(params![msg_id, data])?;
        }
    }

    tx.commit()?;

    Ok(())
}

/// Gets messages by `MessageSendStatus`
pub fn by_send_status(conn: &Conn, send_status: MessageSendStatus) -> Result<Vec<Message>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/by_send_status.sql"))?;
    let res = stmt.query_map(&[send_status], Message::from_db)?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Deletes a message
pub fn delete_message(conn: &Conn, id: &MsgId) -> Result<(), HErr> {
    conn.execute(include_str!("sql/delete_message.sql"), params![id])?;
    Ok(())
}

impl OutboundMessageBuilder {
    pub(crate) fn store_and_send_db<F: FnMut(StoreAndSend) + Send + 'static, D>(
        self,
        mut db: D,
        mut callback: F,
    ) -> Result<(), HErr>
    where
        D: Deref<Target = Database> + DerefMut + Send + 'static,
    {
        let Self {
            conversation,
            mut body,
            op,
            attachments,
            parse_markdown,
        } = self;

        use MissingOutboundMessageField::*;

        if parse_markdown {
            body = match body {
                Some(body) => Some(body.parse_markdown()?),
                None => None,
            };
        }

        if attachments.is_empty() && body.is_none() {
            return Err(MissingBody.into());
        }

        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id: MsgId = utils::rand_id().into();
        let timestamp = Time::now();
        let author = crate::config::Config::static_id()?;
        let send_status = MessageSendStatus::NoAck;

        let receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::default();
        let receipts_bytes = serde_cbor::to_vec(&receipts)?;
        let has_attachments = !attachments.is_empty();

        let msg = Message {
            message_id: msg_id,
            author,
            body: (&body).clone(),
            op,
            conversation: conversation_id,
            timestamp,
            send_status,
            receipts,
            has_attachments,
        };

        callback(StoreAndSend::Msg(msg));

        macro_rules! e {
            ($res: expr) => {
                match $res {
                    Ok(val) => val,
                    Err(e) => {
                        callback(StoreAndSend::Error {
                            error: e.into(),
                            line_number: line!(),
                        });
                        return;
                    }
                }
            };
        }

        std::thread::Builder::new().spawn(move || {
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

            e!(tx.execute(
                include_str!("sql/add.sql"),
                params![
                    msg_id,
                    author,
                    conversation_id,
                    body,
                    send_status,
                    receipts_bytes,
                    has_attachments,
                    timestamp,
                ],
            ));

            e!(tx.execute(
                include_str!("../conversation/sql/update_last_active.sql"),
                params![timestamp, conversation_id],
            ));

            if let Some(op) = op {
                e!(tx.execute(include_str!("sql/add_reply.sql"), params![msg_id, op]));
            }

            if !attachments.is_empty() {
                e!(attachments::add_db(
                    &tx,
                    &msg_id,
                    attachments.iter().map(|a| a.hash_dir())
                ));
            }

            e!(tx.commit());

            callback(StoreAndSend::StoreDone(msg_id));

            let content = cmessages::Message { body, attachments };
            let msg = cmessages::Msg {
                mid: msg_id,
                content,
                op,
            };
            e!(crate::network::send_normal_message(conversation_id, msg));

            callback(StoreAndSend::SendDone(msg_id));
        })?;

        Ok(())
    }
}

impl InboundMessageBuilder {
    pub(crate) fn store_db(self, conn: &mut rusqlite::Connection) -> Result<(), HErr> {
        let Self {
            message_id,
            author,
            conversation,
            body,
            timestamp,
            op,
            attachments,
        } = self;

        use MissingInboundMessageField::*;

        let conversation_id = conversation.ok_or(MissingConversationId)?;
        let msg_id = message_id.ok_or(MissingMessageId)?;
        let timestamp = timestamp.ok_or(MissingTimestamp)?;
        let author = author.ok_or(MissingAuthor)?;

        let res: Result<Vec<PathBuf>, HErr> = attachments.into_iter().map(|a| a.save()).collect();
        let attachment_paths = res?;
        let has_attachments = !attachment_paths.is_empty();

        // this can be inferred from the fact that this message was received
        let send_status = MessageSendStatus::Ack;

        let tx = conn.transaction()?;

        let receipts = get_receipts(&tx, msg_id)?.unwrap_or_default();
        let receipts = serde_cbor::to_vec(&receipts)?;

        tx.execute(
            include_str!("sql/add.sql"),
            params![
                msg_id,
                author,
                conversation_id,
                body,
                send_status,
                receipts,
                has_attachments,
                timestamp,
            ],
        )?;

        tx.execute(
            include_str!("../conversation/sql/update_last_active.sql"),
            params![Time::now(), conversation_id],
        )?;

        if let Some(op) = op {
            tx.execute(include_str!("sql/add_reply.sql"), params![msg_id, op])?;
        }

        if has_attachments {
            attachments::add_db(&tx, &msg_id, attachment_paths.iter().map(|p| p.as_path()))?;
        }

        tx.commit()?;

        Ok(())
    }
}
