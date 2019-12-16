use crate::interface::{
    DocumentAttachmentsEmitter as Emitter, DocumentAttachmentsList as List,
    DocumentAttachmentsTrait as Interface,
};
use crate::{err, none};
use std::{ffi::OsStr, path::PathBuf, str::FromStr};

/// Document attachments
pub struct DocumentAttachments {
    emit: Emitter,
    model: List,
    contents: Vec<PathBuf>,
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

    fn document_attachment_name(
        &self,
        index: usize,
    ) -> String {
        none!(
            self.contents
                .get(index)
                .and_then(|p| p.file_name())
                .and_then(OsStr::to_str)
                .map(String::from_str),
            "".to_owned()
        )
    }

    fn document_attachment_size(
        &self,
        index: usize,
    ) -> u64 {
        let path = none!(self.contents.get(index), 0);
        err!(std::fs::metadata(&path), 0).len()
    }
}

impl DocumentAttachments {
    pub(crate) fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub(crate) fn add_attachment(
        &mut self,
        path: std::path::PathBuf,
    ) -> Option<()> {
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
        self.model.begin_reset_model();
        let all = std::mem::replace(&mut self.contents, Vec::new());
        self.model.end_reset_model();
        all.into_iter()
            .map(|p| p.into_os_string().to_string_lossy().to_string())
            .collect()
    }
}
