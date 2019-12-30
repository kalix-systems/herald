use super::*;
use coremacros::w;
use rusqlite::{named_params, Connection as Conn};

pub(crate) fn get_stale_conversations(conn: &Conn) -> Result<ConvMessages, HErr> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/stale_conversations.sql")));

    let mut convs: HashMap<ConversationId, Vec<MsgId>> = HashMap::new();

    let results = w!(stmt.query_map_named(
        named_params! {
            "@time": Time::now(),
        },
        |row| {
            Ok((
                row.get::<_, ConversationId>("conversation_id")?,
                row.get::<_, MsgId>("msg_id")?,
            ))
        },
    ));

    for res in results {
        let (cid, mid) = res?;
        convs.entry(cid).or_insert_with(Vec::new).push(mid);
    }

    Ok(convs)
}

pub(crate) fn delete_expired(conn: &Conn) -> Result<(), HErr> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/delete_expired.sql")));
    w!(stmt.execute_named(named_params! {
        "@time": Time::now()
    }));
    Ok(())
}
