use super::*;
use crate::message::MessageTime;
use crate::w;
use rusqlite::named_params;

/// Deletes all messages in a conversation.
pub(crate) fn delete_conversation(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("../message/sql/delete_conversation.sql"),
        &[conversation_id],
    ));
    Ok(())
}

/// Get all messages in a conversation.
pub(crate) fn conversation_messages(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<Message>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("../message/sql/conversation_messages.sql")));
    let res = w!(stmt.query_map_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| {
            let message_id = row.get("msg_id")?;
            let receipts = crate::message::db::get_receipts(conn, &message_id)?;
            let replies = crate::message::db::replies(conn, &message_id)?;
            let attachments = crate::message::attachments::db::get(conn, &message_id)?;
            let reactions = crate::message::db::reactions(conn, &message_id)?;

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
                attachments,
                receipts,
                replies,
                reactions,
            })
        },
    ));

    let mut messages = Vec::new();
    for msg in res {
        messages.push(w!(msg));
    }

    Ok(messages)
}

pub(crate) fn conversation_message_meta(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<crate::message::MessageMeta>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("../message/sql/conversation_message_meta.sql")));

    let res = w!(stmt.query_map_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| {
            Ok(crate::message::MessageMeta {
                msg_id: row.get("msg_id")?,
                insertion_time: row.get("insertion_ts")?,
                match_status: crate::message::MatchStatus::NotMatched,
            })
        }
    ));

    let mut messages = Vec::new();
    for msg in res {
        messages.push(w!(msg));
    }

    Ok(messages)
}

/// Get all message metadata in a conversation.

/// Get conversation metadata
pub(crate) fn meta(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<ConversationMeta, HErr> {
    Ok(w!(conn.query_row(
        include_str!("sql/get_conversation_meta.sql"),
        params![conversation_id],
        from_db,
    )))
}

/// Gets expiration period for a conversation
pub(crate) fn expiration_period(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<ExpirationPeriod, HErr> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/expiration_period.sql")));
    Ok(w!(stmt.query_row_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| row.get("expiration_period"),
    )))
}

/// Gets picture for a conversation
pub(crate) fn picture(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Option<String>, HErr> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/picture.sql")));

    Ok(w!(stmt.query_row_named(
        named_params! {
            "@conversation_id": conversation_id
        },
        |row| row.get("picture"),
    )))
}

/// Sets color for a conversation
pub(crate) fn set_color(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    color: u32,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("sql/update_color.sql"),
        params![color, conversation_id],
    ));
    Ok(())
}

/// Sets muted status of a conversation
pub(crate) fn set_muted(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    muted: bool,
) -> Result<(), HErr> {
    w!(conn.execute_named(
        include_str!("sql/update_muted.sql"),
        named_params!["@muted": muted, "@conversation_id": conversation_id],
    ));
    Ok(())
}

/// Sets archive status of a conversation
pub(crate) fn set_status(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    status: Status,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("sql/update_status.sql"),
        params![status, conversation_id],
    ));
    Ok(())
}

/// Sets title for a conversation
pub(crate) fn set_title(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    title: Option<&str>,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("sql/update_title.sql"),
        params![title, conversation_id],
    ));
    Ok(())
}

/// Sets picture for a conversation
pub(crate) fn set_picture(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    picture: Option<image_utils::ProfilePicture>,
) -> Result<Option<String>, HErr> {
    let old_picture = self::picture(&conn, conversation_id)?;

    let path = match picture {
        Some(picture) => Some(
            image_utils::update_picture(picture, old_picture.as_ref().map(String::as_str))?
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
    let mut stmt = w!(conn.prepare(include_str!("sql/all_meta.sql")));
    let res = stmt.query_map(NO_PARAMS, from_db)?;

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
    let mut stmt = w!(conn.prepare(include_str!("sql/pairwise_cid.sql")));

    uids.iter()
        .map(|uid| stmt.query_row(params![uid], |row| Ok(w!(row.get(0)))))
        .map(|res| Ok(w!(res)))
        .collect()
}

pub(crate) fn set_expiration_period(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    expiration_period: ExpirationPeriod,
) -> Result<(), HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/update_expiration_period.sql")));

    w!(stmt.execute_named(named_params! {
        "@conversation_id": conversation_id,
        "@expiration_period": expiration_period,
    }));
    Ok(())
}

fn from_db(row: &rusqlite::Row) -> Result<ConversationMeta, rusqlite::Error> {
    Ok(ConversationMeta {
        conversation_id: row.get("conversation_id")?,
        title: row.get("title")?,
        picture: row.get("picture")?,
        color: row.get("color")?,
        muted: row.get("muted")?,
        pairwise: row.get("pairwise")?,
        last_active: row.get("last_active_ts")?,
        expiration_period: row.get("expiration_period")?,
        status: row.get("status")?,
    })
}
