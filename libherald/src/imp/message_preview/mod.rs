use crate::{
    ffi,
    interface::{
        MessagePreviewEmitter as Emitter, MessagePreviewList as List,
        MessagePreviewTrait as Interface,
    },
    ret_err, spawn,
};
use herald_common::{Time, UserId};
use heraldcore::{
    channel_send_err,
    message::{get_message_opt, Message, MessageBody},
    types::MsgId,
};
use std::convert::TryInto;

mod imp;
pub(crate) mod shared;

/// Message preview
pub struct MessagePreview {
    emit: Emitter,
    msg_id: Option<MsgId>,
    body: Option<MessageBody>,
    author: Option<UserId>,
    time: Option<Time>,
    is_dangling: bool,
    has_attachments: bool,
}

impl Interface for MessagePreview {
    fn new(
        emit: Emitter,
        _: List,
    ) -> Self {
        Self {
            emit,
            msg_id: None,
            body: None,
            author: None,
            time: None,
            is_dangling: true,
            has_attachments: true,
        }
    }

    fn emit(&mut self) -> &mut Emitter {
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

    fn has_attachments(&self) -> bool {
        self.has_attachments
    }

    fn message_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.msg_id.as_ref()?.as_slice())
    }

    fn set_message_id(
        &mut self,
        mid: Option<ffi::MsgIdRef>,
    ) {
        if let Some(mid) = mid {
            let mid = ret_err!(mid.try_into());

            shared::EMITTERS.insert(mid, self.emit().clone());

            self.msg_id.replace(mid);

            self.fill(mid);
        }
    }

    fn can_fetch_more(&self) -> bool {
        let mid = match self.msg_id {
            Some(mid) => mid,
            None => return false,
        };

        let rx = match shared::RXS.get(&mid) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return false,
        };

        !rx.is_empty()
    }

    fn fetch_more(&mut self) {
        self.fetch_more_();
    }

    fn msg_id_set(&self) -> bool {
        self.msg_id.is_some()
    }

    fn row_count(&self) -> usize {
        0
    }
}
