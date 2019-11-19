use super::*;
use crate::message::MessageTime;
use rusqlite::named_params;

/// Deletes all messages in a conversation.
pub(crate) fn delete_conversation(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("../message/sql/delete_conversation.sql"),
        &[conversation_id],
    )?;
    Ok(())
}

/// Get all messages in a conversation.
pub(crate) fn conversation_messages(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<Message>, HErr> {
    let mut stmt = conn.prepare(include_str!("../message/sql/conversation_messages.sql"))?;
    let res = stmt.query_map_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| {
            let message_id = row.get("msg_id")?;
            let receipts = crate::message::db::get_receipts(conn, &message_id)?;
            let replies = crate::message::db::replies(conn, &message_id)?;

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
                conversation: *conversation_id,
                body: row.get("body")?,
                op,
                time,
                send_status: row.get("send_status")?,
                has_attachments: row.get("has_attachments")?,
                receipts,
                replies,
            })
        },
    )?;

    let mut messages = Vec::new();
    for msg in res {
        messages.push(msg?);
    }

    Ok(messages)
}

/// Get conversation metadata
pub(crate) fn meta(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<ConversationMeta, HErr> {
    Ok(conn.query_row(
        include_str!("sql/get_conversation_meta.sql"),
        params![conversation_id],
        ConversationMeta::from_db,
    )?)
}

/// Gets expiration period for a conversation
pub(crate) fn expiration_period(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<ExpirationPeriod, HErr> {
    let mut stmt = conn.prepare_cached(include_str!("sql/expiration_period.sql"))?;
    Ok(stmt.query_row_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| row.get("expiration_period"),
    )?)
}

/// Gets expiration period for a conversation
pub(crate) fn picture(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Option<String>, HErr> {
    let mut stmt = conn.prepare_cached(include_str!("sql/picture.sql"))?;

    Ok(stmt.query_row_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| row.get("picture"),
    )?)
}

/// Sets color for a conversation
pub(crate) fn set_color(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    color: u32,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/update_color.sql"),
        params![color, conversation_id],
    )?;
    Ok(())
}

/// Sets muted status of a conversation
pub(crate) fn set_muted(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    muted: bool,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/update_muted.sql"),
        params![muted, conversation_id],
    )?;
    Ok(())
}

/// Sets title for a conversation
pub(crate) fn set_title(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    title: Option<&str>,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/update_title.sql"),
        params![title, conversation_id],
    )?;
    Ok(())
}

/// Sets picture for a conversation
pub(crate) fn set_picture(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    picture: Option<&str>,
) -> Result<Option<String>, HErr> {
    use crate::image_utils;

    let old_picture = self::picture(&conn, conversation_id)?;

    let path = match picture {
        Some(path) => Some(
            image_utils::update_picture(path, old_picture.as_ref().map(String::as_str))?
                .into_os_string()
                .into_string()?,
        ),
        None => {
            if let Some(old) = old_picture {
                std::fs::remove_file(old).ok();
            }
            None
        }
    };

    conn.execute(
        include_str!("sql/update_picture.sql"),
        params![path, conversation_id],
    )?;

    Ok(path)
}

/// Get metadata of all conversations
pub(crate) fn all_meta(conn: &rusqlite::Connection) -> Result<Vec<ConversationMeta>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/all_meta.sql"))?;
    let res = stmt.query_map(NO_PARAMS, ConversationMeta::from_db)?;

    let mut meta = Vec::new();
    for data in res {
        meta.push(data?);
    }

    Ok(meta)
}

pub(crate) fn get_pairwise_conversations(
    conn: &rusqlite::Connection,
    uids: &[UserId],
) -> Result<Vec<ConversationId>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/pairwise_cid.sql"))?;

    uids.iter()
        .map(|uid| stmt.query_row(params![uid], |row| Ok(row.get(0)?)))
        .map(|res| Ok(res?))
        .collect()
}

pub(crate) fn set_expiration_period(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    expiration_period: ExpirationPeriod,
) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/update_expiration_period.sql"))?;

    stmt.execute_named(named_params! {
        "@conversation_id": conversation_id,
        "@expiration_period": expiration_period,
    })?;
    Ok(())
}
