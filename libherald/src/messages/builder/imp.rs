use super::*;
use heraldcore::message::MsgData;

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
    ) -> OpChanged {
        let was_none = self.inner.op.is_none();

        if let Some(old) = self.inner.op {
            if *op_msg_id == old {
                return OpChanged::NotChanged;
            }
        }

        let reply = match get_reply(&op_msg_id) {
            Some(reply) => reply,
            None => return self.clear_reply_(),
        };

        self.inner.replying_to(Some(*op_msg_id));
        self.op.replace(reply);

        self.emit_op_changed();

        if was_none {
            self.emit.is_reply_changed();
        }

        OpChanged::Changed
    }

    pub(super) fn emit_op_changed(&mut self) {
        self.emit.op_id_changed();
        self.emit.op_body_changed();
        self.emit.op_author_changed();
        self.emit.op_time_changed();
        self.emit.op_doc_attachments_changed();
        self.emit.op_media_attachments_changed();
        self.emit.op_expiration_time_changed();
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
    pub(super) expiration: Option<Time>,
    pub(super) body: Option<MessageBody>,
    pub(super) author: UserId,
    pub(super) doc_attachments_json: Option<String>,
    pub(super) media_attachments_json: Option<String>,
}

impl Reply {
    pub(super) fn from_msg_data(data: &MsgData) -> Reply {
        let (doc_attachments_json, media_attachments_json) = match data.attachments() {
            Some(a) => (
                messages_helper::doc_attachments_json(a, Some(1)),
                messages_helper::media_attachments_json(a, Some(5)),
            ),
            None => (None, None),
        };

        Reply {
            time: data.time.insertion,
            expiration: data.time.expiration,
            body: data.content.body().map(Clone::clone),
            author: data.author,
            doc_attachments_json,
            media_attachments_json,
        }
    }

    pub(super) fn media(&self) -> &str {
        self.media_attachments_json
            .as_ref()
            .map(String::as_str)
            .unwrap_or("")
    }

    pub(super) fn doc(&self) -> &str {
        self.doc_attachments_json
            .as_ref()
            .map(String::as_str)
            .unwrap_or("")
    }
}

fn get_reply(op_msg_id: &MsgId) -> Option<Reply> {
    messages_helper::container::access(op_msg_id, Reply::from_msg_data)
}

#[derive(Debug)]
pub(in crate::messages) enum OpChanged {
    Changed,
    NotChanged,
}
