use crate::{ffi, interface::*, ret_err, spawn};
use herald_common::{Time, UserId};
use heraldcore::{
    channel_send_err,
    message::{get_message_opt, Message, MessageBody},
    types::MsgId,
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
    has_attachments: bool,
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
            has_attachments: true,
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

    fn has_attachments(&self) -> bool {
        self.has_attachments
    }

    fn message_id(&self) -> Option<ffi::MsgIdRef> {
        Some(self.msg_id.as_ref()?.as_slice())
    }

    fn set_message_id(&mut self, mid: Option<ffi::MsgIdRef>) {
        if let Some(mid) = mid {
            let mid = ret_err!(mid.try_into());
            self.msg_id.replace(mid);

            if let Some(Message {
                time,
                body,
                author,
                has_attachments,
                ..
            }) = get_msg(mid)
            {
                self.is_dangling = false;
                self.has_attachments = has_attachments;
                self.time.replace(time.insertion);
                self.author.replace(author);
                self.body = body;

                self.emit.message_id_changed();
                self.emit.msg_id_set_changed();
                self.emit.body_changed();
                self.emit.author_changed();
                self.emit.epoch_timestamp_ms_changed();
                self.emit.is_dangling_changed();

                if self.has_attachments {
                    self.emit.has_attachments_changed();
                }
            }
        }
    }

    fn msg_id_set(&self) -> bool {
        self.msg_id.is_some()
    }
}

fn get_msg(mid: MsgId) -> Option<Message> {
    let (tx, rx) = crossbeam_channel::bounded(1);

    // This is for exception safety
    spawn!(
        match get_message_opt(&mid) {
            Ok(msg) => {
                ret_err!(tx.send(msg).map_err(|_| channel_send_err!()));
            }
            Err(_) => {
                ret_err!(tx.send(None));
            }
        },
        None
    );

    rx.recv().unwrap_or(None)
}
