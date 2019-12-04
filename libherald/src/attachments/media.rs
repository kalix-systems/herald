use crate::interface::{
    MediaAttachmentsEmitter as Emitter, MediaAttachmentsList as List,
    MediaAttachmentsTrait as Interface,
};
use crate::ret_none;
use std::path::Path;

pub(super) const IMG_EXT: [&str; 10] = [
    "BMP", "GIF", "JPG", "JPEG", "PNG", "PBM", "PGM", "PPM", "XBM", "XPM",
];

pub(super) fn is_media(path: &Path) -> bool {
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
}
