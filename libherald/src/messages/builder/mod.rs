use crate::{
    attachments::{DocumentAttachments, MediaAttachments},
    err, ffi,
    interface::*,
    none, spawn,
};
use herald_attachments::is_media;
use herald_common::{Time, UserId};
use heraldcore::{
    message::*,
    types::{ConversationId, MsgId},
};
use std::{convert::TryInto, path::PathBuf};

mod helper_trait_imp;
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
                    self.inner.body(err!(body.try_into()));
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

    fn expiration_period(&self) -> Option<u8> {
        Some(self.inner.expiration_period? as u8)
    }

    fn set_expiration_period(
        &mut self,
        period: u8,
    ) {
        let period: Option<u8> = period.into();
        self.inner.expiration_period = period.map(heraldcore::conversation::ExpirationPeriod::from);
        self.emit.expiration_period_changed();
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
        let builder = std::mem::take(&mut self.inner);
        self.inner.conversation = builder.conversation;
        self.model.end_reset_model();

        if self.op.take().is_some() {
            self.emit.is_reply_changed();
        }

        self.emit.has_media_attachment_changed();
        self.emit.has_doc_attachment_changed();
        self.emit.body_changed();
        self.emit_op_changed();
        self.emit.expiration_period_changed();

        none!(builder.conversation);

        spawn!({ builder.store_and_send() });
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

    fn op_expiration_time(&self) -> Option<i64> {
        self.op.as_ref()?.expiration.map(Time::into)
    }

    fn op_doc_attachments(&self) -> &str {
        self.op.as_ref().map(Reply::doc).unwrap_or("")
    }

    fn op_media_attachments(&self) -> &str {
        self.op.as_ref().map(Reply::media).unwrap_or("")
    }

    fn op_aux_content(&self) -> &str {
        self.op.as_ref().map(Reply::aux).unwrap_or("")
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

    fn set_op_id(
        &mut self,
        op_msg_id: Option<ffi::MsgIdRef>,
    ) {
        match op_msg_id {
            Some(op_msg_id) => {
                let op_msg_id = err!(op_msg_id.try_into());
                self.update_op_id(&op_msg_id);
            }
            None => {
                self.clear_reply_();
            }
        };
    }
}

impl MessageBuilder {
    pub(super) fn set_conversation_id(
        &mut self,
        cid: ConversationId,
    ) {
        self.inner.conversation_id(cid);
    }
}
