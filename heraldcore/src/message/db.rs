use super::*;
use rusqlite::Connection as Conn;
use std::ops::{Deref, DerefMut};

/// Get message by message id
pub(crate) fn get_message(conn: &Conn, msg_id: &MsgId) -> Result<Message, HErr> {
    let receipts = get_receipts(conn, msg_id)?;

    Ok(conn.query_row(
        include_str!("sql/get_message.sql"),
        params![msg_id],
        |row| {
            Ok(Message {
                message_id: row.get(0)?,
                author: row.get(1)?,
                conversation: row.get(2)?,
                body: row.get(3)?,
                op: row.get(4)?,
                timestamp: row.get(5)?,
                send_status: row.get(6)?,
                has_attachments: row.get(7)?,
                receipts,
            })
        },
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
    msg_id: &MsgId,
) -> Result<HashMap<UserId, MessageReceiptStatus>, rusqlite::Error> {
    let mut get_stmt = conn.prepare(include_str!("sql/get_receipts.sql"))?;

    let res = get_stmt.query_map(params![msg_id], |row| Ok((row.get(0)?, row.get(1)?)))?;
    res.collect()
}

pub(crate) fn add_receipt(
    conn: &Conn,
    msg_id: MsgId,
    recip: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/add_receipt.sql"))?;
    stmt.execute(params![msg_id, recip, receipt_status])?;
    Ok(())
}

/// Gets messages by `MessageSendStatus`
pub(crate) fn by_send_status(
    conn: &Conn,
    send_status: MessageSendStatus,
) -> Result<Vec<Message>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/by_send_status.sql"))?;
    let res = stmt.query_map(&[send_status], |row| {
        let message_id = row.get(0)?;
        let receipts = get_receipts(conn, &message_id)?;

        Ok(Message {
            message_id,
            author: row.get(1)?,
            conversation: row.get(2)?,
            body: row.get(3)?,
            op: row.get(4)?,
            timestamp: row.get(5)?,
            send_status: row.get(6)?,
            has_attachments: row.get(7)?,
            receipts,
        })
    })?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Deletes a message
pub(crate) fn delete_message(conn: &Conn, id: &MsgId) -> Result<(), HErr> {
    conn.execute(include_str!("sql/delete_message.sql"), params![id])?;
    Ok(())
}

/// Testing utility
#[cfg(test)]
pub(crate) fn test_outbound_text<D>(db: D, msg: &str, conv: ConversationId) -> (MsgId, Time)
where
    D: Deref<Target = Database> + DerefMut + Send + 'static,
{
    use std::convert::TryInto;

    let mut builder = OutboundMessageBuilder::default();

    builder.conversation_id(conv).body(
        msg.try_into()
            .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!())),
    );
    let out = builder
        .store_and_send_blocking_db(db)
        .unwrap_or_else(|_| panic!("{}:{}:{}", file!(), line!(), column!()));

    (out.message_id, out.timestamp)
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
        let author = crate::config::db::static_id(&db)?;
        let send_status = MessageSendStatus::NoAck;

        let has_attachments = !attachments.is_empty();

        let msg = Message {
            message_id: msg_id,
            author,
            body: (&body).clone(),
            op,
            conversation: conversation_id,
            timestamp,
            send_status,
            receipts: get_receipts(&db, &msg_id)?,
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
                e!(attachments::db::add(
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

    #[cfg(test)]
    pub(crate) fn store_and_send_blocking_db<D>(self, db: D) -> Result<Message, HErr>
    where
        D: Deref<Target = Database> + DerefMut + Send + 'static,
    {
        use crate::{channel_recv_err, loc};
        use crossbeam_channel::*;

        let (tx, rx) = unbounded();
        self.store_and_send_db(db, move |m| {
            tx.send(m)
                .unwrap_or_else(|_| panic!("Send error at {}", loc!()));
        })?;

        let out = match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::Msg(msg) => msg,
            // TODO use line number
            StoreAndSend::Error { error, .. } => return Err(error),
            other => {
                panic!("Unexpected  variant {:?}", other);
            }
        };

        match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::StoreDone(_) => {}
            other => {
                panic!("Unexpected variant {:?}", other);
            }
        }

        match rx.recv().map_err(|_| channel_recv_err!())? {
            StoreAndSend::SendDone(_) => Ok(out),
            other => {
                panic!("Unexpected variant {:?}", other);
            }
        }
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

        tx.execute(
            include_str!("sql/add.sql"),
            params![
                msg_id,
                author,
                conversation_id,
                body,
                send_status,
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
            attachments::db::add(&tx, &msg_id, attachment_paths.iter().map(|p| p.as_path()))?;
        }

        tx.commit()?;

        Ok(())
    }
}
