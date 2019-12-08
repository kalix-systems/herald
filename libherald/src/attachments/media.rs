use crate::interface::{
    MediaAttachmentsEmitter as Emitter, MediaAttachmentsList as List,
    MediaAttachmentsTrait as Interface,
};

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
        self.contents.get(index).map(String::as_str).unwrap_or("")
    }
}

impl MediaAttachments {
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

    pub(crate) fn all(&mut self) -> Vec<std::path::PathBuf> {
        self.model.begin_reset_model();
        let all = std::mem::replace(&mut self.contents, Vec::new())
            .into_iter()
            .map(std::path::PathBuf::from)
            .collect();
        self.model.end_reset_model();
        all
    }
}
