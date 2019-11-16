use super::*;
use crate::{platform_dirs::ATTACHMENTS_DIR, NE};
use herald_common::hash_slice;
use hex::encode;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use tar::{Archive, Builder};

pub(crate) mod db;

/// A message attachmentent
#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
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

        let path = ATTACHMENTS_DIR.join(self.hash_dir());

        archive.unpack(&path)?;

        Ok(self.hash_dir().to_path_buf())
    }
}

/// Gets all attachments associated with a message id
pub fn get(msg_id: &MsgId) -> Result<AttachmentMeta, HErr> {
    let db = Database::get()?;
    db::get(&db, msg_id)
}

/// Attachments
#[derive(Debug)]
pub struct AttachmentMeta(Vec<PathBuf>);

impl AttachmentMeta {
    /// Converts `AttachmentMeta` into a vector of `String`s
    ///
    /// Note: this will ignore empty top level directories.
    pub fn into_flat_strings(self) -> Result<Vec<String>, HErr> {
        let mut out = Vec::with_capacity(self.0.len());
        for p in self.0 {
            let mut path = ATTACHMENTS_DIR.to_path_buf();
            path.push(p);
            for entry in read_dir(path)? {
                out.push(entry?.path().into_os_string().into_string()?);
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests;
