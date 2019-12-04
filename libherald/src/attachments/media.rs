use crate::interface::{
    MediaAttachmentsEmitter as Emitter, MediaAttachmentsList as List,
    MediaAttachmentsTrait as Interface,
};
use crate::ret_none;
use std::path::Path;

pub(super) const IMG_EXT: [&str; 10] = [
    "BMP", "GIF", "JPG", "JPEG", "PNG", "PBM", "PGM", "PPM", "XBM", "XPM",
];

pub(crate) fn is_media(path: &Path) -> bool {
    fn get_extension(path: &Path) -> Option<&str> {
        path.extension()?.to_str()
    }

    get_extension(path)
        .map(|ext| {
            IMG_EXT
                .iter()
                .any(|img_ext| img_ext.eq_ignore_ascii_case(ext))
        })
        .unwrap_or(false)
}

/// Media attachments
pub struct MediaAttachments {
    emit: Emitter,
    model: List,
    contents: Vec<String>,
}

impl Interface for MediaAttachments {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Self {
        Self {
            emit,
            model,
            contents: Vec::new(),
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.contents.len()
    }

    fn media_attachment_one(&self) -> Option<&str> {
        self.contents.get(0).map(String::as_str)
    }

    fn media_attachment_two(&self) -> Option<&str> {
        self.contents.get(1).map(String::as_str)
    }

    fn media_attachment_three(&self) -> Option<&str> {
        self.contents.get(2).map(String::as_str)
    }

    fn media_attachment_four(&self) -> Option<&str> {
        self.contents.get(3).map(String::as_str)
    }

    fn media_attachment_path(
        &self,
        index: usize,
    ) -> &str {
        ret_none!(self.contents.get(index), "")
    }
}

impl MediaAttachments {
    pub(super) fn fill(
        &mut self,
        media: Vec<String>,
    ) {
        self.model.begin_reset_model();
        self.contents = media;
        self.model.end_reset_model();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub(crate) fn add_attachment(
        &mut self,
        path: std::path::PathBuf,
    ) -> Option<()> {
        let path = path.into_os_string().into_string().ok()?;

        self.model
            .begin_insert_rows(self.contents.len(), self.contents.len());
        self.contents.push(path);
        self.model.end_insert_rows();

        Some(())
    }

    pub(crate) fn remove(
        &mut self,
        index: usize,
    ) -> Option<()> {
        if index >= self.contents.len() {
            return None;
        }

        self.model.begin_remove_rows(index, index);
        self.contents.remove(index);
        self.model.end_remove_rows();

        Some(())
    }
}
