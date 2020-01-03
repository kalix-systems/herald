use super::*;
use coremacros::w;
use rusqlite::named_params;

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
#[allow(unused)]
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
    let path = match picture {
        Some(picture) => Some(image_utils::update_picture(picture)?),
        None => None,
    };

    conn.execute(
        include_str!("sql/update_picture.sql"),
        params![path, conversation_id],
    )?;

    Ok(path)
}

/// Sets picture for a conversation given a raw buffer
pub(crate) fn set_picture_buf(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    buf: Option<&[u8]>,
) -> Result<Option<String>, HErr> {
    let path = match buf {
        Some(bytes) => Some(image_utils::update_picture_buf(bytes)?),
        None => None,
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

pub(crate) fn get_all_pairwise_conversations(
    conn: &rusqlite::Connection
) -> Result<Vec<ConversationId>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/all_pairwise_cids.sql")));

    let res = stmt.query_map(NO_PARAMS, |row| {
        row.get::<_, ConversationId>("conversation_id")
    })?;

    res.map(|cid| Ok(w!(cid))).collect()
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

pub(crate) fn update_last_active(
    conn: &rusqlite::Connection,
    time: Time,
    cid: &ConversationId,
) -> Result<(), rusqlite::Error> {
    w!(conn.execute(
        include_str!("sql/update_last_active.sql"),
        params![time, Status::Active, cid],
    ));

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
