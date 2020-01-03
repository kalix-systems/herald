use super::*;
use messages_helper::container::access;

impl Messages {
    fn last_mid(&self) -> Option<&MsgId> {
        Some(&self.container.list.front()?.msg_id)
    }

    pub(crate) fn last_author_(&self) -> Option<ffi::UserId> {
        access(self.last_mid()?, |data| data.author.to_string())
    }

    pub(crate) fn last_aux_code_(&self) -> Option<u8> {
        self.container.aux_data_code_by_id(self.last_mid()?)
    }

    pub(crate) fn last_has_attachments_(&self) -> Option<bool> {
        access(self.last_mid()?, |data| data.has_attachments())
    }

    pub(crate) fn last_body_(&self) -> Option<String> {
        access(self.last_mid()?, |data| -> Option<String> {
            Some(data.text()?.to_owned())
        })
        .flatten()
    }

    pub(crate) fn last_status_(&self) -> Option<u32> {
        access(self.last_mid()?, |data| data.receipts.clone())?
            .values()
            .max()
            .map(|status| *status as u32)
    }

    pub(crate) fn last_time_(&self) -> Option<i64> {
        access(self.last_mid()?, |data| *data.time.insertion.as_i64())
    }
}
