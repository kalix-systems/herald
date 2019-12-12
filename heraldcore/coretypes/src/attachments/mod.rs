use herald_common::*;
use hex::encode;
use location::{loc, Location};
use platform_dirs::attachments_dir;
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
    Image(image_utils::ImageError),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
            Image(e) => write!(f, "Couldn't read image dimensions: {}", e),
        }
    }
}

impl std::error::Error for Error {}

/// A message attachmentent
#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    data: Vec<u8>,
    hash_dir: String,
}

impl Attachment {
    /// Create attachment from path
    pub fn new<P: AsRef<Path>>(path: &P) -> Result<Self, Error> {
        let buf: Vec<u8> = Vec::new();

        let mut a = Builder::new(buf);

        let file_name = path
            .as_ref()
            .file_name()
            .ok_or_else(|| {
                Error::InvalidPathComponent(path.as_ref().to_path_buf().into_os_string())
            })?
            .to_owned();

        a.append_path_with_name(path, file_name)
            .map_err(|e| Error::Read(e, loc!()))?;

        let data = a.into_inner().map_err(|e| Error::Read(e, loc!()))?;

        let hash = kcl::hash::simple_hash(&data);
        let hash_dir = encode(hash);

        Ok(Attachment { data, hash_dir })
    }

    /// Returns hex encoded hash
    pub fn hash_dir(&self) -> &str {
        &self.hash_dir
    }

    /// Saves file to disk
    pub fn save(&self) -> Result<&str, Error> {
        let mut archive = Archive::new(self.data.as_slice());

        let path = attachments_dir().join(self.hash_dir());

        archive.unpack(&path).map_err(|e| Error::Write(e, loc!()))?;

        Ok(self.hash_dir())
    }
}

/// Attachments
#[derive(Debug, Clone, Default)]
pub struct AttachmentMeta(Vec<String>);

impl From<Vec<String>> for AttachmentMeta {
    fn from(v: Vec<String>) -> AttachmentMeta {
        Self(v)
    }
}

impl AttachmentMeta {
    pub fn new(paths: Vec<String>) -> Self {
        Self(paths)
    }

    /// Converts `AttachmentMeta` into a vector of `PathBuf`s
    ///
    /// Note: this will ignore empty top level directories.
    pub fn flat(&self) -> Result<Vec<PathBuf>, Error> {
        let mut out = Vec::with_capacity(self.0.len());

        for p in self.0.iter() {
            let mut path = attachments_dir();
            path.push(p);

            for entry in read_dir(path).map_err(|e| Error::Read(e, loc!()))? {
                out.push(entry.map_err(|e| Error::Read(e, loc!()))?.path());
            }
        }

        Ok(out)
    }

    /// Converts `AttachmentMeta` into a vector of `String`s
    ///
    /// Note: this will ignore empty top level directories.
    pub fn flat_strings(&self) -> Result<Vec<String>, Error> {
        let mut out = Vec::with_capacity(self.0.len());

        for p in self.0.iter() {
            let mut path = attachments_dir();

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

    pub fn media_attachments(&self) -> Result<Vec<MediaMeta>, Error> {
        let mut out = Vec::with_capacity(self.0.len());

        for p in self.0.as_slice().iter() {
            let mut path = attachments_dir();

            path.push(p);

            for entry in read_dir(path).map_err(|e| Error::Read(e, loc!()))? {
                let entry: std::fs::DirEntry = entry.map_err(|e| Error::Read(e, loc!()))?;

                let file = entry.path();

                if is_media(&file) {
                    let name = entry
                        .file_name()
                        .into_string()
                        .map_err(Error::NonUnicodePath)?;
                    let path = file
                        .into_os_string()
                        .into_string()
                        .map_err(Error::NonUnicodePath)?;

                    let (width, height) =
                        image_utils::image_dimensions(&path).map_err(Error::Image)?;

                    out.push(MediaMeta {
                        width,
                        height,
                        name,
                        path,
                    });
                }
            }
        }

        Ok(out)
    }

    pub fn doc_attachments(&self) -> Result<Vec<DocMeta>, Error> {
        let mut out = Vec::with_capacity(self.0.len());

        for p in self.0.iter() {
            let mut path = attachments_dir();

            path.push(p);

            for entry in read_dir(path).map_err(|e| Error::Read(e, loc!()))? {
                let entry: std::fs::DirEntry = entry.map_err(|e| Error::Read(e, loc!()))?;

                let file = entry.path();

                if !is_media(&file) {
                    let size = entry.metadata().map_err(|e| Error::Read(e, loc!()))?.len();
                    let name = entry
                        .file_name()
                        .into_string()
                        .map_err(Error::NonUnicodePath)?;

                    out.push(DocMeta {
                        path: file
                            .into_os_string()
                            .into_string()
                            .map_err(Error::NonUnicodePath)?,
                        name,
                        size,
                    })
                }
            }
        }

        Ok(out)
    }

    /// Indicicates whether `AttachmentMeta` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct MediaMeta {
    pub path: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

impl From<MediaMeta> for json::JsonValue {
    fn from(meta: MediaMeta) -> json::JsonValue {
        let MediaMeta {
            path,
            width,
            height,
            name,
        } = meta;

        use json::object;

        object! {
            "path" => path,
            "width" => width,
            "height" => height,
            "name" => name,
        }
    }
}

pub struct DocMeta {
    pub path: String,
    pub name: String,
    pub size: u64,
}

impl From<DocMeta> for json::JsonValue {
    fn from(meta: DocMeta) -> json::JsonValue {
        let DocMeta { path, name, size } = meta;

        use json::object;

        object! {
            "path" => path,
            "name" => name,
            "size" => size
        }
    }
}

const IMG_EXT: [&str; 8] = ["BMP", "GIF", "JPG", "JPEG", "PNG", "PGM", "PBM", "PPM"];

pub fn is_media<'a, P: AsRef<Path>>(path: &'a P) -> bool {
    let get_extension = |path: &'a Path| -> Option<&'a str> { path.extension()?.to_str() };

    get_extension(path.as_ref())
        .map(|ext| {
            IMG_EXT
                .iter()
                .any(|img_ext| img_ext.eq_ignore_ascii_case(ext))
        })
        .unwrap_or(false)
}
