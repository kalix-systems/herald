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
}
