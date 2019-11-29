use super::*;
use crate::messages::{Container, MsgData};

impl MessageBuilder {
    pub(super) fn clear_reply_(&mut self) -> OpChanged {
        match self.op.take() {
            Some(_) => {
                self.op = None;
                self.inner.replying_to(None);
                self.emit_op_changed();
                self.emit.is_reply_changed();
                OpChanged::Changed
            }
            None => OpChanged::NotChanged,
        }
    }

    pub(super) fn update_op_id(
        &mut self,
        op_msg_id: &MsgId,
        container: &Container,
    ) -> OpChanged {
        if let Some(old) = self.inner.op {
            if *op_msg_id == old {
                return OpChanged::NotChanged;
            }
        }

        let reply = match get_reply(&op_msg_id, container) {
            Some(reply) => reply,
            None => return self.clear_reply_(),
        };

        self.inner.replying_to(Some(*op_msg_id));
        self.op.replace(reply);

        self.emit_op_changed();

        self.emit.is_reply_changed();

        OpChanged::Changed
    }

    pub(in crate::messages) fn op_id_slice(&self) -> Option<ffi::MsgIdRef> {
        Some(self.inner.op.as_ref()?.as_slice())
    }

    pub(super) fn emit_op_changed(&mut self) {
        self.emit.op_id_changed();
        self.emit.op_body_changed();
        self.emit.op_author_changed();
        self.emit.op_time_changed();
        self.emit.op_has_attachments_changed();
    }

    pub(in crate::messages) fn try_clear_reply(
        &mut self,
        msg_id: &MsgId,
    ) {
        if let Some(op_id) = &self.inner.op {
            if op_id == msg_id {
                self.clear_reply_();
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct Reply {
    pub(super) time: Time,
    pub(super) body: Option<MessageBody>,
    pub(super) author: UserId,
    pub(super) has_attachments: bool,
}

impl Reply {
    pub(super) fn from_msg_data(data: &MsgData) -> Reply {
        Reply {
            time: data.time.insertion,
            body: data.body.clone(),
            author: data.author,
            has_attachments: data.has_attachments,
        }
    }
}

fn get_reply(
    op_msg_id: &MsgId,
    container: &Container,
) -> Option<Reply> {
    container.get_data(op_msg_id).map(Reply::from_msg_data)
}

#[derive(Debug)]
pub(in crate::messages) enum OpChanged {
    Changed,
    NotChanged,
}
