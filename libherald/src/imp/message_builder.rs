use crate::{ffi, interface::*, ret_err, ret_none, shared::AddressedBus};
use heraldcore::message::*;
use std::convert::TryInto;
use std::path::PathBuf;

/// Message builder, used for interactively composing messages
pub struct MessageBuilder {
    emit: Emitter,
    model: List,
    inner: OutboundMessageBuilder,
    parse_markdown: bool,
}

type Emitter = MessageBuilderEmitter;
type List = MessageBuilderList;

impl MessageBuilderTrait for MessageBuilder {
    fn new(emit: MessageBuilderEmitter, model: MessageBuilderList) -> Self {
        Self {
            emit,
            model,
            inner: OutboundMessageBuilder::default(),
            parse_markdown: false,
        }
    }

    fn emit(&mut self) -> &mut MessageBuilderEmitter {
        &mut self.emit
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        Some(self.inner.conversation.as_ref()?.as_slice())
    }

    fn set_conversation_id(&mut self, cid: Option<ffi::ConversationIdRef>) {
        if let (None, Some(cid)) = (self.inner.conversation, cid) {
            let cid = ret_err!(cid.try_into());
            self.inner.conversation_id(cid);
        }
    }

    fn replying_to(&self) -> Option<ffi::MsgIdRef> {
        Some(self.inner.op.as_ref()?.as_slice())
    }

    fn is_reply(&self) -> bool {
        self.inner.op.is_some()
    }

    fn is_media_message(&self) -> bool {
        !self.inner.attachments.is_empty()
    }

    fn set_parse_markdown(&mut self, val: bool) {
        self.inner.parse_markdown = val;
    }

    fn parse_markdown(&self) -> bool {
        self.parse_markdown
    }

    fn set_replying_to(&mut self, op_msg_id: Option<ffi::MsgIdRef>) {
        match (op_msg_id, self.inner.op) {
            (Some(op_msg_id), _) => {
                let op_msg_id = ret_err!(op_msg_id.try_into());

                self.inner.replying_to(Some(op_msg_id));
                self.emit.replying_to_changed();
                self.emit.is_reply_changed();
            }
            (None, Some(_)) => {
                self.inner.replying_to(None);
                self.emit.replying_to_changed();
                self.emit.is_reply_changed();
            }
            _ => {}
        }
    }

    fn clear_reply(&mut self) {
        self.set_replying_to(None);
        self.emit.replying_to_changed();
    }

    fn add_attachment(&mut self, path: String) -> bool {
        let path = crate::utils::strip_qrc(path);
        let path = PathBuf::from(path);
        let len = self.inner.attachments.len();

        self.model.begin_insert_rows(len, len);
        self.inner.add_attachment(path);
        self.model.end_insert_rows();

        if len == 0 {
            self.emit.is_media_message_changed();
        }

        true
    }

    fn set_body(&mut self, body: Option<String>) {
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
        let builder = std::mem::replace(&mut self.inner, Default::default());
        self.inner.conversation = builder.conversation;
        self.model.end_reset_model();

        let cid = ret_none!(builder.conversation);

        ret_err!(
            std::thread::Builder::new().spawn(move || builder.store_and_send(move |m| {
                use crate::imp::messages::{shared::MsgUpdate, *};
                use heraldcore::message::StoreAndSend::*;

                match m {
                    Msg(msg) => {
                        ret_err!(Messages::push(cid, MsgUpdate::FullMsg(msg)));
                    }
                    Error { error, line_number } => {
                        // TODO better line number usage
                        eprintln!("Error at {}", line_number);
                        ret_err!(Err(error))
                    }
                    StoreDone(mid) => {
                        ret_err!(Messages::push(cid, MsgUpdate::StoreDone(mid)));
                    }
                    SendDone(_) => {
                        // TODO: send status?
                    }
                }
            }))
        );
    }

    fn remove_attachment(&mut self, path: String) -> bool {
        let path = PathBuf::from(path);
        let pos = ret_none!(
            self.inner.attachments.iter().rposition(|p| p == &path),
            false
        );

        self.model.begin_remove_rows(pos, pos);
        self.inner.attachments.remove(pos);
        self.model.end_remove_rows();

        if self.inner.attachments.is_empty() {
            self.emit.is_media_message_changed();
        }

        true
    }

    fn remove_attachment_by_index(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;

        if row_index > self.inner.attachments.len() {
            return false;
        }

        self.model.begin_remove_rows(row_index, row_index);
        self.inner.attachments.remove(row_index);
        self.model.end_remove_rows();

        if self.inner.attachments.is_empty() {
            self.emit.is_media_message_changed();
        }

        true
    }

    fn remove_last(&mut self) {
        if self.inner.attachments.is_empty() {
            return;
        }
        self.model.begin_remove_rows(
            self.inner.attachments.len().saturating_sub(1),
            self.inner.attachments.len().saturating_sub(1),
        );
        self.inner.attachments.pop();
        self.model.end_remove_rows();

        if self.inner.attachments.is_empty() {
            self.emit.is_media_message_changed();
        }
    }

    fn row_count(&self) -> usize {
        self.inner.attachments.len()
    }

    fn attachment_path(&self, index: usize) -> &str {
        ret_none! {
            ret_none!(self.inner.attachments.get(index), "").to_str(),
            ""
        }
    }
}
