use super::*;

impl Container {
    pub fn op_reply_type(
        &self,
        index: usize,
    ) -> Option<ReplyType> {
        Some(reply_type(&cache::access(self.msg_id(index)?, |m| {
            *m.op()
        })?))
    }

    pub fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<MsgId> {
        match cache::access(self.msg_id(index)?, |m| *m.op())? {
            ReplyId::Known(mid) => Some(mid),
            _ => None,
        }
    }

    pub fn op_author(
        &self,
        index: usize,
    ) -> Option<UserId> {
        let mid = self.op_msg_id(index)?;
        access(&mid, |m| m.author)
    }

    pub fn op_body(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;

        access(&mid, |m| m.text().map(ToString::to_string))
            .flatten()
            .map(|b| self.op_body_elider.elided_body(&b))
    }

    pub fn op_insertion_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        let mid = self.op_msg_id(index)?;
        access(&mid, |m| m.time.insertion)
    }

    pub fn op_expiration_time(
        &self,
        index: usize,
    ) -> Option<Time> {
        let mid = self.op_msg_id(index)?;
        access(&mid, |m| m.time.expiration)?
    }

    pub fn op_doc_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;

        self.get_doc_attachments_data_json(&mid, Some(1))
    }

    pub fn op_media_attachments_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;
        self.get_media_attachments_data_json(&mid, Some(4))
    }

    pub fn op_aux_data_json(
        &self,
        index: usize,
    ) -> Option<String> {
        let mid = self.op_msg_id(index)?;
        self.aux_data_json_by_id(&mid)
    }
}
