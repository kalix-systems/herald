use super::*;
use rusqlite::{named_params, Connection as Conn};

pub(crate) fn add<'a, A: Iterator<Item = &'a Path>>(
    conn: &Conn,
    msg_id: &MsgId,
    attachments: A,
) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/add_attachment.sql"))?;
    for (ix, a) in attachments.enumerate() {
        let hash_dir = a.to_str().ok_or(NE!())?;
        let ix = ix as i64;
        stmt.execute(params![msg_id, ix, hash_dir])?;
    }
    Ok(())
}

/// Gets all attachments associated with a message id
pub(crate) fn get(conn: &Conn, msg_id: &MsgId) -> Result<AttachmentMeta, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_attachments.sql"))?;

    let attachments: Result<Vec<PathBuf>, HErr> = stmt
        .query_map(params![msg_id], |row| row.get::<_, String>(0))?
        .map(|path_string| Ok(PathBuf::from(path_string?)))
        .collect();

    Ok(AttachmentMeta(attachments?))
}

/// Deletes all attachments uniquely associated with a message id
pub(crate) fn delete_unique(conn: &rusqlite::Connection, msg_id: &MsgId) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_unique.sql"))?;

    let results = stmt.query_map_named(named_params! {"@msg_id": msg_id}, |row| {
        row.get::<_, String>("hd")
    })?;

    for res in results {
        if let Ok(path) = res {
            std::fs::remove_dir_all(Path::new("attachments").join(path))
                .expect("failed to remove attachment");
        }
    }

    let mut stmt = conn.prepare(include_str!("sql/delete_attachments_by_msg_id.sql"))?;
    stmt.execute_named(named_params! { "@msg_id": msg_id})?;

    Ok(())
}
