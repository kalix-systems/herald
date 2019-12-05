use crate::interface::{
    MediaAttachmentsEmitter as Emitter, MediaAttachmentsList as List,
    MediaAttachmentsTrait as Interface,
};
use crate::ret_none;
use std::path::Path;

pub(super) const IMG_EXT: [&str; 8] = ["BMP", "GIF", "JPG", "JPEG", "PNG", "PGM", "PBM", "PPM"];

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
    contents: Vec<(String, u32, u32)>,
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
        ret_none!(self.contents.get(index), "").0.as_str()
    }

    fn media_attachment_height(
        &self,
        index: usize,
    ) -> u32 {
        let index = index as usize;
        self.contents.get(index).map(|(_, _, h)| *h).unwrap_or(0)
    }

    fn media_attachment_width(
        &self,
        index: usize,
    ) -> u32 {
        let index = index as usize;
        self.contents.get(index).map(|(_, w, _)| *w).unwrap_or(0)
    }
}

impl MediaAttachments {
    pub(super) fn fill(
        &mut self,
        media: Vec<String>,
    ) {
        self.model.begin_reset_model();
        for path in media {
            if let Ok((width, height)) = heraldcore::image_utils::image_dimensions(&path) {
                self.contents.push((path, width, height));
            }
        }
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
        if let Ok((width, height)) = heraldcore::image_utils::image_dimensions(&path) {
            self.contents.push((path, width, height));
        }
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

    pub(crate) fn all(&mut self) -> Vec<std::path::PathBuf> {
        std::mem::replace(&mut self.contents, Vec::new())
            .into_iter()
            .map(|(p, _, _)| std::path::PathBuf::from(p))
            .collect()
    }
}
