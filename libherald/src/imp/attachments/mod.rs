use crate::{ffi, interface::*, ret_err, ret_none, spawn};
use crossbeam_channel::{bounded, Receiver};
use heraldcore::{channel_send_err, message::attachments, types::MsgId};
use std::{convert::TryInto, ops::Not};

type Emitter = AttachmentsEmitter;
type List = AttachmentsList;

type Contents = Vec<String>;

/// Attachments list model
pub struct Attachments {
    msg_id: Option<MsgId>,
    inner: Contents,
    emit: Emitter,
    model: List,
    rx: Option<Receiver<Contents>>,
}

impl AttachmentsTrait for Attachments {
    fn new(emit: AttachmentsEmitter, model: AttachmentsList) -> Self {
        Self {
            emit,
            model,
            msg_id: None,
            inner: vec![],
            rx: None,
        }
    }

    fn emit(&mut self) -> &mut AttachmentsEmitter {
        &mut self.emit
    }

    fn msg_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.msg_id.as_ref()?.as_slice())
    }

    fn set_msg_id(&mut self, msg_id: Option<ffi::MsgIdRef>) {
        if let (Some(msg_id), None) = (msg_id, self.msg_id) {
            let msg_id = ret_err!(msg_id.try_into());

            self.msg_id = Some(msg_id);
            self.emit.msg_id_changed();

            let (tx, rx) = bounded(1);
            self.rx.replace(rx);

            let mut emit = self.emit().clone();
            spawn!({
                let attachments = ret_err!(attachments::get(&msg_id));
                let attachment_strings = ret_err!(attachments.into_flat_strings());

                if attachment_strings.is_empty() {
                    return;
                }

                ret_err!(tx.send(attachment_strings).map_err(|_| channel_send_err!()));
                emit.new_data_ready();
            });
        }
    }

    fn can_fetch_more(&self) -> bool {
        self.rx
            .as_ref()
            .map(|rx| rx.is_empty().not())
            .unwrap_or(false)
    }

    fn fetch_more(&mut self) {
        if let Some(rx) = self.rx.as_ref() {
            let contents = ret_err!(rx.recv());
            self.model
                .begin_insert_rows(0, contents.len().saturating_sub(1));
            self.inner = contents;
            self.model.end_insert_rows();
        }
    }

    fn row_count(&self) -> usize {
        self.inner.len()
    }

    fn attachment_path(&self, index: usize) -> &str {
        ret_none!(self.inner.get(index), "")
    }
}
