use super::*;

pub(crate) fn replies(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<HashSet<MsgId>, rusqlite::Error> {
    let mut get_stmt = w!(conn.prepare_cached(include_str!("../sql/replies.sql")));

    let res = w!(
        get_stmt.query_map_named(named_params!("@parent_msg_id": msg_id), |row| {
            Ok(row.get("msg_id")?)
        })
    );

    Ok(w!(res.collect()))
}
