use crate::{ffi, interface::*, ret_err};
use herald_common::{Time, UserId};
use heraldcore::{
    message::{get_message_opt, Message},
    types::{MessageBody, MsgId},
};
use std::convert::TryInto;

type Emitter = MessagePreviewEmitter;

/// Message preview
pub struct MessagePreview {
    emit: Emitter,
    msg_id: Option<MsgId>,
    body: Option<MessageBody>,
    author: Option<UserId>,
    time: Option<Time>,
    is_dangling: bool,
}

impl MessagePreviewTrait for MessagePreview {
    fn new(emit: Emitter) -> Self {
        Self {
            emit,
            msg_id: None,
            body: None,
            author: None,
            time: None,
            is_dangling: true,
        }
    }

    fn emit(&mut self) -> &mut MessagePreviewEmitter {
        &mut self.emit
    }

    fn author(&self) -> Option<ffi::UserIdRef> {
        Some(self.author.as_ref()?.as_str())
    }

    fn body(&self) -> Option<&str> {
        Some(self.body.as_ref()?.as_str())
    }

    fn epoch_timestamp_ms(&self) -> Option<i64> {
        Some(self.time?.0)
    }

    fn is_dangling(&self) -> bool {
        self.is_dangling
    }

    fn message_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.msg_id.as_ref()?.as_slice())
    }

    fn set_message_id(&mut self, mid: Option<ffi::MsgIdRef>) {
        if let (Some(mid), None) = (mid, self.msg_id) {
            let mid = ret_err!(mid.try_into());
            self.msg_id.replace(mid);

            if let Some(Message {
                time, body, author, ..
            }) = ret_err!(get_message_opt(&mid))
            {
                self.time.replace(time.insertion);
                self.author.replace(author);
                self.body = body;

                self.emit.message_id_changed();
                self.emit.msg_id_set_changed();
                self.emit.body_changed();
                self.emit.author_changed();
                self.emit.epoch_timestamp_ms_changed();
            }
        }
    }

    fn msg_id_set(&self) -> bool {
        self.msg_id.is_some()
    }
}
