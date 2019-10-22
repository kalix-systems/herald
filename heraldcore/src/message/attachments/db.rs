use super::*;

pub(crate) fn add<'a, A: Iterator<Item = &'a Path>>(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
    attachments: A,
) -> Result<(), HErr> {
    let mut stmt = conn.prepare(include_str!("../sql/add_attachment.sql"))?;
    for (ix, a) in attachments.enumerate() {
        let hash_dir = a.to_str().ok_or(NE!())?;
        let ix = ix as i64;
        stmt.execute(params![msg_id, ix, hash_dir])?;
    }
    Ok(())
}

/// Gets all attachments associated with a message id
pub fn get(conn: &rusqlite::Connection, msg_id: &MsgId) -> Result<AttachmentMeta, HErr> {
    let mut stmt = conn.prepare(include_str!("../sql/get_attachments.sql"))?;

    let attachments: Result<Vec<PathBuf>, HErr> = stmt
        .query_map(params![msg_id], |row| row.get::<_, String>(0))?
        .map(|path_string| Ok(PathBuf::from(path_string?)))
        .collect();

    Ok(AttachmentMeta(attachments?))
}
