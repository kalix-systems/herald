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
    dims: Vec<(u64, u64)>,
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
            dims: Vec::new(),
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

    fn media_attachment_height(
        &self,
        index: usize,
    ) -> u64 {
        let index = index as usize;
        self.dims.get(index).map(|(h, _)| *h).unwrap_or(0)
    }

    fn media_attachment_width(
        &self,
        index: usize,
    ) -> u64 {
        let index = index as usize;
        self.dims.get(index).map(|(_, w)| *w).unwrap_or(0)
    }

    fn set_media_attachment_dims(
        &mut self,
        index: u64,
        height: u64,
        width: u64,
    ) {
        let index = index as usize;
        let dim = ret_none!(self.dims.get_mut(index));
        *dim = (height, width);
        self.model.data_changed(index, index);
    }
}

impl MediaAttachments {
    pub(super) fn fill(
        &mut self,
        media: Vec<String>,
    ) {
        self.model.begin_reset_model();
        self.contents = media;
        self.dims = vec![(0, 0); self.contents.len()];
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

    pub(crate) fn all(&mut self) -> Vec<String> {
        std::mem::replace(&mut self.contents, Vec::new())
    }
}
