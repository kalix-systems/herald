use herald_common::*;
use hex::encode;
use location::{loc, Location};
use platform_dirs::attachments_dir;
use std::{
    ffi::OsString,
    fmt,
    fs::{self, read_dir},
    path::{Path, PathBuf},
};
use tar::{Archive, Builder};

mod errors;
pub use errors::Error;
mod convert;

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
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct AttachmentMeta(Vec<String>);

impl AttachmentMeta {
    pub fn new(paths: Vec<String>) -> Self {
        Self(paths)
    }

    /// Converts `AttachmentMeta` into a vector of `PathBuf`s
    ///
    /// Note: this will ignore empty top level directories.
    pub fn flat(&self) -> Result<Vec<PathBuf>, Error> {
        let mut out = Vec::with_capacity(self.0.len());

        let path = attachments_dir();
        for p in self.0.iter() {
            for entry in read_dir(path.join(p)).map_err(|e| Error::Read(e, loc!()))? {
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

    pub fn media_attachments(
        &self,
        limit: Option<usize>,
    ) -> Result<Media, Error> {
        let mut items = Vec::with_capacity(limit.unwrap_or(8));

        let mut limit = limit.unwrap_or(std::usize::MAX);
        let mut num_more = 0;

        let base = attachments_dir();

        for path in self.0.iter().map(|p| base.join(p)) {
            for entry in read_dir(path).map_err(|e| Error::Read(e, loc!()))? {
                let entry: std::fs::DirEntry = entry.map_err(|e| Error::Read(e, loc!()))?;

                let file = entry.path();

                if is_media(&file) {
                    if limit == 0 {
                        num_more += 1;
                        continue;
                    }

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

                    items.push(MediaMeta {
                        width,
                        height,
                        name,
                        path,
                    });

                    limit -= 1;
                }
            }
        }

        Ok(Media { items, num_more })
    }

    pub fn doc_attachments(
        &self,
        limit: Option<usize>,
    ) -> Result<Docs, Error> {
        let mut items = Vec::with_capacity(limit.unwrap_or(8));

        let mut limit = limit.unwrap_or(std::usize::MAX);
        let mut num_more = 0;

        let base = attachments_dir();
        for path in self.0.iter().map(|p| base.join(p)) {
            for entry in read_dir(path).map_err(|e| Error::Read(e, loc!()))? {
                let entry: std::fs::DirEntry = entry.map_err(|e| Error::Read(e, loc!()))?;

                let file = entry.path();

                if !is_media(&file) {
                    if limit == 0 {
                        num_more += 1;
                        continue;
                    }

                    let size = entry.metadata().map_err(|e| Error::Read(e, loc!()))?.len();
                    let name = entry
                        .file_name()
                        .into_string()
                        .map_err(Error::NonUnicodePath)?;

                    items.push(DocMeta {
                        path: file
                            .into_os_string()
                            .into_string()
                            .map_err(Error::NonUnicodePath)?,
                        name,
                        size,
                    });

                    limit -= 1;
                }
            }
        }

        Ok(Docs { items, num_more })
    }

    /// Indicicates whether `AttachmentMeta` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Saves all attachments to the `dest` directory
    pub fn save_all<P: AsRef<Path>>(
        &self,
        dest: P,
    ) -> Result<(), Error> {
        fs::create_dir_all(&dest).map_err(|e| Error::Write(e, loc!()))?;

        for p in self.flat()? {
            if let Some(dest_tail) = p.file_name() {
                let dest = dest.as_ref().join(dest_tail);

                fs::copy(&p, dest).map_err(|e| Error::Write(e, loc!()))?;
            }
        }

        Ok(())
    }
}

pub struct MediaMeta {
    pub path: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

pub struct Media {
    pub items: Vec<MediaMeta>,
    pub num_more: usize,
}

pub struct Docs {
    pub items: Vec<DocMeta>,
    pub num_more: usize,
}

pub struct DocMeta {
    pub path: String,
    pub name: String,
    pub size: u64,
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
