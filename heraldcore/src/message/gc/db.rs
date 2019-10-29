use super::*;
use rusqlite::{named_params, Connection as Conn};

pub(crate) fn get_stale_conversations(conn: &Conn) -> Result<Vec<ConversationId>, HErr> {
    let mut stmt = conn.prepare_cached(include_str!("sql/stale_conversations.sql"))?;

    let mut convs: Vec<ConversationId> = Vec::new();

    let res = stmt.query_map_named(
        named_params! {
            "@time": Time::now(),
        },
        |row| Ok(row.get::<_, ConversationId>("conversation_id")?),
    )?;

    for cid in res {
        convs.push(cid?);
    }

    Ok(convs)
}

pub(crate) fn delete_expired(conn: &Conn) -> Result<(), HErr> {
    let mut stmt = conn.prepare_cached(include_str!("sql/delete_expired.sql"))?;
    stmt.execute_named(named_params! {
        "@time": Time::now()
    })?;
    Ok(())
}
