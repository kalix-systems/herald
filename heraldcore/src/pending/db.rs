use super::*;

pub(crate) fn add_to_pending(
    conn: &rusqlite::Connection,
    cid: ConversationId,
    content: &ConversationMessage,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/add_to_pending.sql"),
        params![cid, content],
    )?;
    Ok(())
}

pub(crate) fn get_pending(
    conn: &rusqlite::Connection
) -> Result<Vec<(i64, ConversationId, ConversationMessage)>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_pending.sql"))?;

    let res = stmt.query_map(NO_PARAMS, |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    res.map(|triple| Ok(triple?)).collect()
}

pub(crate) fn remove_pending(
    conn: &rusqlite::Connection,
    tag: i64,
) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/remove_pending.sql"))?;
    stmt.execute(params![tag])?;
    Ok(())
}
