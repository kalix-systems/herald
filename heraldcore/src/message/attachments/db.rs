use super::*;
use crate::w;
use platform_dirs::attachments_dir;
use rusqlite::{Connection as Conn, NO_PARAMS};

pub(crate) fn add<'a, A: Iterator<Item = &'a str>>(
    conn: &Conn,
    msg_id: &MsgId,
    attachments: A,
) -> Result<AttachmentMeta, rusqlite::Error> {
    let mut stmt = w!(conn.prepare(include_str!("sql/add_attachment.sql")));

    let mut out: Vec<String> = Vec::new();

    for (ix, hash_dir) in attachments.enumerate() {
        let ix = ix as i64;
        w!(stmt.execute(params![msg_id, ix, hash_dir]));
        out.push(hash_dir.to_owned());
    }

    Ok(out.into())
}

/// Gets all attachments associated with a message id
pub(crate) fn get(
    conn: &Conn,
    msg_id: &MsgId,
) -> Result<AttachmentMeta, rusqlite::Error> {
    let mut stmt = w!(conn.prepare(include_str!("sql/get_attachments.sql")));

    let attachments: Result<Vec<String>, rusqlite::Error> = stmt
        .query_map(params![msg_id], |row| row.get::<_, String>(0))?
        .map(|path_string| Ok(path_string?))
        .collect();

    Ok(AttachmentMeta::new(w!(attachments)))
}

/// Deletes all attachments uniquely associated with a message id
pub(crate) fn gc(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    let mut stmt = w!(conn.prepare(include_str!("sql/get_dangling.sql")));

    let results = w!(stmt.query_map(NO_PARAMS, |row| row.get::<_, String>("hash_dir")));

    for res in results {
        if let Ok(path) = res {
            drop(std::fs::remove_dir_all(attachments_dir().join(path)));
        }
    }

    let mut stmt = w!(conn.prepare(include_str!("sql/gc.sql")));
    w!(stmt.execute(NO_PARAMS));

    Ok(())
}
