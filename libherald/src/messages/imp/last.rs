use super::*;

impl Messages {
    pub(crate) fn last_author_(&self) -> Option<ffi::UserIdRef> {
        let last = self.container.last_msg()?;

        if last.author == self.local_id? {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    pub(crate) fn last_aux_code_(&self) -> Option<u8> {
        self.container
            .aux_data_code_by_id(&self.container.list.front()?.msg_id)
    }

    pub(crate) fn last_body_(&self) -> Option<&str> {
        self.container.last_msg()?.text()
    }

    pub(crate) fn last_time_(&self) -> Option<i64> {
        Some(self.container.last_msg()?.time.insertion.into())
    }
}
