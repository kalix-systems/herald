use crate::interface::{
    DocumentAttachmentsEmitter as Emitter, DocumentAttachmentsList as List,
    DocumentAttachmentsTrait as Interface,
};
use crate::{ret_err, ret_none};

/// Document attachments
pub struct DocumentAttachments {
    emit: Emitter,
    model: List,
    contents: Vec<String>,
}

impl Interface for DocumentAttachments {
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

    fn document_attachment_path(
        &self,
        index: usize,
    ) -> &str {
        ret_none!(self.contents.get(index), "")
    }

    fn document_attachment_size(
        &self,
        index: usize,
    ) -> u64 {
        let path = ret_none!(self.contents.get(index), 0);
        ret_err!(std::fs::metadata(&path), 0).len()
    }
}

impl DocumentAttachments {
    pub(super) fn fill(
        &mut self,
        docs: Vec<String>,
    ) {
        self.model.begin_reset_model();
        self.contents = docs;
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
