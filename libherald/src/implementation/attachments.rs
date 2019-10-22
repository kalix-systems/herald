use crate::{ffi, interface::*, ret_err, ret_none};
use heraldcore::{message::attachments, types::MsgId};
use std::convert::TryInto;

type Emitter = AttachmentsEmitter;
type List = AttachmentsList;

/// Attachments list model
pub struct Attachments {
    msg_id: Option<MsgId>,
    inner: Vec<String>,
    emit: Emitter,
    model: List,
}

impl AttachmentsTrait for Attachments {
    fn new(emit: AttachmentsEmitter, model: AttachmentsList) -> Self {
        Self {
            emit,
            model,
            msg_id: None,
            inner: vec![],
        }
    }

    fn emit(&mut self) -> &mut AttachmentsEmitter {
        &mut self.emit
    }

    fn msg_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.msg_id.as_ref()?.as_slice())
    }

    fn set_msg_id(&mut self, msg_id: Option<ffi::MsgIdRef>) {
        match (msg_id, self.msg_id) {
            (Some(msg_id), None) => {
                let msg_id = ret_err!(msg_id.try_into());

                self.msg_id = Some(msg_id);
                self.emit.msg_id_changed();

                let attachments = ret_err!(attachments::get(&msg_id));
                let attachment_strings = ret_err!(attachments.into_flat_strings());

                if attachment_strings.is_empty() {
                    return;
                }

                self.model
                    .begin_insert_rows(0, attachment_strings.len().saturating_sub(1));
                self.inner = attachment_strings;
                self.model.end_insert_rows();
            }
            _ => {
                return;
            }
        }
    }

    fn row_count(&self) -> usize {
        self.inner.len()
    }

    fn attachment_path(&self, index: usize) -> &str {
        ret_none!(self.inner.get(index), "")
    }
}
