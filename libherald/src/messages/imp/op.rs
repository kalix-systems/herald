use super::*;

impl Messages {
    pub(crate) fn op_body_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.op_body(index)
    }

    pub(crate) fn op_msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgId> {
        self.container.op_msg_id(index).map(MsgId::to_vec)
    }

    pub(crate) fn op_author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.container
            .op_author(index)
            .as_ref()
            .map(UserId::as_str)
            .map(str::to_string)
    }

    pub(crate) fn op_color_(
        &self,
        index: usize,
    ) -> Option<u32> {
        let uid = self.container.op_author(index)?;
        crate::users::shared::color(&uid)
    }

    pub(crate) fn op_name_(
        &self,
        index: usize,
    ) -> Option<String> {
        let uid = self.container.op_author(index)?;
        crate::users::shared::name(&uid)
    }

    pub(crate) fn op_doc_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.op_doc_attachments_json(index)
    }

    pub(crate) fn op_media_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.op_media_attachments_json(index)
    }

    pub(crate) fn op_insertion_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_insertion_time(index)?.into())
    }

    pub(crate) fn op_expiration_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.op_expiration_time(index)?.into())
    }

    pub(crate) fn reply_type_(
        &self,
        index: usize,
    ) -> Option<u8> {
        Some(self.container.op_reply_type(index)? as u8)
    }
}
