use crate::{
    attachments::{DocumentAttachments, MediaAttachments},
    content_push, ffi,
    interface::*,
    push, ret_err, ret_none, spawn,
};
use herald_common::{Time, UserId};
use heraldcore::{
    message::{attachments::is_media, *},
    types::{ConversationId, InvalidRandomIdLength, MsgId},
};
use std::{convert::TryInto, path::PathBuf};

mod imp;
pub(super) use imp::*;

/// Message builder, used for interactively composing messages
pub struct MessageBuilder {
    emit: Emitter,
    model: List,
    inner: OutboundMessageBuilder,
    op: Option<Reply>,
    document_attachments: DocumentAttachments,
    media_attachments: MediaAttachments,
}

type Emitter = MessageBuilderEmitter;
type List = MessageBuilderList;

impl MessageBuilderTrait for MessageBuilder {
    fn new(
        emit: MessageBuilderEmitter,
        model: MessageBuilderList,
        document_attachments: DocumentAttachments,
        media_attachments: MediaAttachments,
    ) -> Self {
        Self {
            emit,
            model,
            inner: OutboundMessageBuilder::default(),
            op: None,
            document_attachments,
            media_attachments,
        }
    }

    fn emit(&mut self) -> &mut MessageBuilderEmitter {
        &mut self.emit
    }

    fn is_reply(&self) -> bool {
        self.op.is_some()
    }

    fn set_body(
        &mut self,
        body: Option<String>,
    ) {
        match body {
            Some(body) => {
                if !body.is_empty() {
                    self.inner.body(ret_err!(body.try_into()));
                } else {
                    self.inner.body = None;
                }
            }
            None => {
                self.inner.body = None;
            }
        }
    }

    fn body(&self) -> Option<&str> {
        Some(self.inner.body.as_ref()?.as_str())
    }

    /// Finalizes the builder, stores and sends the message, and resets the builder.
    fn finalize(&mut self) {
        self.model.begin_reset_model();

        self.inner.attachments.extend(self.media_attachments.all());
        self.inner.attachments.extend(
            self.document_attachments
                .all()
                .into_iter()
                .map(PathBuf::from),
        );

        let builder = std::mem::replace(&mut self.inner, Default::default());
        self.inner.conversation = builder.conversation;
        self.model.end_reset_model();

        self.emit.is_reply_changed();
        self.emit.has_media_attachment_changed();
        self.emit.has_doc_attachment_changed();
        self.emit.body_changed();
        self.emit_op_changed();

        let cid = ret_none!(builder.conversation);

        spawn!({
            builder.store_and_send(move |m| {
                use crate::messages::{MsgUpdate, *};
                use heraldcore::message::StoreAndSend::*;

                match m {
                    Msg(msg) => {
                        ret_err!(content_push(cid, MsgUpdate::BuilderMsg(msg)));
                    }
                    Error { error, location } => {
                        push((Err::<(), HErr>(error), location));
                    }
                    StoreDone(mid, meta) => {
                        ret_err!(content_push(cid, MsgUpdate::StoreDone(mid, meta)));
                    }
                    SendDone(_) => {
                        // TODO: send status?
                    }
                }
            })
        });
    }

    fn row_count(&self) -> usize {
        self.inner.attachments.len()
    }

    fn clear_reply(&mut self) {
        self.clear_reply_();
    }

    fn op_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.inner.op.as_ref()?.as_slice())
    }

    fn op_author(&self) -> Option<ffi::UserIdRef> {
        Some(self.op.as_ref()?.author.as_str())
    }

    fn op_body(&self) -> Option<&str> {
        Some(self.op.as_ref()?.body.as_ref()?.as_str())
    }

    fn op_time(&self) -> Option<i64> {
        Some(self.op.as_ref()?.time.into())
    }

    // TODO
    fn op_doc_attachments(&self) -> Option<&str> {
        None
    }

    fn op_media_attachments(&self) -> Option<&str> {
        None
    }

    fn add_attachment(
        &mut self,
        path: String,
    ) -> bool {
        let path = match crate::utils::strip_qrc(path) {
            Some(path) => path,
            None => return false,
        };

        let path = PathBuf::from(path);

        if is_media(&path) {
            let was_empty = self.media_attachments.is_empty();

            if self.media_attachments.add_attachment(path).is_some() && was_empty {
                self.emit.has_media_attachment_changed();
            }
        } else {
            let was_empty = self.document_attachments.is_empty();

            if self.document_attachments.add_attachment(path).is_some() && was_empty {
                self.emit.has_doc_attachment_changed();
            }
        }

        true
    }

    fn remove_media(
        &mut self,
        index: u64,
    ) -> bool {
        let index = index as usize;
        let was_empty = self.media_attachments.is_empty();

        match self.media_attachments.remove(index) {
            Some(()) => {
                if !was_empty {
                    self.emit.has_media_attachment_changed();
                }
                true
            }
            None => false,
        }
    }

    fn media_attachments(&self) -> &MediaAttachments {
        &self.media_attachments
    }

    fn media_attachments_mut(&mut self) -> &mut MediaAttachments {
        &mut self.media_attachments
    }

    fn remove_doc(
        &mut self,
        index: u64,
    ) -> bool {
        let index = index as usize;
        let was_empty = self.document_attachments.is_empty();

        match self.document_attachments.remove(index) {
            Some(()) => {
                if !was_empty {
                    self.emit.has_doc_attachment_changed();
                }
                true
            }
            None => false,
        }
    }

    fn document_attachments(&self) -> &DocumentAttachments {
        &self.document_attachments
    }

    fn document_attachments_mut(&mut self) -> &mut DocumentAttachments {
        &mut self.document_attachments
    }

    fn has_doc_attachment(&self) -> bool {
        !self.document_attachments.is_empty()
    }

    fn has_media_attachment(&self) -> bool {
        !self.media_attachments.is_empty()
    }
}

impl MessageBuilder {
    pub(super) fn set_conversation_id(
        &mut self,
        cid: ConversationId,
    ) {
        self.inner.conversation_id(cid);
    }

    pub(super) fn set_op_id(
        &mut self,
        op_msg_id: Option<ffi::MsgIdRef>,
        container: &super::Container,
    ) -> Result<OpChanged, InvalidRandomIdLength> {
        match op_msg_id {
            Some(op_msg_id) => {
                let op_msg_id = op_msg_id.try_into()?;
                Ok(self.update_op_id(&op_msg_id, container))
            }
            None => Ok(self.clear_reply_()),
        }
    }
}
