use herald_common::*;
use hex::encode;
use location::{loc, Location};
use platform_dirs::ATTACHMENTS_DIR;
use std::{
    ffi::OsString,
    fmt,
    fs::read_dir,
    path::{Path, PathBuf},
};
use tar::{Archive, Builder};

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error, Location),
    Write(std::io::Error, Location),
    Hash,
    InvalidPathComponent(OsString),
    NonUnicodePath(OsString),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Read(e, loc) => write!(
                f,
                "Read error processing attachment at {location}:  {error}",
                location = loc,
                error = e
            ),
            Write(e, loc) => write!(
                f,
                "Write error processing attachment at {location}: {error}",
                location = loc,
                error = e,
            ),
            NonUnicodePath(os_str) => write!(
                f,
                "Encountered non-unicode path while converting to Strings, path bytes were: {:x?}",
                os_str
            ),
            InvalidPathComponent(os_str) => write!(
                f,
                "Encountered invalid filename while creating attachment, path bytes were: {:x?}",
                os_str
            ),
            Hash => write!(f, "Couldn't hash attachment data"),
        }
    }
}

impl std::error::Error for Error {}

/// A message attachmentent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    data: Vec<u8>,
    hash_dir: PathBuf,
}

impl Attachment {
    /// Create attachment from path
    pub fn new(path: &Path) -> Result<Self, Error> {
        let buf: Vec<u8> = Vec::new();

        let mut a = Builder::new(buf);

        let file_name = path
            .file_name()
            .ok_or_else(|| Error::InvalidPathComponent(path.to_path_buf().into_os_string()))?
            .to_owned();

        a.append_path_with_name(path, file_name)
            .map_err(|e| Error::Read(e, loc!()))?;

        let data = a.into_inner().map_err(|e| Error::Read(e, loc!()))?;

        let hash = hash_slice(&data).ok_or(Error::Hash)?;
        let hash_dir = PathBuf::from(encode(hash));

        Ok(Attachment { data, hash_dir })
    }

    /// Returns hex encoded hash
    pub fn hash_dir(&self) -> &Path {
        &self.hash_dir
    }

    /// Saves file to disk
    pub fn save(&self) -> Result<PathBuf, Error> {
        let mut archive = Archive::new(self.data.as_slice());

        let path = ATTACHMENTS_DIR.join(self.hash_dir());

        archive.unpack(&path).map_err(|e| Error::Write(e, loc!()))?;

        Ok(self.hash_dir().to_path_buf())
    }
}

/// Attachments
#[derive(Debug)]
pub struct AttachmentMeta(Vec<PathBuf>);

impl AttachmentMeta {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self(paths)
    }

    /// Converts `AttachmentMeta` into a vector of `String`s
    ///
    /// Note: this will ignore empty top level directories.
    pub fn into_flat_strings(self) -> Result<Vec<String>, Error> {
        let mut out = Vec::with_capacity(self.0.len());
        for p in self.0 {
            let mut path = ATTACHMENTS_DIR.to_path_buf();
            path.push(p);

            for entry in read_dir(path).map_err(|e| Error::Read(e, loc!()))? {
                out.push(
                    entry
                        .map_err(|e| Error::Read(e, loc!()))?
                        .path()
                        .into_os_string()
                        .into_string()
                        .map_err(Error::NonUnicodePath)?,
                );
            }
        }

        Ok(out)
    }
}
