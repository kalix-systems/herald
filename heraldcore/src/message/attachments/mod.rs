use super::*;
use crate::NE;
use herald_common::hash_slice;
use hex::encode;
use std::path::{Path, PathBuf};
use tar::{Archive, Builder};

/// A message attachmentent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    data: Vec<u8>,
    hash_dir: PathBuf,
}

impl Attachment {
    /// Create attachment from path
    pub fn new(path: &Path) -> Result<Self, HErr> {
        let buf: Vec<u8> = Vec::new();

        let mut a = Builder::new(buf);

        let file_name = path.file_name().ok_or(NE!())?.to_owned();
        a.append_path_with_name(path, file_name)?;

        let data = a.into_inner()?;
        let hash = hash_slice(&data).ok_or(NE!())?;
        let hash_dir = PathBuf::from(encode(hash));

        Ok(Attachment { data, hash_dir })
    }

    /// Returns hex encoded hash
    pub fn hash_dir(&self) -> &Path {
        &self.hash_dir
    }

    /// Saves file to disk
    pub fn save(&self) -> Result<PathBuf, HErr> {
        let mut archive = Archive::new(self.data.as_slice());

        let mut path = PathBuf::from("attachments");
        path.push(self.hash_dir());

        archive.unpack(&path)?;

        Ok(path)
    }
}

pub(super) fn add_db<'a, A: Iterator<Item = &'a Path>>(
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
pub fn get(msg_id: &MsgId) -> Result<AttachmentMeta, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("../sql/get_attachments.sql"))?;

    let attachments: Result<Vec<PathBuf>, HErr> = stmt
        .query_map(params![msg_id], |row| row.get::<_, String>(0))?
        .map(|path_string| Ok(PathBuf::from(path_string?)))
        .collect();

    Ok(AttachmentMeta(attachments?))
}

/// Attachments
pub struct AttachmentMeta(Vec<PathBuf>);

impl AttachmentMeta {
    /// Converts `AttachmentMeta` into a vector of `String`s
    pub fn into_vector_of_strings(self) -> Result<Vec<String>, HErr> {
        self.0
            .into_iter()
            .map(|p| {
                let mut path = PathBuf::from("attachments");
                path.push(p);
                path
            })
            .map(|p| Ok(p.into_os_string().into_string()?))
            .collect()
    }
}

#[cfg(test)]
mod tests;
