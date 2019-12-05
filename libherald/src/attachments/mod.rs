use crate::{
    ffi,
    interface::{AttachmentsEmitter as Emitter, AttachmentsList as List, AttachmentsTrait},
    ret_err, spawn,
};
use crossbeam_channel::{bounded, Receiver};
use heraldcore::{channel_send_err, message::attachments, types::MsgId};
use std::{convert::TryInto, ops::Not, path::PathBuf};

mod documents;
mod media;
pub use documents::DocumentAttachments;
pub(crate) use media::is_media;
pub use media::MediaAttachments;

type Contents = Vec<PathBuf>;

/// Attachments list model
pub struct Attachments {
    msg_id: Option<MsgId>,
    emit: Emitter,
    document_attachments: DocumentAttachments,
    media_attachments: MediaAttachments,
    rx: Option<Receiver<Contents>>,
    loaded: bool,
}

impl AttachmentsTrait for Attachments {
    fn new(
        emit: Emitter,
        _: List,
        document_attachments: DocumentAttachments,
        media_attachments: MediaAttachments,
    ) -> Self {
        Self {
            emit,
            msg_id: None,
            rx: None,
            document_attachments,
            media_attachments,
            loaded: false,
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn attachments_msg_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.msg_id.as_ref()?.as_slice())
    }

    fn set_attachments_msg_id(
        &mut self,
        msg_id: Option<ffi::MsgIdRef>,
    ) {
        if let (Some(msg_id), None) = (msg_id, self.msg_id) {
            let msg_id = ret_err!(msg_id.try_into());

            self.msg_id = Some(msg_id);
            self.emit.attachments_msg_id_changed();

            let (tx, rx) = bounded(1);
            self.rx.replace(rx);

            let mut emit = self.emit().clone();
            spawn!({
                let attachments = ret_err!(attachments::get(&msg_id));
                let attachment_strings = ret_err!(attachments.into_flat());

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
            if let Ok(contents) = rx.recv() {
                let mut media = Vec::new();
                let mut doc = Vec::new();

                for path in contents {
                    if media::is_media(&path) {
                        if let Ok(path) = path.into_os_string().into_string() {
                            media.push(path);
                        }
                    } else if let Ok(path) = path.into_os_string().into_string() {
                        doc.push(path);
                    }
                }

                self.media_attachments.fill(media);
                self.document_attachments.fill(doc);
                self.loaded = true;
                self.emit.loaded_changed();
            }
        }
    }

    fn document_attachments(&self) -> &DocumentAttachments {
        &self.document_attachments
    }

    fn document_attachments_mut(&mut self) -> &mut DocumentAttachments {
        &mut self.document_attachments
    }

    fn media_attachments(&self) -> &MediaAttachments {
        &self.media_attachments
    }

    fn media_attachments_mut(&mut self) -> &mut MediaAttachments {
        &mut self.media_attachments
    }

    fn row_count(&self) -> usize {
        0
    }

    fn loaded(&self) -> bool {
        self.loaded
    }
}
