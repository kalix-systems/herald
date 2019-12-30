use super::*;
use crate::{conversation::db::expiration_period, message::MessageTime};
use coremacros::w;
use herald_attachments::Attachment;
use rusqlite::{named_params, Connection as Conn};
use std::collections::HashSet;

mod builder;
pub(crate) mod receipts;
use receipts::*;
pub(crate) mod reactions;
use reactions::*;
pub(crate) mod replies;
use replies::*;

/// Get message by message id
pub(crate) fn get_message(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<Message, HErr> {
    let receipts = get_receipts(conn, msg_id)?;
    let replies = self::replies(conn, msg_id)?;
    let attachments = crate::message::attachments::db::get(conn, &msg_id)?;
    let reactions = reactions(conn, msg_id)?;

    let mut stmt = conn.prepare_cached(include_str!("../sql/get_message.sql"))?;

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

            let body: Option<MessageBody> = row.get("body")?;
            let update: Option<Update> = row.get("update_item")?;
            let content = Item::from_parts(body, update);

            Ok(Message {
                message_id: row.get("msg_id")?,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                op,
                send_status: row.get("send_status")?,
                attachments,
                time,
                content,
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
    let mut stmt = conn.prepare_cached(include_str!("../sql/message_meta.sql"))?;
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
    let mut stmt = conn.prepare_cached(include_str!("../sql/message_data.sql"))?;

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

            let body: Option<MessageBody> = row.get("body")?;
            let update: Option<Update> = row.get("update_item")?;
            let content = Item::from_parts(body, update);

            Ok(MsgData {
                author: row.get("author")?,
                send_status: row.get("send_status")?,
                op,
                attachments,
                time,
                content,
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
    let mut stmt = w!(conn.prepare(include_str!("../sql/get_message.sql")));

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

            let body: Option<MessageBody> = row.get("body")?;
            let update: Option<Update> = row.get("update_item")?;
            let content = Item::from_parts(body, update);

            Ok(Message {
                message_id: row.get("msg_id")?,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                content,
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
        include_str!("../sql/update_send_status.sql"),
        params![status, msg_id],
    ));

    Ok(())
}

/// Gets messages by `MessageSendStatus`
pub(crate) fn by_send_status(
    conn: &Conn,
    send_status: MessageSendStatus,
) -> Result<Vec<Message>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("../sql/by_send_status.sql")));
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

            let body: Option<MessageBody> = row.get("body")?;
            let update: Option<Update> = row.get("update_item")?;
            let content = Item::from_parts(body, update);

            Ok(Message {
                message_id,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                content,
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
    let mut stmt = w!(conn.prepare(include_str!("../sql/delete_message.sql")));
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
