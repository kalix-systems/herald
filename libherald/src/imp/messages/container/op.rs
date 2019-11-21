use super::*;
use herald_common::Time;

impl Container {
    fn op(
        &self,
        index: usize,
    ) -> Option<&MsgData> {
        match self.msg_data(index)?.op {
            ReplyId::Known(mid) => self.get_data(&mid),
            _ => None,
        }
    }

    pub(in crate::imp::messages) fn op_reply_type(
        &self,
        index: usize,
    ) -> Option<ReplyType> {
        Some(reply_type(&self.msg_data(index)?.op))
    }

    pub(in crate::imp::messages) fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<&MsgId> {
        match &self.msg_data(index).as_ref()?.op {
            ReplyId::Known(mid) => Some(mid),
            _ => None,
        }
    }

    pub(in crate::imp::messages) fn op_author(
        &self,
        index: usize,
    ) -> Option<&UserId> {
        Some(&self.op(index)?.author)
    }

    pub(in crate::imp::messages) fn op_body(
        &self,
        index: usize,
    ) -> Option<&Option<MessageBody>> {
        Some(&self.op(index)?.body)
    }

    pub(in crate::imp::messages) fn op_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        Some(self.op(index)?.time.insertion)
    }

    pub(in crate::imp::messages) fn op_has_attachments(
        &self,
        index: usize,
    ) -> Option<bool> {
        Some(self.op(index)?.has_attachments)
    }
}
