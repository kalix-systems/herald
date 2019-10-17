use crate::{ffi, interface::*, ret_err, ret_none};
use heraldcore::message::*;
use std::convert::TryInto;
use std::path::PathBuf;

/// Message builder, used for interactively composing messages
pub struct MessageBuilder {
    emit: Emitter,
    model: List,
    inner: Option<OutboundMessageBuilder>,
}

type Emitter = MessageBuilderEmitter;
type List = MessageBuilderList;

impl MessageBuilderTrait for MessageBuilder {
    fn new(emit: MessageBuilderEmitter, model: MessageBuilderList) -> Self {
        Self {
            emit,
            model,
            inner: Some(OutboundMessageBuilder::default()),
        }
    }

    fn emit(&mut self) -> &mut MessageBuilderEmitter {
        &mut self.emit
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        Some(self.inner.as_ref()?.conversation.as_ref()?.as_slice())
    }

    fn set_conversation_id(&mut self, cid: Option<ffi::ConversationIdRef>) {
        let cid = ret_err!(ret_none!(cid).try_into());
        ret_none!(&mut self.inner).conversation_id(cid);
    }

    fn replying_to(&self) -> Option<ffi::MsgIdRef> {
        Some(self.inner.as_ref()?.op.as_ref()?.as_slice())
    }

    fn set_replying_to(&mut self, op_msg_id: Option<ffi::MsgIdRef>) {
        match op_msg_id {
            Some(op_msg_id) => {
                let op_msg_id = ret_err!(op_msg_id.try_into());
                ret_none!(&mut self.inner).replying_to(Some(op_msg_id));
            }
            None => {
                ret_none!(&mut self.inner).replying_to(None);
            }
        }
    }

    fn add_attachment(&mut self, path: String) -> bool {
        let path = PathBuf::from(path);
        let inner = ret_none!(&mut self.inner, false);
        let len = inner.attachments.len();

        self.model.begin_insert_rows(len, len);
        inner.add_attachment(path);
        self.model.end_insert_rows();

        true
    }

    fn set_body(&mut self, body: Option<String>) {
        let inner = ret_none!(&mut self.inner);
        match body {
            Some(body) => {
                inner.body(ret_err!(body.try_into()));
            }
            None => {
                inner.body = None;
            }
        }
    }

    fn body(&self) -> Option<&str> {
        None
    }

    fn finalize(&mut self) {
        let builder = ret_none!(self.inner.take());
        ret_err!(builder.store_and_send(|_| {}));
    }

    fn remove_attachment(&mut self, path: String) -> bool {
        let path = PathBuf::from(path);
        let inner = ret_none!(&mut self.inner, false);
        let pos = ret_none!(inner.attachments.iter().rposition(|p| p == &path), false);

        self.model.begin_remove_rows(pos, pos);
        inner.attachments.remove(pos);
        self.model.end_remove_rows();
        true
    }

    fn remove_attachment_by_index(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;
        let inner = ret_none!(&mut self.inner, false);

        if row_index < inner.attachments.len() {
            return false;
        }

        self.model.begin_remove_rows(row_index, row_index);
        inner.attachments.remove(row_index);
        self.model.end_remove_rows();

        true
    }

    fn remove_last(&mut self) {
        let inner = ret_none!(&mut self.inner);
        self.model.begin_remove_rows(
            inner.attachments.len().saturating_sub(1),
            inner.attachments.len().saturating_sub(1),
        );
        inner.attachments.pop();
        self.model.end_remove_rows();
    }

    fn row_count(&self) -> usize {
        ret_none!(&self.inner, 0).attachments.len()
    }

    fn attachment_path(&self, index: usize) -> &str {
        let inner = ret_none!(&self.inner, "");
        ret_none! {
            ret_none!(inner.attachments.get(index), "").to_str(),
            ""
        }
    }
}
