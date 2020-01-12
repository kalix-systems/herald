use super::*;

pub(crate) fn reactions(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<Option<Reactions>, rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("../sql/reactions.sql")));

    let res = w!(
        stmt.query_map_named(named_params!("@msg_id": msg_id), |row| {
            let reactionary = row.get("reactionary")?;
            let react_content = row.get("react_content")?;
            let time = row.get("insertion_ts")?;

            Ok(Reaction {
                reactionary,
                time,
                react_content,
            })
        })
    );

    res.collect::<Result<Vec<_>, _>>().map(Reactions::from_vec)
}

pub(crate) fn add_reaction(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
    reactionary: &UserId,
    react_content: &str,
) -> Result<(), rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("../sql/add_reaction.sql")));

    drop(stmt.execute_named(named_params!(
        "@msg_id": msg_id,
        "@reactionary": reactionary,
        "@react_content": react_content,
        "@insertion_ts": Time::now()
    )));

    Ok(())
}

pub(crate) fn remove_reaction(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
    reactionary: &UserId,
    react_content: &str,
) -> Result<(), rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("../sql/remove_reaction.sql")));

    stmt.execute_named(named_params!(
        "@msg_id": msg_id,
        "@reactionary": reactionary,
        "@react_content": react_content,
    ))?;

    Ok(())
}
