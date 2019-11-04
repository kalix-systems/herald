use super::*;
use rusqlite::{Connection as Conn, NO_PARAMS};

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
pub(crate) fn gc(conn: &rusqlite::Connection) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_dangling.sql"))?;

    let results = stmt.query_map(NO_PARAMS, |row| row.get::<_, String>("hash_dir"))?;

    for res in results {
        if let Ok(path) = res {
            drop(std::fs::remove_dir_all(ATTACHMENTS_DIR.join(path)));
        }
    }

    let mut stmt = conn.prepare(include_str!("sql/gc.sql"))?;
    stmt.execute(NO_PARAMS)?;

    Ok(())
}